use crate::{
    AppState, Interpolated,
    animation::Animatable,
    camera::{SOURCE_LAYER, TARGET_LAYER},
    error_handler,
    points::{Points, PointsMaterial, PointsMeshBuilder, PointsSettings},
    project::LoadProject,
    util::window_position_to_world,
};
use anyhow::Result;
use bevy::{
    ecs::query::{QueryData, QueryEntityError},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Brush::default())
        .insert_resource(Undo::default())
        .insert_resource(DrawingCount::default())
        .add_event::<UndoEvent>()
        .add_systems(OnEnter(AppState::Draw(Interpolated::Source)), start_drawing)
        .add_systems(OnEnter(AppState::Draw(Interpolated::Target)), start_drawing)
        .add_systems(OnExit(AppState::Draw(Interpolated::Source)), end_drawing)
        .add_systems(OnExit(AppState::Draw(Interpolated::Target)), end_drawing)
        .add_systems(Update, draw.run_if(draw_condition))
        .add_systems(
            Update,
            (
                load_project.pipe(error_handler),
                undo_drawing.run_if(in_state(AppState::Idle)),
            ),
        );
}

pub(super) fn player_plugin(app: &mut App) {
    app.insert_resource(Undo::default())
        .insert_resource(DrawingCount::default())
        .add_systems(Update, load_project.pipe(error_handler));
}

#[derive(Event, Default, Debug)]
pub(super) struct UndoEvent;

#[derive(Resource, Copy, Clone)]
pub(crate) struct Brush {
    pub(crate) radius: f32,
    pub(crate) color: Hsva,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            radius: 10.,
            color: Hsva::WHITE,
        }
    }
}

#[derive(Resource, Default)]
struct DrawingCount {
    source: usize,
    target: usize,
}

#[derive(Resource, Default)]
struct Undo {
    entities: Vec<Entity>,
}

impl Undo {
    fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
    fn undo(&mut self) -> Option<Entity> {
        self.entities.pop()
    }
    pub fn iter_mut(&mut self) -> UndoIter {
        UndoIter { undo: self }
    }
}

impl Iterator for Undo {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.undo()
    }
}

struct UndoIter<'a> {
    undo: &'a mut Undo,
}

impl Iterator for UndoIter<'_> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.undo.undo()
    }
}

#[derive(Component)]
struct ActiveDrawing;

#[derive(Component)]
struct MergedDrawing(Entity);

#[derive(Component)]
struct DrawingNumber(usize);

fn draw_condition(state: Res<State<AppState>>, buttons: Res<ButtonInput<MouseButton>>) -> bool {
    match state.get() {
        AppState::Draw(_) => buttons.pressed(MouseButton::Left),
        _ => false,
    }
}

#[allow(clippy::too_many_arguments)]
fn start_drawing(
    mut commands: Commands,
    state: Res<State<AppState>>,
    mut drawing_count: ResMut<DrawingCount>,
    brush: Res<Brush>,
    mut undo: ResMut<Undo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
) {
    let interpolation_type = match state.get() {
        AppState::Draw(interpolated) => interpolated,
        _ => return,
    };

    let (count, render_layers) = match interpolation_type {
        Interpolated::Source => {
            drawing_count.source += 1;
            (drawing_count.source, SOURCE_LAYER)
        }

        Interpolated::Target => {
            drawing_count.target += 1;
            (drawing_count.target, TARGET_LAYER)
        }
    };

    let entity = commands
        .spawn((
            ActiveDrawing,
            DrawingNumber(count),
            render_layers,
            *interpolation_type,
            Mesh2d(meshes.add(Mesh::build(None))),
            Transform::from_xyz(0., 0., count as f32), // use count as Z index
            MeshMaterial2d(materials.add(PointsMaterial {
                source_settings: PointsSettings {
                    color: brush.color.into(),
                    radius: brush.radius,
                },
                target_settings: PointsSettings {
                    color: brush.color.into(),
                    radius: brush.radius,
                },
                t: 0.0,
            })),
        ))
        .id();

    undo.add(entity);
}

#[derive(QueryData)]
struct DrawingQuery {
    entity: Entity,
    number: &'static DrawingNumber,
    interpolation: &'static Interpolated,
}

fn end_drawing(
    mut commands: Commands,
    active_drawing: Single<DrawingQuery, With<ActiveDrawing>>,
    unmerged_drawings: Query<DrawingQuery, (Without<ActiveDrawing>, Without<MergedDrawing>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut points_materials: ResMut<Assets<PointsMaterial>>,
    mesh_query: Query<(&Mesh2d, &MeshMaterial2d<PointsMaterial>), Without<MergedDrawing>>,
) {
    let active_drawing = active_drawing.into_inner();
    commands
        .entity(active_drawing.entity)
        .remove::<ActiveDrawing>();
    // Try to find a drawing of the opposite interpolation with the same number
    for unmerged_drawing in &unmerged_drawings {
        if unmerged_drawing.number.0 == active_drawing.number.0
            && unmerged_drawing.interpolation != active_drawing.interpolation
        {
            let (source_entity, target_entity) = match *active_drawing.interpolation {
                Interpolated::Source => (active_drawing.entity, unmerged_drawing.entity),
                Interpolated::Target => (unmerged_drawing.entity, active_drawing.entity),
            };

            let mut process_mesh_material = |result: Result<
                (&Mesh2d, &MeshMaterial2d<PointsMaterial>),
                QueryEntityError<'_>,
            >| {
                result.ok().and_then(|(mesh2d, material2d)| {
                    meshes.remove(mesh2d).and_then(|mesh| {
                        points_materials
                            .remove(material2d)
                            .map(|material| (mesh, material))
                    })
                })
            };

            let Some((mut source_mesh, mut source_material)) =
                process_mesh_material(mesh_query.get(source_entity))
            else {
                return;
            };
            let Some((target_mesh, mut target_material)) =
                process_mesh_material(mesh_query.get(target_entity))
            else {
                return;
            };

            Points::interpolate(&mut source_mesh, &target_mesh);
            let mesh_handle = meshes.add(source_mesh);

            source_material.target_settings = target_material.source_settings;
            source_material.t = 0.0;
            target_material = source_material;
            target_material.t = 1.0;

            commands.entity(target_entity).insert((
                MergedDrawing(source_entity),
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(points_materials.add(target_material)),
            ));
            commands.entity(source_entity).insert((
                Animatable,
                MergedDrawing(target_entity),
                Mesh2d(mesh_handle),
                MeshMaterial2d(points_materials.add(source_material)),
            ));
        }
    }
}

fn draw(
    mut cursor: EventReader<CursorMoved>,
    drawing: Single<&Mesh2d, With<ActiveDrawing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_query: Query<(&Camera, &Interpolated, &GlobalTransform)>,
    state: Res<State<AppState>>,
) {
    let interpolation_type = match state.get() {
        AppState::Draw(interpolated) => interpolated,
        _ => return,
    };

    let mesh2d = *drawing;
    if let Some(mesh) = meshes.get_mut(mesh2d) {
        for (camera, camera_interpolation_type, camera_transform) in &camera_query {
            if camera_interpolation_type != interpolation_type {
                continue;
            }

            for moved in cursor.read() {
                if let Some(world_position) =
                    window_position_to_world(camera, camera_transform, moved.position)
                {
                    Points::append(mesh, world_position);
                };
            }
        }
    }
}

fn undo_drawing(
    mut commands: Commands,
    mut undo: ResMut<Undo>,
    mut drawing_count: ResMut<DrawingCount>,
    drawings: Query<(DrawingQuery, Option<&MergedDrawing>)>,
    mut events: EventReader<UndoEvent>,
) {
    for _ in events.read() {
        if let Some(entity) = undo.undo() {
            if let Ok((drawing, merged_drawing)) = drawings.get(entity) {
                if let Some(merged_drawing) = merged_drawing {
                    commands
                        .entity(merged_drawing.0)
                        .remove::<(Animatable, MergedDrawing)>();
                };
                match drawing.interpolation {
                    Interpolated::Source => drawing_count.source = drawing.number.0 - 1,
                    Interpolated::Target => drawing_count.target = drawing.number.0 - 1,
                };
                commands.entity(entity).despawn();
            }
        }
    }
}

fn load_project(
    mut events: EventReader<LoadProject>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut points_materials: ResMut<Assets<PointsMaterial>>,
    undo: ResMut<Undo>,
    mut drawing_count: ResMut<DrawingCount>,
) -> Result<()> {
    if let Some(LoadProject(project)) = events.read().last() {
        let undo = undo.into_inner();
        // Clear current project
        for entity in undo.iter_mut() {
            commands.entity(entity).despawn();
        }

        drawing_count.source = 0;
        drawing_count.target = 0;

        for drawing in project.drawings.iter() {
            drawing_count.source += 1;
            drawing_count.target += 1;

            let mesh_handle = meshes.add(Mesh::build_interpolated(
                &drawing.source_points,
                &drawing.target_points,
            )?);

            let target_entity = commands
                .spawn((
                    Interpolated::Target,
                    DrawingNumber(drawing_count.target),
                    TARGET_LAYER,
                    Transform::from_xyz(0., 0., drawing.layer),
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(points_materials.add(PointsMaterial {
                        source_settings: drawing.source_settings,
                        target_settings: drawing.target_settings,
                        t: 1.0,
                    })),
                ))
                .id();

            let source_entity = commands
                .spawn((
                    Animatable,
                    Interpolated::Source,
                    DrawingNumber(drawing_count.source),
                    SOURCE_LAYER,
                    Transform::from_xyz(0., 0., drawing.layer),
                    Mesh2d(mesh_handle),
                    MeshMaterial2d(points_materials.add(PointsMaterial {
                        source_settings: drawing.source_settings,
                        target_settings: drawing.target_settings,
                        t: 0.0,
                    })),
                ))
                .id();

            commands
                .entity(target_entity)
                .insert(MergedDrawing(source_entity));
            commands
                .entity(source_entity)
                .insert(MergedDrawing(target_entity));

            undo.add(source_entity);
            undo.add(target_entity);
        }
    }
    Ok(())
}
