#[macro_use]
extern crate ini;
mod tetromino;
mod config;
mod scene;
mod keys;
mod helper;

use bevy_ecs_tilemap::prelude::*;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use crate::helper::{should_run, clear_lines, draw_piece,
                    handler_key_down, handler_key_repeat, game_over,
                    init_scene, remove_piece, spawn, step_down, EntityContainer};

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
    app.insert_resource(config::load_config("config.ini".to_string()));
    app.insert_resource(scene::init_game_state());
    app.add_plugins((TilemapPlugin, FrameTimeDiagnosticsPlugin));
    app.add_systems(Startup, (init_scene, spawn, draw_piece).chain());
    app.add_systems(Update,
        (
            game_over,
            remove_piece,
            (step_down, handler_key_down, handler_key_repeat),
            clear_lines,
            draw_piece,
        ).chain().run_if(should_run)
    );
    app.run();
}
