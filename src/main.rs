pub mod game;
pub mod menu;
use crate::game::prelude::*;
use crate::menu::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(101.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_state::<AppState>()
        .add_plugins(game::plugin)
        .add_plugins(menu::plugin)
        .insert_resource(Msaa::Off)
        .run();
}
