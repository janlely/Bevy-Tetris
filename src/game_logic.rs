use std::any::{Any, TypeId};
use std::collections::{HashSet};
// use std::os::macos::raw::stat;
// use std::os::unix::process::parent_id;
use bevy::color::Color::Srgba;
use bevy::color::palettes::css::RED;
use bevy::ecs::observer::TriggerTargets;
use bevy::input::ButtonInput;
use bevy::math::IVec2;
use bevy::prelude::{AssetServer, Commands, Entity, KeyCode, Query, Res, ResMut, Resource, SpriteBundle, Time};
use crate::{config, keys, scene, tetromino};
// use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy::prelude::*;
// use bevy::reflect::Map;
// use bevy_ecs_tilemap::prelude::*;
use crate::scene::{FstPreview, SndPreview};

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    RUNNING,
    PAUSED,
    DEAD
}


pub fn can_move_left(
    state: &scene::GameState,
    tile_board: &TileBoard
) -> bool {
    let left_most  = state.current_tetromino.left_most_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x - 1 , p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();
    has_no_tile(&left_most, tile_board)
}

pub fn can_move_right(
    state: &scene::GameState,
    tile_board: &TileBoard
) -> bool {
    let right_most = state.current_tetromino.right_most_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x + 1, p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();
    has_no_tile(&right_most, tile_board)
}

fn has_no_tile(
    position: &[IVec2],
    tile_board: &TileBoard
    // tile_storage: &TileStorage,
) -> bool {
    position.iter().all(|p| {
        let above = p.x >= 0 && p.x <= 9 && p.y > 19;
        let in_and_empty = p.x >= 0 && p.x <= 9 && p.y >= 0 && p.y <= 19 && tile_board.entity_at(p.x as u32,p.y as u32).is_none();
        above || in_and_empty
    })
}

pub fn can_rotate_left(
    state: &scene::GameState,
    tile_board: &TileBoard
) -> bool {
    can_rotate(state, tile_board, 3)
}

fn can_rotate(
    state: &scene::GameState,
    tile_board: &TileBoard,
    i: usize,
) -> bool {
    let rotate = (state.current_tetromino.rotate + i) % 4;
    let org_position = state.current_tetromino.get_position();
    let rotated_position= state.current_tetromino.get_position2(rotate)
        .iter().filter(|p| !org_position.contains(p)).map(|p| {
        IVec2::new(p.x + state.current_position.x, p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();
    // println!("DEBUG: game_logic::can_rotate: {:?}", rotated_position);
    has_no_tile(&rotated_position, tile_board)
}

pub fn can_rotate_right(
    state: &scene::GameState,
    tile_board: &TileBoard,
) -> bool {
    can_rotate(state, tile_board, 1)
}

pub fn can_move_down(
    state: &scene::GameState,
    tile_board: &TileBoard,
) -> bool {
    let down_most: Vec<IVec2> = state.current_tetromino.down_most_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x, p.y + state.current_position.y - 1)
    }).collect();
    // println!("DEBUG: game_logic::can_move_down, down_most: {:?}", down_most);
    has_no_tile(&down_most, tile_board)
}

#[derive(Resource, Default)]
pub struct EntityContainer {
    pub tilemap: Option<Entity>,
    // pub preview1: Option<Entity>,
    // pub preview2: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct TileBoard {
    pub tile_map: Vec<Option<Entity>>,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub tile_size: f32
}

impl TileBoard {
    pub fn new (width: u32, height: u32, scale_factor: f32, tile_size: f32) -> Self {
        Self {
            tile_map: vec![Default::default(); (width * height) as usize],
            width,
            height,
            scale_factor,
            tile_size
        }
    }

    pub fn get_all_entitys(&self) -> Vec<Entity> {
        let mut result = vec![];
        for entity in &self.tile_map {
            if let Some(entity) = entity {
                result.push(*entity);
            }
        }
        result
    }
    
    pub fn remove(&mut self, x: u32, y: u32) -> Option<Entity> {
        let idx = y * self.width + x;
        if let Some(_) = self.tile_map.get(idx as usize) {
            let result = self.tile_map[idx as usize].unwrap();
            self.tile_map[idx as usize] = None;
            return Some(result);
        }
        None
    }

    pub fn set(&mut self, x: u32, y: u32, entity: Entity) -> Transform {
        self.tile_map[(y * self.width + x) as usize] = Some(entity);
        self.get_position((x,y))
    }

    pub fn clear(&mut self) {
        self.tile_map = vec![Default::default(); (self.width * self.height) as usize];
    }

    pub fn entity_at(&self, x: u32, y: u32) -> Option<&Entity> {
        let idx = (y * self.width + x) as usize;
        self.tile_map[idx].as_ref()
    }

    pub fn swap_tile(&mut self, from: (u32, u32), to: (u32, u32)) -> (&Entity, Transform) {
        let from_idx = (from.1 * self.width + from.0) as usize;
        let to_idx = (to.1 * self.width + to.0) as usize;
        self.tile_map.swap(from_idx, to_idx);
        
        (self.tile_map[to_idx].as_ref().unwrap(), self.get_position(to))
    }

    fn get_position(&self, pos: (u32, u32)) -> Transform {
        let x = (pos.0 as f32 - self.width as f32 / 2.0 + 0.5) * self.tile_size * self.scale_factor;
        let y = (pos.1 as f32 - self.height as f32 / 2.0 + 0.5) * self.tile_size * self.scale_factor;
        
        Transform::from_scale(Vec3::new(self.scale_factor, self.scale_factor, 1.0)).with_translation(Vec3::new(x, y, 1.0))
    }
}

#[derive(Resource)]
pub struct Tetrominos([SpriteBundle; 7]);
#[derive(Resource)]
pub struct Tiles([SpriteBundle; 7]);

//生成方块
pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    time: Res<Time>,
    tetrominos: Res<Tetrominos>,
    mut p1_query: Query<Entity, With<scene::FstPreview>>,
    mut p2_query: Query<Entity, With<scene::SndPreview>>,
    // mut entity_container: ResMut<EntityContainer>,
) {
    // println!("DEBUG: helper::spawn, 97");
    //重置方块位置，设置成最上面
    state.current_position = IVec2::new(4, 18);
    //使用预览区1的方块创建游戏方块
    // println!("DEBUG: before spawned, type: {:?}, index: {}", state.current_tetromino.tetromino_type, state.current_tetromino.index);
    state.current_tetromino = tetromino::Tetromino::new(state.next_tetromino.0, state.next_tetromino.1);
    // println!("DEBUG: after spawned, type: {:?}, index: {}", state.current_tetromino.tetromino_type, state.current_tetromino.index);

    //预览区2的方块提升到预览区1，预览区2生成新方块
    state.next_tetromino = state.next_tetromino2;
    state.next_tetromino2 = scene::get_rand_tetromino();
    // println!("DEBUG: next1, type: {:?}, index: {}", state.next_tetromino.0, state.next_tetromino.1);
    // println!("DEBUG: next2, type: {:?}, index: {}", state.next_tetromino2.0, state.next_tetromino2.1);
    //删除预览区的方块精灵
    if let Ok(p1_entity) = p1_query.get_single_mut() {
        commands.entity(p1_entity).despawn();
    }
    if let Ok(p2_entity) = p2_query.get_single_mut() {
        commands.entity(p2_entity).despawn();
    }
    // if let Some(preive1_entity) = entity_container.preview1 {
    //     commands.entity(preive1_entity).despawn_recursive();
    // }
    // if let Some(preive2_entity) = entity_container.preview2 {
    //     commands.entity(preive2_entity).despawn_recursive();
    // }
    //重新生成新的预览区方块精灵
    commands.spawn(tetrominos.0[state.next_tetromino.1].clone())
        .insert(scene::calculate_preview_transform(&config, true))
        .insert(FstPreview);
    commands.spawn(tetrominos.0[state.next_tetromino2.1].clone())
        .insert(scene::calculate_preview_transform(&config, false))
        .insert(SndPreview);
    //保存预览区精灵的句柄
    // entity_container.preview1 = Some(preview1_entity);
    // entity_container.preview2 = Some(preview2_entity);
    //设置预览区精灵的位置
    // commands.entity(preview1_entity)
    //     .insert(scene::calculate_preview_transform(&config, 0));
    // commands.entity(preview2_entity).insert(scene::calculate_preview_transform(&config, 2));
    //重置计时器
    state.step_timer = 0.0;
    state.move_timer = time.elapsed_seconds_f64() + config.game_config.first_repeat_delay;
    println!("DEBUG: spawn: now: {:?}, next1: {:?}, next2: {:?}",
             state.current_tetromino.tetromino_type,
             state.next_tetromino.0,
             state.next_tetromino2.0);
}

pub fn step_down(
    mut state: ResMut<scene::GameState>,
    time: Res<Time>,
    config: Res<config::ConfigData>,
    tile_board: Res<TileBoard>,
) {
    if state.step_timer >= config.game_config.step_delay {
        // println!("DEBUG: game_logic::step_down");
        // let tile_storage = query.single();
        if !can_move_down(&state, &tile_board) {
            return;
        }
        state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        // println!("DEBUG: helper::step_down, 125, y: {}", state.current_position.y);
        state.hit_bottom_timer = 0.0;
        state.step_timer = 0.0;
    }
}

fn handler_key_event(
    time: Res<Time>,
    mut state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    tile_board: Res<TileBoard>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    just_pressed: bool,
) {
    let key_detector: Box<dyn Fn(KeyCode) -> bool>= if just_pressed {
        Box::new(move |x| keyboard_input.just_pressed(x))
    } else {
        Box::new(move |x| keyboard_input.pressed(x))
    };
    if just_pressed && key_detector(keys::from_str(config.keys_config.pause.as_str())) {
        println!("DEBUG: game_logic:handler_key_even, pause");
        next_state.set(AppState::PAUSED);
        return;
    }

    // let tile_storage = query.single();
    let repeat_delay = if just_pressed {config.game_config.first_repeat_delay} else {config.game_config.repeat_delay};
    if key_detector(keys::from_str(config.keys_config.left.as_str()))
        && can_move_left(&state, &tile_board) {
        println!("DEBUG: game_logic:handler_key_even, left");
        state.current_position = IVec2::new(state.current_position.x - 1, state.current_position.y);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.right.as_str()))
        && can_move_right(&state, &tile_board) {
        // println!("DEBUG: game_logic:handler_key_even, right");
        state.current_position = IVec2::new(state.current_position.x + 1, state.current_position.y);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.down.as_str()))
        && can_move_down(&state, &tile_board) {
        // println!("DEBUG: game_logic:handler_key_even, down");
        state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
        state.hit_bottom_timer = 0.0;
    }
    if key_detector(keys::from_str(config.keys_config.rotate_left.as_str()))
        && can_rotate_left(&state, &tile_board) {
        // println!("DEBUG: game_logic:handler_key_even, rotate left");
        state.current_tetromino.rotate_left();
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.rotate_right.as_str()))
        && can_rotate_right(&state, &tile_board) {
        // println!("DEBUG: game_logic:handler_key_even, rotate right");
        state.current_tetromino.rotate_right();
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.drop.as_str())) {
        // println!("DEBUG: game_logic:handler_key_even, drop: {:?}", state.current_tetromino.get_position());
        while can_move_down(&state, &tile_board) {
            // println!("DEBUG: helper::handler_key_event, 180, y: {}", state.current_position.y);
            state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        }
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
        state.hit_bottom_timer += config.game_config.step_delay;
    }
}

pub fn handler_key_down(
    time: Res<Time>,
    state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    tile_board: Res<TileBoard>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    next_state: ResMut<NextState<AppState>>
) {
    handler_key_event(time, state, config, tile_board, keyboard_input, next_state, true);
}

pub fn handler_key_repeat(
    time: Res<Time>,
    state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    tile_board: Res<TileBoard>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    next_state: ResMut<NextState<AppState>>
) {
    // let tile_storage = query.single();
    if state.move_timer < time.elapsed_seconds_f64() {
        handler_key_event(time, state, config, tile_board, keyboard_input, next_state, false);
    }
}

pub fn clear_lines(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    // mut query: Query<&mut TileStorage>,
    // mut query2: Query<&mut TilePos>,
    mut tile_board: ResMut<TileBoard>,
    entity_container: Res<EntityContainer>,
    // world: &mut World
) {

    // let mut tile_storage = query.single_mut();


    // println!("DEBUG: helper::clear_lines, 224, {:?}, {:?}", state.current_tetromino.get_position(), state.current_position);
    // if state.current_tetromino.get_position().iter().any(|p| p.y + state.current_position.y > 19) {
    //     println!("DEBUG: helper::clear_lines, 226");
    //     state.alive = false;
    //     return;
    // }
    let Some(lowest_y) = state.current_tetromino.down_most_position().iter().map(|p| p.y + state.current_position.y).min() else {
        println!("DEBUG: helper::clear_lines, 225, state: {:?}", state);
        panic!("should hive lowest.y");
    };

    // println!("DEBUG: helper::clear_lines, 232, lowest_y: {}", lowest_y);

    //消除满行
    let mut line_to_remove = lowest_y as u32;
    let mut lines_to_remove = vec![];
    let mut count = 0;
    for _i in 0..4 {
        if line_to_remove < 18 && is_full_line(line_to_remove, &tile_board) {
            count += 1;
            lines_to_remove.push(line_to_remove);
            //clear_line
            // println!("DEBUG: helper::clear_lines, 241, clean_line");
            clear_line(&mut commands, line_to_remove, tile_board.as_mut());
        }
        line_to_remove += 1;
    }
    //无可消除行
    if count == 0 {
        // spawn(commands, state, config, time, tetrominos, entity_container);
        return;
    }

    println!("DEBUG: lines_to_remove: {:?}", lines_to_remove);
    let mut first_line = lines_to_remove[0];
    let mut swap: Vec<(u32, u32)> = vec![];
    // let mut swap = HashMap::new();
    let mut idx = first_line;
    for i in first_line..20 {
        if !lines_to_remove.contains(&i) {
            // swap.insert(i, idx);
            println!("DEBUG: to swap: {}, {}", i, idx);
            swap.push((i, idx));
            idx += 1;
        }
    }
    for (y1, y2) in swap.iter() {
        for i in 0..10 {
            let from_pos = (i, *y1);
            let to_pos = (i, *y2);
            if let Some(_) = tile_board.entity_at(i, *y1) {
                println!("DEBUG: from_pos: {:?}, to_pos: {:?}", from_pos, to_pos);
                let (&entity, pos) = tile_board.swap_tile(from_pos, to_pos);
                commands.entity(entity).insert(pos);
            }
        }
    }
    // let mut tile_positions: Vec<_> = query2.iter_mut().collect();
    // tile_positions.sort_by(|a, b| {
    //     a.y.cmp(&b.y)
    // });
    // for mut pos in tile_positions {
    //     if swap.contains_key(&pos.y) {
    //         let Some(entity) = tile_storage.get(&pos) else {
    //             panic!("PANIC: helper::clear_lines, 275, x: {}, y: {}", pos.x, pos.y);
    //         };
    //         let old_pos = *pos;
    //         pos.y = swap[&pos.y];
    //         tile_storage.remove(&old_pos);
    //         tile_storage.set(&pos, entity);
    //         println!("DEBUG: from: {:?}, to: {:?}", old_pos, pos);
    //         let before = tile_storage.get(&old_pos).is_none();
    //         let after = tile_storage.get(&pos).is_none();
    //         if !before || after {
    //             panic!("PANIC: helper::clear_lines, 285");
    //         }
    //     }
    // }
}


fn clear_line(
    commands: &mut Commands,
    line: u32,
    tile_board: &mut TileBoard
    // tile_storage: &mut TileStorage
) {
    for i in 0..10 {
        // let tile_pos = TilePos {
        //     x: i,
        //     y: line
        // };
        if let Some(tile_entity) = tile_board.remove(i, line) {
            commands.entity(tile_entity).despawn_recursive();
        } else {
            panic!("DEBUG: no enity. x: {}, y:{}", i, line);
        }
    }
}


fn is_empty_line(
    line: u32,
    tile_board: Res<TileBoard>,
) -> bool {
    for i in 0..10 {
        // let tile_pos = TilePos {
        //     x: i,
        //     y: line
        // };
        if tile_board.entity_at(i, line).is_some() {
            return false;
        }
    }
    true
}

fn is_full_line(
    line: u32,
    tile_board: &TileBoard
) -> bool {
    for i in 0..10 {
        // let tile_pos = TilePos {
        //     x: i,
        //     y: line
        // };
        if line < 0 || line > 19 {
            panic!("DEBUG: helper::is_full_line, 328, x: {}, y: {}", i, line);
        }
        if tile_board.entity_at(i, line).is_none() {
            return false;
        }
        
        // if tile_storage.get(&tile_pos).is_none() {
        //     return false;
        // }
    }
    true
}

//删除游戏方块，实则仅记录方块原始位置，在后面draw_piece时进行方块移动
pub fn remove_piece(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    tile_board: Res<TileBoard>,
    time: Res<Time>
) {
    // let tile_storage = query.single();
    //遍历当前方块的位置，并更新tilemap
    for positon in state.current_tetromino.get_position().iter() {
        let tile_pos = (
            (positon.x + state.current_position.x) as u32,
            (positon.y + state.current_position.y) as u32
        );
        if tile_pos.0 >= 10 || tile_pos.1 >= 20 {
            continue;
        }
        if let Some(_) = tile_board.entity_at(tile_pos.0, tile_pos.1) {
            // println!("DEBUG: game_logic::remove_piece");
            // commands.entity(tile_entity).despawn_recursive();
            state.tetromino_entities.insert((tile_pos.0, tile_pos.1));
            // tile_storage.remove(&tile_pos);
        }
    }
}

//绘制游戏方块
pub fn draw_piece(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    // entity_container: ResMut<EntityContainer>,
    // mut query: Query<&mut TileStorage>,
    tiles: Res<Tiles>,
    mut tile_board: ResMut<TileBoard>,
    // mut p_query: Query<&mut TilePos>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // println!("DEBUG: helper::draw_piece, x: {}, y: {}", state.current_position.x, state.current_position.y);
    // let mut tile_storage = query.single_mut();
    //获取方块移动的目标位置
    let positions = state.current_tetromino.get_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x, p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();

    //spawn出来的方块必须不能有占位
    if state.tetromino_entities.is_empty() && !has_no_tile(&positions, &tile_board) {
        // println!("DEBUG: type: {:?}, index: {},  position: {:?}", state.current_tetromino.tetromino_type, state.current_tetromino.index , positions);
        // println!("DEBUG: empyt: {}, has_no_tile: {}", state.tetromino_entities.is_empty(), has_no_tile(&positions, &tile_storage));
        next_state.set(AppState::DEAD);
        return;
    }
    // println!("DEBUG: map: {:?}", state.tetromino_entities);

    //原始与位置与目标位置重叠，则无需移动，删除这些重叠的配对
    let mut positions_to_keep = vec![];
    for position in positions.iter() {
        let pos = &(position.x as u32, position.y as u32);
        //目标位置可以在原始位置中找到，说明不需要移动
        if state.tetromino_entities.contains(pos) {
            state.tetromino_entities.remove(pos);
        } else {
            //需要移动则把目标地址保留
            if position.y >= 0 && position.y <= 19 {
                positions_to_keep.push(*position);
            }
        }
    }
    // println!("DEBUG: p2k: {:?}", positions_to_kep);
    for from_pos in state.tetromino_entities.drain() {
        if !positions_to_keep.is_empty() {
            let v2 = positions_to_keep.pop().unwrap();
            println!("DEBUG: from: {:?}, to: {:?}", from_pos, v2);
            let (&entity, pos) = tile_board.swap_tile(from_pos, (v2.x as u32, v2.y as u32));
            commands.entity(entity).insert(pos);
        } else {
            panic!("DEBUG: 不应该还有原始方块未移动");
        }
    }

    //把原始位置的方块移动到目标位置
    // for mut pos in p_query.iter_mut() {
    //     if state.tetromino_entities.contains_key(&(pos.x, pos.y)) && !positions_to_keep.is_empty(){
    //         // println!("DEBUG: game_logic::draw_piece, move tiles");
    //         let old_pos = *pos;
    //         let position = positions_to_keep.pop().unwrap();
    //         pos.x = position.x as u32;
    //         pos.y = position.y as u32;
    //         tile_storage.remove(&old_pos);
    //         tile_storage.set(&&pos, state.tetromino_entities.remove(&(old_pos.x, old_pos.y)).unwrap());
    //     }
    // }
    //原始位置的的方块应该已经全部都移动了
    // for (key,entity) in state.tetromino_entities.drain() {
    //     commands.entity(entity).despawn();
    //     tile_storage.remove(&TilePos{x: key.0, y: key.1});
    // }

    //如果positions不为空，说明还有方块需要新生成
    // println!("DEBUG: game_logic::draw_piece, 483");
    for pos in positions_to_keep.iter() {
        if pos.x >= 10 || pos.y >= 20 {
            continue;
        }
        let entity = commands.spawn_empty().id();
        let transfrom = tile_board.set(pos.x as u32, pos.y as u32, entity);
        // println!("DEBUG: new tile at: {:?}", transfrom);
        commands.entity(entity)
        .insert(tiles.0[state.current_tetromino.index].clone())
        .insert(transfrom);

    }
}

pub fn init_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<config::ConfigData>,
    // mut entity_container: ResMut<EntityContainer>
) {


    commands.insert_resource(TileBoard::new(
        10,
        20,
        config.game_config.scale_factor,
        config.game_config.tile_size
    ));

    //预览区域要显示的方块精灵
    let tetrominos = Tetrominos([
        scene::make_sprite(&asset_server, tetromino::TetrominoType::I),
        scene::make_sprite(&asset_server, tetromino::TetrominoType::J),
        scene::make_sprite(&asset_server, tetromino::TetrominoType::L),
        scene::make_sprite(&asset_server, tetromino::TetrominoType::O),
        scene::make_sprite(&asset_server, tetromino::TetrominoType::S),
        scene::make_sprite(&asset_server, tetromino::TetrominoType::T),
        scene::make_sprite(&asset_server, tetromino::TetrominoType::Z),
    ]);

    //插入到resource中，供后续的system使用
    commands.insert_resource(tetrominos);
    //tile
    // let images: [Handle<Image>; 7] = [
    //     asset_server.load("0.png"),
    //     asset_server.load("1.png"),
    //     asset_server.load("2.png"),
    //     asset_server.load("3.png"),
    //     asset_server.load("4.png"),
    //     asset_server.load("5.png"),
    //     asset_server.load("6.png"),
    // ];
    let tiles = Tiles([
        scene::make_tile(&asset_server, tetromino::TetrominoType::I),
        scene::make_tile(&asset_server, tetromino::TetrominoType::J),
        scene::make_tile(&asset_server, tetromino::TetrominoType::L),
        scene::make_tile(&asset_server, tetromino::TetrominoType::O),
        scene::make_tile(&asset_server, tetromino::TetrominoType::S),
        scene::make_tile(&asset_server, tetromino::TetrominoType::T),
        scene::make_tile(&asset_server, tetromino::TetrominoType::Z),
    ]);
    commands.insert_resource(tiles);
    //相机
    commands.spawn(scene::camera());
    //游戏区域tile_map
    // let tilemap_entity= commands.spawn_empty().id();
    // commands.entity(tilemap_entity).insert(scene::main_tilemap(&asset_server, &config));
    //游戏区域边框
    commands.spawn(scene::main_board(&asset_server, &config));
    //方块预览区1边框
    commands.spawn(scene::preview_board(&asset_server, &config, true));
    commands.spawn(scene::preview_board(&asset_server, &config, false));
    //把tilemap_entity插入到resource，方便以后的system使用
    // entity_container.tilemap = Some(tilemap_entity);
}

// pub fn should_run(state: Res<scene::GameState>) -> bool {
//     // If our barrier isn't ready, return early and wait another cycle
//     !state.paused && state.alive
// }

pub fn hit_bottom(
    state: Res<scene::GameState>,
    // query: Query<&TileStorage>,
    tile_board: Res<TileBoard>,
    config: Res<config::ConfigData>,
) -> bool {
    // let tile_storage = query.single();
    // hit_bottom2(&state, tile_storage, config)
    !can_move_down(&state, &tile_board) && state.hit_bottom_timer >= config.game_config.step_delay
}


pub fn resume (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<config::ConfigData>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.pause.as_str())) {
        // state.paused = !state.paused;
        println!("DEBUG: resumed");
        next_state.set(AppState::RUNNING);
    }
}


pub fn game_over(
    mut commands: Commands,
    state: ResMut<scene::GameState>,
) {
    // if state.alive {
    //     return;
    // }

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
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tile_board: ResMut<TileBoard>,
    // mut query: Query<&mut TileStorage>,
    // mut p_query: Query<&mut TilePos>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.restart.as_str())) {
        for entity in tile_board.get_all_entitys() {
            commands.entity(entity).despawn();
        }
        tile_board.clear();
        //删除所有的方块
        // let mut tile_storage = query.single_mut();
        // for pos in p_query.iter() {
        //     if let Some(entity) = tile_storage.get(pos) {
        //         commands.entity(entity).despawn();
        //         tile_storage.remove(&pos);
        //     }
        // }
        //重置触底判定时间
        state.hit_bottom_timer = 0.0;
        //重置方块自动下降时间
        state.step_timer = 0.0;
        //重置方块位置
        state.current_position = IVec2::new(4, 19);
        //重新开始
        next_state.set(AppState::RUNNING);
    }
}

pub fn update_timer(
    mut state: ResMut<scene::GameState>,
    time: Res<Time>
) {
    // println!("DEBUG: game_logic::update_timer");
    state.hit_bottom_timer += time.delta_seconds_f64();
    state.step_timer += time.delta_seconds_f64();
}


// fn assert_tile_valid(
//     tile_storage: &TileStorage,
//     tile_pos: &[TilePos]
// ) {
//     for pos in tile_pos {
//         if tile_storage.get(pos).is_none() {
//             panic!("DEBUG: assert_tile_valid: tile not found, x: {}, y: {}", pos.x, pos.y);
//         }
//     }
// }

pub fn print_board(
    // query: Query<&TileStorage>,
    tile_board: Res<TileBoard>,
    // p_query: Query<&TilePos>,
) {
    // for pos in p_query.iter() {
    //     print!("({},{}) ", pos.x, pos.y);
    // }
    // let tile_storage = query.single();
    for y in (0..20).rev() {
        for x in 0..10 {
            if tile_board.entity_at(x, y).is_some() {
                print!("*");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
    print!("\n\n\n")
}