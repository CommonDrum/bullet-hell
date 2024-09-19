
// ai/mod.rs
use crate::prelude::*;
use crate::utils::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (movement_system, (aggressive_ai, obstacle_avoidance_system).chain()));
}

fn movement_system(
    mut query: Query<(
        &mut KinematicCharacterController,
        &Speed,
        &Transform,
        &mut DirectionArray,
    )>,
    time: Res<Time>,
) {
    for (mut controller, speed, _transform, mut direction_array) in query.iter_mut() {
        normalize_array(&mut direction_array.0);
        let (_, max_index) = max_element_and_index(&direction_array.0);
        let arr_size = direction_array.0.len();
        let angle = index_to_radians(max_index, arr_size);
        let movement = Vec2::new(angle.cos(), angle.sin());
        let velocity = movement * speed.0 * time.delta_seconds();
        controller.translation = Some(velocity);
    }
}

fn aggressive_ai(
    mut set: ParamSet<(
        Query<(&Transform, &mut DirectionArray)>,
        Query<&Transform, With<Player>>,
    )>,
) {
    let player_position = {
        let binding = set.p1();
        let player_transform = binding.get_single().unwrap();
        Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        )
    };

    for (transform, mut direction_array) in set.p0().iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        let angle = angle_between_points(position, player_position);
        let index = radians_to_index(angle, direction_array.0.len());
        direction_array.change_weight(index, 1.0);
    }
}


fn obstacle_avoidance_system(
    mut query: Query<(&Transform, &mut DirectionArray)>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, mut direction_array) in query.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        let arr_size = direction_array.0.len();

        if arr_size == 0 {
            continue; // Safeguard against empty DirectionArray
        }

        let is_dir_obstructed = round_raycast(&rapier_context, position, arr_size, 10.0, 50.0);

        for (i, is_obstructed) in is_dir_obstructed.iter().enumerate() {
            if *is_obstructed {
                direction_array.change_weight(i, -1.0);
            }
        }
    }
}


//TODO: No need for max element
fn max_element_and_index(arr: &[f32]) -> (f32, usize) {
    assert!(!arr.is_empty(), "Array must not be empty");
    arr.iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, &value)| (value, index))
        .expect("Array is unexpectedly empty")
}


