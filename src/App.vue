<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Player, View, Settings } from "./types";
import { onMounted, Ref, ref } from "vue";
import { getCurrentWindow, PhysicalPosition } from '@tauri-apps/api/window';

import PlayerTable from "./components/PlayerTable.vue";
import TitleBar from "./components/Titlebar.vue";

import About from "./About.vue";
import SettingsView  from "./Settings.vue";
import ViewPlayer from "./ViewPlayer.vue";
import { register } from "@tauri-apps/plugin-global-shortcut";

let players : Ref<Player[]> = ref([]);


let current_view = ref(View.main);

document.addEventListener('mousedown', (e) => {
  let target = e.target as HTMLElement;
  const clickableElement = target.closest("button, a, input, textarea, select, label, [role='button'], [role='link'], .selected-option, .clickable, li, .custom-select");
  if (e.buttons === 1 && !clickableElement) {
    const window = getCurrentWindow()
    e.detail === 2
      ? window.minimize() 
      : window.startDragging();
  }
});
let settings = await invoke<Settings>("get_settings");

onMounted(async () => {
  document.addEventListener('contextmenu', (event) => {
    event.preventDefault();
  });

  listen('player', (event) => {
    const player = event.payload as Player;
    players.value.push(player);
    players.value.sort((a, b) => b.stats.Bedwars.level - a.stats.Bedwars.level)
  });

  listen('player_joined', (event) => {
    const player = event.payload as Player;
    let already_in_list = false;
    for (const i of players.value){
      if (player.username == i.username){
        already_in_list = true;
        break;
      }
    }

    if (!already_in_list){
      players.value.push(player)
      players.value.sort((a, b) => b.stats.Bedwars.level - a.stats.Bedwars.level)
    }
  })

  listen('remove_player', (event) => {
    const player_name = event.payload as string;

    players.value.forEach((player, index) => {
      if (player.username == player_name){
        players.value.splice(index, 1);
      }
    })
  })

  listen('loading', async (event) => {
    const window = getCurrentWindow();
    console.log(event.payload);
    if (event.payload) {
      players.value = [];

      if (await window.isMinimized()){
        await window.unminimize()
      }
      await window.setAlwaysOnTop(true);
      if (!settings.never_minimize){
        //await window.setIgnoreCursorEvents(true)
      }

    } else{
      await new Promise(resolve => setTimeout(resolve, settings.seconds_to_minimize * 1000));

      if (!settings.never_minimize){
        await window.setAlwaysOnTop(false);
        if (!await window.isMinimized()){
          await window.minimize();
        }
      }
      //await window.setIgnoreCursorEvents(false);

    }
  });
  
  listen('change_view', (event) => {
    const view = event.payload as View;
    current_view.value = view;
  })

  listen('settings_changed', (event) => {
    const new_settings = event.payload as Settings;
    settings = new_settings;
    document.documentElement.style.setProperty('--bg-alpha', (settings.transparency / 100).toString())

  })

  await register('alt+z', async (event) => {
    const window = getCurrentWindow();

    if (event.state == "Pressed"){
      if (await window.isMinimized()){
        await window.unminimize()
        await window.setAlwaysOnTop(true);
        await new Promise(resolve => setTimeout(resolve, 500));
        await window.setAlwaysOnTop(false);

      } else{
        await window.minimize();
      }
    }

  });

  document.documentElement.style.setProperty('--bg-alpha', (settings.transparency / 100).toString());
  await getCurrentWindow().setPosition(new PhysicalPosition(10, 10));
  invoke("read_logs");

});


let update_url = ref("");
await invoke("check_updates").then((url) => {
    update_url.value = url as string;
}).catch(() => {console.log("KC Overlay está atualizado.")});


</script>

<template>
    <div v-if="current_view == View.main">
    <TitleBar :update_url="update_url"></TitleBar>
    <PlayerTable v-if="players.length > 0" :players="players" :settings="settings"></PlayerTable>
    <p v-else>Olá! Digite o comando /jogando no chat do Mush para ver os stats de todos os jogadores da sala.</p>
  </div>
  <div v-else-if="current_view == View.settings">
    <SettingsView :settings="settings"></SettingsView>
  </div>
  <div v-else-if="current_view == View.about">
    <About></About>
  </div>
  <div v-else-if="current_view == View.viewPlayer">
    <ViewPlayer></ViewPlayer>
  </div>
</template>

<style>
:root {
  font-family: "Minecraftia", "Symbols";
  color: #ffffff;
  background-color: rgba(24, 24, 37, var(--bg-alpha, 0.75));
  border-radius: 15px;
  font-synthesis: none;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  font-size: 15px;

  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none; 
  cursor: default;
}

@font-face {
  font-family: "Minecraftia";
  src: url("/Minecraftia-Regular.woff") format("truetype");
}

@font-face {
  font-family: "Symbols";
  src: url("/BalsamiqSans-Regular.ttf") format("truetype");
  font-weight: normal;
  font-style: normal;
  font-size: inherit;

}

.container {
  display: flex;
  flex-direction: column;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  font-family: inherit;
  color: #ffffff;
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

.title-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.control-btn:hover {
  background-color: rgba(255, 255, 255, 0.1);
}
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

.list-enter-active,
.list-leave-active {
  position: absolute;
}

input[type="text"] {
  margin-left: 20px;
  padding-left: 10px;

}

::-webkit-scrollbar {
  width: 5px;
  height: 10px;
}

::-webkit-scrollbar-track {
  background: rgb(24, 24, 37);
  border-radius: 10px;
}

::-webkit-scrollbar-thumb {
  background-color: rgb(49, 50, 68);
  border-radius: 10px;
}

</style>