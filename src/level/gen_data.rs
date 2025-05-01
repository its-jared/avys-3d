use std::fs;

use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct NoiseData {
    pub scale: i64, 
    pub ampli: i64, 
    pub offs: i64, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BiomeData {
    pub name: String,
    pub id: i32, 

    pub temp_range: (f64, f64),
    pub mois_range: (f64, f64),
    pub heig_range: (f64, f64), 

    pub grass_color: (f32, f32, f32),
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct GenData {
    pub biome_noise: NoiseData, 
    pub temp_noise: NoiseData, 
    pub mois_noise: NoiseData,
    pub heig_noise: NoiseData, 

    pub biomes: Vec<BiomeData>,
}

pub fn fetch_world_gen_data(mut commands: Commands) {
    let data_path: &str = "assets/data/world_gen.ron";
    
    println!("Loading GenData from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from world_gen file!");
    let val: GenData = ron::from_str(raw.as_str()).unwrap();

    commands.insert_resource(val);
}