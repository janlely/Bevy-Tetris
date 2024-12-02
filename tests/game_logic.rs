#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_system() {
        let mut app = App::new();
        #[cfg(target_os = "macos")]
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
        app.insert_resource(game_logic::EntityContainer {..default()});
        app.insert_resource(config::load_config("config.ini".to_string()));
        app.insert_resource(scene::init_game_state());
        app.add_plugins((TilemapPlugin, FrameTimeDiagnosticsPlugin));
        app.add_systems(Startup, (game_logic::init_scene, game_logic::spawn, game_logic::draw_piece).chain());
    }

}
