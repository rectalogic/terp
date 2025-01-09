use bevy::{color::palettes::basic::RED, math::vec2, prelude::*};

use crate::linestrip2d::LineStrip2d;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(&LineStrip2d::new(vec![
            vec2(10., 10.),
            vec2(10., 20.),
            vec2(20., 20.),
            vec2(40., 30.),
        ])))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
    ));
}
