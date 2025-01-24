use bevy::prelude::*;

mod animation;
pub mod app;
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
