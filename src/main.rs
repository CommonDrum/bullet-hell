pub mod game;
use crate::game::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(101.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_state::<GameState>()
        .add_plugins(game::plugin)
        .insert_resource(Msaa::Off)
        .run();
}
