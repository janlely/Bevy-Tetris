use bevy::prelude::Res;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use crate::{config::ConfigData, scene};

pub fn can_move_left(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    let left_most = state.current_tetromino.0.left_most_position();
    left_most.iter().all(|position| {
        let tile_pos = TilePos {
            x: (position.x + state.current_position.x - 1) as u32,
            y: (position.y + state.current_position.y) as u32
        };
        position.x + state.current_position.x > 0 && tile_storage.get(&tile_pos).is_none()
    })
}

pub fn can_move_right(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    let right_most = state.current_tetromino.0.right_most_position();
    right_most.iter().all(|position| {
        let tile_pos = TilePos {
            x: (position.x + state.current_position.x + 1) as u32,
            y: (position.y + state.current_position.y) as u32
        };
        position.x + state.current_position.x < 9 && tile_storage.get(&tile_pos).is_none()
    })
}


pub fn can_move_down(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    let down_most = state.current_tetromino.0.down_most_position();
    down_most.iter().all(|position| {
        let tile_pos = TilePos {
            x: (position.x + state.current_position.x) as u32,
            y: (position.y + state.current_position.y - 1) as u32
        };
        position.y + state.current_position.y > 0 && tile_storage.get(&tile_pos).is_none()
    })
}

pub fn hit_bottom(
    state: &scene::GameState,
    config: &ConfigData,
    tile_storage: &TileStorage
) -> bool {
    if state.hit_bottom_timer < config.game_config.step_delay {
        return false;
    }
    let down_most = state.current_tetromino.0.down_most_position();
    down_most.iter().all(|position| {
        let tile_pos = TilePos {
            x: (position.x + state.current_position.x) as u32,
            y: (position.y + state.current_position.y - 1) as u32
        };
        position.y + state.current_position.y > 0 && tile_storage.get(&tile_pos).is_none()
    })
}