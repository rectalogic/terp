use bevy::{prelude::*, window::PrimaryWindow};

use crate::points::{Points, PointsMaterial, PointsSettings};

#[derive(Component)]
pub struct Drawing;

fn window_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window_position: Vec2,
) -> Option<Vec2> {
    if let Ok(point) = camera.viewport_to_world_2d(camera_transform, window_position) {
        Some(point)
    } else {
        None
    }
}

pub fn start_drawing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    if let Some(window_position) = window.cursor_position() {
        let (camera, camera_transform) = *camera_query;
        if let Some(world_position) = window_to_world(camera, camera_transform, window_position) {
            commands.spawn((
                Drawing,
                Mesh2d(meshes.add(Mesh::from(Points(vec![world_position])))),
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

pub fn end_drawing(mut commands: Commands, drawings: Query<Entity, With<Drawing>>) {
    for drawing in &drawings {
        commands.entity(drawing).remove::<Drawing>();
    }
}

pub fn draw(
    mut cursor: EventReader<CursorMoved>,
    drawing: Single<&Mesh2d, With<Drawing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *camera_query;
    if let Some(mesh) = meshes.get_mut(*drawing) {
        for moved in cursor.read() {
            if let Some(world_position) = window_to_world(camera, camera_transform, moved.position)
            {
                Points::append(mesh, world_position);
            };
        }
    }
}
