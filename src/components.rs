use bevy::prelude::*;
use crate::utils::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component, PartialEq)]
pub enum AiMode {
    Passive,
    ChasingPlayer,
}

#[derive(Component)]
pub struct Destination(pub Vec3);

#[derive(Event)]
pub struct DamageEvent(pub Entity, pub Damage);

#[derive(Component)]
pub struct DirectionArray(pub [f32; 16]);


impl DirectionArray {
    pub fn change_weight(&mut self, index: usize, value: f32) {
        let arr = &mut self.0;
        let len = arr.len();
        let max_offset = len / 4;

        arr[index] += value;

        for offset in 1..=max_offset {
            let factor = value * (1.0 - (offset as f32) / (max_offset as f32 + 1.0));
            let left_index = (index + len - offset) % len;
            let right_index = (index + offset) % len;
            arr[left_index] += factor;
            arr[right_index] += factor;
        }
    }
}


