// bullets.rs
use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct BulletBundle {
    pub transform: TransformBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub mass_properties: ColliderMassProperties,
    pub locked_axes: LockedAxes,
    pub gravity: GravityScale,
    pub bullet_marker: Bullet,
    pub active: ActiveEvents
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
            active: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
