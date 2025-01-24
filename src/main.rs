use bevy::prelude::*;
use terp::cli;

fn main() {
    App::new().add_plugins(cli::parse_cli()).run();
}
