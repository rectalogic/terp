use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use anyhow::Result;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    animation::Animatable,
    cli,
    points::{Points, PointsMaterial, PointsMeshBuilder, PointsSettings},
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<LoadProjectData>()
        .add_event::<LoadProject>()
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
    app.add_event::<LoadProjectData>()
        .add_event::<LoadProject>()
        .add_systems(Startup, read_project.pipe(error_handler))
        .add_systems(Update, load_project.pipe(error_handler));
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Project {
    pub(crate) drawings: Vec<Drawing>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Drawing {
    pub(crate) source_settings: PointsSettings,
    pub(crate) target_settings: PointsSettings,
    pub(crate) source_points: Points,
    pub(crate) target_points: Points,
}

#[derive(Event)]
struct LoadProjectData(Vec<u8>);

#[derive(Event)]
pub(crate) struct LoadProject(pub(crate) Project);

fn read_project(args: Res<cli::Args>, mut commands: Commands) -> Result<()> {
    let Some(path) = args.project() else {
        return Ok(());
    };
    if !path.exists() {
        return Ok(());
    }
    commands.send_event(LoadProjectData(fs::read(path)?));
    Ok(())
}

fn load_project(mut events: EventReader<LoadProjectData>, mut commands: Commands) -> Result<()> {
    if let Some(event) = events.read().last() {
        let reader = flexbuffers::Reader::get_root(event.0.as_slice())?;
        let project = Project::deserialize(reader)?;
        commands.send_event(LoadProject(project));
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
            let (source_points, target_points) = mesh.to_points().ok()?;
            Some(Drawing {
                source_settings: material.source_settings,
                target_settings: material.target_settings,
                source_points,
                target_points,
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
