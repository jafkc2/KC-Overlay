<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
import { View, StatsType } from '../types'

import Select from './Select.vue'
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

function returnToMainView() {
    emit('change_view', View.main);
}

const stats = ["Bedwars Geral", 'Bedwars Solo', 'Bedwars Duplas', 'Bedwars Trios', 'Bedwars Quartetos', 'Bedwars 1v1', 'Bedwars 2v2'];

let username = ref("")
let stats_type_str = ref("Bedwars Geral")

function stats_type_selected(new_stats_type: string){
    stats_type_str.value = new_stats_type;
     
}
function get_stats() : StatsType{
    let stats_type : StatsType = {type: "BedwarsAll"}
    switch (stats_type_str.value){
        case "Bedwars Geral":
            stats_type = {type: "BedwarsAll"}
            break;
        case "Bedwars Solo":
            stats_type = {type: "BedwarsSolo"}
            break;
        case "Bedwars Doubles":
            stats_type = {type: "BedwarsDoubles"}
            break;
        case "Bedwars Trios":
            stats_type = {type: "BedwarsTrios"}
            break;
        case "Bedwars Quartetos":
            stats_type = {type: "BedwarsQuads"}
            break;
        case "Bedwars 1v1":
            stats_type = {type: "Bedwars1v1"}
            break;
        case "Bedwars 2v2":
            stats_type = {type: "Bedwars2v2"}
            break;
    }
    return stats_type;

}

async function search_player() {
    const stats_type = get_stats();
    console.log(stats_type)
    invoke("search_player", {username: username.value, statsType: stats_type})
}
</script>

<template>
    <div class="titlebar"  data-tauri-drag-region>
        <button v-on:click="returnToMainView" class="flex_button">
            <img src="/back.svg"/>
            <span>Retornar</span>
        </button>

        <input type="text" placeholder="Nome do jogador" v-model="username">
        <button class="flex_button" v-on:click="search_player()">
            <img src="/search.svg">
            <span>Ver</span>
        </button>


        <Select class="select" :value="stats_type_str" :options="stats" @input="stats_type_selected"></Select>

    </div>
</template>

<style scoped>
    .titlebar{
        display: flex;
        margin-bottom: 20px;
    }

    button{
        padding: 6px;
        padding-left: 4px;
        padding-right: 4px;
        font-size: 0.75rem;
        align-items: center;
        text-align: center;
        justify-content: center;
    }
    .flex_button{
        display: flex;
    }
    img{
        margin-right: 10px;
        width: 18px;
    }

    input[type="text"] {
        text-align: left;
        box-sizing: border-box;
        margin-right: 5px;
    }   

    .select{
        width: 185px;
        margin-left: 5px;
    }
</style>