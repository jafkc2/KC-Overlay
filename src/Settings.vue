<script setup lang="ts">
import Return from './components/Return.vue';

// let client : string;
// let custom_client_path
// onMounted(async () => {
//         let settings = await invoke("get_settings");
//     })
</script>

<template>
    <Return></Return>
    <div>
        <p>Geral</p>
        <div class="setting">
            <span>Client:</span>
            <select>
                <option>Geral</option>
                <option>Badlion</option>
                <option>Lunar</option>
                <option>Silent Client</option>
                <option>Legacy Launcher</option>
                <option>Personalizado</option>
            </select>
        </div>
        <div class="setting">
            <span>Stats:</span>
            <select>
                <option>Bedwars Geral</option>
                <option>Bedwars Solo</option>
                <option>Bedwars Duplas</option>
                <option>Bedwars Trios</option>
                <option>Bedwars Quartetos</option>
                <option>Bedwars 1v1</option>
                <option>Bedwars 2v2</option>

            </select>
        </div>
    </div>
</template>

<style scoped>
.setting{
    display: flex;
}


</style>