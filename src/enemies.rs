use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, place_enemy_debug);
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub speed: Speed,
    pub health: Health,
    pub sprite_bundle: SpriteBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub controler: KinematicCharacterController,
    pub enemy_marker: Enemy,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            health: Health(100.0),
            speed: Speed(400.0),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(0.0, 200.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 60.0)),
                    ..Default::default()
                },
                ..default()
            },
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::ball(10.0 / 2.0),
            controler: KinematicCharacterController::default(),
            enemy_marker: Enemy,
        }
    }
}

fn spawn_default_enemy(commands: &mut Commands) {
    commands.spawn(EnemyBundle {
        ..Default::default()
    });
}

fn place_enemy_debug(mut commands: Commands) {
    spawn_default_enemy(&mut commands);
}
