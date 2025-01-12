use bevy::{
    asset::RenderAssetUsages,
    color::palettes::basic::{BLUE, RED},
    math::vec2,
    prelude::*,
    render::mesh::Indices,
    sprite::Material2d,
};

use crate::{
    linestrip2d::LineStrip2d,
    points::{PointsMaterial, PointsSettings},
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
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
        MeshMaterial2d(color_materials.add(ColorMaterial::from_color(RED))),
    ));
    /*
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
    */

    let vertices = vec![
        Vec3::new(10., -20., -1.),
        Vec3::new(10., -20., -1.),
        Vec3::new(10., -20., -1.),
        Vec3::new(10., 20., -1.),
        Vec3::new(10., 20., -1.),
        Vec3::new(10., 20., -1.),
        Vec3::new(80., 80., -1.),
        Vec3::new(80., 80., -1.),
        Vec3::new(80., 80., -1.),
        Vec3::new(120., 30., -1.),
        Vec3::new(120., 30., -1.),
        Vec3::new(120., 30., -1.),
    ];

    let points = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(), //XXX RENDER_WORLD?
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    /*
        let vertices = vec![
            Vec3::new(0.0, 0.5, 0.0),   // top center
            Vec3::new(-0.5, -0.5, 0.0), // bottom left
            Vec3::new(0.5, -0.5, 0.0),  // bottom right
            Vec3::new(0.5, 0.5, 0.0),   // top right
        ];
        let indices = Indices::U32(vec![0, 1, 2, 0, 3, 2]);
        let mut points = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        points.insert_indices(indices);
    */

    commands.spawn((
        // Mesh2d(meshes.add(points)),
        Mesh2d(meshes.add(points)),
        MeshMaterial2d(materials.add(PointsMaterial {
            settings: PointsSettings {
                color: LinearRgba::BLUE,
                radius: 10.,
            },
        })),
        Transform::default(),
    ));
}
