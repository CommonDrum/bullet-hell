use crate::game::prelude::*;

mod tiles;
pub mod pathfinding;

use crate::game::map::tiles::*;
use crate::game::map::pathfinding::*;

pub const PIXELS_PER_TILE: f32 = 16.0;
const BACKGROUND_LAYER: f32 = -1.1;
const MAP_SIZE: i32 = 50;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), (place_background, add_pathfinder));
}

pub fn get_viewport_cords(x: i32, y: i32) -> (f32, f32) {
    (
        x as f32 * PIXELS_PER_TILE,
        y as f32 * PIXELS_PER_TILE,
    )
}

pub fn viewport_to_pos(x: f32, y: f32) -> Pos {
    Pos(
        (x / PIXELS_PER_TILE).floor() as i32,
        (y / PIXELS_PER_TILE).floor() as i32,
    )
}


fn add_pathfinder(mut commands: Commands) {

    let pathfinder = Pathfinder::new(MAP_SIZE);

    commands.insert_resource(pathfinder);
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
