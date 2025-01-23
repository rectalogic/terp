use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::points::PointsMaterial;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Animation::default()).add_systems(
        Update,
        (
            animate.run_if(animating),
            toggle_animation.run_if(input_just_pressed(KeyCode::Space)),
        )
            .chain(),
    );
}

pub(super) fn player_plugin(app: &mut App) {
    app.insert_resource(Animation::default())
        .add_systems(Update, animate);
}

#[derive(Resource)]
struct Animation {
    animating: bool,
    curve: PingPongCurve<f32, LinearReparamCurve<f32, EasingCurve<f32>>>,
    time: f32,
}

impl Animation {
    pub fn new(easing: EaseFunction) -> Self {
        Self {
            animating: false,
            curve: EasingCurve::new(0.0, 1.0, easing)
                .reparametrize_linear(interval(0.0, 2.5).unwrap())
                .expect("good curve")
                .ping_pong()
                .expect("good curve"),
            time: 0.0,
        }
    }
}
impl Default for Animation {
    fn default() -> Self {
        Self::new(EaseFunction::CubicInOut)
    }
}

#[derive(Component)]
pub struct Animatable;

fn animating(animation: Res<Animation>) -> bool {
    animation.animating
}

fn update_times(
    animation_query: Query<&MeshMaterial2d<PointsMaterial>, With<Animatable>>,
    mut points_materials: ResMut<Assets<PointsMaterial>>,
    time: f32,
) {
    for material in &animation_query {
        if let Some(material) = points_materials.get_mut(material) {
            material.t = time;
        }
    }
}

fn toggle_animation(
    mut animation: ResMut<Animation>,
    animation_query: Query<&MeshMaterial2d<PointsMaterial>, With<Animatable>>,
    points_materials: ResMut<Assets<PointsMaterial>>,
) {
    animation.animating = !animation.animating;
    animation.time = 0.0;
    if !animation.animating {
        update_times(animation_query, points_materials, 0.0);
    }
}

fn animate(
    animation_query: Query<&MeshMaterial2d<PointsMaterial>, With<Animatable>>,
    points_materials: ResMut<Assets<PointsMaterial>>,
    mut animation: ResMut<Animation>,
    time: Res<Time>,
) {
    animation.time = (animation.time + time.delta_secs()) % animation.curve.domain().length();
    if let Some(t) = animation.curve.sample(animation.time) {
        update_times(animation_query, points_materials, t);
    }
}
