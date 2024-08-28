pub mod components;
pub mod grid;
pub mod player;
pub mod bullets;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(101.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((grid::plugin, player::plugin))
        .insert_resource(Msaa::Off)
        .run();
}
