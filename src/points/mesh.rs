use std::cmp::Ordering;

use super::ATTRIBUTE_TARGET_POSITION;
use anyhow::{anyhow, Result};
use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::VertexAttributeValues};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Points(pub Vec<Vec2>);

impl Points {
    pub(crate) fn append(mesh: &mut Mesh, point: Vec2) {
        if let Some(VertexAttributeValues::Float32x3(ref mut positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            positions.reserve(3);
            let p = Vec3::from((point, 0.)).to_array();
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
                    let p = Vec3::from((*p, 0.)).to_array();
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
                    .iter()
                    .step_by(3)
                    .map(|p| Vec2::from_slice(&p[0..2]))
                    .collect(),
            )),
            _ => Err("Unsupported vertex type"),
        }
    }
}

pub(crate) trait PointsMeshBuilder {
    fn empty() -> Mesh;
    fn build(points: Option<&Points>) -> Mesh;
    fn build_interpolated<T>(source: T, target: T) -> Result<Mesh>
    where
        T: Into<VertexAttributeValues>;
    fn to_points(&self) -> Result<(Points, Points), &'static str>;
}

impl PointsMeshBuilder for Mesh {
    fn empty() -> Mesh {
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
    }

    fn build(points: Option<&Points>) -> Mesh {
        let mut mesh = Mesh::empty();
        if let Some(points) = points {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        } else {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float32x3(Vec::new()),
            );
        }
        mesh
    }

    fn build_interpolated<T>(source: T, target: T) -> Result<Mesh>
    where
        T: Into<VertexAttributeValues>,
    {
        let source: VertexAttributeValues = source.into();
        let target: VertexAttributeValues = target.into();
        if source.len() != target.len() {
            return Err(anyhow!(
                "source and target have different number of vertices"
            ));
        }
        Ok(Mesh::empty()
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, source)
            .with_inserted_attribute(ATTRIBUTE_TARGET_POSITION, target))
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
    fn test_points_append() {
        let mut mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vec![]),
        );
        Points::append(&mut mesh, Vec2::new(1.0, 2.0));
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        if let VertexAttributeValues::Float32x3(positions) = positions {
            assert_eq!(
                *positions,
                vec![[1.0, 2.0, 0.0], [1.0, 2.0, 0.0], [1.0, 2.0, 0.0]]
            );
        }
    }

    #[test]
    fn test_points_conversion() {
        let points = Points(vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]);
        let vertex_attrs: VertexAttributeValues = (&points).into();
        if let VertexAttributeValues::Float32x3(positions) = vertex_attrs.clone() {
            assert_eq!(positions.len(), 6);
            assert_eq!(positions[0], [1.0, 2.0, 0.0]);
            assert_eq!(positions[3], [3.0, 4.0, 0.0]);
        }

        let points_back = Points::try_from(&vertex_attrs).unwrap();
        assert_eq!(points_back.0.len(), 2);
        assert_eq!(points_back.0[0], Vec2::new(1.0, 2.0));
        assert_eq!(points_back.0[1], Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_mesh_builder() {
        let points = Points(vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]);
        let mesh = Mesh::build(Some(&points));
        let result = mesh.to_points();
        assert!(result.is_err());
    }

    #[test]
    fn test_mesh_builder_interpolated() {
        let source_points = Points(vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]);
        let target_points = Points(vec![Vec2::new(5.0, 6.0), Vec2::new(7.0, 8.0)]);

        let mesh = Mesh::build_interpolated(&source_points, &target_points).unwrap();
        let (source, target) = mesh.to_points().unwrap();

        assert_eq!(source.0, vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]);
        assert_eq!(target.0, vec![Vec2::new(5.0, 6.0), Vec2::new(7.0, 8.0)]);
    }

    #[test]
    fn test_padding() {
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
