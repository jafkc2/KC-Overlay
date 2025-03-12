/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module 'vue' {
  import type { ComponentPublicInstance } from '@vue/runtime-core'
  global {
    interface __VLS_GlobalComponents {
      RouterView: typeof import('vue-router')['RouterView']
      RouterLink: typeof import('vue-router')['RouterLink']
    }
  }
  interface ComponentCustomProperties {
    $props: Record<string, unknown>
  }
}

export {}