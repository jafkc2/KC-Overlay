import { Player, Settings } from "./types";

export function get_ws_color(player: Player, marked_players: string[]) : string{
    if (is_marked(player, marked_players)){
        return "rgb(255, 0, 0)";
    }

    const ws = player.stats.content.winstreak;

    if (ws < 5){
        return "#aaaaaa";
    } else if (ws < 10){
        return "rgb(255, 255, 255)"
    } else if (ws < 20){
        return "rgb(255, 128, 0)";
    } else if (ws < 50){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}

export function get_wlr_color(player: Player, marked_players: string[]) : string{
    if (is_marked(player, marked_players)){
        return "rgb(255, 0, 0)";
    }

    const wlr = player.stats.content.winrate;

    if (wlr < 1){
        return "#aaaaaa";
    } else if (wlr < 2){
        return "rgb(255, 255, 255)";
    } else if (wlr < 3){
        return "rgb(255, 128, 0)";
    } else if (wlr < 5){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}

export function get_fkdr_color(player: Player, marked_players: string[]) : string{
    if (is_marked(player, marked_players)){
        return "rgb(255, 0, 0)";
    }

    if (player.stats.type != "Bedwars"){
        return "#aaaaaa"
    }
    const fkdr = player.stats.content.final_kill_death_ratio;

    if (fkdr < 2){
        return "#aaaaaa";
    } else if(fkdr < 5.0){
        return "rgb(255, 255, 255)";
    } else if (fkdr < 10){
        return "rgb(255, 128, 0)";
    } else if (fkdr < 15){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}

export function get_kdr_color(player: Player, marked_players: string[]) : string{
    if (is_marked(player, marked_players)){
        return "rgb(255, 0, 0)";
    }
    const kdr = player.stats.content.kill_death_ratio;

    if (kdr < 1){
        return "#aaaaaa";
    } else if(kdr < 1.5){
        return "rgb(255, 255, 255)";
    } else if (kdr < 2){
        return "rgb(255, 128, 0)";
    } else if (kdr < 3){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}

export function get_wins_color(player: Player, marked_players: string[]) : string{
    if (is_marked(player, marked_players)){
        return "rgb(255, 0, 0)";
    }
    const wins = player.stats.content.wins;

    if (wins < 1000){
        return "#aaaaaa";
    } else if (wins < 2000){
        return "rgb(255, 255, 255)";
    } else if (wins < 5000){
        return "rgb(255, 128, 0)";
    } else if (wins < 10000){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)"
    }
}

export function get_losses_color(player: Player, marked_players: string[]) : string{
    if (is_marked(player, marked_players)){
        return "rgb(255, 0, 0)";
    }
    return "#aaaaaa"
}

export function format_stats(settings: Settings): string{
    switch (settings.stats_type.type){
        case "BedwarsAll":
            return "Bedwars Geral"
        case "BedwarsSolo":
            return "Bedwars Solo"
        case "BedwarsDoubles":
            return "Bedwars Duplas"
        case "BedwarsTrios":
            return "Bedwars Trios"
        case "BedwarsQuads":
            return "Bedwars quartetos"
        case "Bedwars1v1":
            return "Bedwars 1v1"
        case "Bedwars2v2":
            return "Bedwars 2v2"
        case "TheBridge":
            return "The Bridge"
        case "FireballFight":
            return "Fireball Fight"
        case "BedFight":
            return "Bed Fight"
        case "Uhc":
            return "UHC"
        case "Skywars":
            return "Skywars"
            
    }
}

export function get_stat_types(): string[]{
    return ["Bedwars Geral", 'Bedwars Solo', 'Bedwars Duplas', 'Bedwars Trios', 'Bedwars Quartetos', 'Bedwars 1v1', 'Bedwars 2v2', "The Bridge", "Fireball Fight", "Bed Fight", "UHC", "Skywars"];
}

export function is_marked(player: Player, marked_players: string[]): boolean {
  return marked_players.includes(player.username.toString());
}