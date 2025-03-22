<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event';
import { View } from '../types'
import {update} from '../main'
import { onMounted, ref } from 'vue';

const update_progress = ref(0);
onMounted(async () => {
    listen('update_progress', (event) => {
        update_progress.value = event.payload as number;
    })
})
interface Props {
  update_url: string;
}
defineProps<Props>();
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
        <button v-if="update_url && update_progress == 0" v-on:click="update(update_url)">
            <div>
                <img src="/download.svg"/>
                <span>Atualizar</span>
            </div>
        </button>
        <button v-if="update_progress > 0">
            <img src="/download.svg"/>
            <span>{{update_progress}}%</span>
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