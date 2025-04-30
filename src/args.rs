use std::env;

use crate::data::GameConfig;

pub fn handle_args(config: GameConfig) -> GameConfig {
    let args: Vec<String> = env::args().collect();
    let mut new_config = config;

    for arg in args {
        match arg.as_str() {
            "infinite_terrian" => new_config.defaults.infinite_terrian = false,
            "look_at_origin" => new_config.defaults.look_at_origin = true,
            "check_for_updates" => new_config.defaults.check_for_updates = false,
            _ => ()
        }
    }

    new_config
}