<template>
    <div class="titlebar">
        <button>Configurações</button>
        <button>Ver jogador</button>
        <button>Sobre</button>


        <div class="window_buttons">
            <button>–</button>
            <button>X</button>
        </div>
    </div>
</template>

<style scoped>
    .titlebar{
        display: flex;
        align-items: center;
    }
    .window_buttons{
        margin-left: auto;
    }
    button{
        margin-left: 10px;
        font-size: 0.75rem;
    }
</style>