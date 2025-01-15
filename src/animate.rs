use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::points::PointsMaterial;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Animate::default()).add_systems(
        Update,
        (
            animate.run_if(animating),
            toggle_animation.run_if(input_just_pressed(KeyCode::Space)),
        )
            .chain(),
    );
}

#[derive(Resource)]
struct Animate {
    animating: bool,
    curve: PingPongCurve<f32, LinearReparamCurve<f32, EasingCurve<f32>>>,
}

impl Animate {
    pub fn new(easing: EaseFunction) -> Self {
        Self {
            animating: false,
            curve: EasingCurve::new(0.0, 1.0, easing)
                .reparametrize_linear(interval(0.0, 2.5).unwrap())
                .expect("good curve")
                .ping_pong()
                .expect("good curve"),
        }
    }
}
impl Default for Animate {
    fn default() -> Self {
        Self::new(EaseFunction::CubicInOut)
    }
}

#[derive(Component)]
pub struct Animatable;

fn animating(animate: Res<Animate>) -> bool {
    animate.animating
}

fn update_times(
    animation_query: Query<&MeshMaterial2d<PointsMaterial>, With<Animatable>>,
    mut points_materials: ResMut<Assets<PointsMaterial>>,
    time: f32,
) {
    for material in &animation_query {
        if let Some(material) = points_materials.get_mut(material) {
            material.settings.t = time;
        }
    }
}

fn toggle_animation(
    mut animation: ResMut<Animate>,
    animation_query: Query<&MeshMaterial2d<PointsMaterial>, With<Animatable>>,
    points_materials: ResMut<Assets<PointsMaterial>>,
) {
    animation.animating = !animation.animating;
    if !animation.animating {
        update_times(animation_query, points_materials, 0.0);
    }
}

fn animate(
    animation_query: Query<&MeshMaterial2d<PointsMaterial>, With<Animatable>>,
    points_materials: ResMut<Assets<PointsMaterial>>,
    animation: Res<Animate>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs() % animation.curve.domain().length();
    if let Some(t) = animation.curve.sample(now) {
        update_times(animation_query, points_materials, t);
    }
}
