<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
import { View } from '../types'


function returnToMainView() {
    emit('change_view', View.main);
}
</script>

<template>
    <div class="titlebar" data-tauri-drag-region>
        <button v-on:click="returnToMainView" class="flex_button">
            <div>
                <img src="/back.svg"/>
                <span>Retornar</span>
            </div>
        </button>
    </div>
</template>

<style scoped>
    .titlebar{
        display: flex;
        align-items: center;
        margin-bottom: 20px;
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