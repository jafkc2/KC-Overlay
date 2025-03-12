<script setup lang="ts">
import { onMounted, ref, Ref } from "vue";
import PlayerRow from "./components/PlayerRow.vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { Player } from "./types";

let players : Ref<Player[]> = ref([]);

onMounted(async () => {
  listen('player', (event) => {
    const player = event.payload as Player;
    players.value.push(player);
    console.log(player);
  });

  listen('loading', (event) => {
    if (event.payload) {
      players.value = [];
    }
  });
  
  await invoke("read_logs");
});
</script>

<template>
  <div class="main-container">
    <div class="title-bar">
      <h1 class="text-lg font-bold">KC Overlay</h1>
      <div class="controls">
        <button v-on:click="getCurrentWindow().minimize()" class="control-btn">
          ━
        </button>
        <button v-on:click="getCurrentWindow().close()" class="control-btn">
          ✕
        </button>
      </div>
    </div>

    <div class="players-container">
      <TransitionGroup 
        name="list" 
        tag="div"
        class="space-y-2"
      >
        <PlayerRow
          v-for="player in players"
          :player="player"
        />
      </TransitionGroup>
    </div>
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;

  color: rgb(205, 214, 244);
  background-color: rgba(24, 24, 37, 0.75);
  border-radius: 15px;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  display: flex;
  flex-direction: column;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.row {
  display: flex;
  justify-content: center;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: rgb(205, 214, 244);
  background-color: rgb(49, 50, 68);
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

.main-container {
  padding: 1rem;
  min-width: 400px;
  border-radius: 0.5rem;
  background-color: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(8px);
}

.title-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.controls {
  display: flex;
  gap: 0.5rem;
}

.control-btn {
  padding: 0.25rem;
  border-radius: 0.25rem;
  transition: background-color 0.2s;
}

.control-btn:hover {
  background-color: rgba(255, 255, 255, 0.1);
}
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

.list-leave-active {
  position: absolute;
}
</style>