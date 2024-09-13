use bevy::prelude::*;

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
pub struct DirectionArray(pub [f32; 8]);
