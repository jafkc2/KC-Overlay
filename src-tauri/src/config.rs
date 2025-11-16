//! Módulo com funções para gerenciar o arquivo de configuração.

use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use serde_json::Value;
use tauri::Emitter;
use tokio::sync::Mutex;

use crate::{login::MinecraftAccount, KCOverlay, Settings};

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
    let mut file_exists = Path::new(&get_config_file_path()).exists();
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
            map.insert(
                "client".to_owned(),
                serde_json::to_value(super::minecraft_clients::MineClient::Default).unwrap(),
            );
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
                serde_json::to_value(super::stats::StatsType::BedwarsAll).unwrap(),
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
        if !map.contains_key("transparency") {
            map.insert("transparency".to_owned(), serde_json::to_value(75).unwrap());
        }
        if !map.contains_key("automatic") {
            map.insert("automatic".to_owned(), serde_json::to_value(true).unwrap());
        }
        if !map.contains_key("remove_players") {
            map.insert(
                "remove_players".to_owned(),
                serde_json::to_value(true).unwrap(),
            );
        }
        if !map.contains_key("hotkey") {
            map.insert(
                "hotkey".to_owned(),
                serde_json::to_value("Shift+Alt+Z").unwrap(),
            );
        }
        if !map.contains_key("marked_players") {
            map.insert(
                "marked_players".to_owned(),
                serde_json::to_value(Vec::<String>::new()).unwrap(),
            );
        }
        if !map.contains_key("has_account") {
            map.insert(
                "has_account".to_owned(),
                serde_json::to_value(false).unwrap(),
            );
        }
        if !map.contains_key("account") {
            file_exists = false;
            map.insert(
                "account".to_string(),
                serde_json::to_value(MinecraftAccount {
                    username: "".to_string(),
                    token: "".to_string(),
                    uuid: "".to_string(),
                })
                .unwrap(),
            );
        }
    }


    let serializedjson = serde_json::to_string_pretty(&conf_json).unwrap();

    file.write_all(serializedjson.as_bytes()).unwrap();

    !file_exists
}

/// Função para salvar as configurações para o arquivo.
#[tauri::command]
pub async fn save_settings(
    handle: tauri::AppHandle,
    app: tauri::State<'_, Mutex<KCOverlay>>,
    settings: Settings,
) -> Result<(), ()> {
    app.lock().await.settings = settings.clone();
    app.lock().await.state.cached_players.clear();
    handle.emit("settings_changed", settings.clone()).unwrap();

    let settings_json = serde_json::json!(settings);

    let mut config_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_config_file_path())
        .unwrap();
    config_file
        .write_all(
            serde_json::to_string_pretty(&settings_json)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();

    Ok(())
}
