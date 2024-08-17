use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(i32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Crosshair;
