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
        ))
        .add_systems(OnExit(AppState::Game), despawn_game_entities);
}

fn despawn_game_entities(mut commands: Commands, menu_entities: Query<Entity, With<Game>>) {
    for entity in &menu_entities {
        commands.entity(entity).despawn();
    }
}

