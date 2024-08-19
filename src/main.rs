pub mod components;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::*;

const PLAYER_COLOR: Color = Color::srgb(1., 0., 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (camera_setup, place_player))
        .add_systems(Update, (player_movement))
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn place_player(mut commands: Commands) {
    commands
        .spawn(Player)
        .insert(Speed(500.0))
        .insert(Health(100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(100.0 / 2.0))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(KinematicCharacterController::default());
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut KinematicCharacterController, &mut Transform), With<Player>>,
    speed_query: Query<&Speed, With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if let Ok((mut controller, mut transform)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut rotation_direction = 0.0;

        for key in keyboard_input.get_pressed() {
            match key {
                KeyCode::ArrowLeft | KeyCode::KeyA => direction += Vec3::new(-1.0, 0.0, 0.0),
                KeyCode::ArrowRight | KeyCode::KeyD => direction += Vec3::new(1.0, 0.0, 0.0),
                KeyCode::ArrowUp | KeyCode::KeyW => direction += Vec3::new(0.0, 1.0, 0.0),
                KeyCode::ArrowDown | KeyCode::KeyS => direction += Vec3::new(0.0, -1.0, 0.0),
                KeyCode::KeyZ => rotation_direction += 1.0,
                KeyCode::KeyX => rotation_direction -= 1.0,
                KeyCode::Space => shoot(&mut commands, &transform),
                _ => (),
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let speed = speed_query.get_single().unwrap().0;
        let movement = Some(Vec2::new(direction.x, direction.y) * speed * time.delta_seconds());

        controller.translation = movement;
        transform.rotation *= Quat::from_rotation_z(rotation_direction * 5.0 * time.delta_seconds());
    }
}

fn shoot(commands: &mut Commands, player_transform: &Transform) {
    // Calculate bullet spawn position
    let forward_direction = player_transform.rotation * Vec3::Y;
    let bullet_spawn_position = player_transform.translation + forward_direction * 100.0;

    // Calculate bullet velocity
    let bullet_velocity = forward_direction * 500.0;

    // Spawn the bullet
    commands
        .spawn(Bullet)
        .insert(TransformBundle::from(Transform::from_xyz(
            bullet_spawn_position.x,
            bullet_spawn_position.y,
            0.0,
        )))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.0 / 2.0))
        .insert(Velocity {
            linvel: Vec2::new(bullet_velocity.x, bullet_velocity.y),
            angvel: 0.,
        })
        .insert(GravityScale(0.0));
}
