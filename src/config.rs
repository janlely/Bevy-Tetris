use bevy::prelude::Resource;


pub struct KeysConfig {
    pub left: String,
    pub right: String,
    pub down: String,
    pub drop: String,
    pub rotate_left: String,
    pub rotate_right: String,
    pub pause: String
}

pub struct GameConfig {
    pub step_delay: f32,
    pub first_repeat_delay: f32,
    pub repeat_delay: f32,
    pub scale_factor: f32,
    pub tiles_path: String,
    pub tile_size: f32,
    pub border_img: String,
    pub preview1_img: String,
    pub preview2_img: String,

}

#[derive(Resource)]
pub struct ConfigData {
    pub keysConfig: KeysConfig,
    pub gameConfig: GameConfig
}

pub fn loadConfig(path: String) -> ConfigData {
    let map = ini!(&path);
    // Proceed to use normal HashMap functions on the map:
    ConfigData {
        keysConfig: KeysConfig {
            left: map["keyboard"]["left"].clone().unwrap(),
            right: map["keyboard"]["right"].clone().unwrap(),
            down: map["keyboard"]["down"].clone().unwrap(),
            drop: map["keyboard"]["drop"].clone().unwrap(),
            rotate_left: map["keyboard"]["rotate_left"].clone().unwrap(),
            rotate_right: map["keyboard"]["rotate_right"].clone().unwrap(),
            pause: map["keyboard"]["pause"].clone().unwrap()
        },
        gameConfig: GameConfig {
            step_delay: map["game"]["step_delay"].clone().unwrap().parse().unwrap(),
            first_repeat_delay: map["game"]["first_repeat_delay"].clone().unwrap().parse().unwrap(),
            repeat_delay: map["game"]["repeat_delay"].clone().unwrap().parse().unwrap(),
            scale_factor: map["game"]["scale_factor"].clone().unwrap().parse().unwrap(),
            tiles_path: map["game"]["tiles_path"].clone().unwrap(),
            tile_size: map["game"]["tile_size"].clone().unwrap().parse().unwrap(),
            border_img: map["game"]["border_img"].clone().unwrap(),
            preview1_img: map["game"]["preview1_img"].clone().unwrap(),
            preview2_img: map["game"]["preview2_img"].clone().unwrap(),
        }
    }
}