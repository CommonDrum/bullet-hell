use crate::game::map::pathfinding::Path;
use crate::game::map::pathfinding::*;
use crate::game::map::*;
use crate::game::prelude::*;
use crate::game::utils::*;

pub fn movement_system(
    mut query: Query<(
        &mut KinematicCharacterController,
        &Speed,
        &mut DirectionArray,
    )>,
    time: Res<Time>,
) {
    for (mut controller, speed, mut direction_array) in query.iter_mut() {
        normalize_array(&mut direction_array.0);
        let max_index = max_index(&direction_array.0);
        let arr_size = direction_array.0.len();
        let angle = index_to_radians(max_index, arr_size);
        let movement = Vec2::new(angle.cos(), angle.sin());
        let velocity = movement * speed.0 * time.delta_seconds();
        controller.translation = Some(velocity);
    }
}

pub fn head_to_next_path_pos(mut query: Query<(&Transform, &mut DirectionArray, &Path)>) {
    for (transform, mut direction_array, path) in query.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);

        if let Some((next_pos, _)) = path.0.first() {
            let destination = pos_to_viewport(next_pos);
            let angle = angle_between_points(position, destination);
            let index = radians_to_index(angle, direction_array.0.len());
            direction_array.change_weight(index, 2.0);
        }
    }
}

pub fn obstacle_avoidance_system(
    mut query: Query<(&Transform, &mut DirectionArray)>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, mut direction_array) in query.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        let arr_size = direction_array.0.len();
        let is_dir_obstructed = round_raycast(&rapier_context, position, arr_size, 3.0, 16.0); // THIS
                                                                                               // DEPENDS
                                                                                               // ON
                                                                                               // COLLIDER
                                                                                               // SIEZE

        for (i, is_obstructed) in is_dir_obstructed.iter().enumerate() {
            if *is_obstructed {
                direction_array.change_weight(i, -0.75);
            }
        }
    }
}

pub fn max_index(arr: &[f32]) -> usize {
    arr.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
        .expect("Array is empty")
}

pub fn chase_player(
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut q_enemies: Query<(Entity, &Transform, &AiMode), Without<Player>>,
    mut commands: Commands,
    pathfinder: ResMut<Pathfinder>,
) {
    if let Ok(player_transform) = q_player.get_single() {
        let player_position = viewport_to_pos(
            player_transform.translation.x,
            player_transform.translation.y,
        );

        for (entity, transform, ai_mode) in q_enemies.iter_mut() {
            if *ai_mode == AiMode::Chase {
                let enemy_position =
                    viewport_to_pos(transform.translation.x, transform.translation.y);
                commands.entity(entity).remove::<Path>();

                if let Some(path) = pathfinder.find_path(enemy_position, player_position) {
                    commands.entity(entity).insert(path);
                }
            }
        }
    }
}

pub fn path_update(mut commands: Commands, mut q_paths: Query<(Entity, &mut Path, &Transform)>) {
    for (entity, mut path, transform) in q_paths.iter_mut() {
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
        if let Some((next_pos, _)) = path.0.first() {
            let destination = pos_to_viewport(next_pos);
            let distance = (current_pos - destination).length();
            if distance <= 16.0 {
                path.0.remove(0);
            }
        } else {
            commands.entity(entity).remove::<Path>();
        }
    }
}
