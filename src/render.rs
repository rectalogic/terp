use std::rc::Rc;

use bevy::{color::palettes::basic::BLUE, input::mouse::MouseButtonInput, math::vec2, prelude::*};

use crate::curve2d::Curve2d;

pub fn draw(mut gizmos: Gizmos) {
    let curve = Rc::into_inner(
        Curve2d::new(vec![
            vec2(50., 50.),
            vec2(50., 60.),
            vec2(60., 60.),
            vec2(80., 70.),
        ])
        .curve(),
    )
    .unwrap();
    let domain = curve.domain();
    gizmos.curve_2d(curve, domain.spaced_points(100).unwrap(), BLUE);
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
