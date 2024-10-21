use crate::game::prelude::*;

use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;


use crate::game::map::pathfinding::*;
use crate::game::map::tiles::*;
use bevy::color::palettes::css::*;

const MAP_SIZE: i32 = 50;
const WATER_THRESHOLD: f64 = 0.0001;
const BACKGROUND_LAYER: f32 = -1.1;
const PIXELS_PER_TILE: f32 = 16.0;


enum Biome {
    Forest,
}

impl Biome {
    fn value(&self) -> &str {
        match *self {
            Biome::Forest => "forest",
        }
    }
}

pub fn get_viewport_cords(x: i32, y: i32) -> (f32, f32) {
    (x as f32 * PIXELS_PER_TILE, y as f32 * PIXELS_PER_TILE)
}

pub fn generate_map(mut commands: Commands, tilesets: Res<Tilesets>) {
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    let perlin = Perlin::new(seed);

    for y in -MAP_SIZE..=MAP_SIZE {
        for x in -MAP_SIZE..=MAP_SIZE {
            let (viewport_x, viewport_y) = get_viewport_cords(x, y);
            let position = Vec3::new(viewport_x, viewport_y, BACKGROUND_LAYER);

            if x == -MAP_SIZE || x == MAP_SIZE || y == -MAP_SIZE || y == MAP_SIZE {
                spawn_tree(&tilesets, &mut commands, "res", (26,26), position);
            } else {
                let noise_value = perlin.get([x as f64 / 11.0, y as f64 / 11.0]);

                if noise_value < WATER_THRESHOLD {
                    spawn_wall(&tilesets, &mut commands, "forest", 173, position);
                } else {
                    spawn_tile(&tilesets, &mut commands, "forest", 77, position);
                }
            }
        }
    }
}

