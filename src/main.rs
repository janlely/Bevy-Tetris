mod tetromino;
mod config;
mod scene;
mod keys;
mod game_logic;

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin},
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*
};
use bevy_egui::EguiPlugin;



fn main() {
    let mut app = App::new();
    #[cfg(any(target_os = "macos" , target_os="linux", target_os = "unknown"))]
    {
        app.add_plugins(DefaultPlugins);
    }
    #[cfg(target_os = "windows")]
    {
        use bevy::render::RenderPlugin;
        use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
        app.add_plugins(DefaultPlugins.set(RenderPlugin{
            render_creation: RenderCreation::Automatic(WgpuSettings{
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }));
    }

    app.insert_state(game_logic::AppState::RUNNING);
    app.insert_resource(config::ConfigData::new());
    app.insert_resource(scene::init_game_state());
    // app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(EguiPlugin);
    app.add_systems(Startup, (game_logic::init_scene, game_logic::spawn, game_logic::draw_piece).chain());
    // app.add_systems(Update, game_logic::text_update_system);
    app.add_systems(Update, config::config_setting_panel);
    app.add_systems(Update, (
        game_logic::update_timer,
        game_logic::remove_piece,
        game_logic::step_down,
        game_logic::handler_key_down,
        game_logic::handler_key_repeat,
        game_logic::draw_piece,
        (
            game_logic::clear_lines,
            // game_logic::print_board,
            game_logic::spawn,
            game_logic::draw_piece,
        ).chain().run_if(game_logic::hit_bottom),
    ).chain().run_if(in_state(game_logic::AppState::RUNNING)));
    app.add_systems(Update, game_logic::resume.run_if(in_state(game_logic::AppState::PAUSED)));
    app.add_systems(Update, game_logic::reinit.run_if(in_state(game_logic::AppState::DEAD)));
    app.run();
}
