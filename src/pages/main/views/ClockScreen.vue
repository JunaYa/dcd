<script setup lang="ts">
import { t } from '~/i18n'
import { useClockScreen } from '../composables/useClockScreen'

const { native, settings, now, time, date, error, closing, close } = useClockScreen()
</script>

<template>
  <main class="clock-screen" :aria-label="t('全屏时钟锁屏')">
    <div class="clock-face">
      <p class="clock-date">{{ date }}</p>
      <time class="clock-time" :datetime="now.toISOString()" dir="ltr">{{ time }}</time>
    </div>
    <footer class="clock-controls">
      <p v-if="error" class="clock-error" role="alert">{{ error }}</p>
      <button ref="clockClose" type="button" :disabled="closing" @click="close">
        {{ t('关闭锁屏') }} <kbd>Esc</kbd>
      </button>
      <p v-if="native && settings.shortcutClock" class="clock-shortcut">
        {{
          t('时钟快捷键提示', {
            shortcut: settings.shortcutClock.replace(/CmdOrCtrl/gi, 'Cmd/Ctrl'),
          })
        }}
      </p>
    </footer>
  </main>
</template>

<style scoped>
.clock-screen {
  position: fixed;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100dvh;
  padding: max(24px, env(safe-area-inset-top)) 24px max(24px, env(safe-area-inset-bottom));
  background: #111214;
  color: #f3f3f1;
  color-scheme: dark;
  overflow: auto;
}
.clock-face {
  display: grid;
  gap: 24px;
  text-align: center;
  margin-block: auto;
}
.clock-date {
  color: #aeafb4;
  font-size: clamp(16px, 2vw, 26px);
}
.clock-time {
  font-size: clamp(42px, 15vw, 240px);
  font-weight: 400;
  line-height: 1.1;
  letter-spacing: -0.035em;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}
.clock-controls {
  display: grid;
  justify-items: center;
  gap: 12px;
  padding-top: 24px;
  text-align: center;
}
.clock-controls button {
  min-height: 44px;
  padding: 10px 16px;
  border: 1px solid #44464b;
  border-radius: 8px;
  background: #222326;
  color: #f3f3f1;
  font-size: 16px;
  cursor: pointer;
}
.clock-controls button:focus-visible {
  outline: 2px solid #f3f3f1;
  outline-offset: 4px;
}
.clock-controls button:active {
  transform: scale(0.96);
}
.clock-controls kbd {
  margin-inline-start: 12px;
  color: #aeafb4;
  font: inherit;
  font-size: 13px;
}
.clock-shortcut {
  color: #aeafb4;
  font-size: 13px;
  overflow-wrap: anywhere;
}
.clock-error {
  max-width: 60ch;
  overflow-wrap: anywhere;
}
@media (hover: hover) {
  .clock-controls button:hover {
    background: #303136;
  }
}
</style>
