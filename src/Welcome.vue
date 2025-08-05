<script setup lang="ts">

import { useRouter } from "vue-router";



const router = useRouter();

function go_to_main_route(){
    router.push('/');
}

</script>
<template>
    <div class="welcome">
        <header>
            <h3>
                Seja bem-vindo ao KC Overlay! Escolha o seu client para começar.
            </h3>
        </header>
        <main>
            <p style="color: #aaaaaa;">Caso use um client além dos disponíveis, poderá adicioná-lo manualmente nas configurações.</p>

            <button v-on:click="go_to_main_route">Continuar</button>
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

.client{
    display: flex;
    align-items: center;
    margin-bottom: 10px;
    line-height: 20px;
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
