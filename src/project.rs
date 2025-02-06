use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::{
    animation::Animatable,
    cli, error_handler,
    points::{Points, PointsMaterial, PointsMeshBuilder, PointsSettings},
};
use anyhow::Result;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<LoadProjectData>()
        .add_event::<LoadProject>()
        .add_event::<SaveProjectData>()
        .add_systems(Startup, read_project.pipe(error_handler))
        .add_systems(
            Update,
            (
                load_project_data.pipe(error_handler),
                save_project
                    .pipe(error_handler)
                    .run_if(input_just_pressed(KeyCode::KeyS)),
                save_project_data.pipe(error_handler),
            ),
        );
}

pub(super) fn player_plugin(app: &mut App) {
    app.add_event::<LoadProjectData>()
        .add_event::<LoadProject>()
        .add_systems(Startup, read_project.pipe(error_handler))
        .add_systems(Update, load_project_data.pipe(error_handler));
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
    pub(crate) layer: f32,
}

#[derive(Event, Default, Debug)]
pub(crate) struct LoadProjectData(pub(crate) Vec<u8>);

#[derive(Event)]
pub(crate) struct LoadProject(pub(crate) Project);

#[derive(Event, Default, Debug)]
pub(crate) struct SaveProjectData(pub(crate) Vec<u8>);

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

fn load_project_data(
    mut events: EventReader<LoadProjectData>,
    mut commands: Commands,
) -> Result<()> {
    if let Some(event) = events.read().last() {
        let reader = flexbuffers::Reader::get_root(event.0.as_slice())?;
        let project = Project::deserialize(reader)?;
        commands.send_event(LoadProject(project));
    }
    Ok(())
}

fn save_project(
    mut commands: Commands,
    entities: Query<(&MeshMaterial2d<PointsMaterial>, &Mesh2d, &Transform), With<Animatable>>,
    materials: Res<Assets<PointsMaterial>>,
    meshes: Res<Assets<Mesh>>,
) -> Result<()> {
    let drawings: Vec<Drawing> = entities
        .iter()
        .filter_map(|(material2d, mesh2d, transform)| -> Option<Drawing> {
            let material = materials.get(material2d)?;
            let mesh = meshes.get(mesh2d)?;
            let (source_points, target_points) = mesh.to_points().ok()?;
            Some(Drawing {
                source_settings: material.source_settings,
                target_settings: material.target_settings,
                source_points,
                target_points,
                layer: transform.translation.z,
            })
        })
        .collect();
    let project = Project { drawings };
    let mut serializer = flexbuffers::FlexbufferSerializer::new();
    project.serialize(&mut serializer)?;
    commands.send_event(SaveProjectData(serializer.view().into()));
    Ok(())
}

fn save_project_data(args: Res<cli::Args>, mut events: EventReader<SaveProjectData>) -> Result<()> {
    let Some(path) = args.project() else {
        return Ok(());
    };
    if let Some(event) = events.read().last() {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(&event.0)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_project() {
        let mut app = App::new();
        app.init_resource::<Assets<PointsMaterial>>()
            .init_resource::<Assets<Mesh>>()
            .add_event::<SaveProjectData>()
            .add_systems(Update, save_project.pipe(error_handler));

        let mesh = Mesh::build_interpolated(
            &Points(vec![Vec2::new(0.0, 0.0)]),
            &Points(vec![Vec2::new(1.0, 1.0)]),
        )
        .unwrap();

        let material = PointsMaterial {
            source_settings: PointsSettings {
                radius: 25.0,
                ..default()
            },
            ..default()
        };

        let material_handle = app
            .world_mut()
            .resource_mut::<Assets<PointsMaterial>>()
            .add(material);
        let mesh_handle = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);
        app.world_mut().spawn((
            MeshMaterial2d(material_handle),
            Mesh2d(mesh_handle),
            Transform::from_xyz(0.0, 0.0, 1.0),
            Animatable,
        ));

        app.update();

        let events = app.world().resource::<Events<SaveProjectData>>();
        assert!(events.len() > 0);

        let mut event_cursor = events.get_cursor();
        let event = event_cursor.read(events).last().unwrap();
        let reader = flexbuffers::Reader::get_root(event.0.as_slice()).unwrap();
        let deserialized_project = Project::deserialize(reader).unwrap();

        assert_eq!(deserialized_project.drawings.len(), 1);
        let drawing = &deserialized_project.drawings[0];

        assert_eq!(drawing.source_points.0, vec![Vec2::new(0.0, 0.0)]);
        assert_eq!(drawing.target_points.0, vec![Vec2::new(1.0, 1.0)]);
        assert_eq!(drawing.layer, 1.0);
    }
}
