use bevy::{
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
    app.add_systems(
        Update,
        (
            start_drawing.run_if(input_just_pressed(MouseButton::Left)),
            draw.run_if(input_pressed(MouseButton::Left)),
            end_drawing.run_if(input_just_released(MouseButton::Left)),
        )
            .chain(),
    );
}

#[derive(Component)]
struct ActiveDrawing;

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

            if let Some(world_position) = window_to_world(camera, camera_transform, window_position)
            {
                commands.spawn((
                    ActiveDrawing,
                    camera_render_layers.clone(),
                    camera_interpolation_type.clone(),
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

fn end_drawing(mut commands: Commands, drawings: Query<Entity, With<ActiveDrawing>>) {
    for drawing in &drawings {
        commands.entity(drawing).remove::<ActiveDrawing>();
    }
}

fn draw(
    mut cursor: EventReader<CursorMoved>,
    drawing: Single<(&Mesh2d, &RenderLayers), With<ActiveDrawing>>,
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
