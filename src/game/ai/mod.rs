// ai/mod.rs
use crate::game::prelude::*;
use crate::game::utils::*;
use crate::game::map::pathfinding::Path;
use crate::game::map::pathfinding::*;
use crate::game::map::*;
use std::f32::consts::PI;
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            movement_system.run_if(in_state(AppState::Game)),
            (aggressive_ai, obstacle_avoidance_system, follow_player, path_update )
                .chain()
                .run_if(in_state(AppState::Game)),
        ),
    );
}

fn movement_system(
    mut query: Query<(
        &mut KinematicCharacterController,
        &Speed,
        &mut Transform,
        &mut DirectionArray,
    )>,
    time: Res<Time>,
) {
    for (mut controller, speed, mut transform, mut direction_array) in query.iter_mut() {
        normalize_array(&mut direction_array.0);
        let max_index = max_index(&direction_array.0);
        let arr_size = direction_array.0.len();
        let angle = index_to_radians(max_index, arr_size);
        let movement = Vec2::new(angle.cos(), angle.sin());
        let velocity = movement * speed.0 * time.delta_seconds();
        controller.translation = Some(velocity);
        transform.rotation = Quat::from_rotation_z(angle - PI / 2.0);
    }
}


fn aggressive_ai(
    mut set: ParamSet<(
        Query<(&Transform, &mut DirectionArray, &Path)>,
    )>,
) {
    for (transform, mut direction_array, path) in set.p0().iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);

        if let Some((next_pos, _)) = path.0.first() {
            let next_position = Vec2::new(next_pos.0 as f32, next_pos.1 as f32);
            let angle = angle_between_points(position, next_position);
            let index = radians_to_index(angle, direction_array.0.len());
            direction_array.change_weight(index, 1.0);
        }
    }
}

fn obstacle_avoidance_system(
    mut query: Query<(&Transform, &mut DirectionArray)>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, mut direction_array) in query.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        let arr_size = direction_array.0.len();
        let is_dir_obstructed = round_raycast(&rapier_context, position, arr_size, 10.0, 50.0);

        for (i, is_obstructed) in is_dir_obstructed.iter().enumerate() {
            if *is_obstructed {
                direction_array.change_weight(i, -1.0);
            }
        }
    }
}

fn max_index(arr: &[f32]) -> usize {
    arr.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
        .expect("Array is empty")
}



fn follow_player(
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    q_enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Path>)>,
    mut commands: Commands,
    mut pathfinder: ResMut<Pathfinder>,
) {
    // Get the player's grid position using viewport_to_pos
    let player_position = {
        let transform = q_player.get_single().unwrap();
        viewport_to_pos(transform.translation.x, transform.translation.y)
    };

    // Iterate over each enemy without a path
    for (entity, transform) in q_enemies.iter() {
        // Get the enemy's grid position using viewport_to_pos
        let enemy_position = viewport_to_pos(transform.translation.x, transform.translation.y);

        // Find a path from the enemy to the player
        if let Some(path) = pathfinder.find_path(enemy_position.clone(), player_position.clone()) {
            // Insert the Path component into the enemy entity
            commands.entity(entity).insert(path);
        } else {
            // Handle the case where no path is found
            println!("No path found for enemy at position {:?}", enemy_position);
        }
    }
}

fn path_update(
    mut commands: Commands,
    mut q_paths: Query<(Entity, &mut Path, &Transform)>,
) {
    for (entity, mut path, transform) in q_paths.iter_mut() {
        let current_pos = viewport_to_pos(transform.translation.x, transform.translation.y);

        if let Some((next_pos, _)) = path.0.first() {
            if current_pos == *next_pos {
                path.0.remove(0);

                if path.0.is_empty() {
                    commands.entity(entity).remove::<Path>();
                }
            }
        } else {
            commands.entity(entity).remove::<Path>();
        }
    }
}


