use anyhow::Result;
use bevy::prelude::*;
mod animation;
mod app;
mod camera;
pub mod cli;
mod draw;
mod points;
mod project;
mod ui;
mod util;
#[cfg(target_arch = "wasm32")]
mod webgpu;

#[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Idle,
    Draw(Interpolated),
    BrushSize,
    BrushColor,
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[require(Mesh2d, MeshMaterial2d<points::PointsMaterial>)]
enum Interpolated {
    Source,
    Target,
}

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        println!("Failed to save/load project {:?}", err);
    }
}
