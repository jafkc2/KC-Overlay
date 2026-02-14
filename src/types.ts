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
    skin_hash: string;
    stats: Stats;
  }

export type Rgb = {
    red: number;
    green: number;
    blue: number;
}

type Stats = 
    | {type: "Bedwars", content: Bedwars}
    | {type: "TheBridge", content: TheBridge}
    | {type: "Duels", content: Duels}
    | {type: "Skywars", content: Duels}

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

type TheBridge = {
    level: number,
    level_symbol: string,
    winstreak: number,
    winrate: number,
    kill_death_ratio: number,
    level_color: Rgb,
    wins: number,
    losses: number,
    kills: number,
    deaths: number,
    hours_played: number,
    points: number,
}

type Duels = {
    level: number,
    level_symbol: string,
    winstreak: number,
    winrate: number,
    kill_death_ratio: number,
    level_color: Rgb,
    wins: number,
    losses: number,
    kills: number,
    deaths: number,
    hours_played: number,
}

export type Settings = {
    use_custom_client: boolean,
    custom_client_path: string,
    never_minimize: boolean,
    seconds_to_minimize: number,
    auto_manage_players: boolean,
    stats_type: StatsType,
    window_scale: number,
    rgb_buttons: boolean,
    show_ws: boolean,
    show_wlr: boolean,
    show_fkdr: boolean,
    show_kdr: boolean,
    show_wins: boolean,
    show_losses: boolean,
    show_bans: boolean,
    transparency: number,
    automatic: boolean,
    remove_players: boolean,
    hotkey: string,
    marked_players: string[],
}

export type StatsType =
    | {type: "BedwarsAll"}
    | {type: "BedwarsSolo"}
    | {type: "BedwarsDoubles"}
    | {type: "BedwarsTrios"}
    | {type: "BedwarsQuads"}
    | {type: "Bedwars1v1"}
    | {type: "Bedwars2v2"}
    | {type: "TheBridge"}
    | {type: "FireballFight"}
    | {type: "BedFight"}
    | {type: "Uhc"}
    | {type: "Skywars"}


export type MinecraftAccount = {
  username: string,
  access_token: string
  uuid: string,
}
export type MinecraftRefreshToken = {
  username: string,
  token: string
}

