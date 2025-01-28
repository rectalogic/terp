use crate::app;
use bevy::prelude::*;
use clap::{Arg, Command, ValueHint};
use std::path::PathBuf;

pub fn parse_cli() -> app::AppPlugin {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .propagate_version(true)
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("editor").arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .value_hint(ValueHint::FilePath),
            ),
        )
        .subcommand(
            Command::new("player").arg(
                Arg::new("project")
                    .required(true)
                    .value_hint(ValueHint::FilePath),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("editor", editor_matches)) => {
            app::AppPlugin::Editor(Args::from(editor_matches.get_one::<String>("project")))
        }
        Some(("player", player_matches)) => {
            app::AppPlugin::Player(Args::from(player_matches.get_one::<String>("project")))
        }
        None => app::AppPlugin::Editor(Args::new(None)),
        _ => unreachable!("All commands covered"),
    }
}

#[derive(Resource, Clone, Debug)]
pub struct Args {
    /// Project file
    project: Option<PathBuf>,
}

impl Args {
    pub fn new(project: Option<&str>) -> Self {
        Self {
            project: project.map(|p| p.into()),
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

impl From<&str> for Args {
    fn from(value: &str) -> Self {
        Self {
            project: Some(value.into()),
        }
    }
}

impl From<&String> for Args {
    fn from(value: &String) -> Self {
        Self {
            project: Some(value.into()),
        }
    }
}

impl From<Option<&str>> for Args {
    fn from(value: Option<&str>) -> Self {
        Self {
            project: value.map(|v| v.into()),
        }
    }
}

impl From<Option<&String>> for Args {
    fn from(value: Option<&String>) -> Self {
        Self {
            project: value.map(|v| v.into()),
        }
    }
}
