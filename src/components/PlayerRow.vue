<template>
  <div class="player_row">
      <div class="username">
        <span v-if="player.is_nicked" :style="{color: nicked_color}">[nicked]</span>
        <div v-else :style="{color: rgb_style(player.stats.Bedwars.level_color)}">
          <span class="level">[{{ player.stats.Bedwars.level }}</span>
          <div class="symbol_div">  
            <span class="symbol">{{ player.stats.Bedwars.level_symbol }}</span>
        </div>
          <span>]</span>
        </div>



        <span :style="{color: rgb_style(player.username_color)}">{{ player.username }}</span>
        <span v-if="player.clan" :style="{color: rgb_style(player.clan_color)}">[{{ player.clan }}]</span>
      </div>
    
      <div class="stat">{{ player.stats.Bedwars.winstreak }}</div>

      <div class="stat">{{ player.stats.Bedwars.winrate.toFixed(2) }}</div>
      <div class="stat">{{ player.stats.Bedwars.final_kill_death_ratio.toFixed(2) }}</div>
      <div class="stat">{{ player.stats.Bedwars.kill_death_ratio.toFixed(2) }}</div>
      <div class="stat">{{ player.stats.Bedwars.wins }}</div>
      <div class="stat">{{ player.stats.Bedwars.losses }}</div>



  </div>
</template>

<script setup lang="ts">
import type { Player, Rgb } from '../types.ts';

function rgb_style(rgb: Rgb) : string{
  return `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`
}

const nicked_color = "rgb(255, 255, 0)"

interface Props {
  player: Player;
}

defineProps<Props>();
</script>

<style scoped>
.username{
  display: flex;
  align-items: center;
  vertical-align: middle;
}
.symbol_div{
  display: inline-block;
}
.level{
  margin-right: 0;
}
.symbol{
  font-size: 15px;
  position: relative;
  bottom: 10px;
}

span{
  margin-right: 2px;
  font-size: 15px;
  line-height: 2.0;
}
.player_row{
  display: flex;
  line-height: 12px
}
.stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 60px;
}
</style>