use bevy::{
    ecs::query::QueryData,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};

use crate::{ui::CameraLayout, Interpolated};

pub(crate) const SOURCE_LAYER: RenderLayers = RenderLayers::layer(1);
pub(crate) const TARGET_LAYER: RenderLayers = RenderLayers::layer(2);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_cameras)
        // Need to update cameras after UiLayout which is in PostUpdate
        .add_systems(Last, update_camera_viewports);
}

pub(super) fn player_plugin(app: &mut App) {
    app.add_systems(Startup, setup_player_camera);
}

fn setup_player_camera(mut commands: Commands) {
    commands.spawn((Camera2d, SOURCE_LAYER, Interpolated::Source));
}

fn setup_cameras(mut commands: Commands) {
    // Splitscreen cameras
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        SOURCE_LAYER,
        Interpolated::Source,
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        TARGET_LAYER,
        Interpolated::Target,
    ));
}

#[derive(QueryData)]
struct LayoutQuery {
    interpolated: &'static Interpolated,
    node: &'static ComputedNode,
    global_transform: &'static GlobalTransform,
}

#[derive(QueryData)]
#[query_data(mutable)]
struct CameraQuery {
    interpolated: &'static Interpolated,
    camera: &'static mut Camera,
}

fn update_camera_viewports(
    layout_query: Query<
        LayoutQuery,
        (
            With<CameraLayout>,
            Or<(Changed<ComputedNode>, Changed<GlobalTransform>)>,
        ),
    >,
    mut camera_query: Query<CameraQuery, With<Interpolated>>,
) {
    // Resize camera's viewports to match their transformed layout size.
    for mut camera in camera_query.iter_mut() {
        for layout in layout_query.iter() {
            if layout.node.is_empty() || camera.interpolated != layout.interpolated {
                continue;
            }
            let inset = layout.node.content_inset();
            let size = layout.node.size();
            let size = Vec2::new(
                size.x - (inset.left + inset.right),
                size.y - (inset.top + inset.bottom),
            );
            // https://github.com/bevyengine/bevy/blob/5d0e9cfb36b2baab15e4c8a62bc40f77b5db1a88/crates/bevy_ui/src/focus.rs#L245
            let node_rect =
                Rect::from_center_size(layout.global_transform.translation().truncate(), size);
            camera.camera.viewport = Some(Viewport {
                physical_position: UVec2::new(node_rect.min.x as u32, node_rect.min.y as u32),
                physical_size: UVec2::new(size.x as u32, size.y as u32),
                ..default()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_ui(world: &mut World) {
        world
            .spawn((Node {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::percent(50.0), GridTrack::percent(50.0)],
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn((Interpolated::Source, CameraLayout, Node::default()));
                parent.spawn((Interpolated::Target, CameraLayout, Node::default()));
            });
    }

    #[test]
    fn test_player_camera_creation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_player_camera);
        app.update();

        let mut query = app
            .world_mut()
            .query::<(&Camera2d, &RenderLayers, &Interpolated)>();
        let results: Vec<_> = query.iter(&app.world()).collect();

        assert_eq!(results.len(), 1);
        assert_eq!(*results[0].1, SOURCE_LAYER);
        assert_eq!(*results[0].2, Interpolated::Source);
    }

    #[test]
    fn test_camera_setup() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, (setup_cameras, setup_ui));
        app.update();

        let mut camera_query = app
            .world_mut()
            .query::<(&Camera2d, &RenderLayers, &Interpolated)>();
        let cameras: Vec<_> = camera_query.iter(&app.world()).collect();

        assert_eq!(cameras.len(), 2);
        assert_ne!(cameras[0].1, cameras[1].1);
        assert_ne!(cameras[0].2, cameras[1].2);

        let mut layout_query = app
            .world_mut()
            .query_filtered::<&Interpolated, With<CameraLayout>>();
        let layouts: Vec<_> = layout_query.iter(&app.world()).collect();

        assert_eq!(layouts.len(), 2);
        assert_ne!(layouts[0], layouts[1]);
    }

    #[test]
    fn test_update_camera_viewports() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::transform::TransformPlugin::default(),
            bevy::render::camera::CameraPlugin::default(),
            bevy::asset::AssetPlugin::default(),
            bevy::ui::UiPlugin {
                enable_rendering: false,
                add_picking: false,
            },
            bevy::input::InputPlugin::default(),
            bevy::render::texture::ImagePlugin::default(),
            bevy::text::TextPlugin::default(),
            bevy::window::WindowPlugin::default(),
        ))
        .init_asset::<TextureAtlasLayout>()
        .add_systems(Startup, (setup_cameras, setup_ui))
        .add_systems(Update, update_camera_viewports);

        app.world_mut().spawn((
            Camera2d,
            Camera {
                order: 2,
                ..default()
            },
        ));

        // Run startup systems
        app.update();
        // Run layout
        app.update();

        let mut query = app.world_mut().query::<(&Camera, &Interpolated)>();
        let cameras: Vec<_> = query.iter(&app.world()).collect();

        assert_eq!(cameras.len(), 2);
        for (camera, interpolated) in cameras {
            let viewport = camera
                .viewport
                .as_ref()
                .expect("Camera should have viewport");

            // Verify viewport size (should be roughly half the window width)
            assert!(viewport.physical_size.x > 0);
            assert!(viewport.physical_size.x < 1200); // Should be less than full window width
            assert!(viewport.physical_size.y > 0);

            // Verify position based on interpolated type
            match interpolated {
                Interpolated::Source => {
                    assert_eq!(viewport.physical_position.x, 0); // Left side
                }
                Interpolated::Target => {
                    assert!(viewport.physical_position.x > 0); // Right side
                }
            }
        }
    }
}
