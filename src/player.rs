//player.rs

use crate::bullets::*;
use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, place_player)
        .add_systems(Update, (player_movement, shoot, player_rotation));
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
        commands.spawn(BulletBundle {
            transform: TransformBundle::from(Transform::from_xyz(
                bullet_spawn_position.x,
                bullet_spawn_position.y,
                0.0,
            )),
            velocity: Velocity {
                linvel: Vec2::new(bullet_velocity.x, bullet_velocity.y),
                angvel: 0.,
            },
            ..Default::default()
        });
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

        let difference = screen_center - cursor_position;
        let angle = difference.x.atan2(difference.y);

        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            println!(
                "Collision started between Entity {:?} and Entity {:?}",
                entity1, entity2
            );
            commands.entity(*entity2).despawn();
        }
    }
}
