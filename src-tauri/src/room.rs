// obter dados da sala, incluindo a lista de jogadores

use reqwest::Client;
use serde_json::Value;

pub async fn get_room(id: &str) -> Result<Value, ()> {
    let client = Client::new();
    let url = "https://mush.com.br/api/game/".to_string() + id;

    let request = match client.get(url).send().await {
        Ok(response) => match response.text().await {
            Ok(ok) => ok,
            Err(e) => {
                println!("Falha ao obter texto da API da sala {id}: {e}\n Ignorado.");
                return Err(());
            }
        },
        Err(e) => {
            println!("Falha ao conseguir informações da sala {id}: {e}\n Ignorado.");
            return Err(());
        }
    };

    println!("Obtendo stats da sala {id}");

    let json: Value = match serde_json::from_str(&request) {
        Ok(ok) => ok,
        Err(e) => {
            println!("{e}");
            return Err(());
        }
    };

    if !json["success"].as_bool().unwrap() {
        println!("Falha ao obter informações da sala {id}\n Ignorado.");
        return Err(());
    }
    let response = json["response"].clone();

    Ok(response)
}
