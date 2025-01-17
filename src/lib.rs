use bevy::{app::App, prelude::*, DefaultPlugins};

mod animation;
mod camera;
mod draw;
mod points;
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
    color: Hsla,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            radius: 10.,
            color: Hsla::WHITE,
        }
    }
}

#[derive(Component, Clone, PartialEq)]
#[require(Mesh2d, MeshMaterial2d<points::PointsMaterial>)]
enum Interpolated {
    Source,
    Target,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Terp".into(),
                    resolution: (800., 400.).into(),
                    ..default()
                }),
                ..default()
            }),
            camera::plugin,
            points::plugin,
            draw::plugin,
            animation::plugin,
            ui::plugin,
        ))
        .insert_resource(Brush::default())
        .insert_state(AppState::Idle);
    }
}
