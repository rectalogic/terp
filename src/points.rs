use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::Material2d,
};

#[derive(Debug, Clone, Default, ShaderType)]
pub struct PointsSettings {
    pub color: LinearRgba,
    pub point_radius: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PointsMaterial {
    #[uniform(0)]
    pub settings: PointsSettings,
}

impl Material2d for PointsMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/points.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/points.wgsl".into()
    }
}
