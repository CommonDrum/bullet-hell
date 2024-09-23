use crate::game::prelude::*;
use std::collections::HashMap;

mod asset_list;
use crate::game::preloader::asset_list::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (preload_assets, spawn_forest_sprite).chain());
}

fn preload_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut tilesets = Tilesets::new();

    for (name, path, cols, rows) in ASSET_LIST.iter() {
        let texture_handle = asset_server.load(*path);
        let tile_size = UVec2::new(16, 16);
        let layout = TextureAtlasLayout::from_grid(tile_size, *cols, *rows, None, None);
        let layout_handle = texture_atlas_layouts.add(layout);

        tilesets.atlases.insert(
            (*name).to_string(),
            (layout_handle.clone(), texture_handle.clone()),
        );
    }

    commands.insert_resource(tilesets);
}

#[derive(Resource)]
struct Tilesets {
    atlases: HashMap<String, (Handle<TextureAtlasLayout>, Handle<Image>)>,
}

impl Tilesets {
    fn new() -> Self {
        Self {
            atlases: HashMap::new(),
        }
    }
}

fn spawn_forest_sprite(tilesets: Res<Tilesets>, mut commands: Commands) {
    if let Some((layout_handle, texture_handle)) = tilesets.atlases.get("forest") {
        commands.spawn((
            SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..default()
            },
            TextureAtlas {
                layout: layout_handle.clone(),
                index: 20, // Index of the sprite in the atlas you want to display
            },
        ));
    } else {
        eprintln!("Tileset 'forest' not found");
    }
}
