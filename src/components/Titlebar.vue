<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';
import { View } from '../types'
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

let update_url = ref("");
await invoke("check_updates").then((url) => {
    update_url.value = url as string;
}).catch(() => {console.log("KC Overlay está atualizado.")});

async function update(url: string){
    invoke("install_update", {url: url});
    update_url.value = "";
}
</script>

<template>
    <div class="titlebar" data-tauri-drag-region>
        <button v-on:click="emit('change_view', View.settings)" class="flex_button">
            <div>
                <img src="/settings.svg"/>
                <span>Configurações</span>
            </div>
        </button>
        <button v-on:click="emit('change_view', View.viewPlayer)" class="flex_button">
            <div>
                <img src="/search.svg"/>
                <span>Ver jogador</span>
            </div>
        </button>
        <button v-on:click="emit('change_view', View.about)" class="flex_button">
            <div>
                <img src="/favorite.svg"/>
                <span>Sobre</span>
            </div>
        </button>
        <button v-if="update_url" v-on:click="update(update_url)">
            <div>
                <img src="/download.svg"/>
                <span>Atualizar</span>
            </div>
        </button>

        <div class="window_buttons_div">
            <button class="window_button" v-on:click="getCurrentWindow().minimize()">-</button>
            <button class="window_button" v-on:click="getCurrentWindow().close()">
                <div>
                    <span>x</span>
                </div>
            </button>
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
        display: flex;
        margin-left: auto;
    }
    .window_button{
        padding-left: 12px;
        padding-right: 12px;
        line-height: 10px;
        font-size: 12px;
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
</style>