<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import Titlebar from './components/Titlebar.vue'

const greetMsg = ref("");
const name = ref("");

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main data-tauri-drag-region class="container">
    <Titlebar></Titlebar>
    <p>Digite /jogando no chat do Mush para ver os stats dos jogadores.</p>

    <p>{{ greetMsg }}</p>
  </main>
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
</style>