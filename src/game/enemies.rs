use crate::game::prelude::*;
use rand::Rng;

use crate::game::ai;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ai::plugin)
        .add_systems(Startup, place_enemy_debug.run_if(in_state(AppState::Game)))
        .add_systems(Update, melee_damage.run_if(in_state(AppState::Game)));
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
    pub direction_array: DirectionArray,
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
            direction_array: DirectionArray([
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ]),
        }
    }
}

fn spawn_ant(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
    let texture: Handle<Image> = asset_server.load("sprites/Ants/ant1_v2.png");
    commands
        .spawn(EnemyBundle {
            ..Default::default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            position.x, position.y, position.z,
        )))
        .insert(texture)
        .insert(Melee(100.0));
}

fn place_enemy_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    for _ in 0..1 {
        let x = rng.gen_range(-20.0..200.0);
        let y = rng.gen_range(-200.0..200.0);

        spawn_ant(&mut commands, &asset_server, Vec3::new(x, y, 0.0));
    }
}

fn melee_damage(
    mut set: ParamSet<(
        Query<(&Transform, &mut Melee)>,
        Query<(&Transform, &mut Health), With<Player>>,
    )>,
) {
    let (player_position, mut player_health_value) = {
        let mut player_query = set.p1();
        let (player_transform, player_health) = player_query.get_single_mut().unwrap();
        (
            Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            ),
            player_health.0,
        )
    };

    for (transform, melee) in set.p0().iter() {
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        if (player_position - position).length() <= melee.0 {
            player_health_value -= 10.0;
        }
    }

    {
        let mut player_query = set.p1();
        let (_, mut player_health) = player_query.get_single_mut().unwrap();
        player_health.0 = player_health_value;
    }
}
