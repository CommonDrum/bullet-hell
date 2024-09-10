use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (handle_collision, damage_system));
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
    pub damage: Damage,
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
            damage: Damage(10.0),
        }
    }
}

//TODO: it works now because only bullets send collision events. Make it so when other entiteis
//also want to send them it works.
fn handle_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    damage_q: Query<&Damage>,
    mut health_q: Query<&mut Health>,
    bullet_q: Query<&Bullet>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            if let Ok(mut health) = health_q.get_mut(*entity1) {
                if let Ok(damage) = damage_q.get(*entity2){
                    health.0 -= damage.0;
                }
                println!("Entity1 has Health: {}", health.0);

            } else if let Ok(mut health) = health_q.get_mut(*entity2){
                if let Ok(damage) = damage_q.get(*entity1){
                    health.0 -= damage.0;
                }
                println!("Entity2 has Health: {}", health.0);
            }
            if let Ok(bullet) = bullet_q.get(*entity2){
                commands.entity(*entity2).despawn();
            } else {
                commands.entity(*entity1).despawn();
            }
        }
    }
}

//I don't have an idea on where else to put it. I think only bullets will have damage
//TODO: move the substract logic to this and make it triggered with an event

fn damage_system(
    mut query: Query<(Entity, &mut Health)>, // Entity is copied, so no need for mut
    mut commands: Commands,                  // Commands to handle despawning
){
    for (entity, mut health) in &mut query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn(); // Despawn the entity when health is <= 0
        }
    }
}

