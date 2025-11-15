<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

function go_to_main_route() {
    router.push("/");
}

async function go_login() {
    router.push("/login")
}


</script>
<template>
    <div class="welcome">
        <header>
            <h3>
                Olá, seja bem-vindo ao KC Overlay! Escolha o tipo de conta que
                você usa para começar:
            </h3>
        </header>
        <main>
            <div class="button-row">
                <button v-on:click="go_login">Original</button>
                <button v-on:click="go_to_main_route">Pirata</button>
            </div>
        </main>
    </div>

</template>

<style scoped>
.welcome {
    display: grid;
    line-height: 15px;
}

img {
    margin-right: 10px;
    width: 18px;
}

.gray {
    color: #cdd6f4;
    line-height: 20px;
}

.client {
    display: flex;
    align-items: center;
    margin-bottom: 10px;
    line-height: 20px;
}

.button-row {
    gap: 20px;
    display: flex;
}

.button-row button {
    font-size: 16px;
}

button {
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

.select {
    padding: 13px;
}
</style>
