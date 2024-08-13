pub mod components;

use bevy::prelude::*;
use components::*;

const PLAYER_COLOR: Color = Color::srgb(1., 0., 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera_setup, place_player))
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
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: PLAYER_COLOR,
                        ..default()
                    },
                    ..default()
                });
}
