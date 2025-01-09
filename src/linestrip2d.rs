use bevy::{asset::RenderAssetUsages, math::Vec2, prelude::*};

#[derive(Clone)]
pub struct LineStrip2d {
    points: Vec<Vec2>,
}

impl LineStrip2d {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }

    pub fn points(self) -> Vec<Vec2> {
        self.points
    }
}

impl From<LineStrip2d> for SampleAutoCurve<Vec2> {
    fn from(line: LineStrip2d) -> Self {
        SampleAutoCurve::new(
            Interval::new(0., line.points.len() as f32).unwrap(),
            line.points,
        )
        .expect("should be good")
    }
}

impl From<(SampleAutoCurve<Vec2>, usize)> for LineStrip2d {
    fn from((curve, segments): (SampleAutoCurve<Vec2>, usize)) -> Self {
        Self {
            points: curve
                .sample_iter(curve.domain().spaced_points(segments).unwrap())
                .flatten()
                .collect(),
        }
    }
}

impl From<&LineStrip2d> for Mesh {
    fn from(line: &LineStrip2d) -> Self {
        let points: Vec<Vec3> = line.points.iter().map(|p| Vec3::from((*p, 0.))).collect();
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)
    }
}
