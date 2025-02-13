use crate::{draw::UndoEvent, AppState, Interpolated};
use bevy::{asset::embedded_asset, prelude::*, render::view::RenderLayers};

mod brush_color;
mod brush_size;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_ui_camera, setup_ui))
        .add_systems(Update, active_color_handler)
        .add_plugins((brush_size::plugin, brush_color::plugin));

    embedded_asset!(app, "ui/images/color-wheel.png");
    embedded_asset!(app, "ui/images/resize.png");
    embedded_asset!(app, "ui/images/undo.png");
}

const INACTIVE_COLOR: Color = Color::Srgba(Srgba::rgb(0.4, 0.4, 0.4));
const ACTIVE_COLOR: Color = Color::Srgba(Srgba::rgb(0.7, 0.7, 0.0));

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

fn button_state_handler<E>(state: AppState) -> impl FnMut(Trigger<E>, ResMut<NextState<AppState>>)
where
    E: Event,
{
    move |mut trigger: Trigger<E>, mut next_state: ResMut<NextState<AppState>>| {
        next_state.set(state);
        trigger.propagate(false);
    }
}

fn active_color_handler(
    mut transitions: EventReader<StateTransitionEvent<AppState>>,
    mut borders: Query<(&mut BorderColor, &Interpolated), With<CameraLayout>>,
) {
    let Some(StateTransitionEvent {
        exited: Some(exited),
        entered: Some(entered),
    }) = transitions.read().last()
    else {
        return;
    };
    if exited == entered {
        return;
    }

    if let AppState::Draw(_) = exited {
        for (mut border_color, _) in borders.iter_mut() {
            border_color.0 = INACTIVE_COLOR;
        }
    }

    if let AppState::Draw(interpolated) = entered {
        for (mut border_color, border_interpolated) in borders.iter_mut() {
            if border_interpolated == interpolated {
                border_color.0 = ACTIVE_COLOR;
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .observe(button_state_handler::<Pointer<Down>>(AppState::Idle))
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
                    BorderColor(INACTIVE_COLOR),
                ))
                .observe(button_state_handler::<Pointer<Down>>(AppState::Draw(
                    Interpolated::Source,
                )));

            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(85.0),
                    height: Val::Percent(100.0),
                    flex_shrink: 0.0,
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(
                        parent,
                        asset_server.load(concat!(
                            "embedded://",
                            env!("CARGO_PKG_NAME"),
                            "/ui/images/color-wheel.png"
                        )),
                    )
                    .observe(button_state_handler::<Pointer<Down>>(AppState::BrushColor))
                    .observe(button_state_handler::<Pointer<DragEnd>>(AppState::Idle));
                    spawn_button(
                        parent,
                        asset_server.load(concat!(
                            "embedded://",
                            env!("CARGO_PKG_NAME"),
                            "/ui/images/resize.png"
                        )),
                    )
                    .observe(button_state_handler::<Pointer<Down>>(AppState::BrushSize))
                    .observe(button_state_handler::<Pointer<DragEnd>>(AppState::Idle));
                    spawn_button(
                        parent,
                        asset_server.load(concat!(
                            "embedded://",
                            env!("CARGO_PKG_NAME"),
                            "/ui/images/undo.png"
                        )),
                    )
                    .observe(
                        move |mut trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                            commands.send_event(UndoEvent);
                            trigger.propagate(false);
                        },
                    );
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
                    BorderColor(INACTIVE_COLOR),
                ))
                .observe(button_state_handler::<Pointer<Down>>(AppState::Draw(
                    Interpolated::Target,
                )));
        });
}

fn spawn_button<'a>(parent: &'a mut ChildBuilder<'_>, image: Handle<Image>) -> EntityCommands<'a> {
    let mut commands = parent.spawn((
        Button,
        Node {
            padding: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::MAX,
        BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
    ));
    commands.with_child(ImageNode::new(image));
    commands
}
