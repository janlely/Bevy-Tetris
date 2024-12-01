use bevy::prelude::Resource;


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
    pub tiles_path: String,
    pub tile_size: f32,
    pub border_img: String,
    pub preview_img: String,

}

#[derive(Resource, Debug)]
pub struct ConfigData {
    pub keys_config: KeysConfig,
    pub game_config: GameConfig
}

pub fn load_config(path: String) -> ConfigData {
    let map = ini!(&path);
    // Proceed to use normal HashMap functions on the map:
    let config = ConfigData {
        keys_config: KeysConfig {
            left: map["keyboard"]["left"].clone().unwrap(),
            right: map["keyboard"]["right"].clone().unwrap(),
            down: map["keyboard"]["down"].clone().unwrap(),
            drop: map["keyboard"]["drop"].clone().unwrap(),
            rotate_left: map["keyboard"]["rotate_left"].clone().unwrap(),
            rotate_right: map["keyboard"]["rotate_right"].clone().unwrap(),
            pause: map["keyboard"]["pause"].clone().unwrap(),
            restart: map["keyboard"]["restart"].clone().unwrap(),
        },
        game_config: GameConfig {
            step_delay: map["game"]["step_delay"].clone().unwrap().parse().unwrap(),
            first_repeat_delay: map["game"]["first_repeat_delay"].clone().unwrap().parse().unwrap(),
            repeat_delay: map["game"]["repeat_delay"].clone().unwrap().parse().unwrap(),
            scale_factor: map["game"]["scale_factor"].clone().unwrap().parse().unwrap(),
            tiles_path: map["game"]["tiles_path"].clone().unwrap(),
            tile_size: map["game"]["tile_size"].clone().unwrap().parse().unwrap(),
            border_img: map["game"]["border_img"].clone().unwrap(),
            preview_img: map["game"]["preview_img"].clone().unwrap(),
        }
    };
    println!("DEBUG: config::load_config, 58, data: {:?}", config);
    config
}