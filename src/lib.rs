use bevy::prelude::*;

mod animation;
pub mod app;
mod camera;
pub mod cli;
mod draw;
mod points;
pub mod project;
mod ui;
mod util;
#[cfg(target_arch = "wasm32")]
mod webgpu;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Idle,
    Draw,
    BrushSize,
    BrushColor,
}

#[derive(Component, Clone, PartialEq)]
#[require(Mesh2d, MeshMaterial2d<points::PointsMaterial>)]
enum Interpolated {
    Source,
    Target,
}
