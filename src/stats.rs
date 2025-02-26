//!  Módulo com estruturas de stats.

use std::fmt::Display;

use crate::util::Rgb;

// Todos os stats
#[derive(Debug, Clone)]
pub enum Stats {
    Bedwars(Bedwars),
}

// Stats de Bedwars
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default, PartialEq)]
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

impl StatsType {
    pub fn from_string(string: &str) -> Self {
        match string {
            "Bedwars Geral" => StatsType::BedwarsAll,
            "Bedwars Solo" => StatsType::BedwarsSolo,
            "Bedwars Duplas" => StatsType::BedwarsDoubles,
            "Bedwars Trios" => StatsType::BedwarsTrios,
            "Bedwars Quartetos" => StatsType::BedwarsQuads,
            "Bedwars 1v1" => StatsType::Bedwars1v1,
            "Bedwars 2v2" => StatsType::Bedwars2v2,

            _ => StatsType::BedwarsAll,
        }
    }

    pub fn get_stats_list() -> Vec<StatsType> {
        vec![
            StatsType::BedwarsAll,
            StatsType::BedwarsSolo,
            StatsType::BedwarsDoubles,
            StatsType::BedwarsTrios,
            StatsType::BedwarsQuads,
            StatsType::Bedwars1v1,
            StatsType::Bedwars2v2,
        ]
    }
}

#[derive(Debug, Clone)]
pub enum BedwarStat {
    Ws,
    Wlr,
    Fkdr,
    Kdr,
    Wins,
    Losses,
    Bans,
}
