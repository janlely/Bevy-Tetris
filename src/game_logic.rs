use bevy::input::ButtonInput;
use bevy::math::IVec2;
use bevy::prelude::{AssetServer, Commands, Entity, KeyCode, Query, Res, ResMut, Resource, Time};
use crate::{config, keys, scene, tetromino};
use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

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
    has_no_tile(&down_most, tile_board)
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
pub struct Tetrominos([Sprite; 7]);
#[derive(Resource)]
pub struct Tiles([Sprite; 7]);

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
    //重置方块位置，设置成最上面
    state.current_position = IVec2::new(4, 18);
    //使用预览区1的方块创建游戏方块
    state.current_tetromino = tetromino::Tetromino::new(state.next_tetromino.0, state.next_tetromino.1);

    //预览区2的方块提升到预览区1，预览区2生成新方块
    state.next_tetromino = state.next_tetromino2;
    state.next_tetromino2 = scene::get_rand_tetromino();
    //删除预览区的方块精灵
    if let Ok(p1_entity) = p1_query.get_single_mut() {
        commands.entity(p1_entity).despawn();
    }
    if let Ok(p2_entity) = p2_query.get_single_mut() {
        commands.entity(p2_entity).despawn();
    }
    //重新生成新的预览区方块精灵
    commands.spawn(tetrominos.0[state.next_tetromino.1].clone())
        .insert(scene::calculate_preview_transform(&config, true))
        .insert(FstPreview);
    commands.spawn(tetrominos.0[state.next_tetromino2.1].clone())
        .insert(scene::calculate_preview_transform(&config, false))
        .insert(SndPreview);
    //重置计时器
    state.step_timer = 0.0;
    state.move_timer = time.elapsed_secs_f64() + config.game_config.first_repeat_delay;
}

pub fn step_down(
    mut state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    tile_board: Res<TileBoard>,
) {
    if state.step_timer >= config.game_config.step_delay {
        if !can_move_down(&state, &tile_board) {
            return;
        }
        state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
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
        next_state.set(AppState::PAUSED);
        return;
    }

    let repeat_delay = if just_pressed {config.game_config.first_repeat_delay} else {config.game_config.repeat_delay};
    if key_detector(keys::from_str(config.keys_config.left.as_str()))
        && can_move_left(&state, &tile_board) {
        state.current_position = IVec2::new(state.current_position.x - 1, state.current_position.y);
        state.move_timer = time.elapsed_secs_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.right.as_str()))
        && can_move_right(&state, &tile_board) {
        state.current_position = IVec2::new(state.current_position.x + 1, state.current_position.y);
        state.move_timer = time.elapsed_secs_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.down.as_str()))
        && can_move_down(&state, &tile_board) {
        state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        state.move_timer = time.elapsed_secs_f64() + repeat_delay;
        state.hit_bottom_timer = 0.0;
    }
    if key_detector(keys::from_str(config.keys_config.rotate_left.as_str()))
        && can_rotate_left(&state, &tile_board) {
        state.current_tetromino.rotate_left();
        state.move_timer = time.elapsed_secs_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.rotate_right.as_str()))
        && can_rotate_right(&state, &tile_board) {
        state.current_tetromino.rotate_right();
        state.move_timer = time.elapsed_secs_f64() + repeat_delay;
    }
    if key_detector(keys::from_str(config.keys_config.drop.as_str())) {
        while can_move_down(&state, &tile_board) {
            state.current_position = IVec2::new(state.current_position.x, state.current_position.y - 1);
        }
        state.move_timer = time.elapsed_secs_f64() + repeat_delay;
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
    if state.move_timer < time.elapsed_secs_f64() {
        handler_key_event(time, state, config, tile_board, keyboard_input, next_state, false);
    }
}

pub fn clear_lines(
    mut commands: Commands,
    state: ResMut<scene::GameState>,
    mut tile_board: ResMut<TileBoard>,
) {

    let Some(lowest_y) = state.current_tetromino.down_most_position().iter().map(|p| p.y + state.current_position.y).min() else {
        println!("DEBUG: helper::clear_lines, 225, state: {:?}", state);
        panic!("should hive lowest.y");
    };


    //消除满行
    let mut line_to_remove = lowest_y as u32;
    let mut lines_to_remove = vec![];
    let mut count = 0;
    for _i in 0..4 {
        if line_to_remove < 18 && is_full_line(line_to_remove, &tile_board) {
            count += 1;
            lines_to_remove.push(line_to_remove);
            clear_line(&mut commands, line_to_remove, tile_board.as_mut());
        }
        line_to_remove += 1;
    }
    //无可消除行
    if count == 0 {
        return;
    }

    let first_line = lines_to_remove[0];
    let mut swap: Vec<(u32, u32)> = vec![];
    // let mut swap = HashMap::new();
    let mut idx = first_line;
    for i in first_line..20 {
        if !lines_to_remove.contains(&i) {
            // swap.insert(i, idx);
            swap.push((i, idx));
            idx += 1;
        }
    }
    for (y1, y2) in swap.iter() {
        for i in 0..10 {
            let from_pos = (i, *y1);
            let to_pos = (i, *y2);
            if let Some(_) = tile_board.entity_at(i, *y1) {
                let (&entity, pos) = tile_board.swap_tile(from_pos, to_pos);
                commands.entity(entity).insert(pos);
            }
        }
    }
}


fn clear_line(
    commands: &mut Commands,
    line: u32,
    tile_board: &mut TileBoard
) {
    for i in 0..10 {
        if let Some(tile_entity) = tile_board.remove(i, line) {
            commands.entity(tile_entity).despawn_recursive();
        } else {
            panic!("DEBUG: no enity. x: {}, y:{}", i, line);
        }
    }
}


fn is_full_line(
    line: u32,
    tile_board: &TileBoard
) -> bool {
    for i in 0..10 {
        if line > 19 {
            panic!("DEBUG: helper::is_full_line, 328, x: {}, y: {}", i, line);
        }
        if tile_board.entity_at(i, line).is_none() {
            return false;
        }
    }
    true
}

//删除游戏方块，实则仅记录方块原始位置，在后面draw_piece时进行方块移动
pub fn remove_piece(
    mut state: ResMut<scene::GameState>,
    tile_board: Res<TileBoard>,
) {
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
            state.tetromino_entities.insert((tile_pos.0, tile_pos.1));
        }
    }
}

//绘制游戏方块
pub fn draw_piece(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    tiles: Res<Tiles>,
    mut tile_board: ResMut<TileBoard>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    //获取方块移动的目标位置
    let positions = state.current_tetromino.get_position().iter().map(|p| {
        IVec2::new(p.x + state.current_position.x, p.y + state.current_position.y)
    }).collect::<Vec<IVec2>>();

    //spawn出来的方块必须不能有占位
    if state.tetromino_entities.is_empty() && !has_no_tile(&positions, &tile_board) {
        next_state.set(AppState::DEAD);
        return;
    }

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
    for from_pos in state.tetromino_entities.drain() {
        if !positions_to_keep.is_empty() {
            let v2 = positions_to_keep.pop().unwrap();
            let (&entity, pos) = tile_board.swap_tile(from_pos, (v2.x as u32, v2.y as u32));
            commands.entity(entity).insert(pos);
        } else {
            //说明目标位置在游戏区域外面，直接把原始方块删除
            if let Some(entity) = tile_board.remove(from_pos.0, from_pos.1) {
                commands.entity(entity).despawn();
            }
        }
    }

    for pos in positions_to_keep.iter() {
        if pos.x >= 10 || pos.y >= 20 {
            continue;
        }
        let entity = commands.spawn_empty().id();
        let transfrom = tile_board.set(pos.x as u32, pos.y as u32, entity);
        commands.entity(entity)
        .insert(tiles.0[state.current_tetromino.index].clone())
        .insert(transfrom);

    }
}
#[derive(Component)]
pub struct FpsText;

pub fn init_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<config::ConfigData>,
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

    //游戏方块瓦片
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
    commands.spawn(Camera2d);

    //游戏区域边框
    commands.spawn(scene::main_board(&asset_server, &config));
    //方块预览区1边框
    commands.spawn(scene::preview_board(&asset_server, &config, true));
    commands.spawn(scene::preview_board(&asset_server, &config, false));
    //创建fps计数器
    // commands
    //     .spawn((
    //         Text::new("FPS: "),
    //         TextFont {
    //             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //             font_size: 42.0,
    //             ..default()
    //         },
    //     ))
    //     .with_child((
    //         TextSpan::default(),
    //             (
    //                 TextFont {
    //                     font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                     font_size: 33.0,
    //                     ..Default::default()
    //                 },
    //                 TextColor(GOLD.into()),
    //             ),
    //         FpsText,
    //     ));
}


pub fn hit_bottom(
    state: Res<scene::GameState>,
    tile_board: Res<TileBoard>,
    config: Res<config::ConfigData>,
) -> bool {
    !can_move_down(&state, &tile_board) && state.hit_bottom_timer >= config.game_config.step_delay
}


pub fn resume (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    config: Res<config::ConfigData>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.pause.as_str())) {
        next_state.set(AppState::RUNNING);
    }
}


pub fn reinit(
    mut commands: Commands,
    mut state: ResMut<scene::GameState>,
    config: Res<config::ConfigData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tile_board: ResMut<TileBoard>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(keys::from_str(config.keys_config.restart.as_str())) {
        //删除所有的方块
        for entity in tile_board.get_all_entitys() {
            commands.entity(entity).despawn();
        }
        tile_board.clear();
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
    state.hit_bottom_timer += time.delta_secs_f64();
    state.step_timer += time.delta_secs_f64();
}

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                **span = format!("{value:.2}");
            }
        }
    }
}

// pub fn print_board(
//     tile_board: Res<TileBoard>,
// ) {
//     for y in (0..20).rev() {
//         for x in 0..10 {
//             if tile_board.entity_at(x, y).is_some() {
//                 print!("*");
//             } else {
//                 print!(".");
//             }
//         }
//         print!("\n")
//     }
//     print!("\n\n\n")
// }