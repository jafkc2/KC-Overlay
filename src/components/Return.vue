<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
import { View } from '../types'


function returnToMainView() {
    emit('change_view', View.main);
}
</script>

<template>
    <div class="titlebar">
        <button v-on:click="returnToMainView" class="flex_button">
            <img src="../../assets/back.svg"/>
            <span>Retornar</span>
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