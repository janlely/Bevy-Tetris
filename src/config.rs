use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};


#[derive(Debug)]
pub struct KeysConfig {
    pub left: String,
    pub right: String,
    pub down: String,
    pub drop: String,
    pub rotate_left: String,
    pub rotate_right: String,
    pub pause: String,
    pub restart: String
}

#[derive(Debug)]
pub struct GameConfig {
    pub step_delay: f64,
    pub first_repeat_delay: f64,
    pub repeat_delay: f64,
    pub scale_factor: f32,
    pub tile_size: f32,
    pub border_img: String,
    pub preview_img: String,

}

#[derive(Resource, Debug)]
pub struct ConfigData {
    pub keys_config: KeysConfig,
    pub game_config: GameConfig
}

impl ConfigData {

    pub fn new() -> Self {
        Self {
            keys_config: KeysConfig {
                left: "KeyA".to_string(),
                right: "KeyD".to_string(),
                down: "KeyS".to_string(),
                drop: "Space".to_string(),
                rotate_left: "KeyJ".to_string(),
                rotate_right: "KeyK".to_string(),
                pause: "KeyP".to_string(),
                restart: "Enter".to_string(),
            },
            game_config: GameConfig {
                step_delay: 0.5,
                first_repeat_delay: 0.15,
                repeat_delay: 0.05,
                scale_factor: 0.15,
                tile_size: 192.0,
                border_img: "Border.png".to_string(),
                preview_img: "Next.png".to_string(),
            }
        }
    }
}

pub fn config_setting_panel(
    mut contexts: EguiContexts,
    mut state: ResMut<ConfigData>
) {
    let ctx = contexts.ctx_mut();
    ctx.style_mut(|style| {
        style.spacing.slider_width = 300.0;
    });
    egui::SidePanel::left("config_panel")
    .default_width(400.0)
    .show(ctx, |ui| {
        ui.heading("Settings");

        ui.add(egui::Label::new("keyboard speed"));
        ui.add(egui::Slider::new(&mut state.game_config.repeat_delay, 0.01..=0.5));

        });
}