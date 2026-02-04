<template>
  <Titlebar></Titlebar>
  <div v-if="player">
    <p v-if="player.is_nicked">
      O(a) jogador(a) {{ player.username }} nunca jogou no Mush ou é alguém
      usando /nick.
    </p>
    <div v-else class="player">
      <div>
        <div class="username">
          <div :style="{ color: rgb_style(player.stats.content.level_color) }">
            <span class="level">[{{ player.stats.content.level }}</span>
            <div class="symbol_div">
              <span class="level_symbol">{{
                player.stats.content.level_symbol
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
        <div style="display: flex; justify-content: center; align-items: center; height: 300px;">
          <img class="skin" :src="'https://mc-heads.net/player/' + player.skin_hash" :style="glowStyle"/>
        </div>
      </div>

      <div>
        <p v-if="player.is_connected" class="green">Online</p>
        <p v-else class="red">Offline</p>

        <p v-if="player.is_banned" class="red">Banido</p>
        <p v-if="player.is_muted" class="red">Silenciado</p>

        <p>Primeiro login: {{ format_date(player.account_creation) }}</p>
        <p>Último login: {{ format_date(player.last_login) }}</p>
        <p>Horas jogadas: {{ player.stats.content.hours_played }}</p>
        <p>bans: {{ player.bans }}</p>
        <h2>Stats</h2>

        <div class="flex">
          <div style="margin-right: 30px;">
            <p>Winstreak: {{ player.stats.content.winstreak }}</p>
            <p>WLR: {{ player.stats.content.winrate.toFixed(2) }}</p>
            <p v-if="player.stats.type == 'Bedwars'">FKDR: {{ player.stats.content.final_kill_death_ratio.toFixed(2) }}</p>
            <p>KDR: {{ player.stats.content.kill_death_ratio.toFixed(2) }}</p>
            <p>Vitórias: {{ player.stats.content.wins }}</p>
            <p>Derrotas: {{ player.stats.content.losses }}</p>
          </div>
          <div>
            <p v-if="player.stats.type == 'TheBridge'">Pontos: {{ player.stats.content.points }}</p>
            <p v-if="player.stats.type == 'Bedwars'">Final kills: {{ player.stats.content.final_kills }}</p>
            <p v-if="player.stats.type == 'Bedwars'">Final deaths: {{ player.stats.content.final_deaths }}</p>
            <p>Kills: {{ player.stats.content.kills }}</p>
            <p>Mortes: {{ player.stats.content.deaths }}</p>
            <p v-if="player.stats.type == 'Bedwars'">Assistências: {{ player.stats.content.assists }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
  <p v-else-if="error">
    Não foi possível ver os stats do jogador na API do Mush.
  </p>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import Titlebar from "./components/ViewPlayerTitlebar.vue";
import { Player, Rgb } from "./types";
import { computed, Ref, ref } from "vue";

let player: Ref<Player | null> = ref(null);
let error = ref(false);

function rgb_style(rgb: Rgb): string {
  console.log(rgb)
  return `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
}

listen("player_to_view", (event) => {
  const payload = event.payload as Object;
  console.log(payload);
  if (payload.hasOwnProperty("username")) {
    player.value = payload as Player;
    error.value = false;
    glowColor.value = rgb_style(player.value.stats.content.level_color)
  } else {
    player.value = null;
    error.value = true;
  }
});

const date_format = new Intl.DateTimeFormat('pt-BR', {
  year: 'numeric',
  month: 'short',
  day: 'numeric',
  hour: '2-digit',
  minute: '2-digit'
});

function format_date(date: number) : string{
  if (date == 0){
    return "Estatística oculta"
  }
  return date_format.format((date))
}
const glowColor = ref("#ffffff");

const glowStyle = computed(() => ({
  filter: `drop-shadow(0px 0px 10px ${glowColor.value})`,
}));
</script>

<style scoped>
.green {
  color: green;
}
.red{
  color: red;
}
.flex{
  display: flex;
}
.player {
  padding-top: 10px;
  display: flex;
  flex: 1;
  min-height: 0;
  overflow-y: auto;
}

.top_bar {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

button {
  padding: 6px;
  margin-right: 10px;
  padding-left: 4px;
  padding-right: 4px;
  font-size: 0.75rem;
  align-items: center;
  text-align: center;
  justify-content: center;
}

.flex_button {
  display: flex;
}

.symbol {
  margin-right: 10px;
  width: 18px;
}

.username {
  display: flex;
  vertical-align: middle;
  margin-right: 20px;
}

.symbol_div {
  display: inline-block;
}
.level {
  margin-right: 0;
}
.level_symbol {
  font-size: 15px;
  position: relative;
  bottom: 10px;
}
.skin {
    height: 300px;
    image-rendering: pixelated;
  }
</style>
