use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use serde_json::Value;

pub fn get_config_file_path() -> String {
    format!(
        "{}/kc_overlay_config.json",
        super::util::get_minecraft_dir()
    )
}

pub fn get_config() -> Value {
    super::util::get_json(get_config_file_path())
}

pub fn check_config_file() -> bool {
    let file_exists = Path::new(&get_config_file_path()).exists();
    let mut conf_json = match file_exists {
        true => super::util::get_json(get_config_file_path()),
        false => {
            let minecraft_path = super::util::get_minecraft_dir();
            if !Path::new(&minecraft_path).exists() {
                fs::create_dir_all(minecraft_path).unwrap()
            }
            serde_json::json!({})
        }
    };

    let mut file = File::create(get_config_file_path()).unwrap();

    if let Value::Object(map) = &mut conf_json {
        if !map.contains_key("client") {
            map.insert("client".to_owned(), serde_json::to_value(0).unwrap());
        }
        if !map.contains_key("custom_client_path") {
            map.insert(
                "custom_client_path".to_owned(),
                serde_json::to_value("").unwrap(),
            );
        }
        if !map.contains_key("never_minimize") {
            map.insert(
                "never_minimize".to_owned(),
                serde_json::to_value(false).unwrap(),
            );
        }
        if !map.contains_key("seconds_to_minimize") {
            map.insert(
                "seconds_to_minimize".to_owned(),
                serde_json::to_value(12).unwrap(),
            );
        }
        if !map.contains_key("auto_manage_players") {
            map.insert(
                "auto_manage_players".to_owned(),
                serde_json::to_value(true).unwrap(),
            );
        }
        if !map.contains_key("stats_type") {
            map.insert(
                "stats_type".to_owned(),
                serde_json::to_value("Bedwars Geral").unwrap(),
            );
        }
        if !map.contains_key("window_scale") {
            map.insert(
                "window_scale".to_owned(),
                serde_json::to_value(1.0).unwrap(),
            );
        }
        if !map.contains_key("rgb_buttons") {
            map.insert(
                "rgb_buttons".to_owned(),
                serde_json::to_value(false).unwrap(),
            );
        }
    }

    let serializedjson = serde_json::to_string_pretty(&conf_json).unwrap();

    file.write_all(serializedjson.as_bytes()).unwrap();

    !file_exists
}

pub fn save_settings(
    never_minimize: Option<bool>,
    seconds_to_minimize: Option<u64>,
    auto_manage_players: Option<bool>,
    stats_type: Option<String>,
    window_scale: Option<f64>,
    rgb_buttons: Option<bool>, // Novo parâmetro
) {
    let mut config = get_config();

    if let Some(never_minimize_option) = never_minimize {
        config["never_minimize"] = serde_json::json!(never_minimize_option)
    }
    if let Some(seconds) = seconds_to_minimize {
        config["seconds_to_minimize"] = serde_json::json!(seconds)
    }
    if let Some(auto_manage_players_option) = auto_manage_players {
        config["auto_manage_players"] = serde_json::json!(auto_manage_players_option)
    }
    if let Some(stats_type_option) = stats_type {
        config["stats_type"] = serde_json::json!(stats_type_option)
    }
    if let Some(scale) = window_scale {
        config["window_scale"] = serde_json::json!(scale)
    }
    if let Some(rgb) = rgb_buttons {
        config["rgb_buttons"] = serde_json::json!(rgb)
    }

    let mut config_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_config_file_path())
        .unwrap();
    config_file
        .write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
        .unwrap();
}
