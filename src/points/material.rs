use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::Material2d,
};
use serde::{Deserialize, Serialize};

use super::ATTRIBUTE_TARGET_POSITION;

#[derive(Debug, Copy, Clone, Default, ShaderType, Serialize, Deserialize)]
pub(crate) struct PointsSettings {
    pub(crate) color: LinearRgba,
    pub(crate) radius: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Copy, Clone)]
pub(crate) struct PointsMaterial {
    #[uniform(0)]
    pub(crate) source_settings: PointsSettings,
    #[uniform(1)]
    pub(crate) target_settings: PointsSettings,
    #[uniform(2)]
    pub(crate) t: f32,
}

const SHADER_PATH: &str = concat!(
    "embedded://",
    env!("CARGO_PKG_NAME"),
    "/points/shaders/points.wgsl"
);

impl Material2d for PointsMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::sprite::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        if layout.0.contains(ATTRIBUTE_TARGET_POSITION) {
            let vertex_layout = layout.0.get_layout(&[
                Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
                ATTRIBUTE_TARGET_POSITION.at_shader_location(1),
            ])?;
            descriptor.vertex.buffers = vec![vertex_layout];
            descriptor.vertex.shader_defs.push("INTERPOLATED".into());
        }
        Ok(())
    }
}
