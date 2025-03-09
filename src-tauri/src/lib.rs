use std::{
    collections::VecDeque,
    io::SeekFrom,
    path::Path,
    time::{Duration, Instant},
};

use minecraft_clients::MineClient;
use player::Player;
use stats::{Stats, StatsType};
use tauri::{Emitter, Manager};
use tokio::{
    fs::File, io::{AsyncBufReadExt, AsyncSeekExt, BufReader}, sync::Mutex, time::sleep
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
    fn add_player(&mut self, player: Player) {
        self.state.players.push(player);
        self.state.players.sort_by(|a, b| {
            let b_level = match &b.stats {
                Stats::Bedwars(bedwars) => bedwars.level,
            };
            let a_level = match &a.stats {
                Stats::Bedwars(bedwars) => bedwars.level,
            };
            b_level.partial_cmp(&a_level).unwrap()
        });
        self.state.players.truncate(48);
    }
}

struct State {
    screen_size: (u32, u32),
    players: Vec<Player>,
    cached_players: VecDeque<Player>,
    loading: bool,
    waiting: i32,
    time_next_rate_limit_update: Instant,
    player_to_view_username: String,
    searched_player: Option<Player>,
    searched_player_stats_type: StatsType,
    rgb_offset: f32,
    is_visible: bool,
    // Para reconhecer 2 clicks e minimizar
    click_instant: Instant,
}

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
            app.manage(Mutex::new(KCOverlay {
                state: State {
                    screen_size,
                    players: vec![],
                    cached_players: VecDeque::new(),
                    loading: false,
                    waiting: 0,
                    time_next_rate_limit_update: Instant::now(),
                    player_to_view_username: String::new(),
                    searched_player: None,
                    searched_player_stats_type: StatsType::BedwarsAll,
                    rgb_offset: 0.,
                    is_visible: true,
                    click_instant: Instant::now(),
                },
                settings: Settings {
                    client: MineClient::Default,
                    auto_manage_players: true,
                    never_minimize: false,
                    seconds_to_minimize: 13,
                    stats_type: StatsType::BedwarsAll,
                    window_scale: 1.0,
                    rgb_buttons: false,
                    show_ws: true,
                    show_wlr: true,
                    show_fkdr: true,
                    show_kdr: true,
                    show_wins: true,
                    show_losses: true,
                    show_bans: true,
                },
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn read_logs(
    handle: tauri::AppHandle,
    app: tauri::State<'_, Mutex<KCOverlay>>,
) -> Result<(), ()> {
    println!("Starting");
    handle.emit("player", Player::new_nicked("Jogador_test".to_string(), StatsType::BedwarsAll)).unwrap();
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
        if app.lock().await.settings.client != client || time_since_client_refresh.elapsed() > Duration::from_secs(15){
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

            time_since_client_refresh = Instant::now()
        }
    }
    Ok(())
}

async fn handle_log_line(
    line: String,
    handle: tauri::AppHandle,
    app: &tauri::State<'_, Mutex<KCOverlay>>,
) {
    let mut app = app.lock().await;
    // Checa se algum jogador entrou na partida.
    if app.settings.auto_manage_players {
        if line.contains("entrou na sala") && !app.state.players.is_empty() {
            // com certeza não é a maneira mais eficiente de fazer isso!
            let splitted_line: Vec<&str> = line.split(" ").collect();
            for (index, part) in splitted_line.clone().into_iter().enumerate() {
                if part == "entrou" {
                    let player_name = splitted_line[index - 1];

                    let is_already_in_list = app
                        .state
                        .players
                        .iter()
                        .any(|player| player.username == player_name);

                    if !is_already_in_list {
                        let player =
                            player::get_player(player_name, app.settings.stats_type.clone()).await;

                        if let Ok(ok) = player {
                            app.add_player(ok);
                        }
                    }
                }
            }
        }
        // Checa se o jogador saiu da sala
        else if line.contains("saiu da sala") {
            for (index, player) in app.state.players.clone().iter().enumerate() {
                if line.contains(&player.username) {
                    app.state.players.remove(index);
                }
            }
        }
        // Checa se algum jogador que está na lista foi eliminado da partida.
        else if line.contains("KILL FINAL") {
            for (index, player) in app.state.players.clone().iter().enumerate() {
                if line.contains(&format!("{} morreu", player.username)) {
                    app.state.players.remove(index);
                }
            }
        }
    }

    // Checa se a mensagem possui a lista de jogadores de quando o jogador digita "/jogando".
    if line.contains("[CHAT] Jogadores") && app.state.waiting < 1 && !app.state.loading {
        let split = line.split("):").map(|x| x.to_string());
        let split_vector: Vec<String> = split.clone().collect();

        let str_players: Vec<String> = split_vector[1]
            .trim()
            .replace(" ", "")
            .replace("+", "")
            .split(',')
            .map(|x| x.to_string())
            .collect();

        // Sistema de cachê de jogadores para evitar o uso da api
        for player in app.state.players.clone() {
            let mut already_in_cache = false;
            for cached_player in app.state.cached_players.clone() {
                if player.username == cached_player.username {
                    already_in_cache = true;
                }
            }

            if !already_in_cache {
                app.state.cached_players.push_back(player);
                if app.state.cached_players.len() > 200 {
                    app.state.cached_players.pop_front();
                }
            }
        }

        app.state.players.clear();
        app.state.loading = true;

        player::get_players(str_players, app.settings.stats_type.clone(), app.state.cached_players.clone(), handle).await;
    }
}
