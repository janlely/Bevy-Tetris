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

    app.insert_state(game_logic::AppState::RUNNING);
    app.insert_resource(game_logic::EntityContainer {..default()});
    app.insert_resource(config::load_config("config.ini".to_string()));
    app.insert_resource(scene::init_game_state());
    app.add_plugins((TilemapPlugin, FrameTimeDiagnosticsPlugin));
    app.add_systems(Startup, (game_logic::init_scene, game_logic::spawn, game_logic::draw_piece).chain());
    app.add_systems(Update, (
        game_logic::update_timer,
        game_logic::remove_piece,
        game_logic::step_down,
        game_logic::handler_key_down,
        game_logic::handler_key_repeat,
        (
            game_logic::draw_piece,
            game_logic::clear_lines,
            game_logic::spawn,
        ).chain().run_if(game_logic::hit_bottom),
        game_logic::draw_piece,
    ).chain().run_if(in_state(game_logic::AppState::RUNNING)));
    app.add_systems(Update, game_logic::resume.run_if(in_state(game_logic::AppState::PAUSED)));
    app.add_systems(Update, game_logic::reinit.run_if(in_state(game_logic::AppState::DEAD)));
    app.run();
}
