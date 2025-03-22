import { createApp } from 'vue'
import App from './App.vue'
import { invoke } from '@tauri-apps/api/core';

export async function update(url: string){
  invoke("install_update", {url: url});
}

const TheApp = {
    template: `<Suspense><App /></Suspense>`,
    components: { App },
  };

createApp(TheApp).mount('#app')
