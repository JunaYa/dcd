<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import BreakContent from '../components/BreakContent.vue'
import { useBreakPresentation } from '../composables/useBreakPresentation'
const { settings, remaining } = useMonitor()
const { countdown, backgroundStyle, overlayStyle } = useBreakPresentation()
</script>

<template>
  <div
    class="break-screen"
    :style="overlayStyle"
    :class="[
      `background-${settings.background}`,
      { 'break-window': settings.reminderStyle !== 'fullscreen' },
    ]"
  >
    <div class="break-backdrop" :style="backgroundStyle" aria-hidden="true" />
    <BreakContent :timer="countdown" :caption="remaining > 0 ? t('休息倒计时') : t('休息已完成')" />
  </div>
</template>
