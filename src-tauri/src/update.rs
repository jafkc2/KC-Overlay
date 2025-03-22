//! Módulo com funções para update do KC Overlay.

use std::{env, fs::{self, File}, io::Write};

use serde_json::Value;
use tauri::http::{header::USER_AGENT, HeaderValue};
use tauri_plugin_http::reqwest;

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
            return Err(())} ,
    };

    let last_release_json = match last_release_request.text().await {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(())},
    };

    let content = serde_json::from_str(&last_release_json);

    let j: Value = match content {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(())},
    };

    let current_version = handle.package_info().version.to_string();
    let latest_version = j["tag_name"].as_str().unwrap();

    let numeric_c_version = current_version.replace(".", "").parse::<i32>().unwrap();
    let numeric_l_version = match latest_version
        .replace(".", "")
        .replace("V", "")
        .parse::<i32>()
    {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(())},
    };

    if numeric_l_version > numeric_c_version {
        let mut url = String::new();
        if let Some(release_assets) = j["assets"].as_array() {
            for i in release_assets {
                match env::consts::OS {
                    "windows" => {
                        if i["name"].as_str().unwrap().contains("Windows") {
                            url = i["browser_download_url"].as_str().unwrap().to_string();
                            break;
                        }
                    }
                    "linux" => {
                        if i["name"].as_str().unwrap().contains("Linux") {
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
pub async fn install_update(url: String) -> Result<(), String> {
    match download_update(url).await{
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
        },
        Err(e) => Err(e.to_string()),
    }
}

async fn download_update(url: String) -> Result<(), String>{
    let exec_path = env::current_exe().unwrap();
    let mut exec_file = File::create(exec_path.with_extension("new")).unwrap();

    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permission = std::fs::metadata(exec_path.with_extension("new"))
            .unwrap()
            .permissions();
        permission.set_mode(0o755);
        std::fs::set_permissions(exec_path.with_extension("new"), permission).unwrap();
    }

    println!("{url}");
    let download = reqwest::get(url).await;

    match download {
        Ok(ok) => {
            exec_file.write_all(&ok.bytes().await.unwrap()).unwrap();
            println!("Update baixada com sucesso");
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}