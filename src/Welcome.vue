<script setup lang="ts">
import { ref } from "vue";
import { Settings } from "./types";
import { invoke } from '@tauri-apps/api/core';
import Select from './components/Select.vue';
import { emit } from '@tauri-apps/api/event';
import { View } from './types'


interface Props {
  settings: Settings;
}
const props = defineProps<Props>();

let clients = ["Geral", "Lunar", "Badlion", "CM Client","Silent Client", "Legacy Launcher"];
let settings = ref(props.settings)

let client = ref(format_client());


async function save_settings(){
    if (settings.value.client.type == "Custom"){
        settings.value.client = { type: "Custom", path: settings.value.custom_client_path }
    }
    await invoke("save_settings", {settings: settings.value})
}
function update_client(new_client: string){
    client.value = new_client;
    switch (new_client){
        case "Geral":
            settings.value.client = {type: "Default"}
            break;
        case "Lunar":
            settings.value.client = {type: "Lunar"}
            break;
        case "Badlion":
            settings.value.client = {type: "Badlion"}
            break;
        case "Silent Client":
            settings.value.client = {type: "Silent"}
            break;
        case "Legacy Launcher":
            settings.value.client = {type: "LegacyLauncher"}
            break;
        case "CM Client":
            settings.value.client = {type: "CMClient"}
            break;
        case "Personalizado":
            settings.value.client = { type: "Custom", path: settings.value.custom_client_path };
            break;
    }

    save_settings()
}

function format_client(): string{
    switch (settings.value.client.type){
        case "Default":
            return "Geral"
        case "Badlion":
            return "Badlion"
        case "Lunar":
            return "Lunar"
        case "Silent":
            return "Silent Client"
        case "LegacyLauncher":
            return "Legacy Launcher"
        case "CMClient":
            return "CM Client"
        case "Custom":
            return "Personalizado"
    }
}
</script>
<template>
    <div class="welcome">
        <header>
            <h3>
                Seja bem-vindo ao KC Overlay! Escolha o seu client para começar.
            </h3>
        </header>
        <main>
            <div class="client">
                <span>Client:</span>
                <Select :value="client" :options="clients" @input="update_client" class="select"></Select>
                <span style="margin-left: 10px;" v-if="client == 'Geral'">Vanilla, Forge, etc.</span>
            </div>
            <p style="color: #aaaaaa;">Caso use um client além dos disponíveis, poderá adicioná-lo manualmente nas configurações.</p>

            <button v-on:click="emit('change_view', View.main)">Continuar</button>
        </main>
    </div>
</template>

<style scoped>
.welcome {
    display: grid;
    line-height: 15px;
}

img {
    margin-right: 10px;
    width: 18px;
}

.client{
    display: flex;
    align-items: center;
    margin-bottom: 10px;
    line-height: 20px;
}

button {
    padding: 6px;
    margin-right: 10px;
    padding-left: 4px;
    margin-bottom: 50px;
    padding-right: 4px;
    font-size: 0.75rem;
    align-items: center;
    text-align: center;
    justify-content: center;
    display: flex;
}

.select {
    padding: 13px;
}
</style>
