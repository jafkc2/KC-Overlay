use std::{
    env,
    fs::File,
    io::Write,
};

use reqwest::header::{HeaderValue, USER_AGENT};
use serde_json::Value;

pub async fn check_updates() -> Result<String, String> {
    let last_release_request = match reqwest::Client::new()
        .get("https://api.github.com/repos/jafkc2/KC-Overlay/releases/latest")
        .header(USER_AGENT, HeaderValue::from_static("KC-Overlay"))
        .send()
        .await
    {
        Ok(ok) => ok,
        Err(e) => return Err(format!("Failed to check for updates: {}", e)),
    };

    let last_release_json = match last_release_request.text().await {
        Ok(ok) => ok,
        Err(e) => return Err(format!("Failed to read response: {}", e)),
    };

    let content = serde_json::from_str(&last_release_json);

    let j: Value = match content {
        Ok(ok) => ok,
        Err(e) => return Err(format!("Failed to read json: {}", e)),
    };

    let current_version = env!("CARGO_PKG_VERSION");
    let latest_version = j["tag_name"].as_str().unwrap();

    let numeric_c_version = current_version.replace(".", "").parse::<i32>().unwrap();
    let numeric_l_version = match latest_version
        .replace(".", "")
        .replace("V", "")
        .parse::<i32>()
    {
        Ok(ok) => ok,
        Err(_) => return Err("Failed to parse latest version number.".to_string()),
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
        Err("KC-Overlay está atualizado.".to_string())
    }
}

pub async fn install_update(url: String) -> Result<(), String> {
    let exec_path = env::current_exe().unwrap();
    let mut exec_file = File::create(exec_path.with_extension("new")).unwrap();

    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permission = fs::metadata(exec_path.with_extension("new"))
            .unwrap()
            .permissions();
        permission.set_mode(0o755);
        fs::set_permissions(exec_path.with_extension("new"), permission).unwrap();
    }

    println!("{url}");
    let download = reqwest::get(url).await;

    match download {
        Ok(ok) => {
            exec_file.write_all(&ok.bytes().await.unwrap()).unwrap();
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}
