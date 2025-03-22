use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "path")]
pub enum MineClient {
    #[default]
    Default,
    Badlion,
    Lunar,
    LegacyLauncher,
    Custom(String),
    Silent,
    CMClient
}

// Clients em string.
impl Display for MineClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineClient::Default => write!(f, "Geral"),
            MineClient::Badlion => write!(f, "Badlion"),
            MineClient::Lunar => write!(f, "Lunar"),
            MineClient::LegacyLauncher => write!(f, "Legacy Launcher"),
            MineClient::Custom(_) => write!(f, "Personalizado"),
            MineClient::Silent => write!(f, "Silent Client"),
            MineClient::CMClient => write!(f, "CM Client"),
        }
    }
}

impl MineClient {
    pub fn get_logs_path(&self) -> String {
        let minecraft_dir = crate::util::get_minecraft_dir();

        match self {
            MineClient::Default => format!("{}/logs/latest.log", minecraft_dir),
            MineClient::Badlion => {
                        format!("{}/logs/blclient/minecraft/latest.log", minecraft_dir)
                    }
            MineClient::Lunar => {
                        let lunar_directory = match std::env::consts::OS {
                            "linux" => format!("{}/.lunarclient", std::env::var("HOME").unwrap()),
                            "windows" => format!(
                                "{}/.lunarclient",
                                std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                            ),
                            _ => panic!("System not supported."),
                        };

                        format!("{}/offline/multiver/logs/latest.log", lunar_directory)
                    }
            MineClient::LegacyLauncher => match std::env::consts::OS {
                        "linux" => format!(
                            "{}/.tlauncher/legacy/Minecraft/game/logs/latest.log",
                            std::env::var("HOME").unwrap()
                        ),
                        "windows" => format!(
                            "{}/AppData/Roaming/.tlauncher/legacy/Minecraft/game/logs/latest.log",
                            std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                        ),
                        _ => panic!("System not supported."),
                    },
            MineClient::Custom(path) => path.to_string(),
            MineClient::Silent => {
                        format!("{}/silentclient/logs/main.log", crate::util::get_home_dir())
                    }
            MineClient::CMClient => {
                let cm_directory = match std::env::consts::OS {
                    "linux" => format!("{}/.local/share/.minecraft", std::env::var("HOME").unwrap()),
                    "windows" => format!(
                        "{}/AppData/Local/Programs/cmlauncher",
                        std::env::var("USERPROFILE").unwrap().replace('\\', "/")
                    ),
                    _ => panic!("System not supported."),
                };

                format!("{}/logs/latest.log", cm_directory)
            },
        }
    }
}
