use bevy::{
    color::palettes::basic::PURPLE, input::mouse::MouseButtonInput, math::vec2, prelude::*,
};

use crate::linestrip2d::LineStrip2d;

pub fn draw(mut gizmos: Gizmos, lines: Query<(&LineStrip2d, &Name)>) {
    let mut line1 = None;
    let mut line2 = None;
    for (line, name) in &lines {
        match name.as_str() {
            "line1" => {
                line1 = Some(line);
            }
            "line2" => {
                line2 = Some(line);
            }
            _ => {}
        }
    }
    if let Some(line1) = line1 {
        if let Some(line2) = line2 {
            gizmos.linestrip_2d(line1.interpolate(line2, 0.1), PURPLE);
        }
    }
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
