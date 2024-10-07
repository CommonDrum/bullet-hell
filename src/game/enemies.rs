use crate::game::prelude::*;
use rand::Rng;

use crate::game::ai;
use crate::game::map::pathfinding::DirectionArray;
use crate::game::map::pathfinding::Obstacle;
use crate::game::map::pathfinding::Path;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ai::plugin)
        .add_systems(OnEnter(AppState::Game), place_enemy_debug)
        .add_systems(Update, melee_damage.run_if(in_state(AppState::Game)));
}

pub fn spawn_enemy(commands: &mut Commands, texture: Handle<Image>, position: Vec3) -> Entity {
    let transform = Transform::from_translation(position);
    let enemy = spawn_actor(
        Collider::ball(8.0),
        Size(16.0),
        Health(100.0),
        Speed(70.0),
        transform,
        commands,
    );

    commands.entity(enemy).insert((
        SpriteBundle {
            texture,
            transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Enemy,
        DirectionArray([0.0; 16]),
        Obstacle,
        Path(Vec::new()),
        AiMode::Chase,
    ));

    enemy
}

fn spawn_ant(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) -> Entity {
    let texture: Handle<Image> = asset_server.load("sprites/Ants/ant1_v2.png");
    let entity = spawn_enemy(commands, texture, position);
    commands.entity(entity).insert(Melee(24.0));
    entity
}

fn place_enemy_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
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
