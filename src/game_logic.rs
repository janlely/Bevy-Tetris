use std::collections::{HashMap };
use bevy::color::Color::Srgba;
use bevy::color::palettes::css::RED;
use bevy::input::ButtonInput;
use bevy::math::IVec2;
use bevy::prelude::{AssetServer, Commands, Entity, KeyCode, Query, Res, ResMut, Resource, SpriteBundle, Time};
use crate::{config, keys, scene, tetromino};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy::prelude::*;
use bevy::reflect::Map;
use bevy_ecs_tilemap::prelude::*;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
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
        UVec2::new((p.x + state.current_position.x - 1) as u32 , (p.y + state.current_position.y) as u32)
    }).collect::<Vec<UVec2>>();
    has_no_tile(&left_most, tile_storage)
}

pub fn can_move_right(
    state: &scene::GameState,
    tile_storage: &TileStorage
) -> bool {
    let right_most = state.current_tetromino.right_most_position().iter().map(|p| {
        UVec2::new((p.x + state.current_position.x + 1) as u32, (p.y + state.current_position.y) as u32)
    }).collect::<Vec<UVec2>>();
    has_no_tile(&right_most, tile_storage)
}

fn has_no_tile(
    position: &[UVec2],
    // state: &scene::GameState,
    tile_storage: &TileStorage,
) -> bool {
    position.iter().all(|position| {
        let tile_pos = TilePos {
            x: position.x,
            y: position.y
        };
        (tile_pos.x >= 0 && tile_pos.x <= 9 && tile_pos.y > 19) || tile_storage.get(&tile_pos).is_none()
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
    let org_position = state.current_tetromino.get_position();
    let rotated_position= state.current_tetromino.get_position2(rotate)
        .iter().filter(|p| !org_position.contains(p)).map(|p| {
        UVec2::new((p.x + state.current_position.x) as u32, (p.y + state.current_position.y) as u32)
    }).collect::<Vec<UVec2>>();
    println!("DEBUG: game_logic::can_rotate: {:?}", rotated_position);
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
    let down_most: Vec<UVec2> = state.current_tetromino.down_most_position().iter().map(|p| {
        UVec2::new((p.x + state.current_position.x) as u32, (p.y + state.current_position.y - 1) as u32)
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

//生成方块
pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    time: Res<Time>,
    tetrominos: Res<Tetrominos>,
    mut entity_container: ResMut<EntityContainer>,
) {
    println!("DEBUG: helper::spawn, 97");
    //重置方块位置，设置成最上面
    state.current_position = IVec2::new(4, 18);
    //使用预览区1的方块创建游戏方块
    state.current_tetromino = tetromino::Tetromino::new(state.next_tetromino.0, state.next_tetromino.1);
    //预览区2的方块提升到预览区1，预览区2生成新方块
    state.next_tetromino = state.next_tetromino2;
    state.next_tetromino2 = scene::get_rand_tetromino();
    //删除预览区的方块精灵
    if let Some(preive1_entity) = entity_container.preview1 {
        commands.entity(preive1_entity).despawn_recursive();
    }
    if let Some(preive2_entity) = entity_container.preview2 {
        commands.entity(preive2_entity).despawn_recursive();
    }
    //重新生成新的预览区方块精灵
    let preview1_entity = commands.spawn(tetrominos.0[state.next_tetromino.1].clone()).id();
    let preview2_entity = commands.spawn(tetrominos.0[state.next_tetromino2.1].clone()).id();
    //保存预览区精灵的句柄
    entity_container.preview1 = Some(preview1_entity);
    entity_container.preview2 = Some(preview2_entity);
    //设置预览区精灵的位置
    commands.entity(preview1_entity).insert(scene::calculate_preview_transform(&config, 0));
    commands.entity(preview2_entity).insert(scene::calculate_preview_transform(&config, 2));
    //重置计时器
    state.step_timer = 0.0;
    state.move_timer = time.elapsed_seconds_f64() + config.game_config.first_repeat_delay;
}

pub fn step_down(
    mut state: ResMut<scene::GameState>,
    time: Res<Time>,
    config: Res<config::ConfigData>,
    query: Query<&TileStorage>,
) {
    if state.step_timer >= config.game_config.step_delay {
        // println!("DEBUG: game_logic::step_down");
        let tile_storage = query.single();
        if !can_move_down(&state, tile_storage) {
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
    query: Query<&TileStorage>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    just_pressed: bool,
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.pause.as_str())) {
        println!("DEBUG: game_logic:handler_key_even, pause");
        next_state.set(AppState::PAUSED);
        return;
    }

    let tile_storage = query.single();
    let repeat_delay = if just_pressed {config.game_config.first_repeat_delay} else {config.game_config.repeat_delay};
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.left.as_str()))
        && can_move_left(&state, &tile_storage) {
        println!("DEBUG: game_logic:handler_key_even, left");
        state.current_position = IVec2::new(state.current_position.x - 1, state.current_position.y);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.right.as_str()))
        && can_move_right(&state, &tile_storage) {
        println!("DEBUG: game_logic:handler_key_even, right");
        state.current_position = IVec2::new(state.current_position.x + 1, state.current_position.y);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.down.as_str()))
        && can_move_down(&state, &tile_storage) {
        println!("DEBUG: game_logic:handler_key_even, down");
        state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
        state.hit_bottom_timer = 0.0;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.rotate_left.as_str()))
        && can_rotate_left(&state, &tile_storage) {
        println!("DEBUG: game_logic:handler_key_even, rotate left");
        state.current_tetromino.rotate_left();
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.rotate_right.as_str()))
        && can_rotate_right(&state, &tile_storage) {
        println!("DEBUG: game_logic:handler_key_even, rotate right");
        state.current_tetromino.rotate_right();
        state.move_timer = time.elapsed_seconds_f64() + repeat_delay;
    }
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.drop.as_str())) {
        println!("DEBUG: game_logic:handler_key_even, drop");
        while can_move_down(&state, &tile_storage) {
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
    config: Res<config::ConfigData>,
    query: Query<&TileStorage>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    next_state: ResMut<NextState<AppState>>
) {
    handler_key_event(time, state, config, query, keyboard_input, next_state, true);
}

pub fn handler_key_repeat(
    time: Res<Time>,
    state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    query: Query<&TileStorage>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    next_state: ResMut<NextState<AppState>>
) {
    // let tile_storage = query.single();
    if state.move_timer < time.elapsed_seconds_f64() {
        handler_key_event(time, state, config, query, keyboard_input, next_state, false);
    }
}

pub fn clear_lines(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    mut query: Query<&mut TileStorage>,
    mut query2: Query<&mut TilePos>,
) {

    let mut tile_storage = query.single_mut();


    // println!("DEBUG: helper::clear_lines, 224, {:?}, {:?}", state.current_tetromino.get_position(), state.current_position);
    if state.current_tetromino.get_position().iter().any(|p| p.y + state.current_position.y > 19) {
        println!("DEBUG: helper::clear_lines, 226");
        state.alive = false;
        return;
    }
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
        // spawn(commands, state, config, time, tetrominos, entity_container);
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
}


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

//删除游戏方块
pub fn remove_piece(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    query: Query<&TileStorage>,
    time: Res<Time>
) {
    let tile_storage = query.single();
    //遍历当前方块的位置，并更新tilemap
    for positon in state.current_tetromino.get_position().iter() {
        let tile_pos = TilePos {
            x: (positon.x + state.current_position.x) as u32,
            y: (positon.y + state.current_position.y) as u32
        };
        if tile_pos.x >= 10 || tile_pos.y >= 20 {
            continue;
        }
        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
            // println!("DEBUG: game_logic::remove_piece");
            // commands.entity(tile_entity).despawn_recursive();
            state.tetromino_entities.insert((tile_pos.x, tile_pos.y), tile_entity);
            // tile_storage.remove(&tile_pos);
        }
    }
}

//绘制游戏方块
pub fn draw_piece(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    entity_container: ResMut<EntityContainer>,
    mut query: Query<&mut TileStorage>,
    mut p_query: Query<&mut TilePos>,
    mut next_state: ResMut<NextState<AppState>>,
    config: Res<config::ConfigData>,
) {
    // println!("DEBUG: helper::draw_piece, x: {}, y: {}", state.current_position.x, state.current_position.y);
    let mut tile_storage = query.single_mut();
    //获取方块移动的目标位置
    let positions = state.current_tetromino.get_position().iter().map(|p| {
        UVec2::new((p.x + state.current_position.x) as u32, (p.y + state.current_position.y) as u32)
    }).collect::<Vec<UVec2>>();

    //spawn出来的方块必须不能有占位
    if state.tetromino_entities.is_empty() && !has_no_tile(&positions, &tile_storage) {
        next_state.set(AppState::DEAD);
        return;
    }

    //原始与位置与目标位置重叠，则无需移动，删除这些重叠的配对
    let mut positions_to_keep = vec![];
    for position in positions.iter() {
        let pos = &(position.x, position.y);
        //目标位置可以在原始位置中找到，说明不需要移动
        if state.tetromino_entities.contains_key(pos) {
            state.tetromino_entities.remove(pos);
        } else {
            //需要移动则把目标地址保留
            if position.y >= 0 && position.y <= 19 {
                positions_to_keep.push(*position);
            }
        }
    }

    //把原始位置的方块移动到目标位置
    for mut pos in p_query.iter_mut() {
        if state.tetromino_entities.contains_key(&(pos.x, pos.y)) && !positions_to_keep.is_empty(){
            // println!("DEBUG: game_logic::draw_piece, move tiles");
            let old_pos = *pos;
            let position = positions_to_keep.pop().unwrap();
            pos.x = position.x;
            pos.y = position.y;
            tile_storage.remove(&old_pos);
            tile_storage.set(&&pos, state.tetromino_entities.remove(&(old_pos.x, old_pos.y)).unwrap());
        }
    }
    //原始位置的的方块应该已经全部都移动了
    if (!state.tetromino_entities.is_empty()) {
        panic!("不应该还有方块没有移动");
    }

    //如果positions不为空，说明还有方块需要新生成
    // println!("DEBUG: game_logic::draw_piece, 483");
    for pos in positions_to_keep.iter() {
        let tile_pos = TilePos {
            x: pos.x,
            y: pos.y
        };
        if tile_pos.x >= 10 || tile_pos.y >= 20 {
            continue;
        }
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(entity_container.tilemap.unwrap()),
                texture_index: TileTextureIndex(state.current_tetromino.index as u32),
                ..Default::default()
            })
            .id();
        // println!("DEBUG: game_logic::draw_piece, draw new piece");
        tile_storage.set(&tile_pos, tile_entity);
    }
}

pub fn init_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<config::ConfigData>,
    mut entity_container: ResMut<EntityContainer>
) {

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
    //把tilemap_entity插入到resource，方便以后的system使用
    entity_container.tilemap = Some(tilemap_entity);
}

pub fn should_run(state: Res<scene::GameState>) -> bool {
    // If our barrier isn't ready, return early and wait another cycle
    !state.paused && state.alive
}

pub fn hit_bottom(
    state: Res<scene::GameState>,
    query: Query<&TileStorage>,
    config: Res<config::ConfigData>,
) -> bool {
    let tile_storage = query.single();
    let hit = hit_bottom2(&state, tile_storage, config);
    if hit {
        println!("DEBUG: game_logic::hit_bottom: {}", hit);
    }
    hit
}

pub fn hit_bottom2 (
    state: &scene::GameState,
    tile_storage: &TileStorage,
    config: Res<config::ConfigData>,
)  -> bool {
    !can_move_down(&state, tile_storage) && state.hit_bottom_timer >= config.game_config.step_delay
}

pub fn resume (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<config::ConfigData>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.pause.as_str())) {
        // state.paused = !state.paused;
        next_state.set(AppState::RUNNING);
    }
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
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TileStorage>,
    mut p_query: Query<&mut TilePos>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.restart.as_str())) {
        //删除所有的方块
        let mut tile_storage = query.single_mut();
        for pos in p_query.iter() {
            if let Some(entity) = tile_storage.get(pos) {
                commands.entity(entity).despawn();
                tile_storage.remove(&pos);
            }
        }
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

