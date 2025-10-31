use std::{collections::HashMap, io::Cursor};

use flate2::read::ZlibDecoder;
use rust_mc_proto::DataReader;
use serde::Serialize;
use std::io::Read;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};

#[derive(Serialize)]
struct PacketEvent {
    direction: String,
    packet_id: i32,
    length: u32,
    is_player_list: bool,
    payload_b64: String,
    players: Option<Vec<String>>,
}

#[tauri::command]
pub async fn run_proxy(app_mutex: tauri::State<'_, Mutex<crate::KCOverlay>>) -> Result<(), ()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    let (tx, mut rx) = mpsc::unbounded_channel::<PacketEvent>();

    tokio::spawn(async move {
        while let Some(ev) = rx.recv().await {
            if let Ok(js) = serde_json::to_string(&ev) {
                println!("a: {}", js);
            }
        }
    });

    loop {
        let (client, addr) = listener.accept().await.unwrap();
        println!("New connection from {}", addr);
        let upstream = "l.mush.com.br:25565".to_string();
        let tx = tx.clone();

        if let Err(e) = handle_proxy_connection(client, &upstream, tx, &app_mutex).await {
            eprintln!("Connection error: {:?}", e);
        }
    }
}

async fn handle_proxy_connection(
    mut client: TcpStream,
    upstream: &str,
    events: mpsc::UnboundedSender<PacketEvent>,
    app: &tauri::State<'_, Mutex<crate::KCOverlay>>,
) -> Result<(), ()> {
    let mut server = match TcpStream::connect(upstream).await {
        Ok(s) => s,
        Err(_) => return Err(()),
    };

    // handshake e login
    let client_buf = [0u8; 4096];
    let handshake_packet = match read_mc_frame(&mut client).await {
        Ok(p) => p,
        Err(_) => return Err(()),
    };
    server.write_all(&handshake_packet).await.unwrap();
    let login_start_packet = match read_mc_frame(&mut client).await {
        Ok(p) => p,
        Err(_) => return Err(()),
    };

    server.write_all(&login_start_packet).await.unwrap();
    println!("Pacotes de login e handshake enviados.");

    let mut compression_threshold: Option<i32> = None;

    // pacotes iniciais
    loop {
        let packet = match read_mc_frame(&mut server).await {
            Ok(p) => p,
            Err(_) => return Err(()),
        };
        let mut cursor = Cursor::new(&packet);

        let out_len = cursor.read_varint().unwrap();
        let offset = cursor.position() as usize;
        let payload = &packet[offset..];

        let (packet_id, body) = match parse_packet_id_and_body(payload, compression_threshold) {
            Ok((pid, body)) => (pid, body),
            Err(_) => {
                println!("Erro ao parsear packet id e body, enviando mesmo assim.");
                client.write_all(&packet).await.unwrap();
                continue;
            }
        };

        match packet_id {
            // encrypt, sem suporte ainda
            0x01 => {
                println!("O usuário está numa conta original, encerrando proxy.");
                return Err(());
            }
            // login success
            0x02 => {
                println!("Login foi um sucesso, entrando no jogo.");
                client.write_all(&packet).await.unwrap();
                break;
            }
            // set compression
            0x03 => {
                let mut bc = Cursor::new(&body);
                let threshold = bc.read_varint().unwrap();
                println!(
                    "compression threshold = {}",
                    threshold
                );
                client.write_all(&packet).await.unwrap();
                compression_threshold = Some(threshold);
                continue;
            }
            _ => {
                client.write_all(&packet).await.unwrap();
                continue;
            }
        }
    }

    // main loop
    let (mut cr, mut cw) = client.split();
    let (mut sr, mut sw) = server.split();

    let c2s = async move {
        loop {
            let frame = match read_mc_frame(&mut cr).await {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("erro C->S: {:?}", e);
                    break;
                }
            };
            if let Err(e) = sw.write_all(&frame).await {
                eprintln!("erro de escrita C->S: {:?}", e);
                break;
            }
        }
        Ok::<(), ()>(())

    };

    let s2c = async move {
        loop {
            let frame = match read_mc_frame(&mut sr).await {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("erro S->C: {:?}", e);
                    break;
                }
            };

            let mut cur = Cursor::new(&frame);
            if let Err(e) = cur.read_varint() {
                eprintln!("S->C: erro ao ler frame: {:?}", e);
            } else {
                let payload_offset = cur.position() as usize;
                let payload = &frame[payload_offset..];
                match parse_packet_id_and_body(payload, compression_threshold) {
                    Ok((pid, body)) => {
                        // pacote de lista de jogadores
                        if pid == 0x38 {
                            let (players_added, players_removed) =
                                match extract_player_names_from_player_info(&body) {
                                    Ok((added, removed)) => (added, removed),
                                    Err(_) => (HashMap::new(), Vec::new()),
                                };
                            app.lock().await.update_player_list(players_added, players_removed);
                        }
                    }
                    Err(e) => (),
                }
            }

            if let Err(e) = cw.write_all(&frame).await {
                eprintln!("erro de escrita S->C: {:?}", e);
                break;
            }
        }

        Ok::<(), ()>(())
    };

    let _ = tokio::try_join!(c2s, s2c);

    Ok(())
}

async fn read_mc_frame<R: AsyncRead + Unpin>(r: &mut R) -> Result<Vec<u8>, ()> {
    let (len_val, len_bytes) = match read_varint_from_stream(r).await {
        Ok(v) => v,
        Err(_) => return Err(()),
    };
    let payload_len = len_val as usize;

    let mut payload = vec![0u8; payload_len];

    match r.read_exact(&mut payload).await {
        Ok(_) => (),
        Err(_) => return Err(()),
    };

    let mut frame = len_bytes;
    frame.extend_from_slice(&payload);
    Ok(frame)
}

async fn read_varint_from_stream<R: AsyncRead + Unpin>(r: &mut R) -> Result<(i32, Vec<u8>), ()> {
    let mut result: i32 = 0;
    let mut num_read = 0u32;
    let mut shift = 0;
    let mut varint_bytes = Vec::new();

    loop {
        let mut b = [0u8; 1];
        let n = r.read(&mut b).await.unwrap();
        if n == 0 {
            println!("Erro ao ler varint");
            return Err(());
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
            println!("Erro ao ler varint");
            return Err(());
        }
    }
}
fn read_varint(buf: &[u8]) -> Option<(i32, usize)> {
    let mut num_read = 0usize;
    let mut result = 0i32;
    let mut shift = 0;
    for &byte in buf.iter().take(5) {
        let value = (byte & 0x7F) as i32;
        result |= value << shift;
        num_read += 1;
        if (byte & 0x80) == 0 {
            return Some((result, num_read));
        }
        shift += 7;
    }
    None
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

fn parse_packet_id_and_body(
    payload: &[u8],
    compression_threshold: Option<i32>,
) -> Result<(i32, Vec<u8>), ()> {
    if compression_threshold.is_none() {
        if let Some((pid, id_len)) = read_varint(payload) {
            let body = payload[id_len..].to_vec();
            return Ok((pid, body));
        } else {
            println!("Varint incompleto");
            return Err(());
        }
    }

    let (data_len, dl_len) = read_varint(payload).ok_or_else(|| {
        println!("Varint incompleto");
    })?;

    if data_len == 0 {
        let uncompressed = &payload[dl_len..];
        if let Some((pid, id_len)) = read_varint(uncompressed) {
            let body = uncompressed[id_len..].to_vec();
            return Ok((pid, body));
        } else {
            println!("Varint incompleto");
            return Err(());
        }
    } else {
        let compressed = &payload[dl_len..];
        let mut decoder = ZlibDecoder::new(compressed);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            println!("Erro ao descomprimir: {:?}", e);
        })?;
        if let Some((pid, id_len)) = read_varint(&decompressed) {
            let body = decompressed[id_len..].to_vec();
            return Ok((pid, body));
        } else {
            println!("Varint incompleto");
            return Err(());
        }
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
