// ai/mod.rs
use crate::game::map::pathfinding::Path;
use crate::game::map::pathfinding::*;
use crate::game::map::*;
use crate::game::prelude::*;
use crate::game::utils::*;
use std::f32::consts::PI;

mod movement;
use crate::game::ai::movement::*;
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            movement_system.run_if(in_state(AppState::Game)),
            (
                head_to_next_path_pos,
                path_update,
                obstacle_avoidance_system,
            )
                .chain()
                .run_if(in_state(AppState::Game)),
        ),
    )
    .add_systems(
        FixedUpdate,
        (chase_player).chain().run_if(in_state(AppState::Game)),
    ).insert_resource(Time::<Fixed>::from_seconds(0.8));
}

