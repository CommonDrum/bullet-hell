use crate::game::prelude::*;

mod tiles;
mod pathfinding;

use crate::game::map::tiles::*;
use crate::game::map::pathfinding::*;

pub const BASIC_SIZE_IN_VIEWPORT: f32 = 16.0;
const BACKGROUND_LAYER: f32 = -1.1;
const MAP_SIZE: i32 = 50;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), place_background);
}

pub fn get_viewport_cords(x: i32, y: i32) -> (f32, f32) {
    (
        x as f32 * BASIC_SIZE_IN_VIEWPORT,
        y as f32 * BASIC_SIZE_IN_VIEWPORT,
    )
}

pub fn place_background(mut commands: Commands, tilesets: Res<Tilesets>) {
    for y in -MAP_SIZE..=MAP_SIZE {
        for x in -MAP_SIZE..=MAP_SIZE {
            let (viewport_x, viewport_y) = get_viewport_cords(x, y);
            let position = Vec3::new(viewport_x, viewport_y, BACKGROUND_LAYER);

            if (x % MAP_SIZE == 0 && x != 0) || (y % MAP_SIZE == 0 && y != 0) {
                spawn_wall(&tilesets, &mut commands, "forest", 20, position);
            } else {
                spawn_tile(&tilesets, &mut commands, "forest", 21, position);
            }
        }
    }
}
