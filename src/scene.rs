use std::collections::HashSet;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::config::*;
use crate::tetromino::*;


#[derive(Component, Debug)]
pub struct FstPreview;

#[derive(Component, Debug)]
pub struct SndPreview;

#[derive(Resource, Debug)]
pub struct GameState {
    // pub alive: bool,
    // pub paused: bool,
    pub current_tetromino: Tetromino,
    pub next_tetromino: (TetrominoType, usize),
    pub next_tetromino2: (TetrominoType, usize),
    pub current_position: IVec2,
    pub tetromino_entities: HashSet<(u32, u32)>,
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
        5 => (TetrominoType::T, 5),
        6 => (TetrominoType::Z, 6),
        _ => panic!("Invalid random number!")
    }
}

pub fn make_tile(asset_server: &Res<AssetServer>, tetromino_type: TetrominoType) -> SpriteBundle {
    match tetromino_type {
        TetrominoType::I => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Blue.png"),
                ..default()
            }
        },
        TetrominoType::J => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Cyan.png"),
                ..default()
            }
        },
        TetrominoType::L => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Green.png"),
                ..default()
            }
        },
        TetrominoType::O => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Orange.png"),
                ..default()
            }
        },
        TetrominoType::S => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Purple.png"),
                ..default()
            }
        },
        TetrominoType::T => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Red.png"),
                ..default()
            }
        },
        TetrominoType::Z => {
            SpriteBundle {
                sprite: Sprite {
                    ..default()
                },
                texture: asset_server.load("Yellow.png"),
                ..default()
            }
        },
        // _ => panic!("Invalid tetromino type!")

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
        // alive: true,
        // paused: false,
        current_tetromino: Tetromino::new(t.0, t.1),
        next_tetromino: get_rand_tetromino(),
        next_tetromino2: get_rand_tetromino(),
        current_position: IVec2::new(4, 18),
        tetromino_entities: HashSet::new(),
        hit_bottom_timer: 0.0,
        step_timer: 0.0,
        move_timer: 0.0
    }
}

pub fn camera() -> Camera2dBundle {
    Camera2dBundle::default()
}

pub fn calculate_preview_transform(config: &Res<ConfigData>, fst: bool) -> Transform {
    let x = config.game_config.tile_size * config.game_config.scale_factor * 8.0;
    let y = config.game_config.tile_size * config.game_config.scale_factor * (if fst { 8.0 } else { 3.0 });
    Transform::from_scale(Vec3::new(config.game_config.scale_factor, config.game_config.scale_factor, 1.0))
        .with_translation(Vec3::new(x, y, 0.0))
}

pub fn preview_board(asset_server: &Res<AssetServer>, config: &Res<ConfigData>, fst: bool) -> impl Bundle {
    let texture_handle: Handle<Image> = asset_server.load(config.game_config.preview_img.clone());
    SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture: texture_handle,
        transform: calculate_preview_transform(config, fst),
        ..default()
    }
}



pub fn main_board(asset_server: &Res<AssetServer>, config: &Res<ConfigData>) -> impl Bundle {

    let texture_handle: Handle<Image> = asset_server.load(config.game_config.border_img.clone());

    SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture: texture_handle,
        transform: Transform::from_scale(Vec3::new(config.game_config.scale_factor, config.game_config.scale_factor, 1.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.0)), 
        ..default()
    }
}
