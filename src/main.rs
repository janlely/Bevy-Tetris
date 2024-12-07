#[macro_use]
extern crate ini;

use std::io::Read;
use bevy::ecs::system::lifetimeless::SCommands;
use bevy_ecs_tilemap::prelude::*;
mod scene;
mod config;
mod tetromino;
use bevy::prelude::*;
// use bevy::reflect::List;
use crate::scene::calculate_transform;
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .insert_resource(config::load_config("config.ini".to_string()))
        .insert_resource(LinesToRemove {lines_to_remove: vec![], swap: vec![]})
        .insert_resource(Countdown {
            timer1: Timer::from_seconds(4.0, TimerMode::Repeating),
            timer2: Timer::from_seconds(7.0, TimerMode::Repeating),
            // timer3: Timer::from_seconds(12.0, TimerMode::Repeating),
            // timer4: Timer::from_seconds(14.0, TimerMode::Repeating),
            // timer5: Timer::from_seconds(16.0, TimerMode::Repeating),
            // timer6: Timer::from_seconds(18.0, TimerMode::Repeating),
            // timer7: Timer::from_seconds(20.0, TimerMode::Repeating),
        })
        .insert_resource(OneTime { done: false})
        .add_systems(Startup, setup)
        // .add_systems(Update, (update1, update2, update3).chain())
        .add_systems(Update, update)
        .run();
}

#[derive(Resource)]
struct Countdown {
    timer1: Timer,
    timer2: Timer,
    // timer3: Timer,
    // timer4: Timer,
    // timer5: Timer,
    // timer6: Timer,
    // timer7: Timer,
}

#[derive(Resource)]
struct LinesToRemove {
    lines_to_remove: Vec<u32>,
    swap: Vec<(u32,u32)>
}


#[derive(Resource)]
struct OneTime {
    done: bool
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<config::ConfigData>
) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle: Handle<Image> = asset_server.load(config.game_config.tiles_path.clone());
    let map_size = TilemapSize { x: 10, y: 20 };
    let mut tile_storage = TileStorage::empty(map_size);

    // println!("DEBUG: main_tilemap, 177");
    let tilemap_entity = commands.spawn_empty().id();
    let mut random = thread_rng();

    for x in 0..10 {
        for y in 0..3 {
            let num = random.gen_range(0..7);
            if num > 6 {
                continue;
            }

            let tile_pos = TilePos { x: x, y: y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(num),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    for y in 3..15 {
        for x in [6,7] {
            let num = random.gen_range(0..7);
            let tile_pos = TilePos { x: x, y: y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(num),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }


    for y in (0..20).rev() {
        for x in 0..10 {
            if tile_storage.get(&TilePos{x: x as u32, y: y as u32}).is_some() {
                print!("*");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
    print!("\n\n\n");

    let tile_size = TilemapTileSize { x: config.game_config.tile_size, y: config.game_config.tile_size };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    commands.entity(tilemap_entity).insert(
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: scene::calculate_transform(&map_size, &grid_size, &map_type, config.game_config.scale_factor, 0.0),
            // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        });
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
            commands.entity(tile_entity).despawn_recursive();
            tile_storage.remove(&tile_pos);
        } else {
            panic!("DEBUG: no enity. x: {}, y:{}", i, line);
        }
    }
}

fn update(
    mut commands: Commands,
    mut ts_query: Query<&mut TileStorage>,
    mut countdown: ResMut<Countdown>,
    mut one_time: ResMut<OneTime>,
    mut ltr: ResMut<LinesToRemove>,
    time: Res<Time>
) {
    if one_time.done {
        return;
    }
    let mut tile_storage = ts_query.single_mut();
    if countdown.timer1.tick(time.delta()).finished() {
        for y in 0..3 {
            for x in 0..10 {
                let pos = TilePos{x: x, y: y};
                if let Some(entity) = tile_storage.get(&pos) {
                    commands.entity(entity).despawn_recursive();
                    tile_storage.remove(&pos);
                }
            }
        }
        ltr.lines_to_remove.push(0);
        ltr.lines_to_remove.push(1);
        ltr.lines_to_remove.push(2);

        let mut first_line = ltr.lines_to_remove[0];
        let mut idx = first_line;
        for i in first_line..20 {
            if !ltr.lines_to_remove.contains(&i) {
                // swap.insert(i, idx);
                println!("DEBUG: to swap: {}, {}", i, idx);
                ltr.swap.push((i, idx));
                idx += 1;
            }
        }
    }

    // for (y1, y2) in [(3,0), (4,1), (5,2), (6,3), (7,4)] {
    if countdown.timer2.tick(time.delta()).finished() {
        for (y1, y2) in ltr.swap.iter() {
            for i in 0..10 {
                let from_pos = TilePos { x: i, y: *y1 };
                let to_pos = TilePos { x: i, y: *y2 };
                if let Some(entity) = tile_storage.get(&from_pos) {
                    println!("DEBUG: from_pos: {:?}, to_pos: {:?}", from_pos, to_pos);
                    tile_storage.remove(&from_pos);
                    commands.entity(entity).insert(to_pos);
                    tile_storage.set(&to_pos, entity);
                }
            }
        }
        one_time.done = true;
    }
}
