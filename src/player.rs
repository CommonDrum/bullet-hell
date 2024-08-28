//player.rs

use crate::components::*;
use crate::bullets::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (camera_setup, place_player))
        .add_systems(
            Update,
            (player_movement, camera_system, scroll_events, shoot, player_rotation),
        );
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

//TODO make the values here into constants also maybe put input handeling into separate file
fn place_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Player)
        .insert(Speed(500.0))
        .insert(Health(100.0))
        .insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 60.0)),
                ..Default::default()
            },
            texture: asset_server.load("sprites/Soldier 1/soldier1_gun.png"),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)))
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
) {
    if let Ok((mut controller, mut transform)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        for key in keyboard_input.get_pressed() {
            match key {
                KeyCode::ArrowLeft | KeyCode::KeyA => direction += Vec3::new(-1.0, 0.0, 0.0),
                KeyCode::ArrowRight | KeyCode::KeyD => direction += Vec3::new(1.0, 0.0, 0.0),
                KeyCode::ArrowUp | KeyCode::KeyW => direction += Vec3::new(0.0, 1.0, 0.0),
                KeyCode::ArrowDown | KeyCode::KeyS => direction += Vec3::new(0.0, -1.0, 0.0),
                _ => (),
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let speed = speed_query.get_single().unwrap().0;
        let movement = Some(Vec2::new(direction.x, direction.y) * speed * time.delta_seconds());

        controller.translation = movement;
    }
}

//TODO: Most of the variables need to be moved either
// to resources or spawning function for different types of bullets.

fn shoot(
    mut commands: Commands,
    player_transform_q: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let player_transform = player_transform_q.get_single().unwrap();
    let forward_direction = player_transform.rotation * Vec3::Y;
    let bullet_spawn_position = player_transform.translation + forward_direction * 60.0;

    let bullet_velocity = forward_direction * 500.0;

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn(BulletBundle{
            bullet_spawn_position,
            Velocity {
                    linvel: Vec2::new(bullet_velocity.x, bullet_velocity.y),
                    angvel: 0.,
                },
            ..Default::default()
        });
    }
}

fn camera_system(
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    //This is a very nice way to see how borrow checker works. I first have to get the value and
    //drop the reference and move to the other mutable reference.
    let player_translation = {
        let binding_1 = param_set.p1();
        let player_transform = binding_1.get_single().unwrap();
        player_transform.translation
    };

    let mut binding_0 = param_set.p0();
    let mut camera_transform = binding_0.get_single_mut().unwrap();
    camera_transform.translation = player_translation;
}

fn scroll_events(
    mut evr_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for ev in evr_scroll.read() {
        let scroll_amount = match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y * 0.1,
        };

        for mut projection in query.iter_mut() {
            let mut log_scale = projection.scale.ln();
            log_scale -= scroll_amount * 0.1;
            projection.scale = log_scale.exp();
        }
    }
}



fn player_rotation(
    mut q_transform: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mut transform = q_transform.get_single_mut().unwrap();
    let window = q_windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let screen_center = window_size / 2.0;

        let difference =screen_center - cursor_position;
        let angle = difference.x.atan2(difference.y);

        transform.rotation = Quat::from_rotation_z(angle);
    }
}

