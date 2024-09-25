use crate::game::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), camera_setup)
        .add_systems(
            Update,
            (camera_system, scroll_events).run_if(in_state(AppState::Game)),
        );
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(Game);
}

fn camera_system(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    q_player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = q_player.get_single().unwrap();
    let mut camera_transform = q_camera.get_single_mut().unwrap();
    camera_transform.translation = player_transform.translation;
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
