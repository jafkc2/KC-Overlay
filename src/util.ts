import { Player } from "./types";

export function get_ws_color(player: Player) : string{
    const ws = player.stats.Bedwars.winstreak;

    if (ws < 5){
        return "rgb(255, 255, 255)";
    } else if (ws < 10){
        return "rgb(255, 255, 0)"
    } else if (ws < 20){
        return "rgb(255, 128, 0)";
    } else if (ws < 50){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}

export function get_wlr_color(player: Player) : string{
    const wlr = player.stats.Bedwars.winrate;

    if (wlr < 1){
        return "rgb(255, 255, 255)";
    } else if (wlr < 2){
        return "rgb(255, 255, 0)";
    } else if (wlr < 3){
        return "rgb(255, 128, 0)";
    } else if (wlr < 5){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}

export function get_fkdr_color(player: Player) : string{
    const fkdr = player.stats.Bedwars.final_kill_death_ratio;

    if (fkdr < 2){
        return "rgb(255, 255, 255)";
    } else if(fkdr < 5.0){
        return "rgb(255, 255, 0)";
    } else if (fkdr < 10){
        return "rgb(255, 128, 0)";
    } else if (fkdr < 15){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}

export function get_kdr_color(player: Player) : string{
    const kdr = player.stats.Bedwars.kill_death_ratio;

    if (kdr < 1){
        return "rgb(255, 255, 255)";
    } else if(kdr < 1.5){
        return "rgb(255, 255, 0)";
    } else if (kdr < 2){
        return "rgb(255, 128, 0)";
    } else if (kdr < 3){
        return "rgb(255, 0, 0)";
    } else{
        return "rgb(200, 0, 200)";
    }
}