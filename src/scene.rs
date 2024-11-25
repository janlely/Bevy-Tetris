use bevy::{color::palettes::css::GREEN, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::config::*;

#[derive(Default, Component)]
pub struct LastUpdate {
    value: f64,
}

pub fn camera() -> Camera2dBundle {
    Camera2dBundle::default()
}

pub fn preview1_border(asset_server: &Res<AssetServer>, config: &Res<ConfigData>) -> impl Bundle {
    let texture_handle: Handle<Image> = asset_server.load(config.gameConfig.border_img.clone());
    (SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture: texture_handle,
        transform: Transform::from_scale(Vec3::new(config.gameConfig.scale_factor, config.gameConfig.scale_factor, 1.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.0)), 
        ..default()
    }, LastUpdate::default())
}



pub fn main_border(asset_server: &Res<AssetServer>, config: &Res<ConfigData>) -> impl Bundle {

    let texture_handle: Handle<Image> = asset_server.load(config.gameConfig.border_img.clone());

    (SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture: texture_handle,
        transform: Transform::from_scale(Vec3::new(config.gameConfig.scale_factor, config.gameConfig.scale_factor, 1.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.0)), 
        ..default()
    }, LastUpdate::default())
}

pub fn main_tilemap(asset_server: &Res<AssetServer>, config: &Res<ConfigData>) -> impl Bundle {

    let texture_handle: Handle<Image> = asset_server.load(config.gameConfig.tiles_path.clone());
    let map_size = TilemapSize { x: 10, y: 20 };
    let tile_storage = TileStorage::empty(map_size);
    // let mut random = thread_rng();

    // for x in 0..10u32 {
    //     for y in 0..20u32 {
    //         let tile_pos = TilePos { x, y };
    //         let tile_entity = commands
    //             .spawn(TileBundle {
    //                 position: tile_pos,
    //                 tilemap_id: TilemapId(tilemap_entity),
    //                 texture_index: TileTextureIndex(random.gen_range(0..7)),
    //                 ..Default::default()
    //             })
    //             .id();
    //         tile_storage.set(&tile_pos, tile_entity);
    //     }
    // }

    let tile_size = TilemapTileSize { x: config.gameConfig.tile_size, y: config.gameConfig.tile_size };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    // commands.entity(tilemap_entity).insert((
        (TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: calculate_transform(&map_size, &grid_size, &map_type, config.gameConfig.scale_factor, 0.0),
            ..Default::default()
        },
        LastUpdate::default())
    // ));

}


pub fn calculate_transform(
    size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    scale_factor: f32,
    z: f32,
) -> Transform {

    let low = TilePos::new(0, 0).center_in_world(grid_size, map_type);
    let high = TilePos::new(size.x - 1, size.y - 1).center_in_world(grid_size, map_type);

    let diff = high - low;

    let x = -diff.x * scale_factor / 2.0;
    let y = -diff.y * scale_factor / 2.0;
    Transform::from_scale(Vec3::new(scale_factor, scale_factor, 1.0)).with_translation(Vec3::new(x, y, z))
}