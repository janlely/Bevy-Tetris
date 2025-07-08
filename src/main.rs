mod tetromino;
mod config;
mod scene;
mod keys;
mod game_logic;

use bevy::{
    prelude::*
};
use bevy_egui::EguiPlugin;
use bevy::window::WindowPlugin;



fn main() {
    let mut app = App::new();
    
    // 创建自定义插件配置以设置120fps
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(bevy::window::Window {
            present_mode: bevy::window::PresentMode::Immediate, // 禁用vsync
            ..default()
        }),
        ..default()
    });
    
    #[cfg(any(target_os = "macos" , target_os="linux", target_os = "unknown"))]
    {
        app.add_plugins(default_plugins);
    }
    #[cfg(target_os = "windows")]
    {
        use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
        use bevy::render::RenderPlugin;
        app.add_plugins(default_plugins.set(RenderPlugin{
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
    
    // 输入处理系统需要在Update中运行，确保不会错过按键事件
    app.add_systems(Update, (
        game_logic::handler_key_down,
        game_logic::handler_key_repeat,
    ).run_if(in_state(game_logic::AppState::RUNNING)));
    
    // 游戏逻辑系统使用固定更新频率，设置120fps
    app.add_systems(FixedUpdate, (
        game_logic::update_timer,
        game_logic::remove_piece,
        game_logic::step_down,
        game_logic::draw_piece,
        (
            game_logic::clear_lines,
            // game_logic::print_board,
            game_logic::spawn,
            game_logic::draw_piece,
        ).chain().run_if(game_logic::hit_bottom),
    ).chain().run_if(in_state(game_logic::AppState::RUNNING)));
    
    // 配置固定更新频率为120fps (1/120 = 0.008333...秒)
    app.insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0));
    app.add_systems(Update, game_logic::resume.run_if(in_state(game_logic::AppState::PAUSED)));
    app.add_systems(Update, game_logic::reinit.run_if(in_state(game_logic::AppState::DEAD)));
    app.run();
}
