use bevy::prelude::*;

pub const BASIC_SIZE_IN_VIEWPORT: f32 = 50.0;

pub fn get_grid_coords() {}

pub fn get_viewport_cords(x: i32, y: i32) -> (f32, f32) {
    (
        x as f32 * BASIC_SIZE_IN_VIEWPORT,
        y as f32 * BASIC_SIZE_IN_VIEWPORT,
    )
}
