use bevy::{prelude::*, window::{WindowMode, WindowResolution}};
use game::GamePlugin;

pub mod data;
pub mod game;
pub mod level;
pub mod player;

fn main() {
    let game_config = data::fetch_config_data();
    let mut window_mode = WindowMode::Windowed;

    if game_config.fullscreen {
        window_mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
    }

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        title: game_config.window_title.to_string(),
                        resizable: false, 
                        resolution: WindowResolution::new(
                            game_config.window_size.0, 
                            game_config.window_size.1
                        ),
                        mode: window_mode,
                        ..default()
                    }),
                    ..default()
                }),

            GamePlugin,
        ))
        .insert_resource(game_config)
        .run();
}