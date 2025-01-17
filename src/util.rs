use bevy::prelude::*;

pub(crate) fn window_position_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window_position: Vec2,
) -> Option<Vec2> {
    if let Some(viewport) = camera.logical_viewport_rect() {
        if let Ok(point) =
            camera.viewport_to_world_2d(camera_transform, window_position - viewport.min)
        {
            return Some(point);
        }
    }
    None
}

pub(crate) fn window_to_world(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    if let Some(window_position) = window.cursor_position() {
        return window_position_to_world(camera, camera_transform, window_position);
    }
    None
}
