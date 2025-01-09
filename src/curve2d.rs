use std::{rc::Rc, sync::Arc};

use bevy::{asset::RenderAssetUsages, math::Vec2, prelude::*};

#[derive(Clone)]
pub struct Curve2d {
    curve: Rc<dyn Curve<Vec2>>,
}

impl Primitive2d for Curve2d {}

impl Curve2d {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self {
            curve: Rc::new(SampleAutoCurve::new(Interval::UNIT, points).expect("should be good")),
        }
    }
}

impl Meshable for Curve2d {
    type Output = Curve2dMeshBuilder;

    fn mesh(&self) -> Self::Output {
        Self::Output {
            curve: Rc::clone(&self.curve),
            segments: 32,
        }
    }
}

pub struct Curve2dMeshBuilder {
    curve: Rc<dyn Curve<Vec2>>,
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
        // https://github.com/bevyengine/bevy/blob/main/examples/3d/lines.rs

        let points: Vec<Vec3> = self
            .curve
            .sample_iter((0..=self.segments).map(|n| n as f32 / self.segments as f32))
            .filter_map(|p| p.map(|p| Vec3::from((p, 0.))))
            .collect();
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)
    }
}
