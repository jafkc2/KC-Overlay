<script setup lang="ts">
import { ref } from 'vue';
import Return from './components/Return.vue';
import Select from './components/Select.vue';
import { invoke } from '@tauri-apps/api/core';
import { Settings} from './types';
import { format_stats } from './util';
import { open } from '@tauri-apps/plugin-dialog';

interface Props {
  settings: Settings;
}
const props = defineProps<Props>();

let clients = ["Geral", "Lunar", "Badlion", "CM Client","Silent Client", "Legacy Launcher", "Personalizado"];
let stats = ["Bedwars Geral", 'Bedwars Solo', 'Bedwars Duplas', 'Bedwars Trios', 'Bedwars Quartetos', 'Bedwars 1v1', 'Bedwars 2v2'];

let settings = ref(props.settings)

let client = ref(format_client());
let stats_type = ref(format_stats(settings.value));

console.log(settings.value);
async function save_settings(){
    console.log(settings.value)
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
        case "CMClient":
            return "CM Client"
        case "Custom":
            return "Personalizado"
    }
}

async function select_log_file(){
    const file = await open({
        multiple: false,
        directory: false
    });

    if (file){
        settings.value.custom_client_path = file;
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

        <div v-if="client === 'Personalizado'" class="setting">
            <span>Log do client:</span>

            <input type="text" placeholder="Exemplo: .minecraft/logs/latest.log" v-model="settings.custom_client_path" @input="save_settings()">
            <button v-on:click="select_log_file()">Selecionar arquivo</button>
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
            <span>Tempo para minimizar após ativação:</span>
            <input type="range" @input="save_settings()" v-model.number="settings.seconds_to_minimize" min="2" max="30"/>
            <span>({{ settings.seconds_to_minimize }}s)</span>

        </div>
        <!-- <div class="setting">
            <span>Tamanho:</span>
            <input type="range" @input="save_settings()" v-model.number="settings.window_scale" min="0.25" max="1.25" step="0.01"/>
            <span>({{ (settings.window_scale * 100).toFixed(0) }}%)</span>
        </div> -->
        <div class="setting">
            <span>Nível de transparência:</span>
            <input type="range" @input="save_settings()" v-model.number="settings.transparency" min="0" max="100"/>
            <span>({{settings.transparency}}%)</span>

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

button{
    margin-left: 10px;
}
input[type="text"] {
    width: 360px;
    margin-left: 10px;
}

</style>