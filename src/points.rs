use std::cmp::Ordering;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, VertexAttributeValues},
        render_resource::{AsBindGroup, ShaderRef, ShaderType, VertexFormat},
    },
    sprite::{Material2d, Material2dPlugin},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<PointsMaterial>::default());
}

pub(crate) const ATTRIBUTE_TARGET_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("TargetPosition", 978541968, VertexFormat::Float32x3);

#[derive(Debug, Copy, Clone, Default, ShaderType)]
pub(crate) struct PointsSettings {
    pub color: LinearRgba,
    pub radius: f32,
    pub target_color: LinearRgba,
    pub target_radius: f32,
    pub t: f32,
}

impl PointsSettings {
    pub(crate) fn interpolated(&mut self, target: &Self) {
        self.target_color = target.color;
        self.target_radius = target.radius;
        self.t = 0.0;
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct PointsMaterial {
    #[uniform(0)]
    pub(crate) settings: PointsSettings,
}

impl Material2d for PointsMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/points.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/points.wgsl".into()
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

#[derive(Clone)]
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

impl From<Points> for Mesh {
    fn from(points: Points) -> Self {
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            points
                .0
                .iter()
                // Triple each vertex so we can construct triangles
                .flat_map(|p| {
                    let p = p.to_array();
                    [p, p, p]
                })
                .collect::<Vec<[f32; 3]>>(),
        )
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
