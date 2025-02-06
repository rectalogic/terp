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

#[derive(Asset, TypePath, AsBindGroup, Debug, Default, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_properties() {
        let material = PointsMaterial {
            source_settings: PointsSettings {
                color: LinearRgba::default(),
                radius: 1.0,
            },
            target_settings: PointsSettings {
                color: LinearRgba::default(),
                radius: 2.0,
            },
            t: 0.5,
        };

        assert_eq!(material.t, 0.5);
        assert_eq!(material.source_settings.radius, 1.0);
        assert_eq!(material.target_settings.radius, 2.0);
    }

    #[test]
    fn test_points_settings_default() {
        let settings = PointsSettings::default();
        assert_eq!(settings.color, LinearRgba::default());
        assert_eq!(settings.radius, 0.0);
    }

    #[test]
    fn test_points_settings_clone() {
        let settings = PointsSettings {
            color: LinearRgba::default(),
            radius: 1.0,
        };
        let cloned = settings.clone();

        assert_eq!(settings.color, cloned.color);
        assert_eq!(settings.radius, cloned.radius);
    }
}
