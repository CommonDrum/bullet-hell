use crate::game::prelude::*;

mod tiles;
pub mod pathfinding;

use crate::game::map::tiles::*;
use crate::game::map::pathfinding::*;

pub const BASIC_SIZE_IN_VIEWPORT: f32 = 16.0;
const BACKGROUND_LAYER: f32 = -1.1;
const MAP_SIZE: isize = 50;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), (setup_grid, place_background).chain());
}

fn setup_grid(mut commands: Commands) {
    let map = Map::new(MAP_SIZE , MAP_SIZE);
    commands.insert_resource(map);
}

pub fn get_viewport_cords(x: isize, y: isize) -> (f32, f32) {
    (
        x as f32 * BASIC_SIZE_IN_VIEWPORT,
        y as f32 * BASIC_SIZE_IN_VIEWPORT,
    )
}

pub fn get_map_coords(x: f32, y: f32) -> (isize, isize) {
    (
        (x / BASIC_SIZE_IN_VIEWPORT).floor() as isize,
        (y / BASIC_SIZE_IN_VIEWPORT).floor() as isize,
    )
}



fn place_background(mut commands: Commands, tilesets: Res<Tilesets>, mut grid: ResMut<Map>) {
    for y in -MAP_SIZE..=MAP_SIZE {
        for x in -MAP_SIZE..=MAP_SIZE {
            let (viewport_x, viewport_y) = get_viewport_cords(x, y);
            let position = Vec3::new(viewport_x, viewport_y, BACKGROUND_LAYER);

            if x == 0 && (y != 0 && y != 1) {
                spawn_wall(&tilesets, &mut *grid, &mut commands, "forest", 20, position);
            } else {
                spawn_tile(&tilesets, &mut commands, "forest", 21, position);
            }
        }
    }
}





