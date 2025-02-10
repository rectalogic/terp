use bevy::{ecs::system::IntoObserverSystem, prelude::*, render::view::RenderLayers};

use crate::{AppState, Interpolated};

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
    let button_state_handler = |state: AppState| {
        move |mut trigger: Trigger<Pointer<Down>>, mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(state);
            trigger.propagate(false);
        }
    };

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
            parent
                .spawn((
                    Interpolated::Source,
                    CameraLayout,
                    Node {
                        border: UiRect::all(Val::Px(4.)),
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BorderColor(Srgba::rgb(0.4, 0.4, 0.4).into()),
                ))
                .observe(button_state_handler(AppState::Draw(Interpolated::Source)));

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
                    spawn_button(parent, "Color", button_state_handler(AppState::BrushColor));
                    spawn_button(parent, "Size", button_state_handler(AppState::BrushSize));
                });

            parent
                .spawn((
                    Interpolated::Target,
                    CameraLayout,
                    Node {
                        border: UiRect::all(Val::Px(4.)),
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BorderColor(Srgba::rgb(0.3, 0.3, 0.3).into()),
                ))
                .observe(button_state_handler(AppState::Draw(Interpolated::Target)));
        });
}

fn spawn_button<T, E, B, M>(
    parent: &mut ChildBuilder,
    label: T,
    observer: impl IntoObserverSystem<E, B, M>,
) where
    T: Into<String>,
    E: Event,
    B: Bundle,
{
    parent
        .spawn((
            Button,
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
        .observe(observer)
        .with_child((
            PickingBehavior::IGNORE,
            Text::new(label.into()),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
}
