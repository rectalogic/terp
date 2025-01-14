use bevy::{
    app::{App, Startup},
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    sprite::Material2dPlugin,
    DefaultPlugins,
};

use crate::{camera, points::PointsMaterial, render, setup};

pub fn build() -> App {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Terp".into(),
                resolution: (800., 400.).into(),
                ..default()
            }),
            ..default()
        }),
        Material2dPlugin::<PointsMaterial>::default(),
    ))
    .add_systems(Startup, setup::setup)
    .add_systems(
        Update,
        (
            camera::update_camera_viewports,
            (
                render::start_drawing.run_if(input_just_pressed(MouseButton::Left)),
                render::draw.run_if(input_pressed(MouseButton::Left)),
                render::end_drawing.run_if(input_just_released(MouseButton::Left)),
            )
                .chain(),
        ),
    );
    app
}
