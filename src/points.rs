use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::VertexAttributeValues,
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
    },
    sprite::Material2d,
};

#[derive(Debug, Clone, Default, ShaderType)]
pub struct PointsSettings {
    pub color: LinearRgba,
    pub radius: f32,
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

    /*
    pub fn interpolate<'a>(
        &'a self,
        other: &'a Self,
        t: f32,
    ) -> impl Iterator<Item = Vec2> + use<'a> {
        let segments = max(self.segments, other.segments);
        zip(
            self.curve
                .sample_iter(self.curve.domain().spaced_points(segments).unwrap())
                .flatten(),
            other
                .curve
                .sample_iter(other.curve.domain().spaced_points(segments).unwrap())
                .flatten(),
        )
        .map(move |(v1, v2)| v1.lerp(v2, t))
    }
    */
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
