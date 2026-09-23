<script setup lang="ts">
import { computed, ref } from 'vue'
import { useElementSize, useMediaQuery } from '@vueuse/core'
import { t } from '~/i18n'
import { accentTokens } from '~/theme/accent'
import { useMonitor } from '../composables/useMonitor'
import { useBreakPresentation } from '../composables/useBreakPresentation'
import BreakContent from './BreakContent.vue'
const { settings } = useMonitor()
const { overlayStyle, backgroundStyle } = useBreakPresentation()
const frame = ref<HTMLElement>()
const { width } = useElementSize(frame)
const systemDark = useMediaQuery('(prefers-color-scheme: dark)')
const dark = computed(() =>
  settings.value.background === 'system' ? systemDark.value : settings.value.background !== 'light',
)
const themeStyle = computed(() => ({
  ...overlayStyle.value,
  ...accentTokens(settings.value.accentColor, dark.value),
  '--color-text': dark.value ? '#eeeeef' : 'var(--color-break-light-text)',
  '--color-text-muted': dark.value ? '#a5a5ab' : 'var(--color-break-light-muted)',
}))
const timer = computed(
  () =>
    `${String(settings.value.breakMinutes).padStart(2, '0')}:${String(settings.value.breakSeconds).padStart(2, '0')}`,
)
</script>

<template>
  <figure class="break-preview">
    <figcaption>{{ t('实时预览') }}</figcaption>
    <div ref="frame" class="break-preview-frame" :style="themeStyle" aria-hidden="true">
      <div class="break-preview-stage" :style="{ transform: `scale(${width / 800})` }">
        <div class="break-preview-desktop">
          <div />
          <div />
          <div />
        </div>
        <div
          class="break-screen"
          :class="[
            `background-${settings.background}`,
            { 'break-window': settings.reminderStyle !== 'fullscreen' },
          ]"
        >
          <div class="break-backdrop" :style="backgroundStyle" />
          <BreakContent :timer="timer" :caption="t('休息倒计时')" preview />
        </div>
      </div>
    </div>
    <p class="section-description">{{ t('预览随配置实时更新，桌面背景为示意。') }}</p>
  </figure>
</template>

<style scoped>
.break-preview {
  margin: var(--space-6) 0;
}
figcaption {
  margin-bottom: var(--space-3);
  font-size: var(--text-small);
  color: var(--color-text-muted);
}
.break-preview-frame {
  width: min(100%, 480px);
  aspect-ratio: 10 / 7;
  position: relative;
  overflow: hidden;
  border-radius: var(--radius-lg);
  isolation: isolate;
  border: 1px solid var(--color-border-subtle);
  pointer-events: none;
}
.break-preview-stage {
  width: 800px;
  height: 560px;
  transform-origin: top left;
  position: absolute;
  inset: 0 auto auto 0;
}
.break-preview-desktop {
  position: absolute;
  inset: 0;
  background: #8398a2;
  padding: 56px;
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 24px;
}
.break-preview-desktop > div {
  border-radius: 16px;
  background: #dae0df;
}
.break-preview-desktop > div:first-child {
  grid-row: span 2;
  background: #637a87;
}
.break-preview-desktop > div:last-child {
  background: #acbeb8;
}
.break-screen {
  height: 560px;
}
.break-preview-stage :deep(.break-content) {
  width: 800px;
  min-height: 560px;
  max-height: 560px;
  padding: 32px;
  gap: 32px;
  grid-template-columns: minmax(0, 1.2fr) minmax(0, 1fr);
  grid-template-rows: auto 1fr auto;
}
.break-preview-stage :deep(.break-content > .icon) {
  width: 64px;
  height: 64px;
}
.break-preview-stage :deep(h1) {
  font-size: 44px;
}
.break-preview-stage :deep(.countdown) {
  font-size: 96px;
}
.break-preview-stage :deep(.break-timer) {
  grid-column: 2;
  grid-row: 2;
  margin-block: 0;
}
.break-preview-stage :deep(.break-skip) {
  grid-column: 2;
}
</style>
