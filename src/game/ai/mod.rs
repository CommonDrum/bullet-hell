// ai/mod.rs
use crate::game::prelude::*;
use crate::game::utils::*;
use std::f32::consts::PI;
use crate::game::map::pathfinding::*;
use crate::game::map::*;
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            (
                movement_system,
                find_path,
                pop_path_points,
             ).run_if(in_state(AppState::Game)),
            (
                aggressive_ai,
            )
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



fn pop_path_points(mut commands: Commands, mut q_path: Query<(Entity, &mut Path, &Transform)>) {
    for (entity, mut path, transform) in q_path.iter_mut() {
        let current_tile = get_map_coords(transform.translation.x, transform.translation.y);
        if path.0.is_empty() {
            commands.entity(entity).remove::<Path>();
        }else if  current_tile == path.0[0] {
            path.0.remove(0);
        }
    }
}



fn find_path(
    mut commands: Commands,
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    q_enemy: Query<(Entity, &Transform), (With<Enemy>, Without<Path>)>,
    map: Res<Map>,
) {
    if let Ok(player_transform) = q_player.get_single() {
        let player_pos = get_map_coords(player_transform.translation.x, player_transform.translation.y);

        for (enemy_entity, enemy_transform) in q_enemy.iter() {
            let enemy_pos = get_map_coords(enemy_transform.translation.x, enemy_transform.translation.y);

            if let Some((path, _cost)) = map.find_path(enemy_pos, player_pos) {
                commands.entity(enemy_entity).insert(Path(path));
            }
        }
    }
}


fn aggressive_ai(
    mut q_enemies: Query<(&Transform, &mut DirectionArray, Option<&Path>)>,
    q_player: Query<&Transform, (With<Player>, Without<DirectionArray>)>,
) {
    if let Ok(player_transform) = q_player.get_single() {
        let player_position = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );

        for (transform, mut direction_array, path) in q_enemies.iter_mut() {
            let position = Vec2::new(transform.translation.x, transform.translation.y);

            let destination = if let Some(path) = path {
                if !path.0.is_empty() {
                    Vec2::new(
                        path.0[0].0 as f32 * BASIC_SIZE_IN_VIEWPORT,
                        path.0[0].1 as f32 * BASIC_SIZE_IN_VIEWPORT,
                    )
                } else {
                    player_position
                }
            } else {
                player_position
            };

            let angle = angle_between_points(position, destination);
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
