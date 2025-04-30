use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfigDefaults {
    pub infinite_terrian: bool,
    pub look_at_origin: bool,
    pub check_for_updates: bool,
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub window_size: (f32, f32),
    pub window_title: String, 
    pub fullscreen: bool,

    pub name: String, 
    pub version: (i32, i32, i32),

    pub defaults: GameConfigDefaults, 
}

pub fn fetch_config_data() -> GameConfig {
    let data_path: &str = "assets/data/config.ron";
    
    println!("Loading GameConfig from: {}", data_path);

    let raw = read_to_string(data_path)
        .expect("Error when reading raw string from GameConfig file!");
    let val: GameConfig = ron::from_str(raw.as_str()).unwrap();

    val
}