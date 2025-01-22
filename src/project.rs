use std::{fs::OpenOptions, io::Write, path::Path};

use anyhow::Result;
use bevy::{prelude::*, render::view::RenderLayers};
use serde::{Deserialize, Serialize};

use crate::{
    animation::Animatable,
    camera::{SOURCE_LAYER, TARGET_LAYER},
    points::{Points, PointsMaterial, PointsPair, PointsSettings},
    Interpolated,
};

#[derive(Serialize, Deserialize)]
struct Project {
    drawings: Vec<Drawing>,
}

#[derive(Serialize, Deserialize)]
struct Drawing {
    source_settings: PointsSettings,
    target_settings: PointsSettings,
    source_points: Vec<Vec3>,
    target_points: Vec<Vec3>,
}

pub(crate) fn load_project(
    data: &[u8],
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut points_materials: ResMut<Assets<PointsMaterial>>,
) -> Result<()> {
    let reader = flexbuffers::Reader::get_root(data)?;
    let project = Project::deserialize(reader)?;
    for drawing in project.drawings {
        let mesh_handle = meshes.add(Mesh::from(PointsPair(
            Points(drawing.source_points),
            Points(drawing.target_points),
        )));
        commands.spawn((
            Interpolated::Target,
            RenderLayers::layer(TARGET_LAYER),
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(points_materials.add(PointsMaterial {
                source_settings: drawing.source_settings,
                target_settings: drawing.target_settings,
                t: 1.0,
            })),
        ));
        commands.spawn((
            Animatable,
            Interpolated::Source,
            RenderLayers::layer(SOURCE_LAYER),
            Mesh2d(mesh_handle),
            MeshMaterial2d(points_materials.add(PointsMaterial {
                source_settings: drawing.source_settings,
                target_settings: drawing.target_settings,
                t: 0.0,
            })),
        ));
    }
    Ok(())
}

pub(crate) fn save_project(
    path: &Path,
    entities: Query<(&MeshMaterial2d<PointsMaterial>, &Mesh2d), With<Animatable>>,
    materials: Res<Assets<PointsMaterial>>,
    meshes: Res<Assets<Mesh>>,
) -> Result<()> {
    let drawings: Vec<Drawing> = entities
        .iter()
        .filter_map(|(material2d, mesh2d)| -> Option<Drawing> {
            let material = materials.get(material2d)?;
            let mesh = meshes.get(mesh2d)?;
            let PointsPair(source_points, target_points) = PointsPair::try_from(mesh).ok()?;
            Some(Drawing {
                source_settings: material.source_settings,
                target_settings: material.target_settings,
                source_points: source_points.0,
                target_points: target_points.0,
            })
        })
        .collect();
    let project = Project { drawings };
    let mut serializer = flexbuffers::FlexbufferSerializer::new();
    project.serialize(&mut serializer)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(serializer.view())?;
    Ok(())
}
