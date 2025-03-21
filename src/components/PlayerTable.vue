<template>
<div class="table-container"  data-tauri-drag-region>
  <table>
    <thead>
      <tr class="head-tr">
        <th class="user-th">Top {{ players.length }} jogadores da sala ({{ format_stats(settings) }})</th>
        <th v-if="settings.show_ws">WS</th>
        <th v-if="settings.show_wlr">WLR</th>
        <th v-if="settings.show_fkdr">FKDR</th>
        <th v-if="settings.show_kdr">KDR</th>
        <th v-if="settings.show_wins">Vitórias</th>
        <th v-if="settings.show_losses">Derrotas</th>
      </tr>
    </thead>

    <tbody>
      <tr v-for="player in players">
        <td class="user-td">
          <div class="username">
            <img :src="'https://mc-heads.net/avatar/' + player.skin_hash"/>
            <span v-if="player.is_nicked" :style="{ color: nicked_color }"
              >[nicked]</span
            >
            <div
              v-else
              :style="{ color: rgb_style(player.stats.Bedwars.level_color) }"
            >
              <span class="level">[{{ player.stats.Bedwars.level }}</span>
              <div class="symbol_div">
                <span class="symbol">{{
                  player.stats.Bedwars.level_symbol
                }}</span>
              </div>
              <span>]</span>
            </div>

            <span :style="{ color: rgb_style(player.username_color) }">{{
              player.username
            }}</span>
            <span
              v-if="player.clan"
              :style="{ color: rgb_style(player.clan_color) }"
              >[{{ player.clan }}]</span
            >
          </div>
        </td>

        <td v-if="settings.show_ws" :style="{color: get_ws_color(player)}" class="stat" :class="{ 'super': player.stats.Bedwars.winstreak >= 100 }">{{ player.stats.Bedwars.winstreak == 0 ? '-' : player.stats.Bedwars.winstreak }}</td>
        <td v-if="settings.show_wlr" :style="{color: get_wlr_color(player)}" class="stat" :class="{ 'super': player.stats.Bedwars.winrate >= 8 }">{{ player.stats.Bedwars.winrate.toFixed(2) }}</td>
        <td v-if="settings.show_fkdr" :style="{color: get_fkdr_color(player)}" class="stat" :class="{ 'super': player.stats.Bedwars.final_kill_death_ratio >= 50}">{{ player.stats.Bedwars.final_kill_death_ratio.toFixed(2) }}</td>
        <td v-if="settings.show_kdr" :style="{color: get_kdr_color(player)}" class="stat" :class="{ 'super': player.stats.Bedwars.kill_death_ratio >= 4 }">{{ player.stats.Bedwars.kill_death_ratio.toFixed(2) }}</td>
        <td v-if="settings.show_wins" :style="{color: get_wins_color(player)}">{{ player.stats.Bedwars.wins }}</td>
        <td v-if="settings.show_losses" :style="{color: get_losses_color()}">{{ player.stats.Bedwars.losses }}</td>
      </tr>
    </tbody>
  </table>
</div>
</template>

<script setup lang="ts">
import type { Player, Rgb, Settings } from "../types.ts";
import {get_ws_color, get_wlr_color, get_fkdr_color, get_kdr_color, format_stats, get_wins_color, get_losses_color} from "../util.ts";

function rgb_style(rgb: Rgb): string {
  return `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
}

const nicked_color = "rgb(255, 255, 0)";

interface Props {
  players: Player[];
  settings: Settings;
}
defineProps<Props>();
</script>

<style scoped>
img{
  margin-right: 5px;
  width: 18px;
  height: 18px;
  image-rendering: pixelated;

}
.table-container {
    padding-top: 10px;
    max-height: 380px;
    overflow-y: auto;
}
.table-container::-webkit-scrollbar-track {
  background: rgb(29, 30, 48);
  border-radius: 8px;
}

.table-container::-webkit-scrollbar-thumb {
  background: rgb(49, 50, 68);
  border-radius: 8px;
}

.table-container::-webkit-scrollbar-thumb:hover {
  background: rgb(137, 180, 250);
}

table{
    padding-top: 299px;
    border-collapse: collapse;
}
th{
    font-size: 13px;
}
th, td {
    text-align: center;
    padding: 0px 5px;

}
.user-td, .user-th {
    text-align: left;
    width: 300px;
    white-space: nowrap;
    text-overflow: ellipsis;
}

.head-tr{
    margin-bottom: 20px;
    height: 10px;
    vertical-align: top;
}

.username {
  display: flex;
  vertical-align: middle;
}
.symbol_div {
  display: inline-block;
}
.level {
  margin-right: 0;
}
.symbol {
  font-size: 15px;
  position: relative;
  bottom: 10px;
}

span {
  margin-right: 2px;
  font-size: 14px;
  line-height: 1.5;
}
.player_row {
  display: flex;
  line-height: 12px;
}

</style>
