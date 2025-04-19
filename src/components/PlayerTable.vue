<template>
  <div class="table-container" data-tauri-drag-region>
    <!-- Indicador de carregamento melhorado -->
    <div v-if="loading" class="loading-container">
      <div class="loading-spinner"></div>
      <div class="loading-text">Carregando jogadores...</div>
      <div v-if="cache_stats.total > 0" class="cache-stats">
        <div class="progress-bar">
          <div class="progress" :style="{ width: `${(cache_stats.cached / cache_stats.total) * 100}%` }"></div>
        </div>
        <div class="stats-text">
          {{ cache_stats.cached }} de {{ cache_stats.total }} jogadores carregados do cache ({{ Math.round((cache_stats.cached / cache_stats.total) * 100) }}%)
        </div>
      </div>
    </div>
    
    <!-- Indicador de espera por rate limit -->
    <div v-else-if="wait_time > 0" class="wait-container">
      <div class="wait-icon">⏱️</div>
      <div class="wait-text">
        Limite de API atingido. Aguarde <span class="wait-time">{{ wait_time }}</span> segundos.
      </div>
      <div class="progress-bar">
        <div class="progress" :style="{ width: `${(60 - wait_time) / 60 * 100}%` }"></div>
      </div>
    </div>

    <table v-else>
      <thead>
        <tr class="head-tr">
          <th class="user-th">{{ players.length }} jogadores ({{ format_stats(settings) }})</th>
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
              <div v-else :style="{ color: rgb_style(player.stats.content.level_color) }">
                <span class="level">[{{ player.stats.content.level }}</span>
                <div class="symbol_div">
                  <span class="symbol">{{
                    player.stats.content.level_symbol
                    }}</span>
                </div>
                <span>]</span>
              </div>

              <span :style="{ color: rgb_style(player.username_color) }">{{
                player.username
                }}</span>
              <span v-if="player.clan" :style="{ color: rgb_style(player.clan_color) }">[{{ player.clan }}]</span>

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

          <td v-if="settings.show_ws" :style="{ color: get_ws_color(player) }" class="stat"
            :class="{ 'super': player.stats.content.winstreak >= 100 }">{{ player.stats.content.winstreak == 0 ? '-' :
              player.stats.content.winstreak }}</td>
          <td v-if="settings.show_wlr" :style="{ color: get_wlr_color(player) }" class="stat"
            :class="{ 'super': player.stats.content.winrate >= 8 }">{{ player.stats.content.winrate.toFixed(2) }}</td>
          <td v-if="player.stats.type == 'TheBridge'" class="stat">{{ player.stats.content.points }}</td>
          <td v-if="player.stats.type == 'Bedwars' && settings.show_fkdr" :style="{ color: get_fkdr_color(player) }"
            class="stat" :class="{ 'super': player.stats.content.final_kill_death_ratio >= 50 }">{{
              player.stats.content.final_kill_death_ratio.toFixed(2) }}</td>
          <td v-if="settings.show_kdr" :style="{ color: get_kdr_color(player) }" class="stat"
            :class="{ 'super': player.stats.content.kill_death_ratio >= 4 }">{{
              player.stats.content.kill_death_ratio.toFixed(2)
            }}</td>
          <td v-if="settings.show_wins" :style="{ color: get_wins_color(player) }">{{ player.stats.content.wins }}</td>
          <td v-if="settings.show_losses" :style="{ color: get_losses_color() }">{{ player.stats.content.losses }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import type { Player, Rgb, Settings } from "../types.ts";
import { get_ws_color, get_wlr_color, get_fkdr_color, get_kdr_color, format_stats, get_wins_color, get_losses_color } from "../util.ts";
import { ref, reactive } from "vue";

function rgb_style(rgb: Rgb): string {
  return `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
}

const nicked_color = "rgb(255, 255, 0)";
let wait_time = ref(0);
const cache_stats = reactive({
  cached: 0,
  total: 0
});

listen("wait", async (event) => {
  wait_time.value = event.payload as number;
  while (wait_time.value > 0) {
    await new Promise(resolve => setTimeout(resolve, 1000));
    wait_time.value -= 1;
  }
});

listen("cache_stats", (event) => {
  const stats = event.payload as { cached: number, total: number };
  cache_stats.cached = stats.cached;
  cache_stats.total = stats.total;
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
  padding-top: 5px;
  border-collapse: collapse;
  line-height: 0px;
  width: 100%;
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

/* Estilos para loading */
.loading-container, .wait-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  margin-top: 30px;
  width: 100%;
}

.loading-spinner {
  border: 4px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top: 4px solid #fff;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
  margin-bottom: 10px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.loading-text, .wait-text {
  font-size: 16px;
  margin-bottom: 15px;
  color: white;
}

.wait-time {
  font-weight: bold;
  color: #ff9800;
}

.wait-icon {
  font-size: 24px;
  margin-bottom: 10px;
}

.cache-stats {
  width: 100%;
  margin-top: 10px;
  text-align: center;
}

.progress-bar {
  height: 8px;
  width: 100%;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 5px;
}

.progress {
  height: 100%;
  background-color: #4caf50;
  transition: width 0.5s ease;
}

.stats-text {
  font-size: 12px;
  opacity: 0.7;
}

.super {
  font-weight: bold;
  text-shadow: 0 0 5px currentColor;
}
</style>
