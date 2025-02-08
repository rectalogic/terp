use bevy::{
    input::common_conditions::{input_just_released, input_pressed},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    draw::Brush,
    util::{window_position_to_world, window_to_world},
    AppState,
};

#[derive(Component)]
struct BrushSizeControl;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup).add_systems(
        Update,
        (
            start_resize
                .run_if(in_state(AppState::Idle))
                .run_if(run_if_shift_click),
            (
                resize.run_if(input_pressed(MouseButton::Left)),
                end_resize.run_if(input_just_released(MouseButton::Left)),
            )
                .run_if(in_state(AppState::BrushSize)),
        ),
    );
}

fn run_if_shift_click(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) -> bool {
    buttons.just_pressed(MouseButton::Left)
        && keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
        && !keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
}

fn setup(
    mut commands: Commands,
    brush: Res<Brush>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        BrushSizeControl,
        Visibility::Hidden,
        Mesh2d(meshes.add(Circle::new(0.5))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(brush.color))),
        Transform::from_scale(Vec3::splat(brush.radius)),
    ));
}

fn start_resize(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    brush: Res<Brush>,
    brush_control: Single<
        (Entity, &mut Transform, &MeshMaterial2d<ColorMaterial>),
        With<BrushSizeControl>,
    >,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
) {
    let (camera, camera_transform) = *camera_query;
    let (brush_entity, mut brush_transform, brush_material) = brush_control.into_inner();
    if let Some(material) = materials.get_mut(brush_material) {
        material.color = brush.color.into();
    }
    if let Some(world_position) = window_to_world(*window, camera, camera_transform) {
        *brush_transform = Transform::from_translation(
            Vec3::from((world_position, 0.)) - Vec3::new(brush.radius, -brush.radius, 0.),
        )
        .with_scale(Vec3::splat(brush.radius * 2.0));
        commands.entity(brush_entity).insert(Visibility::Visible);
        next_state.set(AppState::BrushSize);
    }
}

fn resize(
    mut cursor: EventReader<CursorMoved>,
    mut brush: ResMut<Brush>,
    brush_control: Single<(&mut Transform, &GlobalTransform), With<BrushSizeControl>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
) {
    let (camera, camera_transform) = *camera_query;
    let (mut brush_transform, brush_global_transform) = brush_control.into_inner();
    for moved in cursor.read() {
        if let Some(world_position) =
            window_position_to_world(camera, camera_transform, moved.position)
        {
            let scale = brush_global_transform
                .transform_point(Vec3::ZERO)
                .distance(Vec3::from((world_position, 0.)));
            brush.radius = scale;
            brush_transform.scale = Vec3::splat(scale) * 2.0;
        };
    }
}

fn end_resize(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    brush_control: Single<Entity, With<BrushSizeControl>>,
) {
    next_state.set(AppState::Idle);
    commands.entity(*brush_control).insert(Visibility::Hidden);
}
