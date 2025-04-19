use std::{
    collections::VecDeque,
    env, fs,
    io::SeekFrom,
    path::Path,
    time::{Duration, Instant},
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
mod update;
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
    cached_players: VecDeque<Player>,
    loading: bool,
    rates_full_time: Instant,
    is_first_use: bool,
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
    automatic: bool,
    remove_players: bool,
    hotkey: String,
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

            match client {
                MineClient::Custom(_) => client = MineClient::Custom(custom_client_path.clone()),
                _ => (),
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
            let remove_players = config["remove_players"].as_bool().unwrap_or(true);
            let hotkey = config["hotkey"]
                .as_str()
                .unwrap_or("Shift+Alt+Z")
                .to_string();

            app.manage(Mutex::new(KCOverlay {
                state: State {
                    cached_players: VecDeque::new(),
                    loading: false,
                    rates_full_time: Instant::now(),
                    is_first_use,
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
                    automatic,
                    remove_players,
                    hotkey,
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
async fn is_first_use(app: tauri::State<'_, Mutex<KCOverlay>>) -> Result<bool, ()> {
    Ok(app.lock().await.state.is_first_use)
}

#[tauri::command]
async fn get_settings(app: tauri::State<'_, Mutex<KCOverlay>>) -> Result<Settings, ()> {
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
            println!("Aguardando o arquivo de logs existir: {}", &app.lock().await.settings.client.get_logs_path());
            while !Path::new(&app.lock().await.settings.client.get_logs_path()).exists() {
                println!(
                    "{} não existe, tentando novamente...",
                    &app.lock().await.settings.client.get_logs_path()
                );
                sleep(Duration::from_secs(1)).await;
            }
            println!("Arquivo de logs encontrado!");
            client = app.lock().await.settings.client.clone();
            file = Ok(File::open(client.get_logs_path()).await.unwrap())
        }
    }

    let mut reader = BufReader::new(file.unwrap());
    let mut buffer = String::new();
    // Inicia a leitura a partir do final do arquivo para pegar apenas novas entradas
    reader.seek(SeekFrom::End(0)).await.unwrap();

    let mut time_since_client_refresh = Instant::now();

    loop {
        match reader.read_line(&mut buffer).await {
            Ok(0) => {
                // Não há novas linhas, aguarda um pouco antes de tentar novamente
                sleep(Duration::from_millis(200)).await;
            }
            Ok(_) => {
                let line = buffer.trim_end().to_string();
                // Otimização: Filtrar linhas irrelevantes antes de processar
                if line.contains("entrou na party") || 
                   line.contains("entrou na sala") || 
                   line.contains("saiu da sala") || 
                   line.contains("KILL FINAL") || 
                   line.contains("[CHAT] Jogadores") || 
                   line.contains("[CHAT] Enviando para") {
                    
                    handle_log_line(line, handle.clone(), &app).await;
                }
                buffer.clear();
            }
            Err(e) => println!("Erro ao ler logs: {e}"),
        }

        // Atualiza o arquivo de logs do client, se necessário
        if app.lock().await.settings.client != client
            || time_since_client_refresh.elapsed() > Duration::from_secs(15)
        {
            client = app.lock().await.settings.client.clone();

            let logs_path = client.get_logs_path();
            println!("Arquivo de logs atualizado: {}", logs_path.clone());

            match File::open(&logs_path).await {
                Ok(file) => {
                    reader = BufReader::new(file);
                    buffer = String::new();
                    reader.seek(SeekFrom::End(0)).await.unwrap();
                },
                Err(e) => {
                    println!("Erro ao abrir o arquivo de logs: {e}: {}", &logs_path);
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            time_since_client_refresh = Instant::now();
        }
    }
}

async fn handle_log_line(
    line: String,
    handle: tauri::AppHandle,
    app_mutex: &tauri::State<'_, Mutex<KCOverlay>>,
) {
    // Já sabemos que a linha contém um dos padrões relevantes (verificado na função read_logs)
    
    if line.contains("entrou na party") {
        // Otimização: Usar a função find_player_name para extrair de forma mais eficiente
        if let Some(player_name) = find_player_name(&line, "entrou") {
            let stats_type = app_mutex.lock().await.settings.stats_type.clone();
            let cached_players = app_mutex.lock().await.state.cached_players.clone();
            match player::get_player(player_name, stats_type, cached_players).await {
                Ok(player) => {
                    app_mutex.lock().await.add_players_to_cache(vec![player.clone()]);
                    handle.emit("party_player_joined", player).unwrap();
                }
                Err(_) => println!("Não foi possível obter os stats de {}", player_name),
            }
        }
    } else if line.contains("entrou na sala") && app_mutex.lock().await.settings.automatic && !app_mutex.lock().await.state.loading {
        if let Some(player_name) = find_player_name(&line, "entrou") {
            let stats_type = app_mutex.lock().await.settings.stats_type.clone();
            let cached_players = app_mutex.lock().await.state.cached_players.clone();
            match player::get_player(player_name, stats_type, cached_players).await {
                Ok(player) => {
                    app_mutex.lock().await.add_players_to_cache(vec![player.clone()]);
                    handle.emit("player_joined", player).unwrap();
                }
                Err(_) => println!("Não foi possível obter os stats de {}", player_name),
            }
        }
    } else if line.contains("saiu da sala") && app_mutex.lock().await.settings.automatic {
        if let Some(player_name) = find_player_name(&line, "saiu") {
            handle.emit("remove_player", player_name).unwrap();
        }
    } else if line.contains("KILL FINAL") && app_mutex.lock().await.settings.automatic {
        if let Some(player_name) = find_player_name(&line, "morreu") {
            handle.emit("remove_player", player_name).unwrap();
        }
    } else if line.contains("[CHAT] Jogadores") && !app_mutex.lock().await.state.loading {
        println!("Jogador digitou /jogando");
        
        // Otimização: Extrair jogadores mais diretamente
        if let Some(players_segment) = line.split("):").nth(1) {
            let str_players: Vec<String> = players_segment
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
    } else if line.contains("[CHAT] Enviando para") {
        handle.emit("remove_players", true).unwrap();
    } else if (line.contains("SALA") && line.contains("INICIANDO")) || line.contains("O jogo iniciou!") {
        // Detecta quando uma partida começa e tenta buscar automaticamente os jogadores
        // Aguarda um segundo para garantir que todos os jogadores foram atualizados
        let handle_clone = handle.clone();
        let app_mutex_clone = app_mutex.clone();
        
        tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            
            // Verifica se já não está carregando
            if app_mutex_clone.lock().await.state.loading {
                return;
            }
            
            // Envia comando para o chat do jogo simulando o comando "/jogando"
            println!("Partida iniciada! Buscando jogadores automaticamente...");
            handle_clone.emit("auto_jogando", true).unwrap();
            
            // Esperamos 2 segundos para o comando ser processado
            sleep(Duration::from_secs(2)).await;
            
            // Se ainda não estiver carregando (não detectou o comando), forçamos o carregamento com os jogadores detectados até agora
            if !app_mutex_clone.lock().await.state.loading {
                let mut player_names = Vec::new();
                
                // Coleta todos os nomes de jogadores detectados até agora
                {
                    let app = app_mutex_clone.lock().await;
                    for player in app.state.cached_players.iter() {
                        if player.is_connected {
                            player_names.push(player.username.clone());
                        }
                    }
                }
                
                // Se encontrou jogadores, carrega seus status
                if !player_names.is_empty() {
                    println!("Carregando {} jogadores detectados", player_names.len());
                    app_mutex_clone.lock().await.state.loading = true;
                    handle_clone.emit("loading", true).unwrap();
                    
                    let cached_players = app_mutex_clone.lock().await.state.cached_players.clone();
                    let stats_type = app_mutex_clone.lock().await.settings.stats_type.clone();
                    
                    player::get_players(player_names, stats_type, cached_players, handle_clone, &app_mutex_clone).await;
                }
            }
        });
    }
}

// Função auxiliar para extrair nomes de jogadores de forma mais eficiente
fn find_player_name<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let parts: Vec<&str> = line.split(' ').collect();
    for (i, part) in parts.iter().enumerate() {
        if *part == keyword && i > 0 {
            return Some(parts[i - 1]);
        }
    }
    None
}
