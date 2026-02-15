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
    <PlayerTable v-if="players.length > 0" :party_players="party_players" :players="players" :settings="settings"
        :loading="loading"></PlayerTable>
    <div v-else class="info-container">

        <p>Utilize o comando <span style="color:#3eedad">/block + tab</span> </p>
        <p>no servidor para abrir a overlay dentro do jogo.</p>

    </div>
</template>

<style scoped>
.info-container {
    padding: 20px;
    text-align: center;
    text-shadow: 2px 2px #1d1d1d;
}

.info-text {
    margin-bottom: 20px;
    font-size: 1rem;
}

.separator {
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 20px 0;
}

.separator span {
    padding: 0 15px;
    position: relative;
}

.separator::before,
.separator::after {
    content: '';
    flex: 1;
    height: 1px;
    background-color: #45475a;
}

.ip-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    margin-top: 20px;
}

.ip-label {
    font-size: 0.9rem;
    color: #999;
    font-weight: bold;
}

.ip-box {
    display: flex;
    align-items: center;
    gap: 10px;
    border: 1px solid #1e1e2e;
    border-radius: 4px;
    padding: 10px 15px;
    background-color: #45475a;
}

.ip-text {
    font-size: 1.2rem;
    font-weight: bold;
    color: #fff;
    font-family: monospace;
}
</style>
