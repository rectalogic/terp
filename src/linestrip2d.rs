use bevy::{asset::RenderAssetUsages, math::Vec2, prelude::*};
use std::{cmp::max, iter::zip};

#[derive(Clone, Component)]
pub struct LineStrip2d {
    curve: SampleAutoCurve<Vec2>,
    segments: usize,
}

impl LineStrip2d {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self {
            segments: points.len(),
            curve: SampleAutoCurve::new(Interval::new(0., points.len() as f32).unwrap(), points)
                .expect("should be good"),
        }
    }

    // XXX need to implement an iterator, it should spaced_points each curve over it's domain and interpolate each Vec2
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
}

impl From<&LineStrip2d> for Mesh {
    fn from(line: &LineStrip2d) -> Self {
        let points: Vec<Vec3> = line
            .curve
            .sample_iter(line.curve.domain().spaced_points(line.segments).unwrap())
            .flatten()
            .map(|p| Vec3::from((p, 0.)))
            .collect();
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)
    }
}
