use bevy::{app::App, prelude::*, DefaultPlugins};
use cli::Args;

mod animation;
mod camera;
pub mod cli;
mod draw;
mod points;
mod project;
mod ui;
mod util;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Idle,
    Draw,
    BrushSize,
    BrushColor,
}

#[derive(Resource, Copy, Clone)]
struct Brush {
    radius: f32,
    color: Hsva,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            radius: 10.,
            color: Hsva::WHITE,
        }
    }
}

#[derive(Component, Clone, PartialEq)]
#[require(Mesh2d, MeshMaterial2d<points::PointsMaterial>)]
enum Interpolated {
    Source,
    Target,
}

pub enum AppPlugin {
    Editor(Args),
    Player(Args),
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        match self {
            AppPlugin::Editor(args) => {
                app.add_plugins((
                    DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Terp".into(),
                            resolution: (1200., 600.).into(),
                            ..default()
                        }),
                        ..default()
                    }),
                    camera::plugin,
                    points::plugin,
                    draw::plugin,
                    animation::plugin,
                    ui::plugin,
                    project::plugin,
                ))
                .insert_resource(Brush::default())
                .insert_state(AppState::Idle)
                .insert_resource(args.clone());
            }
            AppPlugin::Player(args) => {
                app.add_plugins((
                    DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Terp Player".into(),
                            resolution: (600., 600.).into(),
                            ..default()
                        }),
                        ..default()
                    }),
                    camera::player_plugin,
                    points::plugin,
                    animation::player_plugin,
                    project::player_plugin,
                ))
                .insert_state(AppState::Idle)
                .insert_resource(args.clone());
            }
        }
    }
}
