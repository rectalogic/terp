use bevy::prelude::*;

mod brush_size;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera)
        .add_plugins(brush_size::plugin);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            ..default()
        },
    ));
}
