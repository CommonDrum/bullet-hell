pub mod bullets;
pub mod components;
pub mod grid;
pub mod player;
pub mod prelude;
use crate::prelude::*;

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


#[derive(Component)]
struct CoinTag;

#[derive(Bundle)]
struct CoinBundle {
    sprite: SpriteBundle,
    coin_tag: CoinTag,
    collider: Collider,
}

impl Default for CoinBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle::default(),
            coin_tag: CoinTag,
            collider: Collider::cuboid(10.0, 10.0),
        }
    }
}

