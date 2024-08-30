use crate::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (handle_collision,));
}
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
    pub active: ActiveEvents,
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

fn handle_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            commands.entity(*entity2).despawn();
        }
    }
}
