use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};


const AZURE_CLIENT_ID: &str = "6bd94887-246e-4d0c-9c4b-3d7dc8b1cd9b";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MinecraftAccount {
    pub username: String,
    pub token: String,
    pub uuid: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AuthCode {
    pub code: String,
    pub link: String,
    pub device_code: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Default)]
pub struct XboxLiveData {
    user_hash: String,
    xsts_token: String,
}

#[tauri::command]
pub async fn request_code() -> AuthCode {
    let client = Client::new();
    let response = match client
        .get("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
        .query(&[
            ("client_id", AZURE_CLIENT_ID),
            ("scope", &"XboxLive.signin offline_access".to_string()),
        ])
        .send()
        .await
    {
        Ok(ok) => ok.text().await.unwrap(),
        Err(e) => panic!("{e}"),
    };

    let response_json: Value = serde_json::from_str(&response).unwrap();
    println!("{}", response_json);

    let code = response_json["user_code"].as_str().unwrap().to_owned();
    let link = response_json["verification_uri"]
        .as_str()
        .unwrap()
        .to_owned();

    let device_code = response_json["device_code"].as_str().unwrap().to_owned();

    AuthCode {
        code,
        link,
        device_code,
    }
}

#[tauri::command]
pub async fn wait_for_login(device_code: String) -> AuthToken {
    let client = reqwest::Client::new();
    println!("Obtendo AuthToken...");

    let mut auth_token = None;
    while auth_token.is_none() {
        println!("Obtendo AuthToken...");
        let response = match client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&[
                ("client_id", AZURE_CLIENT_ID),
                ("scope", &"XboxLive.signin offline_access".to_string()),
                (
                    "grant_type",
                    &"urn:ietf:params:oauth:grant-type:device_code".to_string(),
                ),
                ("device_code", &device_code),
            ])
            .send()
            .await
        {
            Ok(ok) => ok,
            Err(e) => panic!("{e}"),
        };
        match response.status() {
            StatusCode::OK => {
                println!("Authtoken obtido.");
                let response_json: Value =
                    serde_json::from_str(&response.text().await.unwrap()).unwrap();

                let access_token = response_json["access_token"].as_str().unwrap().to_string();
                let refresh_token = response_json["refresh_token"].as_str().unwrap().to_string();

                let token = AuthToken {
                    access_token,
                    refresh_token,
                };

                auth_token = Some(token);
            }
            _ => {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    auth_token.unwrap()
}

/* #[tauri::command]
pub async fn login(device_code: String) -> AuthToken{
    let auth = wait_for_login(device_code).await;

    handle.emit("login", 1).unwrap();
    match login_with_refresh_token(auth.refresh_token).await{
        Some(account) => {
            handle.emit("login", 2).unwrap();
            app_mutex.lock().await.state.account = Some(account);
        },
        None => {
            handle.emit("login", 3).unwrap();
        }
    }
    Ok(())
} */

pub async fn login_to_xbox(access_token: String) -> XboxLiveData {
    let client = Client::new();

    let xbox_live_response_request_data = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": &format!("d={}", access_token)
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });

    let xbox_live_response = match client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&xbox_live_response_request_data)
        .send()
        .await
    {
        Ok(ok) => ok.text().await.unwrap(),
        Err(e) => panic!("{e}"),
    };

    let xbox_live_response_json: Value = serde_json::from_str(&xbox_live_response).unwrap();

    let xbox_live_token = xbox_live_response_json["Token"]
        .as_str()
        .unwrap()
        .to_owned();

    let user_hash = xbox_live_response_json["DisplayClaims"]["xui"][0]["uhs"]
        .as_str()
        .unwrap()
        .to_owned();

    let xbox_xsts_response_request_data = json!(
        {
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [
                    xbox_live_token
                ]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
         }
    );

    let xbox_xsts_response = match client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&xbox_xsts_response_request_data)
        .send()
        .await
    {
        Ok(ok) => ok.text().await.unwrap(),
        Err(e) => panic!("{e}"),
    };

    let xbox_xsts_response_json: Value = serde_json::from_str(&xbox_xsts_response).unwrap();

    let xsts_token = xbox_xsts_response_json["Token"]
        .as_str()
        .unwrap()
        .to_owned();

    XboxLiveData {
        xsts_token,
        user_hash,
    }
}

pub async fn login_to_minecraft(xbox_data: XboxLiveData) -> MinecraftAccount {
    let client = Client::new();

    let minecraft_data_response_request_data = json!(
        {
        "identityToken": format!("XBL3.0 x={};{}", xbox_data.user_hash, xbox_data.xsts_token)
        }
    );

    let minecraft_data_response = match client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&minecraft_data_response_request_data)
        .send()
        .await
    {
        Ok(ok) => ok.text().await.unwrap(),
        Err(e) => panic!("{e}"),
    };

    let minecraft_data_json: Value = serde_json::from_str(&minecraft_data_response).unwrap();
    println!("{}", &minecraft_data_response);
    let token = minecraft_data_json["access_token"]
        .as_str()
        .unwrap()
        .to_owned();

    let mc_profile_response = match client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(token.clone())
        .send()
        .await
    {
        Ok(ok) => ok.text().await.unwrap(),
        Err(e) => panic!("{e}"),
    };

    let mc_profile_json: Value = serde_json::from_str(&mc_profile_response).unwrap();
    let uuid = mc_profile_json["id"].as_str().unwrap().to_owned();
    let username = mc_profile_json["name"].as_str().unwrap().to_owned();

    MinecraftAccount {
        username,
        token,
        uuid,
    }
}

#[tauri::command]
pub async fn login_with_refresh_token(refresh_token: String) -> Option<MinecraftAccount> {
    let client = Client::new();

    let response = match client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&[
            ("client_id", AZURE_CLIENT_ID),
            ("scope", &"XboxLive.signin offline_access".to_string()),
            ("grant_type", &"refresh_token".to_string()),
            ("refresh_token", &refresh_token),
        ])
        .send()
        .await
    {
        Ok(ok) => ok.text().await.unwrap(),
        Err(_) => return None,
    };

    let response_json: Value = serde_json::from_str(&response).unwrap();

    let access_token = response_json["access_token"].as_str().unwrap().to_owned();

    let xbox_data = login_to_xbox(access_token).await;

    Some(login_to_minecraft(xbox_data).await)
}
