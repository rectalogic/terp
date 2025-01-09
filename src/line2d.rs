use std::rc::Rc;

use bevy::{asset::RenderAssetUsages, math::Vec2, prelude::*};

#[derive(Clone)]
pub struct Line2d {
    points: Rc<Vec<Vec2>>,
}

impl Primitive2d for Line2d {}

impl Line2d {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self {
            points: Rc::new(points),
        }
    }

    pub fn points(self) -> Rc<Vec<Vec2>> {
        self.points.clone()
    }
}

impl From<Line2d> for SampleAutoCurve<Vec2> {
    fn from(line: Line2d) -> Self {
        SampleAutoCurve::new(
            Interval::new(0., line.points.len() as f32).unwrap(),
            Rc::into_inner(line.points).unwrap(),
        )
        .expect("should be good")
    }
}

impl From<(SampleAutoCurve<Vec2>, usize)> for Line2d {
    fn from((curve, segments): (SampleAutoCurve<Vec2>, usize)) -> Self {
        Self {
            points: Rc::new(
                curve
                    .sample_iter(curve.domain().spaced_points(segments).unwrap())
                    .flatten()
                    .collect(),
            ),
        }
    }
}

impl Meshable for Line2d {
    type Output = Line2dMeshBuilder;

    fn mesh(&self) -> Self::Output {
        Self::Output {
            points: Rc::clone(&self.points),
            segments: self.points.len(),
        }
    }
}

impl From<Line2d> for Mesh {
    fn from(value: Line2d) -> Self {
        value.mesh().into()
    }
}

pub struct Line2dMeshBuilder {
    points: Rc<Vec<Vec2>>,
    segments: usize,
}

pub trait Line2dBuilder {
    fn segments(self, segments: usize) -> Self;
}

impl Line2dBuilder for Line2dMeshBuilder {
    fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }
}

impl MeshBuilder for Line2dMeshBuilder {
    fn build(&self) -> Mesh {
        let points: Vec<Vec3> = self.points.iter().map(|p| Vec3::from((*p, 0.))).collect();
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)
    }
}
