<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';
import { View } from '../types'


</script>

<template>
    <div class="titlebar" data-tauri-drag-region>
        <button v-on:click="emit('change_view', View.settings)" class="flex_button">
            <img src="../../assets/settings.svg"/>
            <span>Configurações</span>
        </button>
        <button v-on:click="emit('change_view', View.viewPlayer)" class="flex_button">
            <img src="../../assets/search.svg"/>
            <span>Ver jogador</span>
        </button>
        <button v-on:click="emit('change_view', View.about)" class="flex_button">
            <img src="../../assets/favorite.svg"/>
            <span>Sobre</span>
        </button>

        <div class="window_buttons_div">
            <button class="window_button" v-on:click="getCurrentWindow().minimize()">-</button>
            <button class="window_button" v-on:click="getCurrentWindow().close()">x</button>
        </div>
    </div>
</template>

<style scoped>
    .titlebar{
        display: flex;
        align-items: center;
        margin-bottom: 20px;
    }
    .window_buttons_div{
        margin-left: auto;
    }
    .window_button{
        padding-left: 12px;
        padding-right: 12px;
    }
    button{
        padding: 6px;
        margin-right: 10px;
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
</style>