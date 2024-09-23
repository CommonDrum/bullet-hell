use crate::game::prelude::*;

pub const BASIC_SIZE_IN_VIEWPORT: f32 = 50.0;
const BACKGROUND_LAYER: f32 = -1.1;
const MAP_SIZE: i32 = 50;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), place_background);
}

pub fn get_grid_coords() {}

pub fn get_viewport_cords(x: i32, y: i32) -> (f32, f32) {
    (
        x as f32 * BASIC_SIZE_IN_VIEWPORT,
        y as f32 * BASIC_SIZE_IN_VIEWPORT,
    )
}

//TODO: Create a tile bundle and functions to create each type of tile.
fn place_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    for y in -MAP_SIZE..=MAP_SIZE {
        for x in -MAP_SIZE..=MAP_SIZE {
            let (viewport_x, viewport_y) = get_viewport_cords(x, y);
            if x % 50 == 0 && x != 0 || y % 50 == 0 && y != 0 {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(
                                BASIC_SIZE_IN_VIEWPORT,
                                BASIC_SIZE_IN_VIEWPORT,
                            )),
                            ..Default::default()
                        },
                        texture: asset_server.load("sprites/Tiles/tile_102.png"),
                        ..default()
                    })
                    .insert(TransformBundle::from(Transform::from_xyz(
                        viewport_x,
                        viewport_y,
                        BACKGROUND_LAYER,
                    )))
                    .insert(RigidBody::KinematicPositionBased)
                    .insert(Collider::cuboid(25.0, 25.0))
                    .insert(LockedAxes::TRANSLATION_LOCKED)
                    .insert(Game);
            } else {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(
                                BASIC_SIZE_IN_VIEWPORT,
                                BASIC_SIZE_IN_VIEWPORT,
                            )),
                            ..Default::default()
                        },
                        texture: asset_server.load("sprites/Tiles/tile_101.png"),
                        ..default()
                    })
                    .insert(TransformBundle::from(Transform::from_xyz(
                        viewport_x,
                        viewport_y,
                        BACKGROUND_LAYER,
                    )))
                    .insert(Game);
            }
        }
    }
}
