<script setup lang="ts">
import { ref } from 'vue'
import type { Snapshot } from '~/eye/model'
import { t } from '~/i18n'
import { provideMonitor } from './composables/useMonitor'
import { useWindowMaterials } from './composables/useWindowMaterials'
import { useAppearance } from './composables/useAppearance'
import TodayView from './views/TodayView.vue'
import TrayView from './views/TrayView.vue'
import AnalysisView from './views/AnalysisView.vue'
import RulesView from './views/RulesView.vue'
import AppearanceView from './views/AppearanceView.vue'
import SettingsView from './views/SettingsView.vue'
import BreakView from './views/BreakView.vue'
import AppSidebar from './components/AppSidebar.vue'
import AppDialogs from './components/AppDialogs.vue'
import AppError from './components/AppError.vue'
import AppToast from './components/AppToast.vue'
const props = defineProps<{ initialSnapshot?: Snapshot }>()
const monitor = provideMonitor(props.initialSnapshot)
const { mode, native, page, loading } = monitor
useAppearance(monitor)
useWindowMaterials(monitor)
const dialogs = ref<InstanceType<typeof AppDialogs>>()
const range = ref(8)
const grouping = ref('day')
</script>

<template>
  <BreakView v-if="mode === 'break'" />
  <div
    v-else
    class="app-shell"
    :class="{ 'tray-app': mode === 'tray', 'native-window': native && mode === 'main' }"
  >
    <AppSidebar v-if="mode !== 'tray'" />
    <main :key="page" class="main-content">
      <div v-if="!native && mode !== 'tray'" class="preview-note">
        {{ t('浏览器预览 · 系统计时在桌面应用中运行') }}
      </div>
      <AppError />
      <div v-if="loading" class="loading-state" role="status">{{ t('正在读取本机记录…') }}</div>
      <TrayView v-else-if="mode === 'tray'" />
      <TodayView v-else-if="page === 'today'" @help="dialogs?.openHelp()" />
      <AnalysisView
        v-else-if="page === 'analysis'"
        v-model:range="range"
        v-model:grouping="grouping"
      />
      <RulesView v-else-if="page === 'rules'" />
      <AppearanceView v-else-if="page === 'appearance'" />
      <SettingsView v-else-if="page === 'settings'" />
    </main>
  </div>
  <AppToast />
  <AppDialogs ref="dialogs" />
</template>
