mod tetromino;
mod config;
mod scene;
use std::ops::Deref;

use bevy::{ecs::entity, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
use bevy_ecs_tilemap::prelude::*;
use config::ConfigData;
use rand::{thread_rng, Rng};
use scene::GameState;
use tetromino::*;

#[macro_use]
extern crate ini;


fn main() {
    let mut app = App::new();
    #[cfg(target_os = "macos")]
    {
        app.add_plugins(DefaultPlugins);
    }
    #[cfg(target_os = "windows")]
    {
        app.add_plugins(DefaultPlugins.set(RenderPlugin{
            render_creation: RenderCreation::Automatic(WgpuSettings{
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }));
    }
    app.insert_resource(EntityContainer {..default()});
    app.insert_resource(config::loadConfig("config.ini".to_string()));
    app.insert_resource(scene::init_game_state());
    app.add_plugins(TilemapPlugin);
    app.add_systems(Startup, startup);
    // app.add_systems(Update, remove_tiles);
    app.run();
}

#[derive(Component)]
struct Name(String);

#[derive(Resource, Default)]
struct EntityContainer {
    tilemap: Option<Entity>,
    preview1: Option<Entity>,
    preview2: Option<Entity>
}

fn spawn_tetromino(
    mut commands: Commands,
    state: &scene::GameState,
    entity_container: &Res<EntityContainer>,
    tetrominos: &Res<Tetrominos>,
    tile_storage: &mut TileStorage
) {

    //主游戏区域放置方块
    for positon in state.current_tetromino.0.positions[state.rotate as usize].iter() {
        let tile_pos = TilePos { x: positon.x + state.current_position.x, y: positon.y + state.current_position.y };
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(entity_container.tilemap.unwrap()),
                texture_index: TileTextureIndex(state.current_tetromino.1),
                ..Default::default()
            })
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }
    //预览区域放置方块
    let preview1 = entity_container.preview1.unwrap();
    
    
}

fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<scene::GameState>,
    entity_container: Res<EntityContainer>,
    tetrominos: Res<Tetrominos>,
    mut query: Query<(&mut scene::LastUpdate, &mut TileStorage)>
) {

    let mut last_update = query.single_mut().0;
    let mut tile_storage = query.single_mut().1;
    // let tetromino = Tetromino::new(TetrominoType::I); 
    

    //spawn tetromino
    if !state.started {
        spawn_tetromino(commands, state.deref(), &entity_container, &tetrominos, &mut tile_storage);
        state.started = true;
    }

    
}

#[derive(Resource)]
struct Tetrominos([SpriteBundle; 7]);

fn startup(
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
    let tilemap_entity= commands.spawn_empty().insert(Name("tilemap".to_string())).id();
    let preview1_entity= commands.spawn_empty().id();
    let preview2_entity: Entity= commands.spawn_empty().id();
    commands.entity(tilemap_entity).insert(scene::main_tilemap(&asset_server, &config));
    //游戏区域边框
    commands.entity(tilemap_entity).insert(scene::main_board(&asset_server, &config));
    //方块预览区1边框
    commands.entity(preview1_entity).insert(scene::preview_board(&asset_server, &config, 0));
    commands.entity(preview2_entity).insert(scene::preview_board(&asset_server, &config, 1));
    entity_container.tilemap = Some(tilemap_entity);
    entity_container.preview1 = Some(preview1_entity);
    entity_container.preview2 = Some(preview2_entity);

    //生成三个方块， 一个放在游戏区域，一个放在预览区1，一个放在预览区2
    
    // commands.entity(preview1_entity).insert()


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

    // let texture_handle: Handle<Image> = asset_server.load(config.gameConfig.tiles_path.clone());
    // let map_size = TilemapSize { x: 10, y: 20 };
    // let tile_storage = TileStorage::empty(map_size);

    // let tile_size = TilemapTileSize { x: config.gameConfig.tile_size, y: config.gameConfig.tile_size };
    // let grid_size = tile_size.into();
    // let map_type = TilemapType::default();
    // commands.entity(tilemap_entity).insert(
    //     (TilemapBundle {
    //         grid_size,
    //         map_type,
    //         size: map_size,
    //         storage: tile_storage,
    //         texture: TilemapTexture::Single(texture_handle),
    //         tile_size,
    //         transform: scene::calculate_transform(&map_size, &grid_size, &map_type, config.gameConfig.scale_factor, 0.0),
    //         ..Default::default()
    //     },
    //     scene::LastUpdate::default())
    // );

}


// fn remove_tiles(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut last_update_query: Query<(&mut LastUpdate, &mut TileStorage)>,
// ) {
//     let current_time = time.elapsed_seconds_f64();
//     for (mut last_update, mut tile_storage) in last_update_query.iter_mut() {
//         // Remove a tile every half second.
//         if (current_time - last_update.value) > 0.1 {
//             let mut random = thread_rng();
//             let position = TilePos {
//                 x: random.gen_range(0..32),
//                 y: random.gen_range(0..32),
//             };

//             if let Some(tile_entity) = tile_storage.get(&position) {
//                 commands.entity(tile_entity).despawn_recursive();
//                 // Don't forget to remove tiles from the tile storage!
//                 tile_storage.remove(&position);
//             }

//             last_update.value = current_time;
//         }
//     }
// }