pub mod components;
pub mod grid;
pub mod player;

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::*;
use grid::*;
use player::*;

const MAP_SIZE: i32 = 50;
const BACKGROUND_LAYER: f32 = -1.1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(101.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((grid::plugin, player::plugin))
        .insert_resource(Msaa::Off)
        .run();
}
