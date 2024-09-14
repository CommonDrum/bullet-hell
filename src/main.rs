pub mod ai;
pub mod bullets;
pub mod camera;
pub mod components;
pub mod enemies;
pub mod grid;
pub mod player;
pub mod utils;
pub mod prelude;
use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(101.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            camera::plugin,
            grid::plugin,
            player::plugin,
            bullets::plugin,
            enemies::plugin,
        ))
        .insert_resource(Msaa::Off)
        .run();
}
