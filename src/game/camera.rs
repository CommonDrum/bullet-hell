use crate::game::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

/* This is a camera plugin that uses player tag component to follow the player around.
 * it also has zooming in and out capabilities*/

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, camera_setup)
        .add_systems(Update, (camera_system, scroll_events).run_if(in_state(GameState::Game)));
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
