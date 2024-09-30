use crate::game::map::pathfinding::Path;
use crate::game::map::pathfinding::*;
use crate::game::map::*;
use crate::game::prelude::*;
use crate::game::utils::*;
use std::f32::consts::PI;


pub fn movement_system(
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

pub fn aggressive_ai(mut set: ParamSet<(Query<(&Transform, &mut DirectionArray, &Path)>,)>) {
    for (transform, mut direction_array, path) in set.p0().iter_mut() {
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
                direction_array.change_weight(i, -1.0);
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

pub fn follow_player(
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    q_enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Path>)>,
    mut commands: Commands,
    pathfinder: ResMut<Pathfinder>,
) {
    let player_position = {
        let transform = q_player.get_single().unwrap();
        viewport_to_pos(transform.translation.x, transform.translation.y)
    };

    for (entity, transform) in q_enemies.iter() {
        let enemy_position = viewport_to_pos(transform.translation.x, transform.translation.y);

        if let Some(path) = pathfinder.find_path(enemy_position.clone(), player_position.clone()) {
            println!("{:?}", path.0);
            commands.entity(entity).insert(path);
        } else {
            println!("No path found for enemy at position {:?}", enemy_position);
        }
    }
}

pub fn path_update(mut commands: Commands, mut q_paths: Query<(Entity, &mut Path, &Transform)>) {
    for (entity, mut path, transform) in q_paths.iter_mut() {
        let current_pos = viewport_to_pos(transform.translation.x, transform.translation.y);

        if let Some((next_pos, _)) = path.0.first() {
            let distance = current_pos.distance(next_pos);
            if distance <= 2 {
                path.0.remove(0);
            }
        } else {
            commands.entity(entity).remove::<Path>();
        }
    }
}
