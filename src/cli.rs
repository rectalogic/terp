use crate::app;
use bevy::prelude::*;
use clap::{Arg, Command, ValueHint};
use std::path::Path;

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
            app::AppPlugin::Editor(Args::new(editor_matches.get_one::<String>("project")))
        }
        Some(("player", player_matches)) => {
            app::AppPlugin::Player(Args::new(player_matches.get_one::<String>("project")))
        }
        None => app::AppPlugin::Editor(Args::new::<String>(None)),
        _ => unreachable!("All commands covered"),
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct Args {
    /// Project file
    project: Option<String>,
}

impl Args {
    pub fn new<I: Into<String>>(project: Option<I>) -> Self {
        Self {
            project: project.map(|p| p.into()),
        }
    }

    pub fn project(&self) -> Option<&Path> {
        if let Some(ref project) = self.project {
            Some(Path::new(project))
        } else {
            None
        }
    }
}
