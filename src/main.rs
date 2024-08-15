pub mod components;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::*;

const PLAYER_COLOR: Color = Color::srgb(1., 0., 0.);
const PLAYER_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (camera_setup, place_player))
        .add_systems(Update, player_movement)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn place_player(
    mut commands: Commands,
    ){
        commands
                .spawn(Player)
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
                .insert(RigidBody::KinematicPositionBased)
                .insert(Collider::ball(100.0 / 2.0))
                .insert(KinematicCharacterController::default());
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let movement = Some(Vec2::new(direction.x, direction.y) * PLAYER_SPEED * time.delta_seconds());

        transform.translation = movement;
    }
}
