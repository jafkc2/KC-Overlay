<script setup lang="ts">
import { StatsType } from '../types'

import Select from './Select.vue'
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { get_stat_types } from '../util';

const router = useRouter();

function returnToMainView() {
    router.push('/');
}

const stats = get_stat_types();

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
        case "The Bridge":
            stats_type = {type: "TheBridge"}
            break;
        case "Fireball Fight":
            stats_type = {type: "FireballFight"}
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
            <div>
                <img src="/back.svg"/>
                <span>Retornar</span>
            </div>

        </button>
            <input type="text" placeholder="Nome do jogador" v-model="username">
        <button class="flex_button" v-on:click="search_player()">
            <div>
                <img src="/search.svg">
                <span>Ver</span>
            </div>
        </button>


        <Select class="select" :value="stats_type_str" :options="stats" @input="stats_type_selected"></Select>

    </div>
</template>

<style scoped>
    .titlebar{
        display: flex;
        margin-bottom: 20px;
        padding: 10px;
    }

    button{
        padding: 6px;
        margin-right: 10px;
        padding-left: 4px;
        padding-right: 4px;
        font-size: 10px;
        line-height: 10px;
    }
    .flex_button{
        display: flex;
    }
    img{
        margin-right: 10px;
        width: 18px;
    }

    .input-container{
        display: flex;
        align-items: center;
        justify-content: center;
    }
    input[type="text"] {
        box-sizing: border-box;
        margin-right: 5px;
        line-height: 0px;
        padding-top: 2px;
    }   

    .select{
        width: 185px;
        margin-left: 5px;
    }
</style>