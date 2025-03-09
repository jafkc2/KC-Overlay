export type Player = {
    username: String;
    username_color: Rgb;
    clan: string | null;
    clan_color: Rgb;
    is_nicked: boolean;
    is_possible_cheater: boolean;
    account_creation: number;
    last_login: number;
    is_connected: boolean;
    bans: number;
    is_muted: boolean;
    is_banned: boolean;
    stats: Stats;
  }

type Rgb = {
    red: number;
    green: number;
    blue: number;
}

type Stats = {
    Bedwars: Bedwars;
}

type Bedwars = {
    level: number;
    level_symbol: string,
    winstreak: number;
    winrate: number;
    final_kill_death_ratio: number;
    kill_death_ratio: number;
    level_color: Rgb,
    wins: number;
    losses: number;
    kills: number;
    deaths: number;
    final_kills: number;
    final_deaths: number;
    hours_played: number;
    assists: number;
}