<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Player, Settings } from "./types";
import { onMounted } from "vue";
import {
    getCurrentWindow,
    PhysicalPosition,
} from "@tauri-apps/api/window";

import { getCurrentWebview } from "@tauri-apps/api/webview";
import { useStore } from "./stores/store";
import { useRouter } from "vue-router";


//const store = useStore()

document.addEventListener("mousedown", (e) => {
    let target = e.target as HTMLElement;
    const clickableElement = target.closest(
        "button, a, input, textarea, select, label, [role='button'], [role='link'], .selected-option, .clickable, li, .custom-select",
    );
    if (e.buttons === 1 && !clickableElement) {
        const window = getCurrentWindow();
        e.detail === 2 ? window.minimize() : window.startDragging();
    }
});


const router = useRouter();
let loadingEventCounter = 0;

onMounted(async () => {
    //await store.get_settings();
    const store = useStore();
    await store.get_settings();
    console.log(store.settings)



    invoke("read_logs").catch((err) => {
        console.error("Leitura de logs falhou:", err);
    });
    // if (await invoke<boolean>("is_first_use")) {
    //     router.push('/welcome');
    // }
    document.addEventListener("contextmenu", (event) => {
        event.preventDefault();
    });

    listen("player", (event) => {
        const player = event.payload as Player;
        store.players.push(player);
        store.players.sort(
            (a, b) => b.stats.content.level - a.stats.content.level,
        );
    });

    listen("player_joined", (event) => {
        const player = event.payload as Player;
        let already_in_list = false;
        if (player.username == "opponent") {
            return;
        }
        for (const i of store.players) {
            if (player.username == i.username) {
                already_in_list = true;
                break;
            }
        }

        if (!already_in_list) {
            store.players.push(player);
            store.players.sort(
                (a, b) => b.stats.content.level - a.stats.content.level,
            );
        }
    });

    listen("remove_player", (event) => {
        const player_name = event.payload as string;

        store.players.forEach((player, index) => {
            if (player.username == player_name) {
                store.players.splice(index, 1);
            }
        });
    });

    listen("remove_players", () => {
        if (store.settings.remove_players) {
            store.players = [];
        }
    });

    listen("loading", async (event) => {
        const currentLoadingEvent = ++loadingEventCounter;
        const window = getCurrentWindow();
        console.log(event.payload);
        if (event.payload) {
            router.push("/");

            store.loading = true;

            store.players = [];
            await window.setAlwaysOnTop(true);

            if (!store.settings.never_minimize) {
                await window.minimize();
                await new Promise((resolve) => setTimeout(resolve, 100));
                await window.unminimize();
                await window.setIgnoreCursorEvents(true);
            }
            await window.unminimize();

        } else {
            store.loading = false;

            await new Promise((resolve) =>
                setTimeout(resolve, store.settings.seconds_to_minimize * 1000),
            );

            if (currentLoadingEvent !== loadingEventCounter) {
                return;
            }

            if (!store.settings.never_minimize) {
                await window.setAlwaysOnTop(false);

                if (!(await window.isMinimized())) {
                    await window.minimize();
                }
            }
            await window.setIgnoreCursorEvents(false).catch((err) => {
                console.error("Falha ao registrar clicks", err);
            });
        }
    });

    listen("settings_changed", async (event) => {
        const new_settings = event.payload as Settings;
        store.settings = new_settings;
        document.documentElement.style.setProperty(
            "--bg-alpha",
            (store.settings.transparency / 100).toString(),
        );
        await getCurrentWebview().setZoom(store.settings.window_scale);
        // await getCurrentWebview().setSize(
        //     new LogicalSize(
        //         745 * store.settings.window_scale,
        //         460 * store.settings.window_scale,
        //     ),
        // );
        // await getCurrentWindow().setSize(
        //     new LogicalSize(
        //         745 * store.settings.window_scale,
        //         460 * store.settings.window_scale,
        //     ),
        // );
    });
    listen("hotkey", async () => {
        const window = getCurrentWindow();

        if (await window.isMinimized()) {
            await window.unminimize();
            await window.setAlwaysOnTop(true);
            await new Promise((resolve) => setTimeout(resolve, 500));
            await window.setAlwaysOnTop(false);
        } else {
            await window.minimize();
        }
    })
    listen("load_hotkey", async () => {
        await invoke("load_stats_tauri");
    })
    // console.log("Hotkey: " + store.settings.hotkey);
    // await register("Shift+Alt+Z", async (event) => {
    //     console.log("Hotkey acionada");

    //     const window = getCurrentWindow();

    //     if (event.state == "Pressed") {
    //         console.log("Hotkey acionada");
    //         if (await window.isMinimized()) {
    //             await window.unminimize();
    //             await window.setAlwaysOnTop(true);
    //             await new Promise((resolve) => setTimeout(resolve, 500));
    //             await window.setAlwaysOnTop(false);
    //         } else {
    //             await window.minimize();
    //         }
    //     }
    // }).catch((err) => {
    //     console.error("Falha ao registrar hotkey:", err);
    // });

    document.documentElement.style.setProperty(
        "--bg-alpha",
        (store.settings.transparency / 100).toString(),
    );
    await getCurrentWebview().setZoom(store.settings.window_scale);
    // await getCurrentWebview().setSize(
    //     new LogicalSize(
    //         745 * store.settings.window_scale,
    //         460 * store.settings.window_scale,
    //     ),
    // );
    // await getCurrentWindow().setSize(
    //     new LogicalSize(
    //         745 * store.settings.window_scale,
    //         460 * store.settings.window_scale,
    //     ),
    // );
    await getCurrentWindow().setPosition(new PhysicalPosition(5, 5));

    await invoke("check_updates")
        .then((url) => {
            store.update_url = url as string;
        })
        .catch(() => {
            console.log("KC Overlay está atualizado.");
        });
});


</script>

<template>
    <router-view></router-view>
</template>

<style>
html,
body {
    height: 100%;
    margin: 0;
    padding: 0;
}

#app {
    height: 100%;
    display: flex;
    flex-direction: column;
}

:root {
    font-family: "Minecraftia", "Symbols";
    color: #ffffff;
    background-color: rgba(24, 24, 37, var(--bg-alpha, 0.75));
    border-radius: 15px;
    font-synthesis: none;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    font-size: 15px;
    text-shadow: 2px 2px #1d1d1d;

    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    cursor: default;
}

@font-face {
    font-family: "Minecraftia";
    src: url("/Minecraftia-Regular.woff") format("truetype");
}

@font-face {
    font-family: "Symbols";
    src: url("/BalsamiqSans-Regular.ttf") format("truetype");
    font-weight: normal;
    font-style: normal;
    font-size: inherit;
}

.container {
    display: flex;
    flex-direction: column;
    text-align: center;
}

.row {
    display: flex;
    justify-content: center;
}

input,
textarea,
button {
    border-radius: 8px;
    border: 1px solid transparent;
    font-family: inherit;
    color: #ffffff;
    background-color: rgb(49, 50, 68);
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
    cursor: pointer;
}

button:hover {
    border-color: #396cd8;
}

button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
}

input,
button {
    outline: none;
}

.title-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.control-btn:hover {
    background-color: rgba(255, 255, 255, 0.1);
}

.list-move,
.list-enter-active,
.list-leave-active {
    transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
    opacity: 0;
    transform: translateX(-30px);
}

.list-enter-active,
.list-leave-active {
    position: absolute;
}

input[type="text"] {
    margin-left: 20px;
    padding-left: 10px;
}

::-webkit-scrollbar {
    width: 5px;
    height: 10px;
}

::-webkit-scrollbar-track {
    background: rgb(24, 24, 37);
    border-radius: 10px;
}

::-webkit-scrollbar-thumb {
    background-color: rgb(49, 50, 68);
    border-radius: 10px;
}

a{
    color:#74c7ec
}
</style>
