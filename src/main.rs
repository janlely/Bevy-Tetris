#[macro_use]
extern crate ini;
mod tetromino;
mod config;
mod scene;
mod keys;
mod game_logic;

use bevy_ecs_tilemap::prelude::*;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};

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
    app.insert_resource(game_logic::EntityContainer {..default()});
    app.insert_resource(config::load_config("config.ini".to_string()));
    app.insert_resource(scene::init_game_state());
    app.add_plugins((TilemapPlugin, FrameTimeDiagnosticsPlugin));
    app.add_systems(Startup, (game_logic::init_scene, game_logic::spawn, game_logic::draw_piece).chain());
    app.add_systems(OnEnter(game_logic::AppState::RUNNING), game_logic::reinit);
    app.add_systems(Update,
        (
            //每帧更新相关的timer
            game_logic::
            //方块位置可能会更新，先删除方块，中间更新位置数据，最后再绘制
            game_logic::remove_piece,
            //处理方块自动下落
            game_logic::step_down,
            //处理用户按键
            game_logic::handler_key_down,
            //处理用户按键长按
            game_logic::handler_key_repeat,
            //处理方块消除, 如果方块已经落下：
            // 1、绘制方块
            // 2、消除满行方块
            // 3、生成新方块
            (game_logic::draw_piece, game_logic::clear_lines, game_logic::spawn).chain()
                .run_if(in_state(game_logic::hit_bottom)),
            //绘制方块
            game_logic::draw_piece,
        ).chain().run_if(in_state(game_logic::AppState::RUNNING))
    );
    app.run();
}
