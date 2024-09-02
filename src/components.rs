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

#[derive(Component)]
pub enum AiMode {
    Passive,
    ChasingPlayer,
}

#[derive(Component)]
pub struct Destination(pub Vec3);
