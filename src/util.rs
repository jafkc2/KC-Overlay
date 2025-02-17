// Código para funções essenciais

use std::{fs::File, io::Read, path::Path, time::Duration};

use chrono::DateTime;
use iced::Color;
use serde_json::Value;
use tokio::time::sleep;

pub fn get_home_dir() -> String {
    match std::env::consts::OS {
        "linux" => std::env::var("HOME").unwrap().to_string(),
        "windows" => format!(
            "{}/AppData/Roaming",
            std::env::var("USERPROFILE").unwrap().replace('\\', "/")
        ),
        _ => panic!("System not supported."),
    }
}
pub fn get_minecraft_dir() -> String {
    format!("{}/.minecraft", get_home_dir())
}

pub fn get_legacy_launcher_dir() -> String {
    match std::env::consts::OS {
        "linux" => format!(
            "{}/.tlauncher/legacy/Minecraft/game/logs/latest.log",
            std::env::var("HOME").unwrap()
        ),
        "windows" => format!(
            "{}/AppData/Roaming/.tlauncher/legacy/Minecraft/game/logs/latest.log",
            std::env::var("USERPROFILE").unwrap().replace('\\', "/")
        ),
        _ => panic!("System not supported."),
    }
}

pub async fn wait(time: Duration) {
    sleep(time).await;
}

pub fn get_json(jpathstring: String) -> Value {
    let jsonpath = Path::new(&jpathstring);

    let mut file = File::open(jsonpath).unwrap();
    let mut fcontent = String::new();
    file.read_to_string(&mut fcontent).unwrap();
    serde_json::from_str(&fcontent).unwrap()
}

pub fn lunar_get_newer_logs_path() -> String {
    let lunar_dir = match std::env::consts::OS {
        "linux" => format!("{}/.lunarclient", std::env::var("HOME").unwrap()),
        "windows" => format!(
            "{}/.lunarclient",
            std::env::var("USERPROFILE").unwrap().replace('\\', "/")
        ),
        _ => panic!("System not supported."),
    };

    format!("{}/offline/multiver/logs/latest.log", lunar_dir)
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;
    
    let (r, g, b) = match (h * 6.0).floor() as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (r + m, g + m, b + m)
}

#[derive(Debug, Clone)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.replace("#", "");

        let red = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let green: u8 = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&hex[4..6], 16).unwrap();

        Rgb { red, green, blue }
    }

    pub fn from_minecraft_color(color_char: &char) -> Self {
        match color_char {
            '0' => Rgb {
                red: 0,
                green: 0,
                blue: 0,
            },
            '1' => Rgb {
                red: 0,
                green: 0,
                blue: 170,
            },
            '2' => Rgb {
                red: 0,
                green: 170,
                blue: 0,
            },
            '3' => Rgb {
                red: 0,
                green: 170,
                blue: 170,
            },
            '4' => Rgb {
                red: 170,
                green: 0,
                blue: 0,
            },
            '5' => Rgb {
                red: 170,
                green: 0,
                blue: 170,
            },
            '6' => Rgb {
                red: 255,
                green: 170,
                blue: 0,
            },
            '7' => Rgb {
                red: 170,
                green: 170,
                blue: 170,
            },
            '8' => Rgb {
                red: 85,
                green: 85,
                blue: 85,
            },
            '9' => Rgb {
                red: 85,
                green: 85,
                blue: 255,
            },
            'a' => Rgb {
                red: 85,
                green: 255,
                blue: 85,
            },
            'b' => Rgb {
                red: 85,
                green: 255,
                blue: 255,
            },
            'c' => Rgb {
                red: 255,
                green: 85,
                blue: 85,
            },
            'd' => Rgb {
                red: 255,
                green: 85,
                blue: 255,
            },
            'e' => Rgb {
                red: 255,
                green: 255,
                blue: 85,
            },
            'f' => Rgb {
                red: 255,
                green: 255,
                blue: 255,
            },
            _ => Rgb {
                red: 255,
                green: 255,
                blue: 255,
            },
        }
    }

    pub fn to_color(&self) -> Color {
        Color::from_rgb8(self.red, self.green, self.blue)
    }
}

pub fn unix_time_to_date(time: i64) -> String {
    let date_time = DateTime::from_timestamp(time / 1000, 0)
        .unwrap()
        .with_timezone(&chrono::FixedOffset::east_opt(-3 * 3600).unwrap());
    date_time.format("%d/%m/%y às %H:%M").to_string()
}
