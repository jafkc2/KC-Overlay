use std::{
    collections::{HashMap, VecDeque},
    env, fs,
    io::SeekFrom,
    path::Path,
    str::FromStr,
    time::{Duration, Instant},
};

use minecraft_clients::MineClient;
use player::Player;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use stats::StatsType;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncSeekExt, BufReader},
    sync::Mutex,
    time::sleep,
};

mod config;
mod minecraft_clients;
mod player;
mod proxy;
mod room;
mod stats;
mod update;
mod util;

struct KCOverlay {
    state: State,
    settings: Settings,
}

impl KCOverlay {
    // Sistema de cachê de jogadores para evitar rate limit da api
    fn add_players_to_cache(&mut self, players: Vec<Player>) {
        for player in players {
            let mut already_in_cache = false;
            for cached_player in self.state.cached_players.clone() {
                if player.username == cached_player.username {
                    already_in_cache = true;
                }
            }

            if !already_in_cache {
                self.state.cached_players.push_back(player);
                if self.state.cached_players.len() > 200 {
                    self.state.cached_players.pop_front();
                }
            }
        }
    }

    fn update_player_list(&mut self, added: HashMap<&[u8], String>, removed: Vec<&[u8]>) {
        for (uuid, username) in added {
            self.state.player_list.insert(uuid.to_vec(), username);
        }

        for uuid in removed {
            self.state.player_list.remove(&uuid.to_vec());
        }
    }
}

struct State {
    player_list: HashMap<Vec<u8>, String>,
    cached_players: VecDeque<Player>,
    loading: bool,
    rates_full_time: Instant,
    is_first_use: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Settings {
    use_custom_client: bool,
    custom_client_path: String,
    never_minimize: bool,
    seconds_to_minimize: u64,
    stats_type: StatsType,
    window_scale: f64,
    show_ws: bool,
    show_wlr: bool,
    show_fkdr: bool,
    show_kdr: bool,
    show_wins: bool,
    show_losses: bool,
    show_bans: bool,
    transparency: i32,
    automatic: bool,
    remove_players: bool,
    hotkey: String,

    marked_players: Vec<String>,
}
pub fn run() {
    // Isso é o processo final da atualização do KC Overlay. Remove o executável antigo, caso exista.
    let old_exec = env::current_exe().unwrap().with_extension("old");
    if Path::new(&old_exec).exists() {
        match fs::remove_file(old_exec) {
            Ok(ok) => ok,
            Err(e) => println!("Failed to delete old executable: {e}"),
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let monitors = app.available_monitors().unwrap();

            let screen = monitors[0].size();
            let screen_size = (screen.width, screen.height);

            let is_first_use = config::check_config_file(screen_size);

            let config = config::get_config();
            let custom_client_path = config["custom_client_path"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let use_custom_client = config["use_custom_client"].as_bool().unwrap_or(false);

            let never_minimize = config["never_minimize"].as_bool().unwrap_or(false);
            let seconds_to_minimize = config["seconds_to_minimize"].as_u64().unwrap_or(10);
            let stats_type = from_value::<StatsType>(serde_json::Value::Object(
                config["stats_type"]
                    .as_object()
                    .unwrap_or(&serde_json::Map::new())
                    .clone(),
            ))
            .unwrap_or(StatsType::BedwarsAll);
            let window_scale = config["window_scale"].as_f64().unwrap_or(1.0);
            let show_ws = config["show_ws"].as_bool().unwrap_or(true);
            let show_wlr = config["show_wlr"].as_bool().unwrap_or(true);
            let show_fkdr = config["show_fkdr"].as_bool().unwrap_or(true);
            let show_kdr = config["show_kdr"].as_bool().unwrap_or(true);
            let show_wins = config["show_wins"].as_bool().unwrap_or(true);
            let show_losses = config["show_losses"].as_bool().unwrap_or(true);
            let show_bans = config["show_bans"].as_bool().unwrap_or(false);
            let transparency = config["transparency"].as_i64().unwrap_or(75) as i32;
            let automatic = config["automatic"].as_bool().unwrap_or(true);
            let remove_players = config["remove_players"].as_bool().unwrap_or(true);
            let hotkey = config["hotkey"]
                .as_str()
                .unwrap_or("Shift+Alt+Z")
                .to_string();

            println!("Hotkey: {}", hotkey.clone());

            let marked_players = config["marked_players"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|x| x.as_str().unwrap_or("").to_string())
                .collect();

            let short = match Shortcut::from_str(hotkey.as_str()) {
                Ok(s) => s,
                Err(_) => Shortcut::new(Some(Modifiers::ALT), Code::KeyZ),
            };
            let load_short = Shortcut::new(Some(Modifiers::ALT), Code::KeyX);
            let hotkey_builder = if let Ok(hotkey_builder) =
                tauri_plugin_global_shortcut::Builder::new().with_shortcuts([short, load_short])
            {
                hotkey_builder
            } else {
                tauri_plugin_global_shortcut::Builder::new()
            };

            app.handle()
                .plugin(
                    hotkey_builder
                        .with_handler(move |app, _shortcut, event| {
                            println!("{}", _shortcut.id);
                            if event.state == ShortcutState::Pressed {
                                match _shortcut {
                                    s if *s == short => {
                                        println!("Atalho de minimizar/desminimizar usado.");
                                        app.emit("hotkey", true).unwrap();
                                    }
                                    s if *s == load_short => {
                                        app.emit("load_hotkey", true).unwrap();
                                    }
                                    _ => (),
                                }
                            }
                        })
                        .build(),
                )
                .unwrap_or(());

            app.manage(Mutex::new(KCOverlay {
                state: State {
                    cached_players: VecDeque::new(),
                    loading: false,
                    rates_full_time: Instant::now(),
                    is_first_use,
                    player_list: HashMap::new(),
                },
                settings: Settings {
                    use_custom_client,
                    custom_client_path,
                    never_minimize,
                    seconds_to_minimize,
                    stats_type,
                    window_scale,
                    show_ws,
                    show_wlr,
                    show_fkdr,
                    show_kdr,
                    show_wins,
                    show_losses,
                    show_bans,
                    transparency,
                    automatic,
                    remove_players,
                    hotkey,
                    marked_players,
                },
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_logs,
            util::get_version,
            get_settings,
            config::save_settings,
            search_player,
            update::check_updates,
            update::install_update,
            is_first_use,
            change_shortcut,
            proxy::run_proxy,
            load_stats_tauri,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn change_shortcut(handle: tauri::AppHandle, shortcut: String) -> bool {
    println!("Mudando atalho de minimizar/desminimizar.");
    tauri_plugin_global_shortcut::GlobalShortcutExt::global_shortcut(handle.app_handle())
        .unregister_all()
        .unwrap();

    match tauri_plugin_global_shortcut::GlobalShortcutExt::global_shortcut(handle.app_handle())
        .on_shortcut(shortcut.as_str(), |app, _, event| {
            if event.state == ShortcutState::Pressed {
                println!("Atalho de minimizar/desminimizar usado.");
                app.emit("hotkey", true).unwrap();
            }
        }) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
async fn is_first_use(app: tauri::State<'_, Mutex<KCOverlay>>) -> Result<bool, ()> {
    Ok(app.lock().await.state.is_first_use)
}

#[tauri::command]
async fn get_settings(app: tauri::State<'_, Mutex<KCOverlay>>) -> Result<Settings, ()> {
    println!("{:?}", app.lock().await.settings.clone());
    Ok(app.lock().await.settings.clone())
}

#[tauri::command]
async fn search_player(handle: tauri::AppHandle, username: String, stats_type: StatsType) {
    let player = player::get_player(&username, stats_type, VecDeque::new()).await;

    match player {
        Ok(ok) => handle.emit("player_to_view", ok).unwrap(),
        Err(_) => handle.emit("player_to_view", "").unwrap(),
    }
}

#[tauri::command]
async fn read_logs(
    handle: tauri::AppHandle,
    app: tauri::State<'_, Mutex<KCOverlay>>,
) -> Result<(), ()> {
    println!("Iniciando leitura de logs");

    let mut log_paths = MineClient::get_all_log_paths();

    let custom_clienth_path = app.lock().await.settings.custom_client_path.clone();
    if !custom_clienth_path.is_empty() {
        log_paths.push(custom_clienth_path);
    }

    let mut readers = get_log_readers(log_paths).await;

    let mut time_since_client_refresh = Instant::now();

    loop {
        let mut any_read = false;
        for (_, reader, buffer) in readers.iter_mut() {
            match reader.read_line(buffer).await {
                Ok(0) => continue,
                Ok(_) => {
                    let line = buffer.trim_end().to_string();
                    if !line.is_empty() {
                        handle_log_line(line, handle.clone(), &app).await;
                    }
                    buffer.clear();
                    any_read = true;
                }
                Err(e) => println!("Erro ao ler logs: {e}"),
            }
        }
        if !any_read {
            sleep(Duration::from_millis(50)).await;
        }
        // Atualiza o arquivo de logs do client, se necessário
        if time_since_client_refresh.elapsed() > Duration::from_secs(15) {
            let mut log_paths = MineClient::get_all_log_paths();

            let custom_clienth_path = app.lock().await.settings.custom_client_path.clone();
            if !custom_clienth_path.is_empty() {
                log_paths.push(custom_clienth_path);
            }
            readers = get_log_readers(log_paths).await;

            time_since_client_refresh = Instant::now();
        }
    }
}

async fn get_log_readers(log_paths: Vec<String>) -> Vec<(String, BufReader<File>, String)> {
    let mut readers = vec![];
    for path in log_paths {
        if Path::new(&path).exists() {
            if let Ok(file) = File::open(&path).await {
                let mut reader = BufReader::new(file);
                reader.seek(SeekFrom::End(0)).await.unwrap();
                readers.push((path, reader, String::new()));
            }
        }
    }

    readers
}

async fn handle_log_line(
    line: String,
    handle: tauri::AppHandle,
    app_mutex: &tauri::State<'_, Mutex<KCOverlay>>,
) {
    // Checa se algum jogador entrou na party
    if line.contains("entrou na party") {
        // com certeza não é a maneira mais eficiente de fazer isso!
        let splitted_line: Vec<&str> = line.split(" ").collect();
        for (index, part) in splitted_line.clone().into_iter().enumerate() {
            if part == "entrou" {
                let player_name: &str = splitted_line[index - 1];
                let stats_type = app_mutex.lock().await.settings.stats_type.clone();
                let cached_players = app_mutex.lock().await.state.cached_players.clone();
                let player = player::get_player(player_name, stats_type, cached_players).await;
                if let Ok(ok) = player {
                    app_mutex
                        .lock()
                        .await
                        .add_players_to_cache(vec![ok.clone()]);
                    handle.emit("party_player_joined", ok).unwrap();
                }

                break;
            }
        }
    }

    // Checa se algum jogador entrou na partida.
    if line.contains("entrou na sala")
        && app_mutex.lock().await.settings.automatic
        && !app_mutex.lock().await.state.loading
    {
        // com certeza não é a maneira mais eficiente de fazer isso!
        let splitted_line: Vec<&str> = line.split(" ").collect();
        for (index, part) in splitted_line.clone().into_iter().enumerate() {
            if part == "entrou" {
                println!("{:?}", app_mutex.lock().await.state.player_list);
                let player_name: &str = splitted_line[index - 1];
                let stats_type = app_mutex.lock().await.settings.stats_type.clone();
                let cached_players = app_mutex.lock().await.state.cached_players.clone();
                let player = player::get_player(player_name, stats_type, cached_players).await;
                if let Ok(ok) = player {
                    app_mutex
                        .lock()
                        .await
                        .add_players_to_cache(vec![ok.clone()]);
                    handle.emit("player_joined", ok).unwrap();
                }

                break;
            }
        }
    }
    // Checa se o jogador saiu da sala
    else if line.contains("saiu da sala") && app_mutex.lock().await.settings.automatic {
        let splitted_line: Vec<&str> = line.split(" ").collect();

        for (index, part) in splitted_line.clone().into_iter().enumerate() {
            if part == "saiu" {
                let player_name: &str = splitted_line[index - 1];
                handle.emit("remove_player", player_name).unwrap();
                break;
            }
        }
    }
    // Checa se algum jogador que está na lista foi eliminado da partida.
    else if line.contains("KILL FINAL") && app_mutex.lock().await.settings.automatic {
        let splitted_line: Vec<&str> = line.split(" ").collect();

        for (index, part) in splitted_line.clone().into_iter().enumerate() {
            if part == "morreu" {
                let player_name: &str = splitted_line[index - 1];
                handle.emit("remove_player", player_name).unwrap();
                break;
            }
        }
    }

    // Checa se a mensagem possui a lista de jogadores de quando o jogador digita "/jogando".
    if line.contains("[CHAT] Jogadores") && !app_mutex.lock().await.state.loading {
        println!("Jogador digitou /jogando");
        let split = line.split("):").map(|x| x.to_string());
        let split_vector: Vec<String> = split.clone().collect();

        let str_players: Vec<String> = split_vector[1]
            .trim()
            .replace(" ", "")
            .replace("+", "")
            .split(',')
            .map(|x| x.to_string())
            .collect();

        app_mutex.lock().await.state.loading = true;

        handle.emit("loading", true).unwrap();

        let cached_players = app_mutex.lock().await.state.cached_players.clone();
        let stats_type = app_mutex.lock().await.settings.stats_type.clone();

        
        player::get_players(str_players, stats_type, cached_players, handle, app_mutex).await;
    } else if line.contains("[CHAT] Enviando para") && !app_mutex.lock().await.state.loading {
        handle.emit("remove_players", true).unwrap();
    } else if line.contains("This server is running") && !app_mutex.lock().await.state.loading {
        println!("Carregando jogadores da sala...");

        load_stats(handle, app_mutex).await;
    }
}

#[tauri::command]
async fn load_stats_tauri(
    handle: tauri::AppHandle,
    app_mutex: tauri::State<'_, Mutex<KCOverlay>>,
) -> Result<(), ()> {
    load_stats(handle, &app_mutex).await;
    Ok(())
}
async fn load_stats(handle: tauri::AppHandle, app_mutex: &tauri::State<'_, Mutex<KCOverlay>>) {
    let cached_players = app_mutex.lock().await.state.cached_players.clone();
    let player_list = app_mutex.lock().await.state.player_list.clone();
    let stats_type = app_mutex.lock().await.settings.stats_type.clone();
    
    handle.emit("loading", true).unwrap();
    app_mutex.lock().await.state.loading = true;
    let str_player_list : Vec<String> = player_list.values().cloned().collect();
    player::get_players(str_player_list, stats_type, cached_players, handle, app_mutex).await;
}
