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
        Olá! Entre no ip localhost:25567 e digite /ver no chat ou use o atalho Alt+X para usar o KC Overlay.
    </p>
</template>
