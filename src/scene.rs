use bevy::{color::palettes::css::GREEN, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::config::*;
use crate::tetromino::*;

#[derive(Default, Component)]
pub struct LastUpdate {
    value: f64,
}

#[derive(Resource)]
pub struct GameState {
    pub alive: bool,
    pub paused: bool,
    pub started: bool,
    pub current_tetromino: Tetromino,
    pub next_tetromino: (TetrominoType, usize),
    pub next_tetromino2: (TetrominoType, usize),
    pub current_position: IVec2,
    pub step_timer: f64,
    pub move_timer: f64,
    pub hit_bottom_timer: f64
}

pub fn get_rand_tetromino() -> (TetrominoType, usize) {
    let mut random = thread_rng();
    let num = random.gen_range(0..7);
    match num {
        0 => (TetrominoType::I, 0),
        1 => (TetrominoType::J, 1),
        2 => (TetrominoType::L, 2),
        3 => (TetrominoType::O, 3),
        4 => (TetrominoType::S, 4),
        5 => (TetrominoType::T, 4),
        6 => (TetrominoType::Z, 6),
        _ => panic!("Invalid random number!")
    }
}


pub fn make_sprite(asset_server: &Res<AssetServer>, tetromino_type: TetrominoType) -> SpriteBundle {
    match tetromino_type {
        TetrominoType::I => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("I.png"),
                ..default()
            }
        },
        TetrominoType::J => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("J.png"),
                ..default()
            }
        },
        TetrominoType::L => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("L.png"),
                ..default()
            }
        },
        TetrominoType::O => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("O.png"),
                ..default()
            }
        },
        TetrominoType::S => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("S.png"),
                ..default()
            }
        },
        TetrominoType::T => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("T.png"),
                ..default()
            }
        },
        TetrominoType::Z => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Z.png"),
                ..default()
            }
        },
        // _ => panic!("Invalid tetromino type!")

    }
} 

pub fn init_game_state() -> GameState {
    let t = get_rand_tetromino();
    GameState {
        alive: true,
        paused: false,
        started: false,
        current_tetromino: Tetromino::new(t.0, t.1),
        next_tetromino: get_rand_tetromino(),
        next_tetromino2: get_rand_tetromino(),
        current_position: IVec2::new(0, 9),
        hit_bottom_timer: 0.0,
        step_timer: 0.0,
        move_timer: 0.0
    }
}

pub fn camera() -> Camera2dBundle {
    Camera2dBundle::default()
}

fn calculate_preview_transform(config: &Res<ConfigData>, i: i32) -> Transform {
    let x = config.game_config.tile_size * config.game_config.scale_factor * 8.0;
    let y = config.game_config.tile_size * config.game_config.scale_factor * (if i == 0 { 8.0 } else { 3.0 });
    Transform::from_scale(Vec3::new(config.game_config.scale_factor, config.game_config.scale_factor, 1.0))
        .with_translation(Vec3::new(x, y, 0.0))
}

pub fn preview_board(asset_server: &Res<AssetServer>, config: &Res<ConfigData>, i: i32) -> impl Bundle {
    let texture_handle: Handle<Image> = asset_server.load(config.game_config.preview_img.clone());
    (SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture: texture_handle,
        transform: calculate_preview_transform(config, i),
        ..default()
    }, LastUpdate::default())
}



pub fn main_board(asset_server: &Res<AssetServer>, config: &Res<ConfigData>) -> impl Bundle {

    let texture_handle: Handle<Image> = asset_server.load(config.game_config.border_img.clone());

    (SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture: texture_handle,
        transform: Transform::from_scale(Vec3::new(config.game_config.scale_factor, config.game_config.scale_factor, 1.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.0)), 
        ..default()
    }, LastUpdate::default())
}

pub fn main_tilemap(asset_server: &Res<AssetServer>, config: &Res<ConfigData>) -> impl Bundle {

    let texture_handle: Handle<Image> = asset_server.load(config.game_config.tiles_path.clone());
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

    let tile_size = TilemapTileSize { x: config.game_config.tile_size, y: config.game_config.tile_size };
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
            transform: calculate_transform(&map_size, &grid_size, &map_type, config.game_config.scale_factor, 0.0),
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