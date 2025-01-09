use bevy::{color::palettes::basic::RED, math::vec2, prelude::*};

use crate::line2d::{Line2d, Line2dBuilder};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(
            Line2d::new(vec![
                vec2(10., 10.),
                vec2(10., 20.),
                vec2(20., 20.),
                vec2(40., 30.),
            ]), // .mesh()
                // .segments(100),
        )),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
    ));
}
