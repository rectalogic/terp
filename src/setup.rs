use bevy::{
    color::palettes::basic::{BLUE, RED},
    math::vec2,
    prelude::*,
};

use crate::linestrip2d::LineStrip2d;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);

    let line1 = LineStrip2d::new(vec![
        vec2(10., -20.),
        vec2(10., 20.),
        vec2(80., 80.),
        vec2(120., 30.),
    ]);
    let mesh1 = Mesh::from(&line1);
    commands.spawn((
        Name::new("line1"),
        line1,
        Mesh2d(meshes.add(mesh1)),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
    ));

    let line2 = LineStrip2d::new(vec![
        vec2(10., -30.),
        vec2(-10., -30.),
        vec2(-80., -90.),
        vec2(-120., -40.),
    ]);
    let mesh2 = Mesh::from(&line2);
    commands.spawn((
        Name::new("line2"),
        line2,
        Mesh2d(meshes.add(mesh2)),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BLUE))),
    ));
}
