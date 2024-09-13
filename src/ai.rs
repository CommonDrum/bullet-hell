//ai.rs
use crate::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, ( movement_system));
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
        
        let angle = (max_index as f32 / arr_size as f32) * 2.0 * PI;
        
        let movement = Vec2::new(angle.cos(), 0.0);
        
        let velocity = movement * speed.0 * time.delta_seconds();
        
        controller.translation = Some(velocity);
    }
}


fn random_point_within_radius(center: Vec3, radius: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let distance = rng.gen_range(-radius..radius);
    let x_offset = distance * angle.cos();
    let y_offset = distance * angle.sin();
    Vec3::new(center.x + x_offset, center.y + y_offset, center.z)
}


fn max_element_and_index(arr: &[f32]) -> (f32, usize) {
    assert!(!arr.is_empty(), "Array must not be empty");
    
    arr.iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, &value)| (value, index))
        .expect("Array is unexpectedly empty")
}
