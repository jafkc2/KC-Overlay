//! Módulo com funções para gerenciar o arquivo de configuração.

use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    sync::Mutex,
};

use serde_json::Value;

use crate::{stats, KCOverlay};

pub fn get_config_file_path() -> String {
    format!(
        "{}/kc_overlay_config.json",
        super::util::get_minecraft_dir()
    )
}

pub fn get_config() -> Value {
    super::util::get_json(get_config_file_path())
}

/// Checa se o arquivo existe.
/// Caso não exista, esta função cria o arquivo e retorna **False**.
/// Caso exista, adiciona configurações ausentes e retorna **True**.
#[tauri::command]
pub fn check_config_file(screen_size: (u32, u32)) -> bool {
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
            let default_window_scale =
                (screen_size.0 as f32 * screen_size.1 as f32 / (1920. * 1080.)).clamp(0.7, 1.25);

            println!("Escala de janela configurada para {}", default_window_scale);

            map.insert(
                "window_scale".to_owned(),
                serde_json::to_value(default_window_scale).unwrap(),
            );
        }
        if !map.contains_key("rgb_buttons") {
            map.insert(
                "rgb_buttons".to_owned(),
                serde_json::to_value(false).unwrap(),
            );
        }
        if !map.contains_key("show_ws") {
            map.insert("show_ws".to_owned(), serde_json::to_value(true).unwrap());
        }
        if !map.contains_key("show_wlr") {
            map.insert("show_wlr".to_owned(), serde_json::to_value(true).unwrap());
        }
        if !map.contains_key("show_fkdr") {
            map.insert("show_fkdr".to_owned(), serde_json::to_value(true).unwrap());
        }
        if !map.contains_key("show_kdr") {
            map.insert("show_kdr".to_owned(), serde_json::to_value(true).unwrap());
        }
        if !map.contains_key("show_wins") {
            map.insert("show_wins".to_owned(), serde_json::to_value(true).unwrap());
        }
        if !map.contains_key("show_losses") {
            map.insert(
                "show_losses".to_owned(),
                serde_json::to_value(true).unwrap(),
            );
        }
        if !map.contains_key("show_bans") {
            map.insert("show_bans".to_owned(), serde_json::to_value(true).unwrap());
        }
    }

    let serializedjson = serde_json::to_string_pretty(&conf_json).unwrap();

    file.write_all(serializedjson.as_bytes()).unwrap();

    !file_exists
}

/// Função para salvar as configurações para o arquivo.
pub fn save_settings(
    never_minimize: Option<bool>,
    seconds_to_minimize: Option<u64>,
    auto_manage_players: Option<bool>,
    stats_type: Option<String>,
    window_scale: Option<f64>,
    rgb_buttons: Option<bool>,
    show_stats: Option<(stats::BedwarStat, bool)>,
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
    if let Some(stat) = show_stats {
        match stat.0 {
            stats::BedwarStat::Ws => config["show_ws"] = serde_json::json!(stat.1),
            stats::BedwarStat::Wlr => config["show_wlr"] = serde_json::json!(stat.1),
            stats::BedwarStat::Fkdr => config["show_fkdr"] = serde_json::json!(stat.1),
            stats::BedwarStat::Kdr => config["show_kdr"] = serde_json::json!(stat.1),
            stats::BedwarStat::Wins => config["show_wins"] = serde_json::json!(stat.1),
            stats::BedwarStat::Losses => config["show_losses"] = serde_json::json!(stat.1),
            stats::BedwarStat::Bans => config["show_bans"] = serde_json::json!(stat.1),
        }
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
