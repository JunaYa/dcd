import type { Snapshot } from '~/eye/model'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { createApp } from 'vue'
import App from './App.vue'
import '~/styles/uno'
import '~/styles/global.css'
import '~/styles/app.css'
import '~/styles/break-typography.css'
import { loadBreakFont } from '~/theme/breakTypography'

async function mount() {
  let initialSnapshot: Snapshot | undefined
  if (isTauri()) {
    try {
      initialSnapshot = await invoke<Snapshot>('eye_snapshot')
    } catch (error) {
      console.error('Unable to load initial appearance', error)
    }
  }
  try {
    const font =
      initialSnapshot?.settings.breakFont ??
      JSON.parse(localStorage.getItem('eye-preview-settings') || '{}').breakFont
    if (font === 'pixel') await loadBreakFont(font)
  } catch (error) {
    console.error('Unable to load break typography', error)
  }
  createApp(App, { initialSnapshot }).mount('#app')
}
mount()
