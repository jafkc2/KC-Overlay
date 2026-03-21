use std::{fmt::Display, fs};

use serde::{Deserialize, Serialize};

use crate::util::get_home_dir;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "path")]
pub enum MineClient {
    #[default]
    Automatic,
    Default,
    Badlion,
    Lunar,
    LegacyLauncher,
    Custom(String),
    Silent,
    CMClient,
    CheatBreaker,
    Salwyrr,
    LunarCelestial,
    LunarQt,
    OwlClient,
}

// Clients em string.
impl Display for MineClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineClient::Automatic => write!(f, "Automático"),
            MineClient::Default => write!(f, "Geral"),
            MineClient::Badlion => write!(f, "Badlion"),
            MineClient::Lunar => write!(f, "Lunar"),
            MineClient::LegacyLauncher => write!(f, "Legacy Launcher"),
            MineClient::Custom(_) => write!(f, "Personalizado"),
            MineClient::Silent => write!(f, "Silent Client"),
            MineClient::CMClient => write!(f, "CM Client"),
            MineClient::CheatBreaker => write!(f, "CheatBreaker"),
            MineClient::Salwyrr => write!(f, "Salwyrr"),
            MineClient::LunarCelestial => write!(f, "Lunar Celestial"),
            MineClient::LunarQt => write!(f, "Lunar Qt"),
            MineClient::OwlClient => write!(f, "Owl Client"),
        }
    }
}

impl MineClient {
    pub fn get_logs_path(&self) -> String {
        let minecraft_dir = crate::util::get_minecraft_dir();
        let os = std::env::consts::OS;

        match self {
            MineClient::Automatic => String::new(),
            MineClient::Default => format!("{}/logs/latest.log", minecraft_dir),
            MineClient::Badlion => {
                format!("{}/logs/blclient/minecraft/latest.log", minecraft_dir)
            }
            MineClient::Lunar => {
                let lunar_directory = match os {
                    "linux" => format!("{}/.lunarclient", std::env::var("HOME").unwrap()),
                    "windows" => format!(
                        "{}/.lunarclient",
                        std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                    ),
                    "macos" => format!("{}/.lunarclient", std::env::var("HOME").unwrap()),
                    _ => panic!("Sistema não suportado."),
                };

                format!("{}/profiles/lunar/1.8/logs/latest.log", lunar_directory)
            }
            MineClient::LegacyLauncher => match os {
                "linux" => format!(
                    "{}/.tlauncher/legacy/Minecraft/game/logs/latest.log",
                    std::env::var("HOME").unwrap()
                ),
                "windows" => format!(
                    "{}/AppData/Roaming/.tlauncher/legacy/Minecraft/game/logs/latest.log",
                    std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                ),
                "macos" => format!(
                    "{}/.tlauncher/legacy/Minecraft/game/logs/latest.log",
                    get_home_dir()
                ),
                _ => panic!("Sistema não suportado."),
            },
            MineClient::Custom(path) => path.to_string(),
            MineClient::Silent => {
                format!("{}/silentclient/logs/main.log", crate::util::get_home_dir())
            }
            MineClient::CMClient => {
                let cm_directory = match os {
                    "linux" => {
                        format!("{}/.local/share/.minecraft", std::env::var("HOME").unwrap())
                    }
                    "windows" => format!(
                        "{}/AppData/Local/Programs/cmlauncher",
                        std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                    ),
                    "macos" => minecraft_dir,
                    _ => panic!("Sistema não suportado."),
                };

                format!("{}/logs/latest.log", cm_directory)
            }
            MineClient::CheatBreaker => match os {
                "linux" => format!(
                    "{}/.cheatbreaker/downloads/logs/1.8.9/latest.log",
                    std::env::var("HOME").unwrap()
                ),
                "windows" => format!(
                    "{}/Appdata/Roaming/CheatBreaker/logs/renderer.log",
                    std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                ),
                "macos" => format!(
                    "{}/.cheatbreaker/downloads/logs/1.8.9/latest.log",
                    std::env::var("HOME").unwrap()
                ),
                _ => panic!("Sistema não suportado"),
            },
            MineClient::Salwyrr => match os {
                "linux" => format!(
                    "{}/.Salwyrr/logs/latest.log",
                    std::env::var("HOME").unwrap()
                ),
                "windows" => format!(
                    "{}/Appdata/Roaming/.Salwyrr/logs/latest.log",
                    std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                ),
                "macos" => format!(
                    "{}/.Salwyrr/logs/latest.log",
                    std::env::var("HOME").unwrap()
                ),
                _ => panic!("Sistema não suportado"),
            },
            MineClient::LunarCelestial => match os {
                "linux" => format!(
                    "{}/.cubewhy/lunarcn/game/logs/latest.log",
                    std::env::var("HOME").unwrap()
                ),
                "windows" => format!(
                    "{}/.cubewhy/lunarcn/game/logs/latest.log",
                    std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                ),
                "macos" => format!(
                    "{}/.cubewhy/lunarcn/game/logs/latest.log",
                    std::env::var("HOME").unwrap()
                ),
                _ => panic!("Sistema não suportado"),
            },
            MineClient::LunarQt => {
                let lunar_directory = match os {
                    "linux" => format!("{}/.lunarclient", std::env::var("HOME").unwrap()),
                    "windows" => format!(
                        "{}/.lunarclient",
                        std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                    ),
                    "macos" => format!("{}/.lunarclient", std::env::var("HOME").unwrap()),
                    _ => panic!("Sistema não suportado."),
                };

                format!("{}/offline/multiver/logs/latest.log", lunar_directory)
            }
            MineClient::OwlClient => {
                let owl_dir = match std::env::consts::OS {
                    "linux" => format!("{}/.owlclient", std::env::var("HOME").unwrap()),
                    "windows" => format!(
                        "{}/.owlclient",
                        std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                    ),
                    "macos" => format!("{}/.owlclient", std::env::var("HOME").unwrap()),
                    _ => panic!("System not supported."),
                };

                let logs_path = format!("{}/logs/launcher", owl_dir);

                if !fs::exists(logs_path.clone()).unwrap() {
                    return format!("{}/logs/latest.log", minecraft_dir);
                }

                fs::create_dir_all(logs_path.clone()).unwrap();

                let mut newer_log_path = String::new();
                let mut shortest_modification_time = std::time::SystemTime::UNIX_EPOCH;
                for log in fs::read_dir(logs_path).unwrap() {
                    let log = log.unwrap();
                    let metadata = log.metadata().unwrap();

                    let modified_time = metadata.modified().unwrap();
                    if modified_time > shortest_modification_time {
                        newer_log_path = log.path().to_string_lossy().to_string();
                        shortest_modification_time = modified_time;
                    }
                }

                newer_log_path
            }
        }
    }

    pub fn get_all_log_paths() -> Vec<String> {
        let mut paths = vec![];
        for client in [
            MineClient::Automatic,
            MineClient::Default,
            MineClient::Badlion,
            MineClient::Lunar,
            MineClient::LegacyLauncher,
            MineClient::Silent,
            MineClient::CMClient,
            MineClient::CheatBreaker,
            MineClient::Salwyrr,
            MineClient::LunarCelestial,
            MineClient::LunarQt,
        ] {
            paths.push(client.get_logs_path());
        }
        paths
    }
}
