use bevy::prelude::*;
use terp::PlayerPlugin;

fn main() {
    App::new().add_plugins(PlayerPlugin).run();
}
