<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import Return from "./components/Return.vue";
    import { invoke } from "@tauri-apps/api/core";
    import { openPath } from '@tauri-apps/plugin-opener';

    let version = ref("v");

    onMounted(async () => {
        version.value = await invoke("get_version");
    })

    async function open_link(url: string){
        await openPath(url)
    }
</script>
<template>
    <Return></Return>
    <div class="about">
        <header>
            <h1>Muito obrigado por usar o KC Overlay!</h1>
        </header>
        <main>
            <p>Considere entrar no servidor do Discord para saber das novidades, fazer sugestões, reportar bugs e interagir com a comunidade.</p>
            <button v-on:click="open_link('https://discord.gg/SKwZSpPCN5')">
                <img src="../../assets/discord.svg"/>
                <span>Discord</span>
            </button>
            <p>Acompanhe o desenvolvimento no Github!</p>
            <button v-on:click="open_link('https://github.com/jafkc2/KC-Overlay')">
                <img src="../../assets/github.svg"/>
                <span>Github</span>
            </button>

        </main>
        <footer>KC Overlay {{ version }} - Desenvolvido por JafKC com a ajuda de oRvdy</footer>
    </div>
</template>

<style scoped>
.about{
    display: grid;
    line-height: 15px;
}

img{
    margin-right: 10px;
    width: 18px;
}

button{
        padding: 6px;
        margin-right: 10px;
        padding-left: 4px;
        margin-bottom: 50px;
        padding-right: 4px;
        font-size: 0.75rem;
        align-items: center;
        text-align: center;
        justify-content: center;
        display: flex;
    }

</style>