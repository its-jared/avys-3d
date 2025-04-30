use std::collections::HashMap;
use bevy::prelude::*;

pub mod build;

#[derive(Resource)]
pub struct ChunkStore(pub HashMap<IVec2, Handle<Mesh>>);

#[derive(Component)]
pub struct Chunk;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkStore(HashMap::default()))
            .add_systems(Startup, build::setup_world);
    }
}