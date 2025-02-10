use bevy::{prelude::*, render::view::RenderLayers};

use crate::Interpolated;

mod brush_color;
mod brush_size;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_ui_camera, setup_ui))
        .add_plugins((brush_size::plugin, brush_color::plugin));
}

#[derive(Component)]
pub(super) struct CameraLayout;

#[derive(Component)]
pub(super) struct ControlsCamera;

#[derive(Component)]
pub(super) struct BrushColorButton;

#[derive(Component)]
pub(super) struct BrushSizeButton;

// Brush color/size controls need to layer on top of UI
pub(super) const CONTROLS_LAYER: RenderLayers = RenderLayers::layer(3);

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Camera {
            order: 2,
            ..default()
        },
    ));
    commands.spawn((
        Camera2d,
        CONTROLS_LAYER,
        ControlsCamera,
        Camera {
            order: 3,
            ..default()
        },
    ));
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|parent| {
            // Layout for cameras. Camera viewports track Nodes with CameraLayout
            parent.spawn((
                Interpolated::Source,
                CameraLayout,
                Node {
                    border: UiRect::all(Val::Px(4.)),
                    flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    ..default()
                },
                BorderColor(Srgba::rgb(0.4, 0.4, 0.4).into()),
            ));

            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(100.0),
                    height: Val::Percent(100.0),
                    flex_shrink: 0.0,
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, BrushColorButton, "Color");
                    spawn_button(parent, BrushSizeButton, "Size");
                });

            parent.spawn((
                Interpolated::Target,
                CameraLayout,
                Node {
                    border: UiRect::all(Val::Px(4.)),
                    flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    ..default()
                },
                BorderColor(Srgba::rgb(0.3, 0.3, 0.3).into()),
            ));
        });
}

fn spawn_button<T>(parent: &mut ChildBuilder, component: impl Component, label: T)
where
    T: Into<String>,
{
    parent
        .spawn((
            Button,
            component,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
        ))
        .with_child((
            Text::new(label.into()),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
}
