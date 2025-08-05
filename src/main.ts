import { createApp } from 'vue'
import App from './App.vue'
import { invoke } from '@tauri-apps/api/core';
import SettingsView from './Settings.vue';
import { createRouter, createWebHistory } from 'vue-router';
import MainView from './MainView.vue';
import { createPinia } from 'pinia';
import About from './About.vue';
import ViewPlayer from './ViewPlayer.vue';

export async function update(url: string){
  invoke("install_update", {url: url});
}

const routes = [
  {
    path: '/',
    component: MainView
  },
  {
    path: '/settings',
    component: SettingsView
  },
  {
    path: '/about',
    component: About
  },
  {
    path: '/view_player',
    component: ViewPlayer
  }

];

const router = createRouter({
  routes,
  history: createWebHistory()
});


const app = createApp(App);
const pinia = createPinia();

app.use(router);
app.use(pinia)
app.mount('#app')
