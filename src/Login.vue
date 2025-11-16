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

    <div class="login-page-container">
        
        <header class="login-header">
            <h3>Fazer login na conta</h3>
        </header>

        <div class="login-box card">
            <p>
                Para fazer login, acesse
                <a href="#" @click.prevent="open_link(code.link)">{{
                    code.link
                }}</a>
                e digite o código abaixo.
            </p>

            <div class="code-row">
                <span class="code-display">{{ code.code }}</span>
                <button class="copy-button" v-on:click="writeText(code.code)">Copiar</button>
            </div>

            <p class="login-status">{{ login_state }}</p>
        </div>

        <div class="faq-box card">
            <div class="faq-item">
                <h4>"Por que é necessário fazer login agora?"</h4>
                <p class="gray">
                    Resposta: Como o Mush removeu o comando /jogando, foi necessário
                    implementar outro método para que a overlay continuasse
                    funcionando. De forma resumida, o método atual consiste em criar
                    um servidor intermediário entre o Mush e o usuário (você). Esse
                    servidor é capaz de capturar várias informações do Mush, como a
                    lista de jogadores no servidor. Mas, para que esse servidor
                    intermediário funcione em contas originais, é preciso fazer
                    login com sua conta do Minecraft, caso contrário, o Mush vai
                    identificar a conta como pirata e impedir a entrada.
                </p>
            </div>

            <div class="faq-item">
                <h4>"É seguro? Como garanto que minha conta não vai ser roubada?"</h4>
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
            </div>
        </div>

    </div>
</template>


<style scoped>
.gray {
    color: #cdd6f4;
    line-height: 20px;
    opacity: 0.9;
}

.login-page-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    padding: 20px;
    max-width: 800px;
    margin: 20px auto;
}

.login-header {
    text-align: center;
    width: 100%;
}

.login-header h3 {
    font-size: 1.5rem;
    color: #f5e0dc;
    margin-bottom: 0;
}

.card {
    background-color: #313244;
    border-radius: 8px;
    padding: 24px;
    width: 100%;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.login-box {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

a {
    color: #94e2d5;
    text-decoration: none;
    font-weight: 500;
}
a:hover {
    text-decoration: underline;
}

.code-row {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
}

.code-display {
    font-size: 1.4rem;
    font-weight: bold;
    color: #f9e2af;
    background-color: #1e1e2e;
    padding: 10px 16px;
    border-radius: 6px;
    letter-spacing: 2px;
    flex-grow: 1;
    text-align: center;
}

.copy-button {
    background-color: #89b4fa;
    color: #1e1e2e;
    border: none;
    border-radius: 6px;
    padding: 12px 16px;
    font-size: 0.9rem;
    font-weight: bold;
    cursor: pointer;
    transition: background-color 0.2s ease;
}
.copy-button:hover {
    background-color: #a6caff;
}

.login-status {
    text-align: center;
    font-style: italic;
    color: #cdd6f4;
    opacity: 0.8;
    padding-top: 10px;
}

.faq-box {
    display: flex;
    flex-direction: column;
    gap: 24px;
}

.faq-item h4 {
    font-size: 1.1rem;
    color: #f5e0dc;
    margin-bottom: 8px;
}

</style>