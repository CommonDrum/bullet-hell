use crate::game::bullets::*;
use crate::game::prelude::*;
use bevy::window::PrimaryWindow;

mod camera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin)
        .add_systems(OnEnter(AppState::Game), place_player)
        .add_systems(
            Update,
            (player_movement, shoot, player_flip).run_if(in_state(AppState::Game)),
        );
}

pub fn place_player(mut commands: Commands, tilesets: Res<Tilesets>) {
    let atlas_name = "player";
    let sprite_index = 21;
    let position = Vec3::new(50.0, 200.0, 0.0);
    let transform = Transform::from_translation(position);

    if let Some((layout_handle, texture_handle)) = tilesets.atlases.get(atlas_name) {
        let player = spawn_actor(
            Collider::ball(8.0),
            Size(16.0),
            Health(100.0),
            Speed(270.0),
            transform,
            &mut commands,
        );

        commands.entity(player).insert((
            Player,
            SpriteBundle {
                texture: texture_handle.clone(),
                transform,
                ..default()
            },
            TextureAtlas{
                layout: layout_handle.clone(),
                index: sprite_index,
                ..Default::default()
            },
        ));
    } else {
        eprintln!("Tileset '{}' not found", atlas_name);
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    speed_query: Query<&Speed, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut controller) = player_query.get_single_mut() {
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

fn shoot(
    mut commands: Commands,
    player_transform_q: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    let player_transform = player_transform_q.get_single().unwrap();
    let forward_direction = player_transform.rotation * Vec3::Y;
    let bullet_spawn_position_1 = player_transform.translation + forward_direction * 60.0;
    let bullet_spawn_position = Vec2::new(bullet_spawn_position_1.x, bullet_spawn_position_1.y);
    let position = Vec3::new(bullet_spawn_position.x, bullet_spawn_position.y, 0.0);

    let bullet_velocity = forward_direction * 500.0;

    if keyboard_input.pressed(KeyCode::Space) | buttons.just_pressed(MouseButton::Left) {
        spawn_default_bullet(&mut commands, &asset_server, position, bullet_velocity);
        {}
    }
}

fn player_flip(
    mut q_transform: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mut transform = q_transform.get_single_mut().unwrap();
    let window = q_windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let screen_center = window_size / 2.0;

        let difference = screen_center - cursor_position;

        if difference.x > 0.0 {
            transform.scale.x = -transform.scale.x.abs(); // Flip left
        } else {
            transform.scale.x = transform.scale.x.abs(); // Flip right
        }
    }
}
