<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { openPath } from '@tauri-apps/plugin-opener';
import { ref } from 'vue';
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useStore } from "./stores/store";
import { MinecraftAccount } from "./types";
import { useRouter } from 'vue-router';
import Return from './components/Return.vue';
let store = useStore();
let router = useRouter();
type AuthCode = {
    code: string;
    link: string;
    device_code: string;
};
let code = ref({ code: "", link: "", device_code: "" });

async function go_login() {
    code.value = await invoke<AuthCode>("request_code");
    wait_login()
}
async function open_link(url: string) {
    await openPath(url);
}
let login_state = ref("Obtendo código...")
async function wait_login() {
    login_state.value = "Esperando login..."
    console.log(code.value.device_code);
    let auth_token = await invoke<{ access_token: string, refresh_token: string }>("wait_for_login", { deviceCode: code.value.device_code });
    login_state.value = "Login feito. Acessando a conta..."

    let account: MinecraftAccount | null = await invoke("login_with_refresh_token", { refreshToken: auth_token.refresh_token });
    if (account != null) {
        store.settings.account = { username: account.username, token: auth_token.refresh_token };
        login_state.value = "Login concluido com sucesso!"
        
    } else {
        login_state.value = "Ocorreu um erro ao acessar a conta, tente novamente."
    }
    await invoke("save_settings", { settings: store.settings })
    setTimeout(() => {
        router.push("/")
    }, 250);
}

go_login()
</script>

<template>
    <Return></Return>
    <div>
        <header>
            <h3>Fazer login na conta</h3>
            <p>
                Para fazer login, acesse
                <a href="#" @click.prevent="open_link(code.link)">{{
                    code.link
                    }}</a>
                e digite o código
            </p>
            <div class="button-row">
                <h4>{{ code.code }}</h4>
                <button v-on:click="writeText(code.code)">copiar</button>
            </div>
            <p>{{ login_state }}</p>
        </header>
        <main>
            <p>"Por que é necessário fazer login agora?"</p>
            <p class="gray">
                Resposta: Como o Mush removeu o comando /jogando, foi necessário
                implementar outro método para que a overlay continuasse
                funcionando. De forma resumida, o método atual consiste em criar
                um servidor intermediário entre o Mush e o usuário (você). Esse
                servidor é capaz de capturar várias informações do Mush, como a
                lista de jogadores no server. Mas, para que esse servidor
                intermediário funcione em contas originais, é preciso fazer
                login com sua conta do Minecraft, caso contrário, o Mush vai
                identificar a conta como pirata e impedir a entrada.
            </p>

            <p>"É seguro? Como garanto que minha conta não vai ser roubada?"</p>
            <p class="gray">
                Resposta: É seguro. Eu não tenho acesso aos dados da conta. O
                processo de login é feito no site da Microsoft, e os dados da
                conta são salvos apenas no seu computador, de forma segura. Caso
                tenha conhecimento em programação e queira verificar o código
                usado no KC Overlay,
                <a href="#" @click.prevent="
                    open_link(
                        'https://github.com/jafkc2/KC-Overlay/blob/master/src-tauri/src/login.rs',
                    )
                    ">Clique aqui.</a>
            </p>
        </main>
    </div>
</template>


<style scoped>
.gray {
    color: #cdd6f4;
    line-height: 20px;
}
</style>