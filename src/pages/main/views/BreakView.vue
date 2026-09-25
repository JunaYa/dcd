<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import BreakContent from '../components/BreakContent.vue'
import { useBreakPresentation } from '../composables/useBreakPresentation'
const { settings, remaining, native, report } = useMonitor()
const { countdown, backgroundStyle, overlayStyle } = useBreakPresentation()
const entrance = ref(0)
const hidden = ref(document.hidden)
let unlisten: UnlistenFn | undefined
let disposed = false
function updateVisibility() {
  hidden.value = document.hidden
}
onMounted(async () => {
  document.addEventListener('visibilitychange', updateVisibility)
  if (!native) return
  try {
    const stop = await listen('eye-break-show', () => {
      hidden.value = false
      entrance.value++
    })
    if (disposed) stop()
    else unlisten = stop
  } catch (error) {
    report(error)
  }
})
onUnmounted(() => {
  disposed = true
  unlisten?.()
  document.removeEventListener('visibilitychange', updateVisibility)
})
</script>

<template>
  <div
    :key="entrance"
    class="break-screen break-entrance"
    :style="overlayStyle"
    :class="[
      `background-${settings.background}`,
      { 'break-window': settings.reminderStyle !== 'fullscreen', 'break-hidden': hidden },
    ]"
  >
    <div class="break-backdrop" :style="backgroundStyle" aria-hidden="true" />
    <BreakContent :timer="countdown" :caption="remaining > 0 ? t('休息倒计时') : t('休息已完成')" />
  </div>
</template>
