use bevy::{
    color::palettes::basic::RED,
    input::mouse::{AccumulatedMouseMotion, MouseButtonInput},
    math::vec2,
    prelude::*,
};

pub fn draw(mut gizmos: Gizmos) {
    let curve = SampleAutoCurve::new(
        Interval::UNIT,
        [
            vec2(10., 10.),
            vec2(10., 20.),
            vec2(20., 20.),
            vec2(40., 30.),
        ],
    )
    .expect("should be good");
    gizmos.curve_2d(curve, (0..=100).map(|n| n as f32 / 100.0), RED);
}

pub fn handle_mouse(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut button_events: EventReader<MouseButtonInput>,
) {
    for button_event in button_events.read() {
        if button_event.button != MouseButton::Left {
            continue;
        }
        // *mouse_pressed = MousePressed(button_event.state.is_pressed());
        println!("mouse pressed");
    }
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    dbg!(point);
}
