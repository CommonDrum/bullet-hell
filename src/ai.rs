//ai.rs
use crate::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, ( movement_system, aggressive_ai));
}

fn movement_system(
    mut query: Query<(
        &mut KinematicCharacterController,
        &Speed,
        &Transform,
        &DirectionArray,
    )>,
    time: Res<Time>,
) {
    for (mut controller, speed, transform, direction_array) in query.iter_mut() {
        let (_, max_index) = max_element_and_index(&direction_array.0);
        let arr_size = direction_array.0.len();
        let angle = index_to_radians(max_index as i32, arr_size);
        let movement = Vec2::new(angle.cos(), angle.sin());
        let velocity = movement * speed.0 * time.delta_seconds();
        controller.translation = Some(velocity.into());
    }
}

fn aggressive_ai(
    mut set: ParamSet<(
        Query<(&Transform, &mut DirectionArray)>,
        Query<&Transform, With<Player>>,
    )>,
) {
   let player_position = {
        let binding_1 = set.p1();
        let player_transform = binding_1.get_single().unwrap();
        Vec2::new(player_transform.translation.x, player_transform.translation.y)    };    

   for (transform, mut direction_array) in set.p0().iter_mut() {
       let position = Vec2::new(transform.translation.x, transform.translation.y);
       let angle = angle_between_points(position, player_position);
       direction_array.0[2] += 0.01;
    }


    
}
fn max_element_and_index(arr: &[f32]) -> (f32, usize) {
    assert!(!arr.is_empty(), "Array must not be empty");
    
    arr.iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, &value)| (value, index))
        .expect("Array is unexpectedly empty")
}


fn radians_to_index(angle: f32, arr_size: usize) -> i32 {
    let segment_size = 2.0 * std::f32::consts::PI / arr_size as f32;

    for i in 0..arr_size {
        if angle < segment_size * (i + 1) as f32 {
            return i as i32;
        }
    }

    (arr_size - 1) as i32
}

fn index_to_radians(index: i32, arr_size: usize) -> f32 {
    let segment_size = 2.0 * std::f32::consts::PI / arr_size as f32;
    let clamped_index = index.clamp(0, (arr_size as i32 - 1));
    
    segment_size * (clamped_index as f32) + segment_size / 2.0
}

fn normalize_array(arr: &mut[f32]) {
    let min_val = arr.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_val = arr.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    if max_val != min_val {
        for x in arr.iter_mut() {
            *x = (*x - min_val) / (max_val - min_val);
        }
    }
}

fn angle_between_points(point1: Vec2, point2: Vec2) -> f32 {
    let difference = point2 - point1;
    difference.x.atan2(difference.y)
}
