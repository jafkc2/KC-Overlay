use std::{
    collections::VecDeque,
    io::SeekFrom,
    path::Path,
    time::{Duration, Instant},
};

use minecraft_clients::MineClient;
use player::Player;
use serde::Serialize;
use stats::StatsType;
use tauri::{Emitter, Manager};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncSeekExt, BufReader},
    sync::Mutex,
    time::sleep,
};

mod config;
mod minecraft_clients;
mod player;
mod stats;
mod util;

struct KCOverlay {
    state: State,
    settings: Settings,
}

impl KCOverlay {
    fn add_players_to_cache(&mut self, players: Vec<Player>) {
        // Sistema de cachê de jogadores para evitar o uso da api
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
}

struct State {
    screen_size: (u32, u32),
    cached_players: VecDeque<Player>,
    loading: bool,
    waiting: i32,
    rates_full_time: Instant,
    searched_player_stats_type: StatsType,
}

#[derive(Serialize, Clone)]
struct Settings {
    client: MineClient,
    never_minimize: bool,
    seconds_to_minimize: u64,
    auto_manage_players: bool,
    stats_type: StatsType,
    window_scale: f64,
    rgb_buttons: bool,
    show_ws: bool,
    show_wlr: bool,
    show_fkdr: bool,
    show_kdr: bool,
    show_wins: bool,
    show_losses: bool,
    show_bans: bool,
}
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
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
            let client = match config["client"].as_i64().unwrap_or(0) {
                0 => MineClient::Default,
                1 => MineClient::Badlion,
                2 => MineClient::Lunar,
                3 => MineClient::LegacyLauncher,
                4 => MineClient::Custom(custom_client_path),
                5 => MineClient::Silent,
                _ => MineClient::Default,
            };
            let never_minimize = config["never_minimize"].as_bool().unwrap_or(false);
            let seconds_to_minimize = config["seconds_to_minimize"].as_u64().unwrap_or(10);
            let auto_manage_players = config["auto_manage_players"].as_bool().unwrap_or(true);
            let stats_type_str = config["stats_type"].as_str().unwrap_or("Bedwars Geral");
            let stats_type = StatsType::from_string(stats_type_str);
            let window_scale = config["window_scale"].as_f64().unwrap_or(1.0);
            let rgb_buttons = config["rgb_buttons"].as_bool().unwrap_or(false);
            let show_ws = config["show_ws"].as_bool().unwrap_or(true);
            let show_wlr = config["show_wlr"].as_bool().unwrap_or(true);
            let show_fkdr = config["show_fkdr"].as_bool().unwrap_or(true);
            let show_kdr = config["show_kdr"].as_bool().unwrap_or(true);
            let show_wins = config["show_wins"].as_bool().unwrap_or(true);
            let show_losses = config["show_losses"].as_bool().unwrap_or(true);
            let show_bans = config["show_bans"].as_bool().unwrap_or(false);

            app.manage(Mutex::new(KCOverlay {
                state: State {
                    screen_size,
                    cached_players: VecDeque::new(),
                    loading: false,
                    waiting: 0,
                    rates_full_time: Instant::now(),
                    searched_player_stats_type: StatsType::BedwarsAll,
                },
                settings: Settings {
                    client,
                    auto_manage_players,
                    never_minimize,
                    seconds_to_minimize,
                    stats_type,
                    window_scale,
                    rgb_buttons,
                    show_ws,
                    show_wlr,
                    show_fkdr,
                    show_kdr,
                    show_wins,
                    show_losses,
                    show_bans,
                },
            }));

            // let window = app.get_webview_window("main").unwrap();
            // let size = PhysicalSize::new(745. * window_scale, 460. * window_scale);
            // println!("{:?} {}", window.inner_size().unwrap(), window.scale_factor().unwrap());
            // window.set_size(size).unwrap();
            // println!("{:?} {}", window.inner_size().unwrap(), window.scale_factor().unwrap());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_settings(app: tauri::State<'_, KCOverlay>) -> Settings{
    app.settings.clone()
}
#[tauri::command]
async fn read_logs(
    handle: tauri::AppHandle,
    app: tauri::State<'_, Mutex<KCOverlay>>,
) -> Result<(), ()> {
    println!("Starting");
    handle
        .emit(
            "player",
            Player::new_nicked("Jogador_test".to_string(), StatsType::BedwarsAll),
        )
        .unwrap();
    let mut client = app.lock().await.settings.client.clone();
    let logs_path = client.get_logs_path();
    let mut file = File::open(&logs_path).await;

    /*
     * Se o arquivo de logs existir, tudo certo. Caso contrário, espera a lógica principal enviar um que exista.
     * O usuário pode selecionar um client que ele não tenha instalado ou colocar um custom client que não exista,
     * fazendo o programa procurar por um log inexistente.
     */
    match file {
        Ok(ok) => {
            file = Ok(ok);
        }
        Err(_) => {
            while !Path::new(&app.lock().await.settings.client.get_logs_path()).exists() {
                println!("Waiting logs");
                sleep(Duration::from_secs(1)).await;
            }
            client = app.lock().await.settings.client.clone();
            file = Ok(File::open(client.get_logs_path()).await.unwrap())
        }
    }

    let mut reader = BufReader::new(file.unwrap());
    let mut buffer = String::new();
    reader.seek(SeekFrom::End(0)).await.unwrap();

    let mut time_since_client_refresh = Instant::now();

    loop {
        match reader.read_line(&mut buffer).await {
            Ok(0) => {
                sleep(Duration::from_millis(500)).await;
            }
            Ok(_) => {
                let line = buffer.trim_end().to_string();
                handle_log_line(line, handle.clone(), &app).await;
                buffer.clear();
            }
            Err(e) => println!("Error at reading logs: {e}"),
        }

        // Atualiza o arquivo de logs do client, se necessário
        if app.lock().await.settings.client != client
            || time_since_client_refresh.elapsed() > Duration::from_secs(15)
        {
            println!("Refreshed client file");
            client = app.lock().await.settings.client.clone();
            let logs_path = client.get_logs_path();

            let file = match File::open(&logs_path).await {
                Ok(ok) => ok,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };

            reader = BufReader::new(file);
            buffer = String::new();
            reader.seek(SeekFrom::End(0)).await.unwrap();

            time_since_client_refresh = Instant::now();

            println!(
                "Cachê de jogadores: {} jogadores",
                app.lock().await.state.cached_players.len()
            );
        }
    }
}

async fn handle_log_line(
    line: String,
    handle: tauri::AppHandle,
    app_mutex: &tauri::State<'_, Mutex<KCOverlay>>,
) {
    // Checa se algum jogador entrou na partida.
    if app_mutex.lock().await.settings.auto_manage_players {
        let app = app_mutex.lock().await;
        if line.contains("entrou na sala") {
            // com certeza não é a maneira mais eficiente de fazer isso!
            let splitted_line: Vec<&str> = line.split(" ").collect();
            for (index, part) in splitted_line.clone().into_iter().enumerate() {
                if part == "entrou" {
                    let player_name: &str = splitted_line[index - 1];

                    let player =
                        player::get_player(player_name, app.settings.stats_type.clone()).await;

                    if let Ok(ok) = player {
                        handle.emit("player", ok).unwrap();
                    }

                    break;
                }
            }
        }
        // Checa se o jogador saiu da sala
        else if line.contains("saiu da sala") {
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
        else if line.contains("KILL FINAL") {
            let splitted_line: Vec<&str> = line.split(" ").collect();

            for (index, part) in splitted_line.clone().into_iter().enumerate() {
                if part == "morreu" {
                    let player_name: &str = splitted_line[index - 1];
                    handle.emit("remove_player", player_name).unwrap();
                    break;
                }
            }
        }
    }

    // Checa se a mensagem possui a lista de jogadores de quando o jogador digita "/jogando".
    if line.contains("[CHAT] Jogadores")
        && app_mutex.lock().await.state.waiting < 1
        && !app_mutex.lock().await.state.loading
    {
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
    }
}
