use crate::game::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (bullet_collision, enemy_death_system, player_death_system)
            .run_if(in_state(AppState::Game)),
    );
}

pub fn spawn_bullet(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: Vec3,
    velocity: Vec3,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::ball(1.0 / 2.0),
            Sensor,
            Velocity {
                linvel: Vec2::new(velocity.x, velocity.y),
                angvel: 0.0,
            },
            ColliderMassProperties::Density(0.2),
            LockedAxes::ROTATION_LOCKED,
            GravityScale(0.0),
            Bullet,
            ActiveEvents::COLLISION_EVENTS,
            Damage(1.0),
            Game,
        ))
        .id()
}

pub fn spawn_default_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    velocity: Vec3,
) -> Entity {
    let texture: Handle<Image> = asset_server.load("sprites/bullet.png");
    spawn_bullet(commands, texture, position, velocity) //think about inserting the values of
                                                        //position, veolcity etc. insead of
                                                        //embeding in funciton.
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

fn enemy_death_system(query: Query<(Entity, &Health), With<Enemy>>, mut commands: Commands) {
    for (entity, health) in &query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn player_death_system(
    mut next_app: ResMut<NextState<AppState>>,
    query: Query<(Entity, &Health), With<Player>>,
    mut commands: Commands,
) {
    for (entity, health) in &query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
            next_app.set(AppState::Menu);
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


//speed
