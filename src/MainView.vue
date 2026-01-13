<script lang="ts" setup>
import { ref, toRef } from 'vue';
import { useStore } from './stores/store';
import TitleBar from './components/Titlebar.vue';
import PlayerTable from './components/PlayerTable.vue';
import { listen } from '@tauri-apps/api/event';

const store = useStore();
const update_url = ref(store.update_url);

let players = toRef(store, 'players');
let party_players = toRef(store, 'party_players');

let loading = toRef(store, 'loading');

let settings = toRef(store, 'settings');

let text = ref("Olá! Entre no ip 127.0.0.1:25567 e digite /ver no chat ou use o atalho shift+alt+e para usar o KC Overlay.")
listen("not_logged", () => {
    text.value = "Você tentou entrar com uma conta original, porém não fez login no KC Overlay. Faça login para poder usar a overlay."
});
</script>

<template>
    <TitleBar :update_url="update_url"></TitleBar>
    <PlayerTable v-if="players.length > 0" :party_players="party_players" :players="players" :settings="settings" :loading="loading"></PlayerTable>
    <p v-else>
        {{text}}
    </p>
</template>
