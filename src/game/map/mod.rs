use crate::game::prelude::*;

pub mod pathfinding;
mod tiles;

use crate::game::map::pathfinding::Path;
use crate::game::map::pathfinding::*;
use crate::game::map::tiles::*;
use bevy::color::palettes::css::*;

pub const PIXELS_PER_TILE: f32 = 16.0;
const BACKGROUND_LAYER: f32 = -1.1;
const MAP_SIZE: i32 = 50;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Game),
        (place_background, add_pathfinder, place_the_wall),
    )
    .add_event::<SetPathEvent>()
    .add_systems(
        Update,
        (update_obstacles, update_path, visualize_path).run_if(in_state(AppState::Game)),
    );
}

pub fn get_viewport_cords(x: i32, y: i32) -> (f32, f32) {
    (x as f32 * PIXELS_PER_TILE, y as f32 * PIXELS_PER_TILE)
}

pub fn viewport_to_pos(x: f32, y: f32) -> Pos {
    Pos(
        (x / PIXELS_PER_TILE).floor() as i32,
        (y / PIXELS_PER_TILE).floor() as i32,
    )
}

pub fn pos_to_viewport(pos: &Pos) -> Vec2 {
    Vec2::new(
        pos.0 as f32 * PIXELS_PER_TILE,
        pos.1 as f32 * PIXELS_PER_TILE,
    )
}

fn add_pathfinder(mut commands: Commands) {
    let pathfinder = Pathfinder::new(MAP_SIZE);
    let pos = viewport_to_pos(50.0, 200.0);
    println!("{},{}", pos.0, pos.1);

    commands.insert_resource(pathfinder);
}

pub fn place_background(mut commands: Commands, tilesets: Res<Tilesets>) {
    for y in -MAP_SIZE..=MAP_SIZE {
        for x in -MAP_SIZE..=MAP_SIZE {
            let (viewport_x, viewport_y) = get_viewport_cords(x, y);
            let position = Vec3::new(viewport_x, viewport_y, BACKGROUND_LAYER);

            if (x % MAP_SIZE == 0 && x != 0) || (y % MAP_SIZE == 0 && y != 0) {
                spawn_wall(&tilesets, &mut commands, "forest", 48, position);
            } else {
                spawn_tile(&tilesets, &mut commands, "forest", 48, position);
            }
        }
    }
}

fn place_the_wall(mut commands: Commands, tilesets: Res<Tilesets>) {
    for x in -MAP_SIZE..=MAP_SIZE {
        let (viewport_x, viewport_y) = get_viewport_cords(x, 0);
        let position = Vec3::new(viewport_x, viewport_y, BACKGROUND_LAYER);

        if (0..=3).contains(&x) {
            continue;
        }
        spawn_wall(&tilesets, &mut commands, "forest", 20, position);
    }
}

fn update_obstacles(
    mut pathfinder: ResMut<Pathfinder>,
    q_obstacles: Query<(&Transform, Entity), With<Obstacle>>,
) {
    for (transform, entity) in q_obstacles.iter() {
        let pos = viewport_to_pos(transform.translation.x, transform.translation.y);
        if !pathfinder.obstacles.contains_key(&entity) {
            pathfinder.add_obstacle(entity, pos);
        }
    }
}

fn visualize_path(mut gizmos: Gizmos, q_paths: Query<&Path>) {
    for path in q_paths.iter() {
        for (pos, _) in &path.0 {
            let viewport_pos = pos_to_viewport(pos);
            gizmos.circle_2d(viewport_pos, 5.0, NAVY);
        }
    }
}
