use std::cmp::Ordering;

use bevy::{
    asset::{embedded_asset, RenderAssetUsages},
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, VertexAttributeValues},
        render_resource::{AsBindGroup, ShaderRef, ShaderType, VertexFormat},
    },
    sprite::{Material2d, Material2dPlugin},
};
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<PointsMaterial>::default());
    embedded_asset!(app, "shaders/points.wgsl")
}

pub(crate) const ATTRIBUTE_TARGET_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("TargetPosition", 978541968, VertexFormat::Float32x3);

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
    "/shaders/points.wgsl"
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

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Points(pub Vec<Vec3>);

impl Points {
    pub(crate) fn append(mesh: &mut Mesh, point: Vec3) {
        if let Some(VertexAttributeValues::Float32x3(ref mut positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            positions.reserve(3);
            let p = point.to_array();
            positions.push(p);
            positions.push(p);
            positions.push(p);
        }
    }

    // Merge target into source interpolated
    pub(crate) fn interpolate(source: &mut Mesh, target: &Mesh) {
        let Some(VertexAttributeValues::Float32x3(ref mut source_positions)) =
            source.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        else {
            return;
        };
        let Some(VertexAttributeValues::Float32x3(ref target_positions)) =
            target.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            return;
        };
        let source_len = source_positions.len();
        let target_len = target_positions.len();
        match source_len.cmp(&target_len) {
            Ordering::Greater => {
                source.insert_attribute(
                    ATTRIBUTE_TARGET_POSITION,
                    Self::pad_positions(target_positions, source_len),
                );
            }
            Ordering::Less => {
                *source_positions = Self::pad_positions(source_positions, target_len);
                source.insert_attribute(ATTRIBUTE_TARGET_POSITION, target_positions.clone());
            }
            Ordering::Equal => {
                source.insert_attribute(ATTRIBUTE_TARGET_POSITION, target_positions.clone());
            }
        }
    }

    // Pad positions to match target_len, both lengths must be divisible by 3
    fn pad_positions<T>(positions: &[T], target_len: usize) -> Vec<T>
    where
        T: Copy,
    {
        const TRIPLE: usize = 3;

        if positions.is_empty() || target_len == 0 || positions.len() >= target_len {
            panic!("Positions is empty or too long");
        }
        if positions.len() % TRIPLE != 0 || target_len % TRIPLE != 0 {
            panic!("Positions must be triples");
        }

        let mut result = Vec::with_capacity(target_len);

        // Calculate how many times each position needs to be repeated on average
        let ratio = (target_len as f32) / (positions.len() as f32);
        let positions_triples_len = positions.len() / TRIPLE;

        // For each target position, figure out which source position should be used
        for i in 0..target_len / 3 {
            // Convert to index
            let src_pos =
                ((i as f32 / ratio).floor() as usize).min(positions_triples_len - 1) * TRIPLE;
            result.extend(&positions[src_pos..(src_pos + TRIPLE)]);
        }

        result
    }
}

impl From<&Points> for VertexAttributeValues {
    fn from(points: &Points) -> Self {
        VertexAttributeValues::Float32x3(
            points
                .0
                .iter()
                // Triple each vertex so we can construct triangles
                .flat_map(|p| {
                    let p = p.to_array();
                    [p, p, p]
                })
                .collect(),
        )
    }
}

impl TryFrom<&VertexAttributeValues> for Points {
    type Error = &'static str;

    fn try_from(vertices: &VertexAttributeValues) -> Result<Self, Self::Error> {
        match vertices {
            VertexAttributeValues::Float32x3(points) => Ok(Points(
                points
                    .into_iter()
                    .step_by(3)
                    .map(|p| Vec3::from(*p))
                    .collect(),
            )),
            _ => Err("Unsupported vertex type"),
        }
    }
}

pub(super) trait PointsMeshBuilder {
    fn build(points: &Points) -> Mesh;
    fn build_interpolated<T>(source: T, target: T) -> Mesh
    where
        T: Into<VertexAttributeValues>;
    fn to_points(&self) -> Result<(Points, Points), &'static str>;
}

impl PointsMeshBuilder for Mesh {
    fn build(points: &Points) -> Mesh {
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)
    }

    fn build_interpolated<T>(source: T, target: T) -> Mesh
    where
        T: Into<VertexAttributeValues>,
    {
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, source)
        .with_inserted_attribute(ATTRIBUTE_TARGET_POSITION, target)
    }

    fn to_points(&self) -> Result<(Points, Points), &'static str> {
        Ok((
            Points::try_from(
                self.attribute(Mesh::ATTRIBUTE_POSITION)
                    .ok_or("No position attribute")?,
            )?,
            Points::try_from(
                self.attribute(ATTRIBUTE_TARGET_POSITION)
                    .ok_or("No target position attribute")?,
            )?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding() {
        assert_eq!(
            Points::pad_positions(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4], 7 * 3),
            vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4,]
        );
        assert_eq!(
            Points::pad_positions(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4], 14 * 3),
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3,
                3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4
            ]
        );
    }
}
