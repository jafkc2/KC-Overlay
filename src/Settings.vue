<script setup lang="ts">
import { ref } from 'vue';
import Return from './components/Return.vue';
import Select from './components/Select.vue';
import { invoke } from '@tauri-apps/api/core';
import { Settings} from './types'

interface Props {
  settings: Settings;
}
const props = defineProps<Props>();

let clients = ["Geral", "Lunar", "Badlion", "Silent Client", "Legacy Launcher", "Personalizado"];
let stats = ["Bedwars Geral", 'Bedwars Solo', 'Bedwars Duplas', 'Bedwars Trios', 'Bedwars Quartetos', 'Bedwars 1v1', 'Bedwars 2v2'];

let settings = ref(props.settings)

let client = ref(format_client());
let stats_type = ref(format_stats());

console.log(settings.value);
async function save_settings(){
    console.log(settings.value)
    await invoke("save_settings", {settings: settings.value})
}

function update_client(new_client: string){
    client.value = new_client;
    switch (new_client){
        case "Geral":
            settings.value.client.type = "Default"
            break;
        case "Lunar":
            settings.value.client.type = "Lunar"
            break;
        case "Badlion":
            settings.value.client.type = "Badlion"
            break;
        case "Silent Client":
            settings.value.client.type = "Silent"
            break;
        case "Legacy Launcher":
            settings.value.client.type = "LegacyLauncher"
            break;
        case "Personalizado":
            settings.value.client.type = "Custom"
            break;
    }

    save_settings()
}

function update_stats(new_stats: string){
    stats_type.value = new_stats;
    switch (new_stats){
        case "Bedwars Geral":
            settings.value.stats_type.type = "BedwarsAll"
            break;
        case "Bedwars Solo":
            settings.value.stats_type.type = "BedwarsSolo"
            break;
        case "Bedwars Doubles":
            settings.value.stats_type.type = "BedwarsDoubles"
            break;
        case "Bedwars Trios":
            settings.value.stats_type.type = "BedwarsTrios"
            break;
        case "Bedwars Quartetos":
            settings.value.stats_type.type = "BedwarsQuads"
            break;
        case "Bedwars 1v1":
            settings.value.stats_type.type = "Bedwars1v1"
            break;
        case "Bedwars 2v2":
            settings.value.stats_type.type = "Bedwars2v2"
            break;
    }

    save_settings()

}

function format_client(): string{
    console.log(settings.value)
    console.log(settings.value.client.type)
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
        case "Custom":
            return "Personalizado"
    }
}

function format_stats(): string{
    switch (settings.value.stats_type.type){
        case "BedwarsAll":
            return "Bedwars Geral"
        case "BedwarsSolo":
            return "Bedwars Solo"
        case "BedwarsDoubles":
            return "Bedwars Duplas"
        case "BedwarsTrios":
            return "Bedwars Trios"
        case "BedwarsQuads":
            return "Bedwars quartetos"
        case "Bedwars1v1":
            return "Bedwars 1v1"
        case "Bedwars2v2":
            return "Bedwars 2v2"
    }
}


</script>

<template>
    <Return></Return>
    <div class="container">
        <p>Geral</p>

        <div class="setting">
            <span>Client:</span>
            <Select :value="client" :options="clients" @input="update_client"></Select>
        </div>

        <div class="setting">
            <span>Stats:</span>
            <Select :value="stats_type" :options="stats" @input="update_stats"></Select>

        </div>

        <p>Janela</p>

        <div class="setting">
            <span>Nunca minimizar automaticamente</span>
            <button @click="settings.never_minimize = !settings.never_minimize; save_settings()" :class="{ active: settings.show_wlr }">
                <span v-if="settings.never_minimize">Ligado</span>
                <span v-else>Desligado</span>
            </button>
        </div>
        <div class="setting">
            <span>Tempo para minimizar após ativação({{ settings.seconds_to_minimize }}s):</span>
            <input type="range" @mouseup="save_settings()" v-model.number="settings.seconds_to_minimize" min="2" max="30"/>

        </div>
        <div class="setting">
            <span>Tamanho({{ settings.window_scale }}):</span>
            <input type="range" @mouseup="save_settings()" v-model.number="settings.window_scale" min="25" max="125"/>
        </div>
        <div class="setting">
            <span>Nível de transparência({{settings.transparency}}%):</span>
            <input type="range" @mouseup="save_settings()" v-model.number="settings.transparency" min="25" max="125"/>

        </div>

        <p>Stats</p>

        <div class="setting">
            <span>Mostrar winstreak:</span>
            <button @click="settings.show_ws = !settings.show_ws; save_settings()" :class="{ active: settings.show_ws }">
                <span v-if="settings.show_ws">Sim</span>
                <span v-else>Não</span>
            </button>
        </div>
        <div class="setting">
            <span>Mostrar WLR:</span>
            <button @click="settings.show_wlr = !settings.show_wlr; save_settings()" :class="{ active: settings.show_wlr }">
                <span v-if="settings.show_wlr">Sim</span>
                <span v-else>Não</span>
            </button>
        </div>
        <div class="setting">
            <span>Mostrar FKDR:</span>
            <button @click="settings.show_fkdr = !settings.show_fkdr; save_settings()" :class="{ active: settings.show_fkdr }">
                <span v-if="settings.show_fkdr">Sim</span>
                <span v-else>Não</span>
            </button>
        </div>
        <div class="setting">
            <span>Mostrar KDR:</span>
            <button @click="settings.show_kdr = !settings.show_kdr; save_settings()" :class="{ active: settings.show_kdr }">
                <span v-if="settings.show_kdr">Sim</span>
                <span v-else>Não</span>
            </button>
        </div>
        <div class="setting">
            <span>Mostrar vitórias:</span>
            <button @click="settings.show_wins = !settings.show_wins; save_settings()" :class="{ active: settings.show_wins }">
                <span v-if="settings.show_wins">Sim</span>
                <span v-else>Não</span>
            </button>
        </div>
        <div class="setting">
            <span>Mostrar derrotas:</span>
            <button @click="settings.show_losses = !settings.show_losses; save_settings()" :class="{ active: settings.show_losses }">
                <span v-if="settings.show_losses">Sim</span>
                <span v-else>Não</span>
            </button>
        </div>
        <div class="setting">
            <span>Mostrar bans:</span>
            <button @click="settings.show_bans = !settings.show_bans; save_settings()" :class="{ active: settings.show_bans }">
                <span v-if="settings.show_bans">Sim</span>
                <span v-else>Não</span>
            </button>
        </div>

    </div>
</template>

<style scoped>
.container {
    padding-top: 10px;
    max-height: 380px;
    overflow-y: auto;
}
.setting{
    display: flex;
    align-items: center;
    margin-left: 20px;
    margin-bottom: 10px;
}
</style>