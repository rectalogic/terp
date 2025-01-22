use bevy::prelude::*;
use clap::{Parser, ValueHint};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Args::parse());
}

#[derive(Resource, Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Project file
    #[arg(short, long, value_hint(ValueHint::FilePath))]
    project: Option<std::path::PathBuf>,
}

impl Args {
    pub fn project(&self) -> Option<&std::path::Path> {
        if let Some(ref project) = self.project {
            Some(project.as_path())
        } else {
            None
        }
    }
}
