use crate::game::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (bullet_collision, damage_system).run_if(in_state(AppState::Game)));
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
    pub sensor: Sensor,
    pub game: Game,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            transform: TransformBundle::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(1.0 / 2.0),
            sensor: Sensor,
            velocity: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0,
            },
            mass_properties: ColliderMassProperties::Density(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            gravity: GravityScale(0.0),
            bullet_marker: Bullet,
            active: ActiveEvents::COLLISION_EVENTS,
            damage: Damage(1.0),
            game: Game,

        }
    }
}

fn despawn_if_bullet(entity: Entity, commands: &mut Commands, bullet_q: &Query<&Bullet>) {
    if bullet_q.get(entity).is_ok() {
        commands.entity(entity).despawn();
    }
}

fn bullet_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    damage_q: Query<&Damage>,
    mut health_q: Query<&mut Health>,
    bullet_q: Query<&Bullet>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            apply_damage(*entity1, *entity2, &damage_q, &mut health_q);
            apply_damage(*entity2, *entity1, &damage_q, &mut health_q);
            despawn_if_bullet(*entity1, &mut commands, &bullet_q);
            despawn_if_bullet(*entity2, &mut commands, &bullet_q);
        }
    }
}

pub fn spawn_default_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    velocity: Vec3,
) {
    let texture: Handle<Image> = asset_server.load("sprites/bullet.png");
    commands
        .spawn(BulletBundle {
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            position.x, position.y, position.z,
        )))
        .insert(Velocity {
            linvel: Vec2::new(velocity.x, velocity.y),
            angvel: 0.,
        })
        .insert(texture);
}

fn damage_system(query: Query<(Entity, &Health), With<Enemy>>, mut commands: Commands) {
    for (entity, health) in &query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn apply_damage(
    attacker: Entity,
    target: Entity,
    damage_q: &Query<&Damage>,
    health_q: &mut Query<&mut Health>,
) {
    if let Ok(damage) = damage_q.get(attacker) {
        if let Ok(mut health) = health_q.get_mut(target) {
            health.0 -= damage.0;
        }
    }
}
