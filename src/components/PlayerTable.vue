<template>
  <div class="table-container" data-tauri-drag-region>
    <table>
      <thead>
        <tr class="head-tr">
          <th v-if="loading" class="user-th">Carregando jogadores...</th>
          <th v-else-if="wait_time > 0" class="user-th">Espere {{ wait_time }} segundos.</th>
          <th v-else class="user-th">{{ players.length }} jogadores ({{ format_stats(settings) }})</th>

          <th v-if="settings.show_ws">WS</th>
          <th v-if="settings.show_wlr">WLR</th>
          <th v-if="settings.show_fkdr && settings.stats_type.type.includes('Bedwars')">FKDR</th>
          <th v-else-if="settings.stats_type.type == 'TheBridge'">Pontos</th>

          <th v-if="settings.show_kdr">KDR</th>
          <th v-if="settings.show_wins">Vitórias</th>
          <th v-if="settings.show_losses">Derrotas</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="player in players">
          <td class="user-td">
            <div class="username">
              <img :src="'https://mc-heads.net/avatar/' + player.skin_hash" />
              <span v-if="player.is_nicked" :style="{ color: nicked_color }">[nicked]</span>
              <div v-else-if="is_marked(player, settings.marked_players)" :style="{ color: 'rgb(255, 0, 0)'}">
                <span class="level">[{{ player.stats.content.level }}</span>
                <div class="symbol_div">
                  <span class="symbol">{{
                    player.stats.content.level_symbol
                  }}</span>
                </div>
                <span>]</span>
              </div>
              <div v-else :style="{ color: rgb_style(player.stats.content.level_color) }">
                <span class="level">[{{ player.stats.content.level }}</span>
                <div class="symbol_div">
                  <span class="symbol">{{
                    player.stats.content.level_symbol
                  }}</span>
                </div>
                <span>]</span>
              </div>

              <span v-if="is_marked(player, settings.marked_players)" :style="{ color: 'rgb(255, 0, 0)' }">{{
                player.username
              }}</span>
              <span v-else :style="{ color: rgb_style(player.username_color) }">{{
                player.username
              }}</span>

              <span v-if="player.clan && is_marked(player, settings.marked_players)" :style="{ color: 'rgb(255, 0, 0)' }">[{{ player.clan }}]</span>
              <span v-else-if="player.clan" :style="{ color: rgb_style(player.clan_color) }">[{{ player.clan }}]</span>

              <img v-if="player.is_possible_cheater" src="/radioactive.svg" class="player-indicator" />

              <img
                v-if="player.stats.type == 'Bedwars' && player.stats.content.losses / player.stats.content.final_deaths > 1.5 && player.stats.content.losses / player.stats.content.final_deaths < 2"
                src="/knife.svg" />
              <img
                v-if="player.stats.type == 'Bedwars' && player.stats.content.losses / player.stats.content.final_deaths > 2"
                src="/knife2.svg" />
              <img v-if="player.is_muted" src="/mute.svg" />

              <span v-if="settings.show_bans && player.bans > 0">{{ player.bans }}</span>
              <img v-if="settings.show_bans && player.bans > 0" src="/hammer.svg" />
            </div>


          </td>

          <td v-if="settings.show_ws" :style="{ color: get_ws_color(player, settings.marked_players) }" class="stat"
            :class="{ 'super': player.stats.content.winstreak >= 100 }">{{ player.stats.content.winstreak == 0 ? '-' :
              player.stats.content.winstreak }}</td>
          <td v-if="settings.show_wlr" :style="{ color: get_wlr_color(player, settings.marked_players) }" class="stat"
            :class="{ 'super': player.stats.content.winrate >= 8 }">{{ player.stats.content.winrate.toFixed(2) }}</td>
          <td v-if="player.stats.type == 'TheBridge'" class="stat">{{ player.stats.content.points }}</td>
          <td v-if="player.stats.type == 'Bedwars' && settings.show_fkdr"
            :style="{ color: get_fkdr_color(player, settings.marked_players) }" class="stat"
            :class="{ 'super': player.stats.content.final_kill_death_ratio >= 50 }">{{
              player.stats.content.final_kill_death_ratio.toFixed(2) }}</td>
          <td v-if="settings.show_kdr" :style="{ color: get_kdr_color(player, settings.marked_players) }" class="stat"
            :class="{ 'super': player.stats.content.kill_death_ratio >= 4 }">{{
              player.stats.content.kill_death_ratio.toFixed(2)
            }}</td>
          <td v-if="settings.show_wins" :style="{ color: get_wins_color(player, settings.marked_players) }">{{
            player.stats.content.wins }}</td>
          <td v-if="settings.show_losses" :style="{ color: get_losses_color(player, settings.marked_players) }">{{
            player.stats.content.losses }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import type { Player, Rgb, Settings } from "../types.ts";
import { get_ws_color, get_wlr_color, get_fkdr_color, get_kdr_color, format_stats, get_wins_color, get_losses_color, is_marked } from "../util.ts";
import { ref } from "vue";

function rgb_style(rgb: Rgb): string {
  return `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
}


const nicked_color = "rgb(255, 255, 0)";
let wait_time = ref(0);

listen("wait", async (event) => {
  wait_time.value = event.payload as number;
  while (wait_time.value > 0) {
    await new Promise(resolve => setTimeout(resolve, 1000));
    wait_time.value -= 1;
  }
});

interface Props {
  party_players: Player[];
  players: Player[];
  loading: boolean;
  settings: Settings;
}
defineProps<Props>();
</script>

<style scoped>
.table-container {
  padding-top: 10px;
  max-height: 380px;
  overflow-y: auto;
}

table {
  padding-top: 299px;
  border-collapse: collapse;
  line-height: 0px;
}

th {
  font-size: 13px;
}

th,
td {
  text-align: center;
  padding: 0px 6.5px;
}

.user-td,
.user-th {
  text-align: left;
  width: 300px;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.head-tr {
  margin-bottom: 20px;
  height: 10px;
}

.username {
  display: flex;
  text-align: center;
  vertical-align: top;
  align-items: center;
}

.username img {
  width: 16px;
  height: 16px;
  image-rendering: pixelated;
  margin-right: 5px;

  position: relative;
  top: 40%;
  transform: translateY(-20%);
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
