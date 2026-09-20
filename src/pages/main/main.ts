import type { Snapshot } from '~/eye/model'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { createApp } from 'vue'
import App from './App.vue'
import 'virtual:uno.css'
import '~/styles/global.css'
import '~/styles/app.css'

async function mount() {
  let initialSnapshot: Snapshot | undefined
  if (isTauri()) {
    try {
      initialSnapshot = await invoke<Snapshot>('eye_snapshot')
    }
    catch (error) {
      console.error('Unable to load initial appearance', error)
    }
  }
  createApp(App, { initialSnapshot }).mount('#app')
}
mount()
