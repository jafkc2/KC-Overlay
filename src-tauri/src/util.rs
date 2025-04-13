//! Módulo para funções de utilidade geral.

use std::{fs::File, io::Read, path::Path};

use serde::Serialize;
use serde_json::Value;

pub fn get_home_dir() -> String {
    match std::env::consts::OS {
        "linux" => std::env::var("HOME").unwrap().to_string(),
        "windows" => format!(
            "{}/AppData/Roaming",
            std::env::var("USERPROFILE").unwrap().replace('\\', "/")
        ),
        "macos" => format!(
            "{}/Library/Application Support",
            std::env::var("HOME").unwrap()
        ),
        _ => panic!("System not supported."),
    }
}
pub fn get_minecraft_dir() -> String {
    let minecraft_name = match std::env::consts::OS {
        "macos" => "minecraft",
        _ => ".minecraft",
    };
    format!("{}/{}", get_home_dir(), minecraft_name)
}

pub fn get_json(jpathstring: String) -> Value {
    let jsonpath = Path::new(&jpathstring);

    let mut file = File::open(jsonpath).unwrap();
    let mut fcontent = String::new();
    file.read_to_string(&mut fcontent).unwrap();
    serde_json::from_str(&fcontent).unwrap_or(serde_json::json!({}))
}

#[derive(Debug, Clone, Serialize)]
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
}

#[tauri::command]
pub fn get_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}
