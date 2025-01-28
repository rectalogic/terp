use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use anyhow::Result;
use bevy::{input::common_conditions::input_just_pressed, prelude::*, render::view::RenderLayers};
use serde::{Deserialize, Serialize};

use crate::{
    animation::Animatable,
    camera::{SOURCE_LAYER, TARGET_LAYER},
    cli,
    points::{Points, PointsMaterial, PointsPair, PointsSettings},
    Interpolated,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<LoadProject>()
        .add_systems(Startup, read_project.pipe(error_handler))
        .add_systems(
            Update,
            (
                load_project.pipe(error_handler),
                save_project
                    .pipe(error_handler)
                    .run_if(input_just_pressed(KeyCode::KeyS)),
            ),
        );
}

pub(super) fn player_plugin(app: &mut App) {
    app.add_event::<LoadProject>()
        .add_systems(Startup, read_project.pipe(error_handler))
        .add_systems(Update, load_project.pipe(error_handler));
}

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

#[derive(Event)]
pub struct LoadProject(Vec<u8>);

impl LoadProject {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

fn read_project(args: Res<cli::Args>, mut commands: Commands) -> Result<()> {
    let Some(path) = args.project() else {
        return Ok(());
    };
    if !path.exists() {
        return Ok(());
    }
    commands.send_event(LoadProject::new(fs::read(path)?));
    Ok(())
}

fn load_project(
    mut events: EventReader<LoadProject>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut points_materials: ResMut<Assets<PointsMaterial>>,
) -> Result<()> {
    for event in events.read() {
        let reader = flexbuffers::Reader::get_root(event.0.as_slice())?;
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
    }
    Ok(())
}

fn save_project(
    args: Res<cli::Args>,
    entities: Query<(&MeshMaterial2d<PointsMaterial>, &Mesh2d), With<Animatable>>,
    materials: Res<Assets<PointsMaterial>>,
    meshes: Res<Assets<Mesh>>,
) -> Result<()> {
    let Some(path) = args.project() else {
        return Ok(());
    };

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

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        println!("Failed to save/load project {:?}", err);
    }
}
