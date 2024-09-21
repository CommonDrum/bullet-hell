pub mod ai;
pub mod bullets;
pub mod camera;
pub mod components;
pub mod enemies;
pub mod grid;
pub mod player;
pub mod prelude;
pub mod utils;

use crate::game::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
            camera::plugin,
            grid::plugin,
            player::plugin,
            bullets::plugin,
            enemies::plugin,
        ));
}
