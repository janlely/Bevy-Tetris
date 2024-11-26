use bevy::prelude::Res;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use crate::scene;

pub fn can_move_left(
    state: Res<scene::GameState>,
    tile_storage: &TileStorage
) -> bool {
    state.current_tetromino.0.positions[state.current_tetromino.0.rotate as usize].iter().all(|position| {
        position.x + state.current_position.x > -5
    }) && state.current_tetromino.0.left_most_position().iter().all(|position| {
        let tile_pos = TilePos {x: position.x - 1 + state.current_position, y: position.y + state.current_position};
        tile_storage.get(&tile_pos).is_none()
    })
}