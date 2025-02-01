use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::{mesh::MeshVertexAttribute, render_resource::VertexFormat},
    sprite::Material2dPlugin,
};

pub(super) mod material;
pub(super) mod mesh;

pub(super) use material::{PointsMaterial, PointsSettings};
pub(super) use mesh::{Points, PointsMeshBuilder};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<PointsMaterial>::default());
    embedded_asset!(app, "points/shaders/points.wgsl")
}

pub(crate) const ATTRIBUTE_TARGET_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("TargetPosition", 978541968, VertexFormat::Float32x3);
