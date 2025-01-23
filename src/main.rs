use bevy::prelude::*;
use terp::EditorPlugin;

fn main() {
    App::new().add_plugins(EditorPlugin).run();
}
