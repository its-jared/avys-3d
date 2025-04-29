use bevy::prelude::*;

pub mod build;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build::build);
    }
}