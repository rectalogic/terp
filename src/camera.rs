use bevy::{
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
    window::WindowResized,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_cameras)
        .add_systems(Update, update_camera_viewports);
}

#[derive(Component)]
pub enum CameraType {
    Source,
    Target,
}

fn setup_cameras(mut commands: Commands) {
    // Splitscreen cameras
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        RenderLayers::layer(1),
        CameraType::Source,
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        RenderLayers::layer(2),
        CameraType::Target,
    ));
}

fn update_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<(&CameraType, &mut Camera)>,
) {
    // Resize camera's viewports to split horizontal screen when window size changes.
    // A resize_event is also sent when the window is first created.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let size = UVec2::new(window.physical_width() / 2, window.physical_height());
        for (camera_type, mut camera) in &mut query {
            let x = match camera_type {
                CameraType::Source => 0,
                CameraType::Target => size.x,
            };
            camera.viewport = Some(Viewport {
                physical_position: UVec2::new(x, 0),
                physical_size: size,
                ..default()
            });
        }
    }
}
