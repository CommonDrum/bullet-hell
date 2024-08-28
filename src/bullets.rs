// bullets.rs
use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;
use crate::components::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct BulletBundle {
    transform: TransformBundle,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    mass_properties: ColliderMassProperties,
    locked_axes: LockedAxes,
    gravity: GravityScale,
    bullet_marker: Bullet,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            transform: TransformBundle::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(10.0 / 2.0),
            velocity: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
            mass_properties: ColliderMassProperties::Density(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            gravity: GravityScale(0.0),
            bullet_marker: Bullet,
        }
    }
}
