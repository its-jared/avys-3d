use std::collections::HashMap;
use bevy::prelude::*;

use crate::data::GameConfig;

pub mod build;
pub mod infinite;
pub mod gen_data;

#[derive(Resource)]
pub struct ChunkStore(pub HashMap<IVec2, Handle<Mesh>>);

#[derive(Component)]
pub struct Chunk;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkStore(HashMap::default()))
            .add_systems(Startup, build::setup_world)
            .add_systems(Update, (
                build::dynamic_scene,
                infinite::manage_chunks.run_if(resource_exists::<GameConfig>)
            ));
    }
}