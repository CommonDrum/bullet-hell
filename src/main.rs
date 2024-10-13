pub mod game;
pub mod menu;
use crate::game::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(101.0))
        .init_state::<AppState>()
        .add_plugins(game::plugin)
        .add_plugins(menu::plugin)
        .insert_resource(Msaa::Off)
        .run();
}
