use bevy::{
    app::{App, Startup},
    prelude::*,
    DefaultPlugins,
};

use crate::{render, setup};

pub fn build() -> App {
    let mut app = App::new();

    app.insert_resource(AmbientLight {
        brightness: 1000.0,
        ..default()
    })
    .add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Curves".into(),
            resolution: (800., 800.).into(),
            ..default()
        }),
        ..default()
    }),))
    .add_systems(Startup, setup::setup)
    .add_systems(Update, (render::handle_mouse, render::draw));
    app
}
