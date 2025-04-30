use bevy::prelude::*;

use crate::{level::LevelPlugin, player::PlayerPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LevelPlugin,
            PlayerPlugin,
        ));
    }
}