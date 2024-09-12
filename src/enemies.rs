use crate::prelude::*;
use rand::Rng;

use crate::ai;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ai::plugin)
        .add_systems(Startup, place_enemy_debug);
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
    pub ai_mode: AiMode,
    pub destination: Destination,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            health: Health(100.0),
            speed: Speed(80.0),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(0.0, 200.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 60.0)),
                    ..Default::default()
                },
                ..default()
            },
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::ball(25.0),
            controler: KinematicCharacterController::default(),
            enemy_marker: Enemy,
            ai_mode: AiMode::Passive,
            destination: Destination(Vec3::new(0.0, 200.0, 0.0)),
        }
    }
}

fn spawn_default_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
    let texture: Handle<Image> = asset_server.load("sprites/Soldier 1/soldier1_gun.png");
    commands
        .spawn(EnemyBundle {
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            position.x, position.y, position.z,
        )))
        .insert(texture);
}

fn place_enemy_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x = rng.gen_range(-500.0..500.0);
        let y = rng.gen_range(-500.0..500.0);

        spawn_default_enemy(&mut commands, &asset_server, Vec3::new(x, y, 0.0));
    }
}
