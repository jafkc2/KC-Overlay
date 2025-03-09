<script setup lang="ts">
defineProps(['username', 'level', 'level_color'])
</script>

<template>
    <div class="player_row">{{ username }} Level {{level}}</div>
</template>

<style scoped>
    player_row{
        display: flex;
    }
</style>