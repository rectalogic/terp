use std::rc::Rc;

use bevy::{asset::RenderAssetUsages, math::Vec2, prelude::*};

#[derive(Clone)]
pub struct Curve2d {
    curve: Rc<SampleAutoCurve<Vec2>>,
}

impl Primitive2d for Curve2d {}

impl Curve2d {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self {
            curve: Rc::new(
                SampleAutoCurve::new(Interval::new(0., points.len() as f32).unwrap(), points)
                    .expect("should be good"),
            ),
        }
    }

    pub fn curve(self) -> Rc<SampleAutoCurve<Vec2>> {
        self.curve.clone()
    }
}

impl Meshable for Curve2d {
    type Output = Curve2dMeshBuilder;

    fn mesh(&self) -> Self::Output {
        Self::Output {
            curve: Rc::clone(&self.curve),
            segments: self.curve.domain().length() as usize,
        }
    }
}

impl From<Curve2d> for Mesh {
    fn from(value: Curve2d) -> Self {
        value.mesh().into()
    }
}

pub struct Curve2dMeshBuilder {
    curve: Rc<SampleAutoCurve<Vec2>>,
    segments: usize,
}

pub trait Curve2dBuilder {
    fn segments(self, segments: usize) -> Self;
}

impl Curve2dBuilder for Curve2dMeshBuilder {
    fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }
}

impl MeshBuilder for Curve2dMeshBuilder {
    fn build(&self) -> Mesh {
        let points: Vec<Vec3> = self
            .curve
            .sample_iter(self.curve.domain().spaced_points(self.segments).unwrap())
            .filter_map(|p| p.map(|p| Vec3::from((p, 0.))))
            .collect();
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)
    }
}
