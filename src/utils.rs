// utils.rs
use crate::prelude::*;
use std::f32::consts::PI;

pub fn radians_to_index(angle: f32, arr_size: usize) -> usize {
    let mut angle = angle % (2.0 * PI);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    let index = ((angle / (2.0 * PI)) * arr_size as f32).floor() as usize;
    index % arr_size
}

pub fn index_to_radians(index: usize, arr_size: usize) -> f32 {
    if index >= arr_size {
        panic!("Index out of bounds");
    }
    let mut angle = (index as f32 / arr_size as f32) * 2.0 * PI;
    if angle > PI {
        angle -= 2.0 * PI;
    }
    angle
}

pub fn normalize_array(arr: &mut [f32]) {
    let min_val = arr.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_val = arr.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    if max_val != min_val {
        for x in arr.iter_mut() {
            *x = (*x - min_val) / (max_val - min_val);
        }
    }
}

pub fn angle_between_points(point1: Vec2, point2: Vec2) -> f32 {
    let difference = point2 - point1;
    difference.y.atan2(difference.x)
}

pub fn distance_between_points(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn round_raycast(
    rapier_context: &RapierContext,
    entity_position: Vec2,
    arr_size: usize,
    ray_length: f32,
    collider_radius: f32,
) -> Vec<bool> {
    let mut hit_results = vec![false; arr_size];
    let filter = QueryFilter::default();

    for i in 0..arr_size {
        let angle = index_to_radians(i, arr_size);
        let ray_dir = Vec2::new(angle.cos(), angle.sin());

        let start_point = entity_position + ray_dir * collider_radius;

        if rapier_context
            .cast_ray(start_point, ray_dir, ray_length, true, filter)
            .is_some()
        {
            hit_results[i] = true;
        }
    }
    hit_results
}
