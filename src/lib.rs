use bevy::{app::App, prelude::*, DefaultPlugins};

mod animate;
mod camera;
mod draw;
mod points;

#[derive(Component, Clone, PartialEq)]
enum InterpolationType {
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
            animate::plugin,
        ));
    }
}
