//!  Módulo com estruturas de stats.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::util::Rgb;

// Todos os stats
#[derive(Debug, Clone, Serialize)]
pub enum Stats {
    Bedwars(Bedwars),
}

// Stats de Bedwars
#[derive(Debug, Clone, Serialize)]
pub struct Bedwars {
    pub level: i32,
    pub level_symbol: String,
    pub winstreak: i32,
    pub winrate: f32,
    pub final_kill_death_ratio: f32,
    pub kill_death_ratio: f32,
    pub level_color: Rgb,
    pub wins: u64,
    pub losses: u64,
    pub kills: u64,
    pub deaths: u64,
    pub final_kills: u64,
    pub final_deaths: u64,
    pub hours_played: u64,
    pub assists: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StatsType {
    #[default]
    BedwarsAll,
    BedwarsSolo,
    BedwarsDoubles,
    BedwarsTrios,
    BedwarsQuads,
    Bedwars1v1,
    Bedwars2v2,
}

impl Display for StatsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatsType::BedwarsAll => write!(f, "Bedwars Geral"),
            StatsType::BedwarsSolo => write!(f, "Bedwars Solo"),
            StatsType::BedwarsDoubles => write!(f, "Bedwars Duplas"),
            StatsType::BedwarsTrios => write!(f, "Bedwars Trios"),
            StatsType::BedwarsQuads => write!(f, "Bedwars Quartetos"),
            StatsType::Bedwars1v1 => write!(f, "Bedwars 1v1"),
            StatsType::Bedwars2v2 => write!(f, "Bedwars 2v2"),
        }
    }
}