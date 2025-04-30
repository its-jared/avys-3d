use serde::{Deserialize, Serialize};

use crate::data::GameConfig;

pub const ONLINE_CONFIG_PATH: &str = "https://raw.githubusercontent.com/its-jared/avys-3d/refs/heads/master/assets/data/update.ron";

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateData {
    pub name: String, 
    pub version: (i32, i32, i32),
}

pub fn check_for_updates(current_config: &GameConfig) -> Result<bool, ureq::Error> {
    println!("Checking for updates from: {}", ONLINE_CONFIG_PATH);

    if !check_internet_connection() {
        println!("No internet!");
        return Ok(false);
    }

    if !current_config.defaults.check_for_updates {
        println!("Update checker disabled.");
        return Ok(false);
    }

    let mut update_needed = false;
    let raw_online_config = 
        ureq::get(ONLINE_CONFIG_PATH)
        .call()?
        .body_mut()
        .read_to_string()?;
    let online_config: UpdateData = ron::from_str(raw_online_config.as_str()).unwrap();

    if online_config.version.0 > current_config.version.0 {
        update_needed = true;
    }
    if online_config.version.1 > current_config.version.1 {
        update_needed = true;
    }
    if online_config.version.2 > current_config.version.2 {
        update_needed = true;
    }

    if update_needed {
        println!("Update needed! Visit https://github.com/its-jared/avys-3d/ to update.");
    } else {
        println!("Up-to-date!");
    }

    Ok(update_needed)
}

fn check_internet_connection() -> bool {
    match ureq::get(ONLINE_CONFIG_PATH).call() {
        Ok(_) => return true,
        Err(_) => return false,
    }
}