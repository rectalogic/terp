use bevy::{
    ecs::query::QueryEntityError,
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    render::view::RenderLayers,
    window::PrimaryWindow,
};

use crate::{
    points::{Points, PointsMaterial, PointsSettings},
    InterpolationType,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(DrawingCount::default()).add_systems(
        Update,
        (
            start_drawing.run_if(input_just_pressed(MouseButton::Left)),
            draw.run_if(input_pressed(MouseButton::Left)),
            end_drawing.run_if(input_just_released(MouseButton::Left)),
        )
            .chain(),
    );
}

#[derive(Resource, Default)]
struct DrawingCount {
    source: usize,
    target: usize,
}

#[derive(Component)]
struct ActiveDrawing;

#[derive(Component)]
struct DrawingNumber {
    count: usize,
}

fn window_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window_position: Vec2,
) -> Option<Vec2> {
    if let Some(viewport) = camera.logical_viewport_rect() {
        if let Ok(point) =
            camera.viewport_to_world_2d(camera_transform, window_position - viewport.min)
        {
            return Some(point);
        }
    }
    None
}

fn start_drawing(
    mut commands: Commands,
    mut drawing_count: ResMut<DrawingCount>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &RenderLayers, &GlobalTransform, &InterpolationType)>,
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
                InterpolationType::Source => {
                    drawing_count.source += 1;
                    drawing_count.source
                }

                InterpolationType::Target => {
                    drawing_count.target += 1;
                    drawing_count.target
                }
            };

            if let Some(world_position) = window_to_world(camera, camera_transform, window_position)
            {
                commands.spawn((
                    ActiveDrawing,
                    DrawingNumber { count },
                    camera_render_layers.clone(),
                    camera_interpolation_type.clone(),
                    Mesh2d(meshes.add(Mesh::from(Points(vec![Vec3::from((
                        world_position,
                        count as f32, // use count as Z index
                    ))])))),
                    MeshMaterial2d(materials.add(PointsMaterial {
                        settings: PointsSettings {
                            color: LinearRgba::rgb(
                                window_position.x / window.width(),
                                window_position.y / window.height(),
                                1.0,
                            ),
                            radius: 20.,
                            target_color: LinearRgba::rgb(1., 1., 1.),
                            target_radius: 5.,
                            t: 0.5,
                            //XXX need a Z layer here, increment for each new drawing (to prevent Z-fighting)
                        },
                    })),
                ));
            }
        }
    }
}

fn end_drawing(
    mut commands: Commands,
    active_drawing: Single<(Entity, &DrawingNumber, &InterpolationType), With<ActiveDrawing>>,
    unmerged_drawings: Query<(Entity, &DrawingNumber, &InterpolationType), Without<ActiveDrawing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
    mesh_query: Query<(&Mesh2d, &MeshMaterial2d<PointsMaterial>), With<DrawingNumber>>,
) {
    let (active_drawing, active_drawing_number, active_interpolation_type) = *active_drawing;
    commands.entity(active_drawing).remove::<ActiveDrawing>();
    // Try to find a drawing of the opposite interpolation with the same number
    for (unmerged_drawing, unmerged_drawing_number, unmerged_interpolation_type) in
        &unmerged_drawings
    {
        if unmerged_drawing_number.count == active_drawing_number.count
            && *unmerged_interpolation_type != *active_interpolation_type
        {
            let (source_entity, target_entity) = match *active_interpolation_type {
                InterpolationType::Source => (active_drawing, unmerged_drawing),
                InterpolationType::Target => (unmerged_drawing, active_drawing),
            };

            let mut process_mesh_material = |result: Result<
                (&Mesh2d, &MeshMaterial2d<PointsMaterial>),
                QueryEntityError<'_>,
            >| {
                result.ok().and_then(|(mesh2d, material2d)| {
                    meshes.remove(mesh2d).and_then(|mesh| {
                        materials
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

            source_material
                .settings
                .interpolated(&target_material.settings);
            target_material.settings = source_material.settings;
            target_material.settings.t = 1.0;

            commands
                .entity(target_entity)
                .insert((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(materials.add(target_material)),
                ))
                .remove::<DrawingNumber>();
            commands
                .entity(source_entity)
                .insert((
                    Mesh2d(mesh_handle),
                    MeshMaterial2d(materials.add(source_material)),
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
                    window_to_world(camera, camera_transform, moved.position)
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
