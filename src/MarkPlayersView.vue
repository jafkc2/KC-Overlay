<script setup lang="ts">
import { ref } from 'vue';
import Return from './components/Return.vue';
import { useStore } from './stores/store';
import { invoke } from '@tauri-apps/api/core';
let store = useStore();
let marked_players_list = ref(store.settings.marked_players.join(','));


async function update_marked_players() {
    store.settings.marked_players = marked_players_list.value.split(',').map(name => name.trim());
    await invoke("save_settings", { settings: store.settings })
}

</script>

<template>
    <Return></Return>
    <p>Os jogadores que você escrever no campo abaixo terão o nome destacado com uma cor avermelhada.</p>
    <textarea class="w-full p-2 border rounded" placeholder="Digite os nomes dos jogadores, separados por vírgulas. Ex: JafKC,Player1,Player2" v-model="marked_players_list" @input="update_marked_players"></textarea>
</template>

<style scoped>
    textarea {
        width: 322px;
        height: 199px;
        resize: both;
    }
</style>