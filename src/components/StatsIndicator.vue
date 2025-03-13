<template>
  <div class="main">
    <p style="width: 320px;">Top {{ player_number }} jogadores da sala</p>
    <div class="stat">WS</div>
    <div class="stat">WLR</div>
    <div class="stat">FKDR</div>
    <div class="stat">KDR</div>
    <div class="stat">Vitórias</div>
    <div class="stat">Derrotas</div>



  </div>
</template>

<script setup lang="ts">
interface Props {
  player_number: number;
}

defineProps<Props>();
</script>

<style scoped>
  .main{
    display: flex;
    font-size: 12px;
  }

  .stat {
  display: flex;
  align-items: center;
  text-align: center;
  margin-right: 20px;
}
</style>