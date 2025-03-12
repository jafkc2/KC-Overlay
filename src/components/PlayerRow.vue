<template>
  <div class="player-row">
    <div class="flex-1">
      <div class="font-semibold">{{ player.username }}</div>
    </div>
    
    <div class="stat-group">
      <div class="stat-value">{{ player.stats.Bedwars.level }}</div>
      <div class="stat-label">Level</div>
    </div>

    <div class="stat-group">
      <div class="stat-value">{{ player.stats.Bedwars.kills }}</div>
      <div class="stat-label">Kills</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Player } from '../types.ts';

interface Props {
  player: Player;
}

defineProps<Props>();
</script>

<style scoped>
.stat-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 60px;
}
</style>