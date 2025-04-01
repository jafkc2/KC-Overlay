use std::{
    collections::VecDeque, env, fs, io::SeekFrom, path::Path, time::{Duration, Instant}
};

use minecraft_clients::MineClient;
use player::Player;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
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
mod update;

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
    cached_players: VecDeque<Player>,
    loading: bool,
    rates_full_time: Instant,
    is_first_use : bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Settings {
    client: MineClient,
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
    automatic: bool
}
pub fn run() {
        // Isso é o processo final do update. Remove o executável antigo, caso exista.
        let old_exec = env::current_exe().unwrap().with_extension("old");
        if Path::new(&old_exec).exists() {
            match fs::remove_file(old_exec) {
                Ok(ok) => ok,
                Err(e) => println!("Failed to delete old executable: {e}"),
            }
        }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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
            let mut client = from_value::<MineClient>(serde_json::Value::Object(
                config["client"]
                    .as_object()
                    .unwrap_or(&serde_json::Map::new())
                    .clone(),
            ))
            .unwrap_or(MineClient::Default);

            match client{
                MineClient::Custom(_) => client = MineClient::Custom(custom_client_path.clone()),
               _ => ()
            }

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

            app.manage(Mutex::new(KCOverlay {
                state: State {
                    cached_players: VecDeque::new(),
                    loading: false,
                    rates_full_time: Instant::now(),
                    is_first_use
                },
                settings: Settings {
                    client,
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
                    automatic
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
            is_first_use
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn is_first_use(app: tauri::State<'_, Mutex<KCOverlay>>) -> Result<bool, ()>{
    Ok(app.lock().await.state.is_first_use)
}

#[tauri::command]
async fn get_settings(app: tauri::State<'_, Mutex<KCOverlay>>) -> Result<Settings, ()> {
    Ok(app.lock().await.settings.clone())
}

#[tauri::command]
async fn search_player(handle: tauri::AppHandle, username: String, stats_type: StatsType){
    let player = player::get_player(&username, stats_type, VecDeque::new()).await;

    match player{
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

    let mut client = app.lock().await.settings.client.clone();
    let logs_path = client.get_logs_path();
    let mut file = File::open(&logs_path).await;

    /*
     * Se o arquivo de logs existir, tudo certo. Caso contrário, espera um client com logs ser selecionado.
     * O usuário pode selecionar um client que ele não tenha instalado ou colocar um custom client que não exista,
     * fazendo o programa procurar por um log inexistente.
     */
    match file {
        Ok(ok) => {
            file = Ok(ok);
        }
        Err(_) => {

            while !Path::new(&app.lock().await.settings.client.get_logs_path()).exists() {
                println!("{} não existe", &app.lock().await.settings.client.get_logs_path());
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
            client = app.lock().await.settings.client.clone();

            let logs_path = client.get_logs_path();
            println!("Refreshed client file {}", logs_path.clone());

            let file = match File::open(&logs_path).await {
                Ok(ok) => ok,
                Err(e) => {
                    println!("{e}: {}", &logs_path);
                    continue;
                }
            };

            reader = BufReader::new(file);
            buffer = String::new();
            reader.seek(SeekFrom::End(0)).await.unwrap();

            time_since_client_refresh = Instant::now();
        }
    }
}

async fn handle_log_line(
    line: String,
    handle: tauri::AppHandle,
    app_mutex: &tauri::State<'_, Mutex<KCOverlay>>,
) {
    // Checa se algum jogador entrou na partida.
    //let app = app_mutex.lock().await;
    if line.contains("entrou na sala") && app_mutex.lock().await.settings.automatic {
        // com certeza não é a maneira mais eficiente de fazer isso!
        let splitted_line: Vec<&str> = line.split(" ").collect();
        for (index, part) in splitted_line.clone().into_iter().enumerate() {
            if part == "entrou" {
                let player_name: &str = splitted_line[index - 1];
                let stats_type = app_mutex.lock().await.settings.stats_type.clone();
                let cached_players = app_mutex.lock().await.state.cached_players.clone();
                let player = player::get_player(
                    player_name,
                    stats_type,
                    cached_players
                )
                .await;
                if let Ok(ok) = player {
                    app_mutex.lock().await.add_players_to_cache(vec![ok.clone()]);
                    handle.emit("player_joined", ok).unwrap();
                }

                break;
            }
        }
    }
    // Checa se o jogador saiu da sala
    else if line.contains("saiu da sala") && app_mutex.lock().await.settings.automatic{
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
    else if line.contains("KILL FINAL") && app_mutex.lock().await.settings.automatic{
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
    if line.contains("[CHAT] Jogadores")
        && !app_mutex.lock().await.state.loading
    {
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
    }
}
