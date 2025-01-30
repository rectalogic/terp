use bevy::{
    ecs::query::QueryData,
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
};

use crate::Interpolated;

#[derive(Component)]
struct CameraLayout;

pub(crate) const SOURCE_LAYER: RenderLayers = RenderLayers::layer(1);
pub(crate) const TARGET_LAYER: RenderLayers = RenderLayers::layer(2);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_cameras)
        .add_systems(Update, update_camera_viewports);
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
    // Layout for cameras. We track Node layout with camera Viewport.
    commands
        .spawn((Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            grid_template_columns: vec![GridTrack::percent(50.0), GridTrack::percent(50.0)],
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Interpolated::Source,
                CameraLayout,
                Node {
                    border: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                BorderColor(Srgba::rgb(0.4, 0.4, 0.4).into()),
            ));
            parent.spawn((
                Interpolated::Target,
                CameraLayout,
                Node {
                    border: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                BorderColor(Srgba::rgb(0.3, 0.3, 0.3).into()),
            ));
        });
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
