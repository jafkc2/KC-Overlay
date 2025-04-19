//! Módulo com funções para update do KC Overlay.

use std::{
    env,
    fs::{self, File},
    io::Write,
};

use futures::StreamExt;
use reqwest::Client;
use serde_json::Value;
use tauri::{
    http::{header::USER_AGENT, HeaderValue},
    Emitter,
};

#[tauri::command]
pub async fn check_updates(handle: tauri::AppHandle) -> Result<String, ()> {
    let last_release_request = match reqwest::Client::new()
        .get("https://api.github.com/repos/jafkc2/KC-Overlay/releases/latest")
        .header(USER_AGENT, HeaderValue::from_static("KC-Overlay"))
        .send()
        .await
    {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    let last_release_json = match last_release_request.text().await {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    let content = serde_json::from_str(&last_release_json);

    let j: Value = match content {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    let current_version = handle.package_info().version.to_string();
    let latest_version = j["tag_name"].as_str().unwrap_or(&current_version);

    let numeric_c_version = current_version.replace(".", "").parse::<i32>().unwrap();
    let numeric_l_version = match latest_version
        .replace(".", "")
        .replace("V", "")
        .parse::<i32>()
    {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    if numeric_l_version > numeric_c_version {
        let mut url = String::new();
        if let Some(release_assets) = j["assets"].as_array() {
            for i in release_assets {
                match env::consts::OS {
                    "windows" => {
                        if i["name"]
                            .as_str()
                            .unwrap()
                            .to_lowercase()
                            .contains("windows")
                        {
                            url = i["browser_download_url"].as_str().unwrap().to_string();
                            break;
                        }
                    }
                    "linux" => {
                        if i["name"].as_str().unwrap().to_lowercase().contains("linux") {
                            url = i["browser_download_url"].as_str().unwrap().to_string();
                            break;
                        }
                    }
                    "macos" => {
                        if i["name"].as_str().unwrap().to_lowercase().contains("macos") {
                            url = i["browser_download_url"].as_str().unwrap().to_string();
                            break;
                        }
                    }
                    _ => panic!("System not supported."),
                }
            }
        }
        Ok(url)
    } else {
        Err(())
    }
}

#[tauri::command]
pub async fn install_update(handle: tauri::AppHandle, url: String) -> Result<(), String> {
    match download_update(handle, url).await {
        Ok(_) => {
            let exec_path = env::current_exe().unwrap();

            let exec_name = exec_path
                .clone()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            // Coloca a extensão .old no executável antigo e renomeia o executável atualizado para o nome do executável antigo.
            fs::rename(&exec_path, exec_path.with_extension("old")).unwrap();
            fs::rename(exec_path.with_extension("new"), exec_path).unwrap();

            let mut new_exe_path = env::current_exe().unwrap();
            new_exe_path.pop();

            new_exe_path = new_exe_path.join(exec_name);

            match std::process::Command::new(new_exe_path).spawn() {
                Ok(_) => std::process::exit(0),
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

async fn download_update(handle: tauri::AppHandle, url: String) -> Result<(), String> {
    let exec_path = env::current_exe().unwrap();
    let mut exec_file = File::create(exec_path.with_extension("new")).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permission = std::fs::metadata(exec_path.with_extension("new"))
            .unwrap()
            .permissions();
        permission.set_mode(0o755);
        std::fs::set_permissions(exec_path.with_extension("new"), permission).unwrap();
    }

    println!("{url}");
    let client = Client::new();
    let download = client.get(url).send().await;

    match download {
        Ok(ok) => {
            let total_size = ok
                .headers()
                .get("content-length")
                .and_then(|v| v.to_str().ok()?.parse::<u64>().ok())
                .unwrap_or(0);

            let mut downloaded = 0;

            let mut stream = ok.bytes_stream();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.unwrap();
                exec_file.write_all(&chunk).unwrap();
                downloaded += chunk.len() as u64;

                handle
                    .emit("update_progress", downloaded * 100 / total_size)
                    .unwrap();
            }

            println!("Update baixada com sucesso");
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}
