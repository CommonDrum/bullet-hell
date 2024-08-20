use bevy::prelude::*;

pub struct GridPlugin {
    pub granularity: i32,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
    }
}


pub fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos * tile_size - (bound_window / 2.0) + (tile_size / 2.0)
    }

