use bevy::{
    DefaultPlugins,
    app::App,
    prelude::*,
    winit::{WakeUp, WinitPlugin},
};

#[cfg(target_arch = "wasm32")]
use crate::webgpu;
use crate::{
    AppState, animation, camera, cli, draw, points,
    project::{self, LoadProjectData},
    ui,
};

pub enum AppPlugin {
    Editor(cli::Args),
    Player(cli::Args),
}

impl AppPlugin {
    pub fn run(self) -> AppExit {
        App::new().add_plugins(self).run()
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn app(self) -> App {
        let mut app = App::new();
        app.add_plugins(self);
        app
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
        let (title, rez) = match self {
            AppPlugin::Editor(args) => (title_suffix("Terp", args), (1200., 600.)),
            AppPlugin::Player(args) => (title_suffix("Terp Player", args), (600., 600.)),
        };
        let default_plugins = DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title,
                    resolution: rez.into(),
                    ..default()
                }),
                ..default()
            })
            .build()
            .disable::<WinitPlugin<WakeUp>>()
            .add(WinitPlugin::<LoadProjectData>::default());

        match self {
            AppPlugin::Editor(args) => {
                app.add_plugins((
                    default_plugins,
                    camera::plugin,
                    points::plugin,
                    draw::plugin,
                    animation::plugin,
                    ui::plugin,
                    project::plugin,
                    #[cfg(target_arch = "wasm32")]
                    webgpu::plugin,
                ))
                .insert_state(AppState::Idle)
                .insert_resource(args.clone());
            }
            AppPlugin::Player(args) => {
                app.add_plugins((
                    default_plugins,
                    camera::player_plugin,
                    points::plugin,
                    draw::player_plugin,
                    animation::player_plugin,
                    project::player_plugin,
                    #[cfg(target_arch = "wasm32")]
                    webgpu::plugin,
                ))
                .insert_state(AppState::Idle)
                .insert_resource(args.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_suffix() {
        let args = cli::Args::default();
        assert_eq!(title_suffix("Test", &args), "Test");

        let args_with_project = cli::Args::new(Some("example.terp"));
        assert_eq!(
            title_suffix("Test", &args_with_project),
            "Test - example.terp"
        );
    }
}
