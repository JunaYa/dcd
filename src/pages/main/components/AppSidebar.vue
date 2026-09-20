<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import Icon from '~/components/eye/Icon.vue'
const { native, page, paused, inform, report } = useMonitor()
const navigation = computed(() => [
  { id: 'today', label: t('今日'), icon: 'sun' },
  { id: 'analysis', label: t('分析'), icon: 'chart' },
  { id: 'rules', label: t('规则'), icon: 'clock' },
  { id: 'settings', label: t('设置'), icon: 'settings' },
])
async function windowAction(name: 'close' | 'minimize' | 'toggleMaximize') {
  if (!native) {
    inform(t('窗口控制仅在桌面应用中可用'))
    return
  }
  try {
    await getCurrentWindow()[name]()
  } catch (cause) {
    report(cause)
  }
}
</script>

<template>
  <aside class="sidebar">
    <div class="window-drag" data-tauri-drag-region>
      <div class="traffic-lights">
        <button class="red" :aria-label="t('关闭窗口')" @click="windowAction('close')" /><button
          class="yellow"
          :aria-label="t('最小化')"
          @click="windowAction('minimize')"
        /><button
          class="green"
          :aria-label="t('切换最大化')"
          @click="windowAction('toggleMaximize')"
        />
      </div>
    </div>
    <nav :aria-label="t('主导航')">
      <button
        v-for="item in navigation"
        :key="item.id"
        :aria-label="item.label"
        :class="{ selected: page === item.id }"
        :aria-current="page === item.id ? 'page' : undefined"
        @click="page = item.id"
      >
        <Icon :name="item.icon" /><span>{{ item.label }}</span>
      </button>
    </nav>
    <div class="sidebar-bottom">
      <span class="status-dot" :class="{ paused }" />{{ paused ? t('提醒已暂停') : t('appName') }}
    </div>
  </aside>
</template>
