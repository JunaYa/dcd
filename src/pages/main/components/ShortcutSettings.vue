<script setup lang="ts">
import { computed } from 'vue'
import { defaults } from '~/eye/model'
import { t } from '~/i18n'
import { useMonitor } from '../composables/useMonitor'

const { native, settings, snapshot, saving, action } = useMonitor()
const shortcuts = computed(() => [
  { key: 'shortcutMain' as const, label: t('打开主窗口') },
  { key: 'shortcutBreak' as const, label: t('手动开始一次休息') },
  { key: 'shortcutSkip' as const, label: t('跳过本次休息') },
  { key: 'shortcutPause' as const, label: t('暂停或恢复提醒') },
  { key: 'shortcutClock' as const, label: t('打开或关闭时钟锁屏') },
])
function reset() {
  for (const { key } of shortcuts.value) settings.value[key] = defaults[key]
}
</script>

<template>
  <section aria-labelledby="shortcuts-heading">
    <h2 id="shortcuts-heading">{{ t('全局快捷键') }}</h2>
    <p id="shortcuts-help" class="section-description">{{ t('全局快捷键说明') }}</p>
    <p v-if="!native" class="section-description">{{ t('全局快捷键仅支持桌面应用') }}</p>
    <p v-if="snapshot.shortcutError" class="shortcut-error" role="alert">
      {{ snapshot.shortcutError }}
    </p>
    <div v-for="shortcut in shortcuts" :key="shortcut.key" class="setting-row shortcut-row">
      <label :for="shortcut.key">{{ shortcut.label }}</label>
      <input
        :id="shortcut.key"
        v-model.trim="settings[shortcut.key]"
        class="shortcut-input"
        type="text"
        :disabled="saving || !native"
        :placeholder="t('未设置')"
        :spellcheck="false"
        autocomplete="off"
        aria-describedby="shortcuts-help"
        maxlength="80"
      />
    </div>
    <button class="button" type="button" :disabled="saving || !native" @click="reset">
      {{ t('恢复默认快捷键') }}
    </button>
    <button class="button clock-launch" type="button" @click="action('toggle-clock')">
      {{ t('打开时钟锁屏') }}
    </button>
    <p class="section-description clock-description">{{ t('时钟锁屏说明') }}</p>
  </section>
</template>

<style scoped>
.shortcut-input {
  width: 240px;
  max-width: 100%;
  min-height: 40px;
  font-size: 16px;
}
.clock-launch {
  margin-inline-start: 8px;
}
.clock-description {
  margin-top: 12px;
}
.shortcut-error {
  color: var(--color-text);
  overflow-wrap: anywhere;
}
@media (max-width: 600px) {
  .shortcut-row {
    flex-wrap: wrap;
    gap: 8px;
    padding-block: 8px;
  }
}
</style>
