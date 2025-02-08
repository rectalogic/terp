use bevy::prelude::*;

use crate::Interpolated;

mod brush_color;
mod brush_size;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_ui_camera, setup_ui))
        .add_plugins((brush_size::plugin, brush_color::plugin));
}

#[derive(Component)]
pub(super) struct CameraLayout;

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Camera {
            order: 2,
            ..default()
        },
    ));
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((Node {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|parent| {
            // Layout for cameras. Camera viewports track Nodes with CameraLayout
            parent.spawn((
                Interpolated::Source,
                CameraLayout,
                Node {
                    border: UiRect::all(Val::Px(4.)),
                    flex_grow: 1.0,
                    ..default()
                },
                BorderColor(Srgba::rgb(0.4, 0.4, 0.4).into()),
            ));
            parent.spawn((
                Interpolated::Target,
                CameraLayout,
                Node {
                    border: UiRect::all(Val::Px(4.)),
                    flex_grow: 1.0,
                    ..default()
                },
                BorderColor(Srgba::rgb(0.3, 0.3, 0.3).into()),
            ));
        });
}
