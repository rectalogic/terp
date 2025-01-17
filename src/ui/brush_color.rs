use bevy::{
    input::common_conditions::{input_just_released, input_pressed},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
    window::PrimaryWindow,
};

use crate::{
    util::{window_position_to_world, window_to_world},
    AppState, Brush, Interpolated,
};

const RADIUS: f32 = 50.0;

#[derive(Component)]
struct BrushColorControl;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                start_select_color
                    .run_if(in_state(AppState::Idle))
                    .run_if(run_if_ctrl_click),
                (
                    select_color.run_if(input_pressed(MouseButton::Left)),
                    end_select_color.run_if(input_just_released(MouseButton::Left)),
                )
                    .run_if(in_state(AppState::BrushColor))
                    .chain(),
            )
                .chain(),
        )
        .add_plugins(Material2dPlugin::<HslMaterial>::default());
}

fn run_if_ctrl_click(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) -> bool {
    buttons.just_pressed(MouseButton::Left)
        && keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && !keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
}

fn setup(
    mut commands: Commands,
    brush: Res<Brush>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HslMaterial>>,
) {
    commands.spawn((
        BrushColorControl,
        Visibility::Hidden,
        Mesh2d(meshes.add(Rectangle::new(RADIUS * 2.0, RADIUS * 2.0))),
        MeshMaterial2d(materials.add(HslMaterial {
            color: brush.color.into(),
        })),
        Transform::default(),
    ));
}

fn start_select_color(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    brush: Res<Brush>,
    brush_control: Single<
        (Entity, &MeshMaterial2d<HslMaterial>, &mut Transform),
        With<BrushColorControl>,
    >,
    mut materials: ResMut<Assets<HslMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform), Without<Interpolated>>,
) {
    let (camera, camera_transform) = *camera_query;
    let (brush_entity, brush_material, mut brush_transform) = brush_control.into_inner();
    if let Some(world_position) = window_to_world(*window, camera, camera_transform) {
        //XXX offset so mouse is over current color (angle/distance)
        *brush_transform = Transform::from_translation(Vec3::from((world_position, 0.)));
        if let Some(material) = materials.get_mut(brush_material) {
            material.color = brush.color.into();
        }
        commands.entity(brush_entity).insert(Visibility::Visible);
        next_state.set(AppState::BrushColor);
    }
}

fn select_color(
    mut cursor: EventReader<CursorMoved>,
    mut brush: ResMut<Brush>,
    brush_control: Single<
        (
            &GlobalTransform,
            &MeshMaterial2d<HslMaterial>,
            &mut Transform,
        ),
        With<BrushColorControl>,
    >,
    mut materials: ResMut<Assets<HslMaterial>>,
    camera_query: Single<(&Camera, &GlobalTransform), Without<Interpolated>>,
) {
    let (camera, camera_transform) = *camera_query;
    let (brush_global_transform, brush_material, brush_transform) = brush_control.into_inner();
    for moved in cursor.read() {
        if let Some(world_position) =
            window_position_to_world(camera, camera_transform, moved.position)
        {
            //XXX figure distance and angle to origin - convert to HSL
            //XXX position at pointer, offset so current color is selected
            let origin = brush_global_transform.transform_point(Vec3::ZERO).xy();
            let up = brush_global_transform.transform_point(Vec3::Y).xy();
            let distance = world_position.distance(origin);
            let angle = world_position.angle_to(up);

            if let Some(material) = materials.get_mut(brush_material) {
                material.color = brush.color.into(); //XXX set to selected color computed above
            }
        };
    }
}

fn end_select_color(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    brush_control: Single<Entity, With<BrushColorControl>>,
) {
    next_state.set(AppState::Idle);
    commands.entity(*brush_control).insert(Visibility::Hidden);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct HslMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for HslMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsl.wgsl".into()
    }
}
