<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { computed } from 'vue'
import Icon from '~/components/eye/Icon.vue'
const { page, paused } = useMonitor()
const navigation = computed(() => [
  { id: 'today', label: t('今日'), icon: 'sun' },
  { id: 'analysis', label: t('分析'), icon: 'chart' },
  { id: 'rules', label: t('规则'), icon: 'clock' },
  { id: 'appearance', label: t('外观'), icon: 'palette' },
  { id: 'settings', label: t('设置'), icon: 'settings' },
])
</script>

<template>
  <aside class="sidebar">
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
