import { createApp } from 'vue'
import App from './App.vue'

const TheApp = {
    template: `<Suspense><App /></Suspense>`,
    components: { App },
  };

createApp(TheApp).mount('#app')
