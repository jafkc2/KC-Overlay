<script lang="ts" setup>
import { ref, toRef } from 'vue';
import { useStore } from './stores/store';
import TitleBar from './components/Titlebar.vue';
import PlayerTable from './components/PlayerTable.vue';

const store = useStore();
const update_url = ref(store.update_url);

let players = toRef(store, 'players');
let party_players = toRef(store, 'party_players');

let loading = toRef(store, 'loading');

let settings = toRef(store, 'settings');


</script>

<template>
    <TitleBar :update_url="update_url"></TitleBar>
    <PlayerTable v-if="players.length > 0" :party_players="party_players" :players="players" :settings="settings" :loading="loading"></PlayerTable>
    <p v-else>
        Olá! Digite o comando /jogando no chat do Mush para ver os stats de
        todos os jogadores da sala.
    </p>
    <p v-if="settings.client && settings.client.type == 'CMClient'">Caso você estiver no CMClient e a overlay não estiver funcionando, mude para o client "Geral" nas configurações do KC Overlay.</p>
</template>
