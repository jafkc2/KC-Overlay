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

export type Settings = {
    client: MineClient,
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
    transparency: number
}

export type MineClient = 
    | {type : "Default"}
    | {type : "Badlion"}
    | {type : "Lunar"}
    | {type : "Silent"}
    | {type : "LegacyLauncher"}
    | {type : "Custom"};

export type StatsType =
    | {type: "BedwarsAll"}
    | {type: "BedwarsSolo"}
    | {type: "BedwarsDoubles"}
    | {type: "BedwarsTrios"}
    | {type: "BedwarsQuads"}
    | {type: "Bedwars1v1"}
    | {type: "Bedwars2v2"};

export enum View{
    main,
    about,
    settings,
    viewPlayer
  }