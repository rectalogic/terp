use bevy::prelude::*;
use clap::{Arg, Command, ValueHint};
use std::path::PathBuf;

pub(super) fn plugin(app: &mut App) {
    let c = command().arg(
        Arg::new("project")
            .short('p')
            .long("project")
            .value_hint(ValueHint::FilePath),
    );
    app.insert_resource(Args::new(c));
}

pub(super) fn player_plugin(app: &mut App) {
    let c = command().arg(
        Arg::new("project")
            .required(true)
            .value_hint(ValueHint::FilePath),
    );
    //XXX clap should panic if not set?
    app.insert_resource(Args::new(c));
}

fn command() -> Command {
    Command::new(clap::crate_name!()).version(clap::crate_version!())
}

#[derive(Resource, Debug)]
pub struct Args {
    /// Project file
    project: Option<PathBuf>,
}

impl Args {
    fn new(command: Command) -> Self {
        if let Some(project) = command.get_matches().get_one::<String>("project") {
            Self {
                project: Some(project.into()),
            }
        } else {
            Self { project: None }
        }
    }
    pub fn project(&self) -> Option<&std::path::Path> {
        if let Some(ref project) = self.project {
            Some(project.as_path())
        } else {
            None
        }
    }
}
