use crate::game::map::pathfinding::*;
use crate::game::map::*;

pub fn spawn_tile(
    tilesets: &Res<Tilesets>,
    commands: &mut Commands,
    atlas_name: &str,
    sprite_index: usize,
    position: Vec3,
) -> Entity {
    if let Some((layout_handle, texture_handle)) = tilesets.atlases.get(atlas_name) {
        commands
            .spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                TextureAtlas {
                    layout: layout_handle.clone(),
                    index: sprite_index,
                },
                Game,
            ))
            .id()
    } else {
        eprintln!("Tileset '{}' not found", atlas_name);
        commands.spawn_empty().id()
    }
}

pub fn spawn_wall(
    tilesets: &Res<Tilesets>,
    commands: &mut Commands,
    atlas_name: &str,
    sprite_index: usize,
    position: Vec3,
) -> Entity {
    let entity = spawn_tile(tilesets, commands, atlas_name, sprite_index, position);
    commands
        .entity(entity)
        .insert(Collider::cuboid(8.0, 8.0))
        .insert(Obstacle);

    entity
}

pub fn spawn_tree(
    tilesets: &Res<Tilesets>,
    commands: &mut Commands,
    atlas_name: &str,
    sprite_indices: (usize, usize),
    position: Vec3,
) {
    let trunk_entity = spawn_wall(tilesets, commands, atlas_name, sprite_indices.0, position);
    let top_position = position + Vec3::new(16.0, 0.0, 0.0);
    let top_entity = spawn_tile(tilesets, commands, atlas_name, sprite_indices.1, top_position);
    commands.entity(trunk_entity).add_child(top_entity);
}

