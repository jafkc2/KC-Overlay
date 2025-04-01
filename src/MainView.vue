<script lang="ts" setup>
import { ref, toRef } from 'vue';
import { useStore } from './stores/store';
import TitleBar from './components/Titlebar.vue';
import PlayerTable from './components/PlayerTable.vue';

const store = useStore();
const update_url = ref(store.update_url);

let players = toRef(store, 'players');
let loading = toRef(store, 'loading');
let settings = toRef(store, 'settings');


</script>

<template>
    <TitleBar :update_url="update_url"></TitleBar>
    <PlayerTable v-if="players.length > 0" :players="players" :settings="settings" :loading="loading"></PlayerTable>
    <p v-else>
        Olá! Digite o comando /jogando no chat do Mush para ver os stats de
        todos os jogadores da sala.
    </p>
</template>
