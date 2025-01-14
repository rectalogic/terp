use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, VertexAttributeValues},
        render_resource::{AsBindGroup, ShaderRef, ShaderType, VertexFormat},
    },
    sprite::Material2d,
};

pub const ATTRIBUTE_TARGET_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("TargetPosition", 978541968, VertexFormat::Float32x3);

#[derive(Debug, Clone, Default, ShaderType)]
pub struct PointsSettings {
    pub color: LinearRgba,
    pub radius: f32,
    pub target_color: LinearRgba,
    pub target_radius: f32,
    pub t: f32,
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
pub struct Points(pub Vec<Vec2>);

impl Points {
    pub fn append(mesh: &mut Mesh, point: Vec2) {
        if let Some(VertexAttributeValues::Float32x3(ref mut positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            positions.reserve(3);
            let p = [point.x, point.y, 0.0];
            positions.push(p);
            positions.push(p);
            positions.push(p);
        }
    }

    // XXX add a combine() that takes 2 Mesh and makes one the target of the other by padding points

    // XXX should this handle triples?
    pub fn pad<T>(points: Vec<T>, target_len: usize) -> Vec<T>
    where
        T: Copy,
    {
        if points.is_empty() || target_len == 0 || points.len() >= target_len {
            return points;
        }

        let mut result = Vec::with_capacity(target_len);

        // Calculate how many times each element needs to be repeated on average
        let ratio = (target_len as f32) / (points.len() as f32);

        // For each target position, figure out which source element should be used
        for i in 0..target_len {
            // Convert to index
            let src_pos = ((i as f32 / ratio).floor() as usize).min(points.len() - 1);
            result.push(points[src_pos]);
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
                    let p = [p.x, p.y, 0.0];
                    [p, p, p]
                })
                .collect::<Vec<[f32; 3]>>(),
        )
    }
}
