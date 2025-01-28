use bevy::{
    ecs::query::{QueryData, QueryEntityError},
    input::common_conditions::{input_just_pressed, input_just_released},
    prelude::*,
    render::view::RenderLayers,
    window::PrimaryWindow,
};

use crate::{
    animation::Animatable,
    points::{Points, PointsMaterial, PointsSettings},
    util::window_position_to_world,
    AppState, Interpolated,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Brush::default())
        .insert_resource(Undo::default())
        .insert_resource(DrawingCount::default())
        .add_systems(
            Update,
            (
                undo_drawing
                    .run_if(input_just_pressed(KeyCode::Backspace))
                    .run_if(in_state(AppState::Idle)),
                (
                    start_drawing
                        .run_if(in_state(AppState::Idle))
                        .run_if(run_if_start_drawing),
                    (
                        draw.run_if(run_if_draw),
                        end_drawing.run_if(input_just_released(MouseButton::Left)),
                    )
                        .run_if(in_state(AppState::Draw))
                        .chain(),
                )
                    .chain(),
            ),
        );
}

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
}

#[derive(Component)]
struct ActiveDrawing;

#[derive(Component)]
struct DrawingNumber {
    count: usize,
}

fn run_if_start_drawing(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) -> bool {
    // Left button with no modifiers (since they are used for brush configuration)
    buttons.just_pressed(MouseButton::Left)
        && !keys.any_pressed([
            KeyCode::ShiftLeft,
            KeyCode::ShiftRight,
            KeyCode::ControlLeft,
            KeyCode::ControlRight,
        ])
}

fn run_if_draw(buttons: Res<ButtonInput<MouseButton>>, keys: Res<ButtonInput<KeyCode>>) -> bool {
    // Discontiguous drawing, hold Alt (Option) to move without drawing
    buttons.pressed(MouseButton::Left) && !keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight])
}

#[allow(clippy::too_many_arguments)]
fn start_drawing(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut drawing_count: ResMut<DrawingCount>,
    brush: Res<Brush>,
    mut undo: ResMut<Undo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &RenderLayers, &GlobalTransform, &Interpolated)>,
) {
    if let Some(window_position) = window.cursor_position() {
        for (camera, camera_render_layers, camera_transform, camera_interpolation_type) in
            &camera_query
        {
            if !camera
                .logical_viewport_rect()
                .unwrap()
                .contains(window_position)
            {
                continue;
            }

            let count = match camera_interpolation_type {
                Interpolated::Source => {
                    drawing_count.source += 1;
                    drawing_count.source
                }

                Interpolated::Target => {
                    drawing_count.target += 1;
                    drawing_count.target
                }
            };

            if let Some(world_position) =
                window_position_to_world(camera, camera_transform, window_position)
            {
                let entity = commands
                    .spawn((
                        ActiveDrawing,
                        DrawingNumber { count },
                        camera_render_layers.clone(),
                        camera_interpolation_type.clone(),
                        Mesh2d(meshes.add(Mesh::from(Points(vec![Vec3::from((
                            world_position,
                            count as f32, // use count as Z index
                        ))])))),
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

                next_state.set(AppState::Draw);
            }
        }
    }
}

#[derive(QueryData)]
struct DrawingQuery {
    entity: Entity,
    number: &'static DrawingNumber,
    interpolation: &'static Interpolated,
}

fn end_drawing(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    active_drawing: Single<DrawingQuery, With<ActiveDrawing>>,
    unmerged_drawings: Query<DrawingQuery, Without<ActiveDrawing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut points_materials: ResMut<Assets<PointsMaterial>>,
    mesh_query: Query<(&Mesh2d, &MeshMaterial2d<PointsMaterial>), With<DrawingNumber>>,
) {
    next_state.set(AppState::Idle);

    let active_drawing = active_drawing.into_inner();
    commands
        .entity(active_drawing.entity)
        .remove::<ActiveDrawing>();
    // Try to find a drawing of the opposite interpolation with the same number
    for unmerged_drawing in &unmerged_drawings {
        if unmerged_drawing.number.count == active_drawing.number.count
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

            commands
                .entity(target_entity)
                .insert((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(points_materials.add(target_material)),
                ))
                .remove::<DrawingNumber>();
            commands
                .entity(source_entity)
                .insert((
                    Animatable,
                    Mesh2d(mesh_handle),
                    MeshMaterial2d(points_materials.add(source_material)),
                ))
                .remove::<DrawingNumber>();
        }
    }
}

fn draw(
    mut cursor: EventReader<CursorMoved>,
    drawing: Single<(&Mesh2d, &RenderLayers, &DrawingNumber), With<ActiveDrawing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_query: Query<(&Camera, &RenderLayers, &GlobalTransform)>,
) {
    let (mesh2d, drawing_render_layers, drawing_number) = *drawing;
    if let Some(mesh) = meshes.get_mut(mesh2d) {
        for (camera, camera_render_layers, camera_transform) in &camera_query {
            if !camera_render_layers.intersects(drawing_render_layers) {
                continue;
            }

            for moved in cursor.read() {
                if let Some(world_position) =
                    window_position_to_world(camera, camera_transform, moved.position)
                {
                    Points::append(
                        mesh,
                        Vec3::from((world_position, drawing_number.count as f32)),
                    );
                };
            }
        }
    }
}

fn undo_drawing(mut commands: Commands, mut undo: ResMut<Undo>) {
    if let Some(entity) = undo.undo() {
        commands.entity(entity).despawn();
    }
}
