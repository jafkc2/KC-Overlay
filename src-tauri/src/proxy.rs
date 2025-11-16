use aes::cipher::generic_array::GenericArray;
use aes::cipher::BlockEncrypt;
use aes::Aes128;
use aes_gcm::KeyInit;
use anyhow::anyhow;
use anyhow::Result;
use flate2::read::ZlibDecoder;
use generic_array::typenum::U16;
use lazy_static::lazy_static;
use num_bigint::BigInt;
use rand::rngs::OsRng;
use reqwest::Client;
use rsa::{pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey};
use rust_mc_proto::DataReader;
use serde::Serialize;
use sha1::Digest;
use tauri::Emitter;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};

use crate::login;

#[derive(Serialize)]
struct PacketEvent {
    direction: String,
    packet_id: i32,
    length: u32,
    is_player_list: bool,
    payload_b64: String,
    players: Option<Vec<String>>,
}
#[derive(Serialize)]
struct SessionJoinBody<'a> {
    accessToken: &'a str,
    selectedProfile: &'a str,
    serverId: &'a str,
}
lazy_static! {
    static ref PROXY_RSA_KEYPAIR: Mutex<RsaPrivateKey> =
        Mutex::new(RsaPrivateKey::new(&mut OsRng, 1024).expect("Falha ao gerar chave rsa"));
}

async fn get_proxy_public_key_der() -> Vec<u8> {
    let key = PROXY_RSA_KEYPAIR.lock().await;
    let pubkey = RsaPublicKey::from(key.clone());
    pubkey.to_public_key_der().unwrap().as_ref().to_vec()
}

fn compute_server_hash(server_id: &str, shared_secret: &[u8], server_pubkey_der: &[u8]) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(server_id.as_bytes());
    hasher.update(shared_secret);
    hasher.update(server_pubkey_der);
    let digest = hasher.finalize(); // 20 bytes

    let bigint = BigInt::from_signed_bytes_be(&digest);
    bigint.to_str_radix(16)
}
async fn perform_session_join_async(
    client: &Client,
    access_token: &str,
    selected_profile: &str,
    server_hash: &str,
) -> anyhow::Result<()> {
    let body = SessionJoinBody {
        accessToken: access_token,
        selectedProfile: selected_profile,
        serverId: server_hash,
    };

    let url = "https://sessionserver.mojang.com/session/minecraft/join";
    let resp = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("falha ao autenticar: {}", e))?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("falha ao autenticar: {} - {}", status, text);
    }
}
#[tauri::command]
pub async fn run_proxy(app_mutex: tauri::State<'_, Mutex<crate::KCOverlay>>, handle: tauri::AppHandle) -> Result<(), ()> {
    let listener = TcpListener::bind("127.0.0.1:25567").await.unwrap();

    let (tx, mut rx) = mpsc::unbounded_channel::<PacketEvent>();

    let refresh_token = app_mutex.lock().await.settings.account.token.clone();
    if !refresh_token.is_empty() {
        app_mutex.lock().await.state.account =
            match login::login_with_refresh_token(refresh_token.clone()).await {
                Some(acc) => Some(acc),
                None => None,
            };
    }

    tokio::spawn(async move {
        while let Some(ev) = rx.recv().await {
            if let Ok(js) = serde_json::to_string(&ev) {
                println!("a: {}", js);
            }
        }
    });

    loop {
        if refresh_token != app_mutex.lock().await.settings.account.token {
            let refresh_token = app_mutex.lock().await.settings.account.token.clone();
            if !refresh_token.is_empty() {
                println!("Conta mudou, realizando login novamente...");
                app_mutex.lock().await.state.account =
                    match login::login_with_refresh_token(refresh_token).await {
                        Some(acc) => Some(acc),
                        None => None,
                    };
            }
        }
        let (client, addr) =
            match tokio::time::timeout(Duration::from_secs(1), listener.accept()).await {
                Ok(result) => result.unwrap(),
                Err(_) => continue,
            };
        println!("{}", addr);
        let upstream = "l.mush.com.br:25565".to_string();
        let tx = tx.clone();

        let acc = app_mutex.lock().await.state.account.clone();
        if let Err(e) = handle_proxy_connection(client, &upstream, tx, &app_mutex, acc, &handle).await {
            eprintln!("Erro de conexão: {:?}", e);
        }
    }
}

async fn handle_proxy_connection(
    mut client: TcpStream,
    upstream: &str,
    events: mpsc::UnboundedSender<PacketEvent>,
    app: &tauri::State<'_, Mutex<crate::KCOverlay>>,
    acc: Option<login::MinecraftAccount>,
    handle: &tauri::AppHandle
) -> Result<()> {
    client.set_nodelay(true)?;

    let mut server = tokio::time::timeout(
        Duration::from_secs(5),
        TcpStream::connect(upstream)
    ).await
    .map_err(|_| anyhow::anyhow!("Timeout ao conectar ao servidor upstream"))??;
    
    server.set_nodelay(true)?;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let handshake_packet = tokio::time::timeout(
        Duration::from_secs(10),
        read_full_mc_frame(&mut client)
    ).await
    .map_err(|_| anyhow::anyhow!("Timeout ao ler pacote de handshake"))??;
    
    let mut cursor = Cursor::new(&handshake_packet);
    let _packet_len = cursor.read_varint()?;
    let _packet_id = cursor.read_varint()?;
    let _protocol_version = cursor.read_varint()?;
    let _server_address = cursor.read_string()?;
    let _server_port = cursor.read_u16().await?;
    let next_state = cursor.read_varint()?;
    
    server.write_all(&handshake_packet).await?;

    // verificar se é um pedido de ping
    if next_state == 1 {
        let status_request_packet = tokio::time::timeout(
            Duration::from_secs(10),
            read_full_mc_frame(&mut client)
        ).await
        .map_err(|_| anyhow::anyhow!("Timeout ao ler pacote de status"))??;
        
        server.write_all(&status_request_packet).await?;
        
        let status_response_packet = tokio::time::timeout(
            Duration::from_secs(10),
            read_full_mc_frame(&mut server)
        ).await
        .map_err(|_| anyhow::anyhow!("Timeout ao ler resposta de status"))??;
        
        client.write_all(&status_response_packet).await?;
        
        let ping_packet = tokio::time::timeout(
            Duration::from_secs(10),
            read_full_mc_frame(&mut client)
        ).await
        .map_err(|_| anyhow::anyhow!("Timeout ao ler pacote de ping"))??;
        
        server.write_all(&ping_packet).await?;
        
        let pong_packet = tokio::time::timeout(
            Duration::from_secs(10),
            read_full_mc_frame(&mut server)
        ).await
        .map_err(|_| anyhow::anyhow!("Timeout ao ler resposta de pong"))??;
        
        client.write_all(&pong_packet).await?;
        
        return Ok(());
    }
    
    let login_start_packet = tokio::time::timeout(
        Duration::from_secs(10),
        read_full_mc_frame(&mut client)
    ).await
    .map_err(|_| anyhow::anyhow!("Timeout ao ler pacote de login start"))??;
    
    server.write_all(&login_start_packet).await?;

    println!("Handshake concluído com sucesso");

    let mut secret: Option<Vec<u8>> = None;
    let mut compression_threshold: Option<i32> = None;
    let mut seen_login_success = false;

    let mut iv_c2s_forward = [0u8; 16];
    let mut iv_s2c_forward = [0u8; 16];
    let mut iv_s2c_parse = iv_s2c_forward;
    let mut pending_s2c: Vec<u8> = Vec::new();

    'login: loop {
        if seen_login_success {
            break;
        }

        let enc_req_packet =
            tokio::time::timeout(Duration::from_secs(10), read_full_mc_frame(&mut server))
                .await
                .map_err(|_| anyhow::anyhow!("Timeout ao ler pacotes do servidor"))??;

        let mut cursor = Cursor::new(&enc_req_packet);

        let _outer_len = cursor.read_varint()?;
        let payload_offset = cursor.position() as usize;
        let payload = &enc_req_packet[payload_offset..];

        let (packet_id, body) = match parse_packet_id_and_body(payload, compression_threshold) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("falha ao realizar parse de pacote de login: {}", e);
                client.write_all(&enc_req_packet).await?;
                continue;
            }
        };

        match packet_id {
            0x01 => {
                let mut pc = Cursor::new(&body);
                let server_id = pc.read_string()?;
                let pubkey_len = pc.read_varint()? as usize;
                let server_pubkey_bytes = pc.read_bytes(pubkey_len)?.to_vec();
                let verify_len = pc.read_varint()? as usize;
                let verify_token = pc.read_bytes(verify_len)?.to_vec();

                fn write_varint(buf: &mut Vec<u8>, mut value: i32) {
                    loop {
                        if (value & !0x7F) == 0 {
                            buf.push(value as u8);
                            return;
                        } else {
                            buf.push(((value & 0x7F) | 0x80) as u8);
                            value = ((value as u32) >> 7) as i32;
                        }
                    }
                }
                fn write_mc_string(buf: &mut Vec<u8>, s: &str) {
                    write_varint(buf, s.len() as i32);
                    buf.extend_from_slice(s.as_bytes());
                }

                let proxy_pubkey = get_proxy_public_key_der().await;
                let mut payload2 = Vec::new();
                write_mc_string(&mut payload2, &server_id);
                write_varint(&mut payload2, proxy_pubkey.len() as i32);
                payload2.extend_from_slice(&proxy_pubkey);
                write_varint(&mut payload2, verify_token.len() as i32);
                payload2.extend_from_slice(&verify_token);

                let mut packet_data = Vec::new();
                write_varint(&mut packet_data, 0x01);
                packet_data.extend_from_slice(&payload2);
                let mut full_packet = Vec::new();
                write_varint(&mut full_packet, packet_data.len() as i32);
                full_packet.extend_from_slice(&packet_data);

                client.write_all(&full_packet).await?;
                println!("pedido de encryption enviado ao client");

                let enc_resp_packet = read_full_mc_frame(&mut client).await?;
                let mut rc = Cursor::new(&enc_resp_packet);
                let _ = rc.read_varint()?;
                let resp_packet_id = rc.read_varint()?;
                if resp_packet_id == 0x03 {
                    server.write_all(&enc_resp_packet).await?;
                    continue;
                }
                if resp_packet_id != 0x01 {
                    anyhow::bail!("id=0x{:X} em vez de 0x01", resp_packet_id);
                }

                let secret_len = rc.read_varint()? as usize;
                let secret_enc = rc.read_bytes(secret_len)?.to_vec();
                let verify_len = rc.read_varint()? as usize;
                let verify_enc = rc.read_bytes(verify_len)?.to_vec();

                use rsa::pkcs1v15::Pkcs1v15Encrypt;
                let key = PROXY_RSA_KEYPAIR.lock().await;
                let shared_secret = key
                    .decrypt(Pkcs1v15Encrypt, &secret_enc)
                    .map_err(|e| anyhow::anyhow!("Falha ao descriptografar shared secret: {e}"))?;
                let verify_token = key
                    .decrypt(Pkcs1v15Encrypt, &verify_enc)
                    .map_err(|e| anyhow::anyhow!("Falha ao descriptografar verify token: {e}"))?;
                println!("shared secret descriptografado");

                let http_client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()?;
                let server_hash =
                    compute_server_hash(&server_id, &shared_secret, &server_pubkey_bytes);
                println!("server_hash: {}", server_hash);

                if let Some(acc) = acc {
                    perform_session_join_async(&http_client, &acc.token, &acc.uuid, &server_hash)
                        .await
                        .map_err(|e| anyhow::anyhow!("falha ao logar: {e}"))?;
                    println!("logado como {}", &acc.username);
                } else {
                    handle.emit("not_logged", ()).unwrap();
                    println!("Usuário entrou usando uma conta original, porém não há nenhuma conta logada no KC Overlay!")
                }

                use rand::rngs::OsRng;
                use rsa::pkcs8::DecodePublicKey;
                let server_pubkey = rsa::RsaPublicKey::from_public_key_der(&server_pubkey_bytes)
                    .map_err(|e| anyhow::anyhow!("Erro ao fazer parse de chave pública: {e}"))?;
                let mut rng = OsRng;
                let secret_enc_server = server_pubkey
                    .encrypt(&mut rng, rsa::pkcs1v15::Pkcs1v15Encrypt, &shared_secret)
                    .map_err(|e| {
                        anyhow::anyhow!("Erro ao criptografar chave secreta para o servidor: {e}")
                    })?;
                let verify_enc_server = server_pubkey
                    .encrypt(&mut rng, rsa::pkcs1v15::Pkcs1v15Encrypt, &verify_token)
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Erro ao criptografar token de verificação para o servidor: {e}"
                        )
                    })?;

                let mut payload3 = Vec::new();
                write_varint(&mut payload3, secret_enc_server.len() as i32);
                payload3.extend_from_slice(&secret_enc_server);
                write_varint(&mut payload3, verify_enc_server.len() as i32);
                payload3.extend_from_slice(&verify_enc_server);

                let mut packet_data2 = Vec::new();
                write_varint(&mut packet_data2, 0x01);
                packet_data2.extend_from_slice(&payload3);
                let mut full_packet2 = Vec::new();
                write_varint(&mut full_packet2, packet_data2.len() as i32);
                full_packet2.extend_from_slice(&packet_data2);

                server.write_all(&full_packet2).await?;

                use aes::Aes128;
                let mut key_bytes = [0u8; 16];
                if shared_secret.len() < 16 {
                    anyhow::bail!("shared_secret curto");
                }
                key_bytes.copy_from_slice(&shared_secret[..16]);
                let aes = Aes128::new(GenericArray::from_slice(&key_bytes));

                let mut iv_c2s_forwardb = [0u8; 16];
                let mut iv_s2c_forwardb = [0u8; 16];
                iv_c2s_forwardb.copy_from_slice(&shared_secret[..16]);
                iv_s2c_forwardb.copy_from_slice(&shared_secret[..16]);

                let mut iv_s2c_parseb = iv_s2c_forwardb;
                let mut pending_s2cb: Vec<u8> = Vec::new();

                loop {
                    let mut buf = [0u8; 8192];
                    let n = server.read(&mut buf).await?;
                    if n == 0 {
                        anyhow::bail!("n == 0");
                    }
                    let raw = &buf[..n];

                    let mut discard = raw.to_vec();
                    cfb8_decrypt_in_place(&aes, &mut iv_s2c_forwardb, &mut discard);

                    let mut dec_for_parse = raw.to_vec();
                    cfb8_decrypt_in_place(&aes, &mut iv_s2c_parseb, &mut dec_for_parse);

                    pending_s2cb.extend_from_slice(&dec_for_parse);

                    let mut offset = 0usize;
                    while offset < pending_s2cb.len() {
                        let rem = &pending_s2cb[offset..];
                        let (packet_len, len_bytes) = match read_varint_from_slice(rem) {
                            Some(v) => v,
                            None => break,
                        };

                        let packet_len_usize = packet_len as usize;
                        let total_needed = len_bytes + packet_len_usize;
                        if rem.len() < total_needed {
                            break;
                        }

                        let packet_payload = &rem[len_bytes..total_needed];
                        let compression_now = compression_threshold;
                        let parse_res = parse_packet_id_and_body(packet_payload, compression_now);

                        match parse_res {
                            Ok((pid, body)) => {
                                println!("L: S->C packet id: {}", pid);

                                if pid == 0x03 {
                                    let mut rc = Cursor::new(&body);

                                    let threshold = rc.read_varint().unwrap_or(-1);

                                    println!("Nível de compressão = {}", threshold);
                                    compression_threshold = Some(threshold);
                                } else if pid == 0x02 {
                                    println!("O Login foi um sucesso!");
                                    secret = Some(shared_secret.clone());
                                    seen_login_success = true;
                                    iv_c2s_forward = iv_c2s_forwardb;
                                    iv_s2c_forward = iv_s2c_forwardb;
                                    iv_s2c_parse = iv_s2c_parseb;
                                    break;
                                }
                            }
                            Err(e) => {
                                if compression_now.is_some() {
                                    eprintln!("parse error (zlib?): {}", e);
                                } else {
                                    eprintln!("parse error (uncompressed): {}", e);
                                }
                            }
                        }

                        offset += total_needed;
                    }

                    if offset > 0 {
                        pending_s2cb.drain(..offset);
                    }

                    client.write_all(raw).await?;

                    if seen_login_success {
                        break;
                    }
                }

                pending_s2c = pending_s2cb;
                break 'login;
            }

            0x03 => {
                let mut bc = Cursor::new(&body);
                let threshold = bc.read_varint()?;
                println!("O server solicitou compressão: {} de threshold", threshold);
                client.write_all(&enc_req_packet).await?;
                compression_threshold = Some(threshold);
                continue;
            }

            0x02 => {
                println!("O login foi um sucesso! Sem criptografia.");
                client.write_all(&enc_req_packet).await?;
                break;
            }

            other => {
                println!("repassando pacote com id 0x{:X}", other);
                client.write_all(&enc_req_packet).await?;
                continue;
            }
        }
    }

    if secret.is_none() {
        let (mut cr, mut cw) = client.split();
        let (mut sr, mut sw) = server.split();

        let compression_threshold = compression_threshold;
        let mut pending_s2c: Vec<u8> = Vec::new();
        let mut pending_c2s: Vec<u8> = Vec::new();

        let stop_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop_flag_c2s = stop_flag.clone();
        let stop_flag_s2c = stop_flag.clone();

        let c2s_plain = async move {
            loop {
                if stop_flag_c2s.load(std::sync::atomic::Ordering::Relaxed) {
                    eprintln!("C->S: Recebido sinal de parada");
                    break;
                }

                let mut buf = [0u8; 8192];
                match cr.read(&mut buf).await {
                    Ok(0) => {
                        eprintln!("C->S conexão fechada pelo cliente");
                        stop_flag_c2s.store(true, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                    Ok(n) => {
                        let raw = &buf[..n];
                        pending_c2s.extend_from_slice(raw);

                        let mut offset = 0usize;
                        while offset < pending_c2s.len() {
                            let rem = &pending_c2s[offset..];
                            match read_varint_from_slice(rem) {
                                Some((packet_len, len_bytes)) => {
                                    let packet_len_usize = packet_len as usize;
                                    let total_needed = len_bytes + packet_len_usize;
                                    if rem.len() < total_needed {
                                        break;
                                    }

                                    let packet = &rem[..total_needed];
                                    if let Err(e) = sw.write_all(packet).await {
                                        eprintln!("C->S erro de escrita: {:?}", e);
                                        stop_flag_c2s
                                            .store(true, std::sync::atomic::Ordering::Relaxed);
                                        break;
                                    }
                                    offset += total_needed;
                                }
                                None => {
                                    break;
                                }
                            }
                        }

                        if offset > 0 {
                            pending_c2s.drain(..offset);
                        }
                    }
                    Err(e) => {
                        eprintln!("C->S erro de leitura: {:?}", e);
                        stop_flag_c2s.store(true, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        };

        let s2c_plain = async move {
            loop {
                if stop_flag_s2c.load(std::sync::atomic::Ordering::Relaxed) {
                    eprintln!("S->C: Recebido sinal de parada");
                    break;
                }

                let mut buf = [0u8; 8192];
                match sr.read(&mut buf).await {
                    Ok(0) => {
                        eprintln!("S->C conexão fechada pelo servidor");
                        stop_flag_s2c.store(true, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                    Ok(n) => {
                        let raw = &buf[..n];
                        pending_s2c.extend_from_slice(raw);

                        let mut offset = 0usize;
                        while offset < pending_s2c.len() {
                            let rem = &pending_s2c[offset..];
                            match read_varint_from_slice(rem) {
                                Some((packet_len, len_bytes)) => {
                                    let packet_len_usize = packet_len as usize;
                                    let total_needed = len_bytes + packet_len_usize;
                                    if rem.len() < total_needed {
                                        break;
                                    }

                                    let packet = &rem[..total_needed];

                                    let mut cur = Cursor::new(packet);
                                    if let Ok(_) = cur.read_varint() {
                                        let payload_offset = cur.position() as usize;
                                        let payload = &packet[payload_offset..];
                                        match parse_packet_id_and_body(
                                            payload,
                                            compression_threshold,
                                        ) {
                                            Ok((pid, body)) => {
                                                if pid == 0x38 {
                                                    let (players_added, players_removed) =
                                                        match extract_player_names_from_player_info(
                                                            &body,
                                                        ) {
                                                            Ok((added, removed)) => {
                                                                (added, removed)
                                                            }
                                                            Err(_) => (HashMap::new(), Vec::new()),
                                                        };
                                                    app.lock().await.update_player_list(
                                                        players_added,
                                                        players_removed,
                                                    );
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!(
                                                    "(sem criptografia) S->C erro de parse: {}",
                                                    e
                                                );
                                            }
                                        }
                                    }

                                    if let Err(e) = cw.write_all(packet).await {
                                        eprintln!("S->C erro de escrita: {:?}", e);
                                        stop_flag_s2c
                                            .store(true, std::sync::atomic::Ordering::Relaxed);
                                        break;
                                    }
                                    offset += total_needed;
                                }
                                None => {
                                    break;
                                }
                            }
                        }

                        if offset > 0 {
                            pending_s2c.drain(..offset);
                        }
                    }
                    Err(e) => {
                        eprintln!("S->C erro de leitura: {:?}", e);
                        stop_flag_s2c.store(true, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        };

        let _ = tokio::try_join!(c2s_plain, s2c_plain);
    } else {
        let secret_vec = secret
            .take()
            .ok_or_else(|| anyhow::anyhow!("shared secret inexistente!"))?;
        if secret_vec.len() < 16 {
            anyhow::bail!("shared_secret curto");
        }

        let mut key_bytes = [0u8; 16];
        key_bytes.copy_from_slice(&secret_vec[..16]);
        let aes = Aes128::new(GenericArray::from_slice(&key_bytes));

        let mut iv_c2s_forward = iv_c2s_forward;
        let mut iv_s2c_forward = iv_s2c_forward;
        let mut iv_c2s_parse = iv_c2s_forward;
        let mut iv_s2c_parse = iv_s2c_forward;

        let (mut cr, mut cw) = client.split();
        let (mut sr, mut sw) = server.split();

        let mut pending_s2c: Vec<u8> = pending_s2c;
        let mut pending_c2s: Vec<u8> = Vec::new();

        println!(
            "Lendo pacotes. threshold de compressão = {:?}",
            compression_threshold
        );

        let stop_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        loop {
            if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                eprintln!("Recebido sinal de parada");
                break;
            }

            tokio::select! {
                res = sr.read_buf(&mut pending_s2c) => {
                    let n = res.map_err(|e| anyhow!("server read error: {}", e))?;
                    if n == 0 {
                        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                        anyhow::bail!("n==0");
                    }

                    let raw_start_index = pending_s2c.len() - n;

                    let raw_clone_for_forwarding = pending_s2c[raw_start_index..].to_vec();
                    let mut iv_discard = raw_clone_for_forwarding.clone();

                    let dec_for_parse_slice = &mut pending_s2c[raw_start_index..];
                    cfb8_decrypt_in_place(&aes, &mut iv_s2c_parse, dec_for_parse_slice);

                    cfb8_decrypt_in_place(&aes, &mut iv_s2c_forward, &mut iv_discard);

                    let mut offset = 0usize;
                    while offset < pending_s2c.len() {
                        let rem = &pending_s2c[offset..];
                        let (packet_len, len_bytes) = match read_varint_from_slice(rem) { Some(v) => v, None => break };
                        let total_needed = len_bytes + (packet_len as usize);
                        if rem.len() < total_needed { break; }

                        let packet_payload = &rem[len_bytes..total_needed];

                        match parse_packet_id_and_body(packet_payload, compression_threshold) {
                            Ok((pid, body)) => {
                                if pid == 0x38 {
                                    let (players_added, players_removed) =
                                    match extract_player_names_from_player_info(&body) {
                                        Ok((added, removed)) => (added, removed),
                                        Err(_) => (HashMap::new(), Vec::new()),
                                    };
                                    app.lock().await.update_player_list(players_added, players_removed);
                                }
                            },
                            Err(e) => eprintln!("(com criptografia) S->C erro de parse: {}", e),
                        }
                        offset += total_needed;
                    }
                    if offset > 0 { pending_s2c.drain(..offset); }

                    if let Err(e) = cw.write_all(&raw_clone_for_forwarding).await {
                        eprintln!("S->C erro de escrita: {:?}", e);
                        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                }

                res = cr.read_buf(&mut pending_c2s) => {
                    let n = res.map_err(|e| anyhow!("client erro de leitura: {}", e))?;
                    if n == 0 {
                        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                        anyhow::bail!("n==0");
                    }

                    let raw_start_index = pending_c2s.len() - n;

                    let raw_clone_for_forwarding = pending_c2s[raw_start_index..].to_vec();
                    let mut iv_discard = raw_clone_for_forwarding.clone();

                    let dec_for_parse_slice = &mut pending_c2s[raw_start_index..];
                    cfb8_decrypt_in_place(&aes, &mut iv_c2s_parse, dec_for_parse_slice);

                    cfb8_decrypt_in_place(&aes, &mut iv_c2s_forward, &mut iv_discard);

                    let mut offset = 0usize;
                    while offset < pending_c2s.len() {
                        let rem = &pending_c2s[offset..];
                        let (packet_len, len_bytes) = match read_varint_from_slice(rem) { Some(v) => v, None => break };
                        let total_needed = len_bytes + (packet_len as usize);
                        if rem.len() < total_needed { break; }

                        let packet_payload = &rem[len_bytes..total_needed];

                        offset += total_needed;
                    }
                    if offset > 0 { pending_c2s.drain(..offset); }

                    if let Err(e) = sw.write_all(&raw_clone_for_forwarding).await {
                        eprintln!("C->S erro de escrita: {:?}", e);
                        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                }
            }

            if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                eprintln!("Recebido sinal de parada");
                break;
            }
        }
    }

    Ok(())
}

async fn run_play_relay(
    mut client: TcpStream,
    mut server: TcpStream,
    mut secret: Option<Vec<u8>>,
    compression_threshold: Option<i32>,
    initial_pending_s2c: Vec<u8>,
    initial_iv_s2c_parse: [u8; 16],
    initial_iv_s2c_forward: [u8; 16],
    initial_pending_c2s: Vec<u8>,
    initial_iv_c2s_parse: [u8; 16],
    initial_iv_c2s_forward: [u8; 16],
    app: &tauri::State<'_, Mutex<crate::KCOverlay>>,
) -> anyhow::Result<()> {
    let secret_vec = secret
        .take()
        .ok_or_else(|| anyhow::anyhow!("shared secret inexistente!"))?;
    if secret_vec.len() < 16 {
        anyhow::bail!("shared_secret curto");
    }

    let mut key_bytes = [0u8; 16];
    key_bytes.copy_from_slice(&secret_vec[..16]);
    let aes = Aes128::new(GenericArray::from_slice(&key_bytes));

    let mut iv_c2s_forward = initial_iv_c2s_forward;
    let mut iv_s2c_forward = initial_iv_s2c_forward;
    let mut iv_c2s_parse = initial_iv_c2s_parse;
    let mut iv_s2c_parse = initial_iv_s2c_parse;

    let (mut cr, mut cw) = client.split();
    let (mut sr, mut sw) = server.split();

    let mut pending_s2c: Vec<u8> = initial_pending_s2c;
    let mut pending_c2s: Vec<u8> = initial_pending_c2s;

    println!(
        "Lendo pacotes. threshold de compressão = {:?}",
        compression_threshold
    );
    loop {
        tokio::select! {
                    res = sr.read_buf(&mut pending_s2c) => {
                        let n = res.map_err(|e| anyhow!("server read error: {}", e))?;
                        if n == 0 { anyhow::bail!("n==0"); }

                        let raw_start_index = pending_s2c.len() - n;

                        let raw_clone_for_forwarding = pending_s2c[raw_start_index..].to_vec();
                        let mut iv_discard = raw_clone_for_forwarding.clone();

                        let dec_for_parse_slice = &mut pending_s2c[raw_start_index..];
                        cfb8_decrypt_in_place(&aes, &mut iv_s2c_parse, dec_for_parse_slice);

                        cfb8_decrypt_in_place(&aes, &mut iv_s2c_forward, &mut iv_discard);

                        let mut offset = 0usize;
                        while offset < pending_s2c.len() {
                            let rem = &pending_s2c[offset..];
                            let (packet_len, len_bytes) = match read_varint_from_slice(rem) { Some(v) => v, None => break };
                            let total_needed = len_bytes + (packet_len as usize);
                            if rem.len() < total_needed { break; }

                            let packet_payload = &rem[len_bytes..total_needed];

                            match parse_packet_id_and_body(packet_payload, compression_threshold) {
                                Ok((pid, body)) => {
                                    if pid == 0x38 {
                                        let (players_added, players_removed) =
                                        match extract_player_names_from_player_info(&body) {
                                            Ok((added, removed)) => (added, removed),
                                            Err(_) => (HashMap::new(), Vec::new()),
                                        };
                                        app.lock().await.update_player_list(players_added, players_removed);
                                    }
                                },
                                Err(e) => eprintln!("(sem criptografia) S->C erro de parse: {}", e),
                            }
                            offset += total_needed;
                        }
                        if offset > 0 { pending_s2c.drain(..offset); }

                        cw.write_all(&raw_clone_for_forwarding).await?;
                    }

                    res = cr.read_buf(&mut pending_c2s) => {
                        let n = res.map_err(|e| anyhow!("client erro de leitura: {}", e))?;
                        if n == 0 { anyhow::bail!("n==0"); }

                        let raw_start_index = pending_c2s.len() - n;

                        let raw_clone_for_forwarding = pending_c2s[raw_start_index..].to_vec();
                        let mut iv_discard = raw_clone_for_forwarding.clone();

                        let dec_for_parse_slice = &mut pending_c2s[raw_start_index..];
                        cfb8_decrypt_in_place(&aes, &mut iv_c2s_parse, dec_for_parse_slice);

                        cfb8_decrypt_in_place(&aes, &mut iv_c2s_forward, &mut iv_discard);

                        let mut offset = 0usize;
                        while offset < pending_c2s.len() {
                            let rem = &pending_c2s[offset..];
                            let (packet_len, len_bytes) = match read_varint_from_slice(rem) { Some(v) => v, None => break };
                            let total_needed = len_bytes + (packet_len as usize);
                            if rem.len() < total_needed { break; }

                            let packet_payload = &rem[len_bytes..total_needed];

        /*                     match parse_packet_id_and_body(packet_payload, compression_threshold) {
                                Ok((pid, _body)) => (),
                                Err(e) => eprintln!("(PLAIN) C->S parse error (logging only): {}", e),
                            } */
                            offset += total_needed;
                        }
                        if offset > 0 { pending_c2s.drain(..offset); }

                        sw.write_all(&raw_clone_for_forwarding).await?;
                    }
                }
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn parse_packet_id_and_body(
    payload: &[u8],
    compression_threshold: Option<i32>,
) -> anyhow::Result<(i32, Vec<u8>)> {
    if compression_threshold.is_none() {
        if let Some((pid, id_len)) = read_varint_from_slice(payload) {
            let body = payload[id_len..].to_vec();
            return Ok((pid, body));
        } else {
            anyhow::bail!("varint incompleto (sem compressão)");
        }
    }

    let (data_len, dl_len) = read_varint_from_slice(payload)
        .ok_or_else(|| anyhow::anyhow!("varint incompleto (compressão)"))?;

    if data_len == 0 {
        let uncompressed = &payload[dl_len..];
        if let Some((pid, id_len)) = read_varint_from_slice(uncompressed) {
            let body = uncompressed[id_len..].to_vec();
            return Ok((pid, body));
        } else {
            anyhow::bail!("varint incompleto");
        }
    } else {
        let compressed = &payload[dl_len..];
        let mut decoder = ZlibDecoder::new(compressed);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| anyhow::anyhow!("erro de descompactação: {}", e))?;

        if let Some((pid, id_len)) = read_varint_from_slice(&decompressed) {
            let body = decompressed[id_len..].to_vec();
            return Ok((pid, body));
        } else {
            anyhow::bail!("varint incompleto");
        }
    }
}

fn cfb8_decrypt_in_place(aes: &Aes128, iv: &mut [u8; 16], buf: &mut [u8]) {
    for b in buf.iter_mut() {
        let mut block = GenericArray::<u8, U16>::clone_from_slice(iv);
        aes.encrypt_block(&mut block);
        let msb = block[0];
        let ciphertext = *b;
        let plaintext = ciphertext ^ msb;
        *b = plaintext;
        iv.rotate_left(1);
        iv[15] = ciphertext;
    }
}

fn extract_player_names_from_player_info(
    payload: &[u8],
) -> Result<(HashMap<&[u8], String>, Vec<&[u8]>), ()> {
    let mut cur = Cursor::new(payload);
    let action = read_varint_from_cursor(&mut cur)?; // 0..4
    let num_players = read_varint_from_cursor(&mut cur)? as usize;

    //let mut added = Vec::new();
    let mut added = HashMap::new();
    let mut removed = Vec::new();

    for _ in 0..num_players {
        let uuid = read_bytes_from_cursor(&mut cur, 16)?;

        match action {
            // add player
            0 => {
                let name = read_mc_string_from_cursor(&mut cur)?;
                //added.push(name);

                // bots do mush tem nomes hexadecimais, esse if é pra ignorá-los
                if !name.chars().all(|c| c.is_ascii_hexdigit()) {
                    added.insert(uuid, name);
                }
            }
            // update gamemode, irrelevante
            1 => {}
            // update ping, útil no futuro para adicionar stat de ping
            2 => {
                //let ping = read_varint_from_cursor(&mut cur)?;
            }
            // update display name, irrelevante
            3 => {}
            // remove player
            4 => {
                removed.push(uuid);
            }

            _ => {
                return Err(());
            }
        }
    }

    Ok((added, removed))
}

fn read_bytes_from_cursor<'a>(cur: &mut Cursor<&'a [u8]>, len: usize) -> Result<&'a [u8], ()> {
    let start = cur.position() as usize;
    let end = start + len;
    let buf_len = cur.get_ref().len();
    if end > buf_len {
        return Err(());
    }
    cur.set_position(end as u64);
    Ok(&cur.get_ref()[start..end])
}

fn read_mc_string_from_cursor(cur: &mut Cursor<&[u8]>) -> Result<String, ()> {
    let len = read_varint_from_cursor(cur)? as usize;
    let pos = cur.position() as usize;
    let buf = cur.get_ref();
    if pos + len > buf.len() {
        return Err(());
    }
    let s = String::from_utf8_lossy(&buf[pos..pos + len]).to_string();
    cur.set_position((pos + len) as u64);
    Ok(s)
}

fn read_varint_from_cursor(cur: &mut Cursor<&[u8]>) -> Result<i32, ()> {
    let mut num_read = 0;
    let mut result: i32 = 0;
    loop {
        let pos = cur.position() as usize;
        if pos >= cur.get_ref().len() {
            return Err(());
        }
        let byte = cur.get_ref()[pos];
        cur.set_position((pos + 1) as u64);
        let value = (byte & 0x7F) as i32;
        result |= value << (7 * num_read);
        num_read += 1;
        if num_read > 5 {
            return Err(());
        }
        if (byte & 0x80) == 0 {
            break;
        }
    }
    Ok(result)
}

async fn read_varint_from_stream<R: AsyncRead + Unpin>(r: &mut R) -> Result<(i32, Vec<u8>)> {
    let mut result: i32 = 0;
    let mut num_read = 0u32;
    let mut shift = 0;
    let mut varint_bytes = Vec::new();

    loop {
        let mut b = [0u8; 1];
        let n = r.read(&mut b).await?;
        if n == 0 {
            anyhow::bail!("EOF while reading varint");
        }
        varint_bytes.push(b[0]);

        let byte = b[0];
        let value = (byte & 0x7F) as i32;
        result |= value << shift;
        num_read += 1;
        if (byte & 0x80) == 0 {
            return Ok((result, varint_bytes));
        }
        shift += 7;
        if num_read > 5 {
            anyhow::bail!("VarInt too long");
        }
    }
}

fn read_varint_from_slice(data: &[u8]) -> Option<(i32, usize)> {
    let mut num_read = 0usize;
    let mut result: i32 = 0;
    let mut shift = 0;
    for &b in data.iter().take(5) {
        let value = (b & 0x7F) as i32;
        result |= value << shift;
        num_read += 1;
        if (b & 0x80) == 0 {
            return Some((result, num_read));
        }
        shift += 7;
    }
    None
}

async fn read_full_mc_frame<R: AsyncRead + Unpin>(r: &mut R) -> Result<Vec<u8>> {
    let (len_val, len_bytes) = read_varint_from_stream(r).await?;
    let payload_len = len_val as usize;

    let mut payload = vec![0u8; payload_len];
    r.read_exact(&mut payload).await?;

    let mut frame = len_bytes;
    frame.extend_from_slice(&payload);
    Ok(frame)
}
