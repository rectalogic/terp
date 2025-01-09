use bevy::{color::palettes::basic::BLUE, input::mouse::MouseButtonInput, math::vec2, prelude::*};

use crate::linestrip2d::LineStrip2d;

pub fn draw(mut gizmos: Gizmos) {
    let points = LineStrip2d::new(vec![
        vec2(50., 50.),
        vec2(50., 60.),
        vec2(60., 60.),
        vec2(80., 70.),
    ])
    .points();
    gizmos.linestrip_2d(points, BLUE);
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
