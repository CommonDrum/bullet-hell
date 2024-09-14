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
