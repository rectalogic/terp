use bevy::{app::App, prelude::*, DefaultPlugins};

use crate::{animation, camera, cli, draw, points, project, ui, AppState};

pub enum AppPlugin {
    Editor(cli::Args),
    Player(cli::Args),
}

impl AppPlugin {
    pub fn run(self) -> AppExit {
        App::new().add_plugins(self).run()
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn run_with_event(self, event: impl Event) -> AppExit {
        let mut app = App::new();
        app.add_plugins(self);
        app.world_mut().send_event(event);
        app.run()
    }
}

fn title_suffix(title: &str, args: &cli::Args) -> String {
    if let Some(project) = args.project() {
        format!("{} - {}", title, project.display())
    } else {
        title.into()
    }
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let title = match self {
            AppPlugin::Editor(args) => title_suffix("Terp", &args),
            AppPlugin::Player(args) => title_suffix("Terp Player", &args),
        };

        match self {
            AppPlugin::Editor(args) => {
                app.add_plugins((
                    DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(Window {
                            title,
                            resolution: (1200., 600.).into(),
                            ..default()
                        }),
                        ..default()
                    }),
                    camera::plugin,
                    points::plugin,
                    draw::plugin,
                    animation::plugin,
                    ui::plugin,
                    project::plugin,
                ))
                .insert_state(AppState::Idle)
                .insert_resource(args.clone());
            }
            AppPlugin::Player(args) => {
                app.add_plugins((
                    DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(Window {
                            title,
                            resolution: (600., 600.).into(),
                            ..default()
                        }),
                        ..default()
                    }),
                    camera::player_plugin,
                    points::plugin,
                    draw::player_plugin,
                    animation::player_plugin,
                    project::player_plugin,
                ))
                .insert_state(AppState::Idle)
                .insert_resource(args.clone());
            }
        }
    }
}
