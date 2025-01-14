use crate::camera::CameraType;
use bevy::{prelude::*, render::view::RenderLayers};

pub fn setup(mut commands: Commands) {
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
