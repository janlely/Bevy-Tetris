use std::collections::HashMap;
use std::ops::{Deref};
use bevy::color::Color::Srgba;
use bevy::color::palettes::css::RED;
use bevy::input::ButtonInput;
use bevy::math::IVec2;
use bevy::prelude::{AssetServer, Commands, Entity, KeyCode, Query, Res, ResMut, Resource, SpriteBundle, Time};
use crate::{config, keys, scene, tetromino};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
// use crate::tetromino::{Tetromino, TetrominoType};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    RUNNING,
    PAUSED,
    DEAD
}

pub fn can_move_left(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    let left_most  = state.current_tetromino.left_most_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x - 1 , p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();
    has_no_tile(&left_most, tile_storage)
}

pub fn can_move_right(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    let right_most = state.current_tetromino.right_most_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x + 1, p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();
    has_no_tile(&right_most, tile_storage)
}

fn has_no_tile(
    position: &[IVec2],
    // state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    position.iter().all(|position| {
        let tile_pos = TilePos {
            x: position.x as u32,
            y: position.y as u32
        };
        tile_pos.x >= 0 && tile_pos.x <= 9 && tile_pos.y >= 0 && tile_pos.y <= 19 && tile_storage.get(&tile_pos).is_none()
    })
}

pub fn can_rotate_left(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    can_rotate(state, tile_storage, 3)
}

fn can_rotate(
    state: &scene::GameState,
    tile_storage: &TileStorage,
    i: usize,
) -> bool {
    let rotate = (state.current_tetromino.rotate + i) % 4;
    let rotated_position= state.current_tetromino.get_position2(rotate)
        .iter().map(|p| {
        IVec2::new(p.x + state.current_position.x, p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();
    has_no_tile(&rotated_position, tile_storage)
}

pub fn can_rotate_right(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    can_rotate(state, tile_storage, 1)
}

pub fn can_move_down(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    let down_most: Vec<IVec2> = state.current_tetromino.down_most_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x, p.y + state.current_position.y - 1)
    }).collect();
    has_no_tile(&down_most, tile_storage)
}

#[derive(Resource, Default)]
pub struct EntityContainer {
    pub tilemap: Option<Entity>,
    pub preview1: Option<Entity>,
    pub preview2: Option<Entity>,
}

#[derive(Resource)]
pub struct Tetrominos([SpriteBundle; 7]);

pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    config: Res<ConfigData>,
    time: Res<Time>,
    tetrominos: Res<Tetrominos>,
    mut entity_container: ResMut<EntityContainer>,
) {
    println!("DEBUG: helper::spawn, 97");
    //更新方块
    state.current_position = IVec2::new(4, 18);
    state.current_tetromino = Tetromino::new(state.next_tetromino.0, state.next_tetromino.1);
    state.next_tetromino = state.next_tetromino2;
    state.next_tetromino2 = scene::get_rand_tetromino();
    if let Some(preive1_entity) = entity_container.preview1 {
        commands.entity(preive1_entity).despawn_recursive();
    }
    if let Some(preive2_entity) = entity_container.preview2 {
        commands.entity(preive2_entity).despawn_recursive();
    }
    let preview1_entity = commands.spawn(tetrominos.0[state.next_tetromino.1].clone()).id();
    let preview2_entity = commands.spawn(tetrominos.0[state.next_tetromino2.1].clone()).id();
    entity_container.preview1 = Some(preview1_entity);
    entity_container.preview2 = Some(preview2_entity);
    commands.entity(preview1_entity).insert(scene::calculate_preview_transform(&config, 0));
    commands.entity(preview2_entity).insert(scene::calculate_preview_transform(&config, 2));
    //重置计时器
    state.step_timer = time.elapsed_seconds_f64() + config.game_config.step_delay;
    state.hit_bottom_timer = 0.0;
}

pub fn step_down(
    mut state: ResMut<scene::GameState>,
    time: Res<Time>,
    config: Res<ConfigData>,
    query: Query<&TileStorage>,
) {
    if state.step_timer < time.elapsed_seconds_f64() {
        let tile_storage = query.single();
        if !can_move_down(&state, tile_storage) {
            return;
        }
        state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        state.step_timer = time.elapsed_seconds_f64() + config.game_config.step_delay;
        println!("DEBUG: helper::step_down, 125, y: {}", state.current_position.y);
        state.hit_bottom_timer = 0.0;
    }
}

fn handler_key_event(
    time: Res<Time>,
    mut state: ResMut<scene::GameState>,
    config: Res<ConfigData>,
    query: Query<&TileStorage>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    just_pressed: bool
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.pause.as_str())) {
        state.paused = !state.paused;
        return;
    }

    let tile_storage = query.single();
    let repeat_delay = if just_pressed {config.game_config.first_repeat_delay} else {config.game_config.repeat_delay};
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.left.as_str()))
        && can_move_left(state.deref(), &tile_storage) {
        state.current_position = IVec2::new(state.current_position.x - 1, state.current_position.y);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.right.as_str()))
        && can_move_right(state.deref(), &tile_storage) {
        state.current_position = IVec2::new(state.current_position.x + 1, state.current_position.y);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.down.as_str()))
        && can_move_down(state.deref(), &tile_storage) {
        state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
        state.hit_bottom_timer = 0.0;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.rotate_left.as_str()))
        && can_rotate_left(state.deref(), &tile_storage) {
        state.current_tetromino.rotate_left();
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.rotate_right.as_str()))
        && can_rotate_right(state.deref(), &tile_storage) {
        state.current_tetromino.rotate_right();
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.drop.as_str())) {
        while can_move_down(state.deref(), &tile_storage) {
            println!("DEBUG: helper::handler_key_event, 180, y: {}", state.current_position.y);
            state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        }
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
        state.hit_bottom_timer += config.game_config.step_delay;
    }
}

pub fn handler_key_down(
    time: Res<Time>,
    state: ResMut<scene::GameState>,
    config: Res<ConfigData>,
    query: Query<&TileStorage>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    handler_key_event(time, state, config, query, keyboard_input, true);
}

pub fn handler_key_repeat(
    time: Res<Time>,
    state: ResMut<scene::GameState>,
    config: Res<ConfigData>,
    query: Query<&TileStorage>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    // let tile_storage = query.single();
    if state.move_timer < time.elapsed_seconds_f64() {
        handler_key_event(time, state, config, query, keyboard_input, false);
    }
}

pub fn clear_lines(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    config: Res<ConfigData>,
    mut query: Query<&mut TileStorage>,
    mut query2: Query<&mut TilePos>,
    time: Res<Time>,
    tetrominos: Res<Tetrominos>,
    entity_container: ResMut<EntityContainer>,
) {
    // println!("DEBUG: helper::clear_lines, 224, {:?}, {:?}", state.current_tetromino.get_position(), state.current_position);
    if state.current_tetromino.get_position().iter().any(|p| p.y + state.current_position.y > 19) {
        println!("DEBUG: helper::clear_lines, 226");
        state.alive = false;
        return;
    }
    let mut tile_storage = query.single_mut();
    let Some(lowest_y) = state.current_tetromino.down_most_position().iter().map(|p| p.y + state.current_position.y).min() else {
        println!("DEBUG: helper::clear_lines, 225, state: {:?}", state);
        panic!("should hive lowest.y");
    };

    println!("DEBUG: helper::clear_lines, 232, lowest_y: {}", lowest_y);

    //消除满行
    let mut line_to_remove = lowest_y as u32;
    let mut lines_to_remove = vec![];
    let mut count = 0;
    for _i in 0..4 {
        if line_to_remove < 18 && is_full_line(line_to_remove as u32, &tile_storage) {
            count += 1;
            lines_to_remove.push(line_to_remove);
            //clear_line
            println!("DEBUG: helper::clear_lines, 241, clean_line");
            clear_line(&mut commands, line_to_remove as u32, tile_storage.as_mut());
        }
        line_to_remove += 1;
    }
    //无可消除行
    if count == 0 {
        spawn(commands, state, config, time, tetrominos, entity_container);
        return;
    }

    let mut first_line = lines_to_remove[0];
    // let mut swap: Vec<(u32, u32)> = vec![];
    let mut swap = HashMap::new();
    let mut idx = first_line;
    for i in first_line..20 {
        if !lines_to_remove.contains(&i) {
            swap.insert(i, idx);
            idx += 1;
        }
    }
    for mut pos in query2.iter_mut() {
        if swap.contains_key(&pos.y) {
            let Some(entity) = tile_storage.get(&pos) else {
                panic!("PANIC: helper::clear_lines, 275, x: {}, y: {}", pos.x, pos.y);
            };
            let old_pos = *pos;
            pos.y = swap[&pos.y];
            tile_storage.remove(&old_pos);
            tile_storage.set(&pos, entity);
            let before = tile_storage.get(&old_pos).is_none();
            let after = tile_storage.get(&pos).is_none();
            if !before || after {
                panic!("PANIC: helper::clear_lines, 285");
            }
        }
    }

    spawn(commands, state, config, time, tetrominos, entity_container);
}

// fn move_line_down(
    // commands: &mut Commands,
    // up: u32,
    // down: u32,
    // tile_storage: &mut TileStorage,
    // entity_container: Res<EntityContainer>,
    // mut query: Query<&mut TilePos>,
// ) {
    // for i in 0..10 {
    //     let tile_pos_up = TilePos {
    //         x: i,
    //         y: up
    //     };
    //     let tile_pos_down= TilePos {
    //         x: i,
    //         y: down
    //     };

        // if let Some(tile_entity) = tile_storage.get(&tile_pos_up) {
        //     let tile_entity = commands
        //         .spawn(TileBundle {
        //             position: tile_pos_down,
        //             tilemap_id: TilemapId(entity_container.tilemap.unwrap()),
        //             texture_index: TileTextureIndex(state.current_tetromino.index as u32),
        //             ..Default::default()
        //         })
        //         .id();
        //     tile_storage.set(&tile_pos_down, tile_entity);
        //     tile_storage.remove(&tile_pos_up);
        // }
    // }
// }

fn clear_line(
    commands: &mut Commands,
    line: u32,
    tile_storage: &mut TileStorage
) {
    for i in 0..10 {
        let tile_pos = TilePos {
            x: i,
            y: line
        };
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            commands.entity(tile_entity).despawn();
            tile_storage.remove(&tile_pos);
        }
    }
}


fn is_empty_line(
    line: u32,
    tile_storage: &TileStorage
) -> bool {
    for i in 0..10 {
        let tile_pos = TilePos {
            x: i,
            y: line
        };
        if tile_storage.get(&tile_pos).is_some() {
            return false;
        }
    }
    true
}

fn is_full_line(
    line: u32,
    tile_storage: &TileStorage
) -> bool {
    for i in 0..10 {
        let tile_pos = TilePos {
            x: i,
            y: line
        };
        if tile_pos.y < 0 || tile_pos.y > 19 {
            panic!("DEBUG: helper::is_full_line, 328, x: {}, y: {}", tile_pos.x, tile_pos.y);
        }
        if tile_storage.get(&tile_pos).is_none() {
            return false;
        }
    }
    true
}

pub fn remove_piece(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    mut query: Query<&mut TileStorage>,
    time: Res<Time>
) {
    //触底判定时间更新
    state.hit_bottom_timer += time.delta_seconds_f64();
    let mut tile_storage = query.single_mut();
    for positon in state.current_tetromino.get_position().iter() {
        let tile_pos = TilePos {
            x: (positon.x + state.current_position.x) as u32,
            y: (positon.y + state.current_position.y) as u32
        };
        if tile_pos.x >= 10 || tile_pos.y >= 20 {
            continue;
        }
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            commands.entity(tile_entity).despawn_recursive();
            // Don't forget to remove tiles from the tile storage!
            tile_storage.remove(&tile_pos);
        }
    }
}

pub fn draw_piece(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    entity_container: ResMut<EntityContainer>,
    mut query: Query<&mut TileStorage>
) {
    // println!("DEBUG: helper::draw_piece, 369, x: {}, y: {}", state.current_position.x, state.current_position.y);
    let mut tile_storage = query.single_mut();
    for positon in state.current_tetromino.get_position().iter() {
        let tile_pos = TilePos {
            x: (positon.x + state.current_position.x) as u32,
            y: (positon.y + state.current_position.y) as u32
        };
        if tile_pos.x >= 10 || tile_pos.y >= 20 {
            continue;
        }
        // println!("DEBUG: helper::draw_piece, 353, x: {}, y: {}", tile_pos.x, tile_pos.y);
        // if let Some(_some_entity) = tile_storage.get(&tile_pos) {
        //     state.alive = false;
        //     return;
        // }
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(entity_container.tilemap.unwrap()),
                texture_index: TileTextureIndex(state.current_tetromino.index as u32),
                ..Default::default()
            })
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }
}
pub fn init_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<ConfigData>,
    mut entity_container: ResMut<EntityContainer>
) {

    let tetrominos = Tetrominos([
        scene::make_sprite(&asset_server, TetrominoType::I),
        scene::make_sprite(&asset_server, TetrominoType::J),
        scene::make_sprite(&asset_server, TetrominoType::L),
        scene::make_sprite(&asset_server, TetrominoType::O),
        scene::make_sprite(&asset_server, TetrominoType::S),
        scene::make_sprite(&asset_server, TetrominoType::T),
        scene::make_sprite(&asset_server, TetrominoType::Z),
    ]);

    commands.insert_resource(tetrominos);
    //相机
    commands.spawn(scene::camera());
    //游戏区域tile_map
    let tilemap_entity= commands.spawn_empty().id();
    commands.entity(tilemap_entity).insert(scene::main_tilemap(&asset_server, &config));
    //游戏区域边框
    commands.spawn(scene::main_board(&asset_server, &config));
    //方块预览区1边框
    commands.spawn(scene::preview_board(&asset_server, &config, 0));
    commands.spawn(scene::preview_board(&asset_server, &config, 1));
    entity_container.tilemap = Some(tilemap_entity);
}

pub fn should_run(state: Res<scene::GameState>) -> bool {
    // If our barrier isn't ready, return early and wait another cycle
    !state.paused && state.alive
}

pub fn hit_bottom(
    state: Res<scene::GameState>,
    query: Query<&TileStorage>,
    config: Res<ConfigData>,
) -> bool {
    let tile_storage = query.single();
    let should1 = !can_move_down(&state, tile_storage);
    let should2 = state.hit_bottom_timer >= config.game_config.step_delay;
    // println!("DEBUG: helper::should_clear, 435, should1: {}, hit_timer: {}, should2: {}", should1, state.hit_bottom_timer, should2);
    should1 && should2
}

pub fn game_over(
    mut commands: Commands,
    state: ResMut<scene::GameState>,
) {
    if state.alive {
        return;
    }

    let text_style = TextStyle {
        color: Srgba(RED),
        ..default()
    };
    commands.spawn(
        TextBundle::from_section("Game Over", text_style).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
    );
}

pub fn reinit(
    mut state: ResMut<scene::GameState>,
    time: Res<Time>,
    config: Res<config::ConfigData>
) {

    //重置触底判定时间
    state.hit_bottom_timer = 0.0;
    //重置方块自动下降时间
    state.step_timer = time.elapsed_seconds_f64() + config.game_config.step_delay;
    //重置方块位置
    state.current_position = IVec2::new(4, 19);

}