use bevy::{prelude::*, render::view::RenderLayers, window::PrimaryWindow};

use crate::points::{Points, PointsMaterial, PointsSettings};

#[derive(Component)]
pub struct Drawing;

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

pub fn start_drawing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &RenderLayers, &GlobalTransform)>,
) {
    if let Some(window_position) = window.cursor_position() {
        for (camera, camera_render_layers, camera_transform) in &camera_query {
            if !camera
                .logical_viewport_rect()
                .unwrap()
                .contains(window_position)
            {
                continue;
            }

            if let Some(world_position) = window_to_world(camera, camera_transform, window_position)
            {
                commands.spawn((
                    Drawing,
                    camera_render_layers.clone(),
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
}

pub fn end_drawing(mut commands: Commands, drawings: Query<Entity, With<Drawing>>) {
    for drawing in &drawings {
        commands.entity(drawing).remove::<Drawing>();
    }
}

pub fn draw(
    mut cursor: EventReader<CursorMoved>,
    drawing: Single<(&Mesh2d, &RenderLayers), With<Drawing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera_query: Query<(&Camera, &RenderLayers, &GlobalTransform)>,
) {
    let (mesh2d, drawing_render_layers) = *drawing;
    if let Some(mesh) = meshes.get_mut(mesh2d) {
        for (camera, camera_render_layers, camera_transform) in &camera_query {
            if !camera_render_layers.intersects(drawing_render_layers) {
                continue;
            }

            for moved in cursor.read() {
                if let Some(world_position) =
                    window_to_world(camera, camera_transform, moved.position)
                {
                    Points::append(mesh, world_position);
                };
            }
        }
    }
}
