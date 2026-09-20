<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { dateKey } from '~/eye/model'
import { formatDuration as duration, locale } from '~/i18n'
import Chart from '~/components/eye/Chart.vue'
import Icon from '~/components/eye/Icon.vue'
const emit = defineEmits<{ help: [] }>()
const { mode, native, snapshot, settings, demo, display, paused, action, inform, report } =
  useMonitor()
const pauseOpen = ref(false)
async function runAction(name: string, minutes?: number) {
  pauseOpen.value = false
  await action(name, minutes)
}
const today = computed(
  () =>
    display.value.days[display.value.date] || {
      seconds: 0,
      peakSeconds: 0,
      breaks: 0,
    },
)
const change = computed(() => {
  const date = new Date(`${display.value.date}T12:00:00`)
  date.setDate(date.getDate() - 1)
  const previous = display.value.days[dateKey(date)]?.seconds || 0
  if (!previous) return null
  const delta = Math.round((today.value.seconds / previous - 1) * 100)
  return `${delta > 0 ? '+' : ''}${delta}%`
})
const pauseLabel = computed(() =>
  paused.value
    ? t('暂停至', {
        time: new Date(snapshot.value.pausedUntil * 1000).toLocaleString(locale.value, {
          month: 'numeric',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit',
        }),
      })
    : '',
)
async function share() {
  const text = t('摘要', {
    duration: duration(today.value.seconds),
    fatigue: display.value.fatigue,
  })
  try {
    if (native) await writeText(text)
    else await navigator.clipboard.writeText(text)
    inform(t('今日摘要已复制，可以粘贴分享'))
  } catch (cause) {
    report(cause)
  }
}
function keydown(event: KeyboardEvent) {
  if (event.key === 'Escape') pauseOpen.value = false
}
function clickOutside(event: MouseEvent) {
  if (!(event.target as HTMLElement).closest('.pause-control')) pauseOpen.value = false
}

onMounted(() => {
  document.addEventListener('keydown', keydown)
  document.addEventListener('click', clickOutside)
})
onUnmounted(() => {
  document.removeEventListener('keydown', keydown)
  document.removeEventListener('click', clickOutside)
})
</script>

<template>
  <header v-if="mode !== 'tray'" class="today-toolbar">
    <div class="page-heading">
      <div>
        <h1>{{ t('今日') }}</h1>
        <p>{{ t('留意用眼节奏，给双眼一点休息。') }}</p>
      </div>
    </div>
    <button
      class="demo-button"
      :class="{ active: demo }"
      :aria-pressed="demo"
      @click="demo = !demo"
    >
      {{ demo ? t('示例数据 · 返回实时') : t('预览示例数据') }}
    </button>
    <button
      class="button primary break-action"
      :title="t('手动开始一次休息')"
      :aria-label="t('手动开始一次休息')"
      @click="runAction('break')"
    >
      <Icon name="eye" /><span>{{ t('休息一下') }}</span>
    </button>
    <div class="pause-control">
      <button
        class="icon-button"
        :class="{ active: paused }"
        :title="t('暂停提醒说明')"
        :aria-label="t('暂停提醒')"
        :aria-expanded="pauseOpen"
        @click.stop="pauseOpen = !pauseOpen"
      >
        <Icon name="moon" />
      </button>
      <div v-if="pauseOpen" class="pause-menu">
        <strong>{{ t('暂停通知') }}</strong
        ><button @click="runAction('pause', 30)">
          {{ t('30分钟') }}</button
        ><button @click="runAction('pause', 60)">
          {{ t('1小时') }}</button
        ><button @click="runAction('pause', 0)">
          {{ t('直到明天') }}</button
        ><button @click="runAction('resume')">
          {{ t('取消暂停') }}
        </button>
      </div>
    </div>
    <button
      class="icon-button"
      :title="t('使用说明')"
      :aria-label="t('使用说明')"
      @click="emit('help')"
    >
      <Icon name="help" />
    </button>
  </header>
  <div class="overview" :class="[{ 'tray-overview': mode === 'tray' }]">
    <div class="stat-cards">
      <section class="stat-card fatigue-card">
        <h2>{{ t('疲劳值') }}</h2>
        <strong>{{ display.fatigue }}%</strong>
      </section>
      <section class="stat-card duration-card">
        <h2>{{ t('今日时长') }}</h2>
        <div>
          <strong>{{ duration(today.seconds) }}</strong
          ><span v-if="change" class="badge" :title="t('与昨日全天相比')">{{ change }}</span>
        </div>
      </section>
    </div>
    <div v-if="mode === 'tray'" class="tray-actions">
      <button @click="runAction('break')"><Icon name="eye" />{{ t('手动开始一次休息') }}</button>
      <div class="pause-control">
        <button
          :class="{ active: pauseOpen }"
          :aria-expanded="pauseOpen"
          @click.stop="pauseOpen = !pauseOpen"
        >
          <Icon name="moon" />{{ paused ? t('提醒已暂停') : t('暂停提醒') }}
        </button>
        <div v-if="pauseOpen" class="pause-menu">
          <strong>{{ t('暂停通知') }}</strong
          ><button @click="runAction('pause', 30)">
            {{ t('30分钟') }}</button
          ><button @click="runAction('pause', 60)">
            {{ t('1小时') }}</button
          ><button @click="runAction('pause', 0)">
            {{ t('直到明天') }}</button
          ><button @click="runAction('resume')">
            {{ t('取消暂停') }}
          </button>
        </div>
      </div>
      <button @click="runAction('main')"><Icon name="window" />{{ t('打开主窗口') }}</button
      ><button @click="runAction('settings')"><Icon name="settings" />{{ t('设置') }}</button
      ><button @click="share"><Icon name="share" />{{ t('分享 DCD') }}</button
      ><button @click="runAction('quit')"><Icon name="power" />{{ t('退出') }}</button>
    </div>
  </div>
  <p v-if="paused" class="pause-status">
    {{ pauseLabel
    }}<button @click="runAction('resume')">
      {{ t('恢复提醒') }}
    </button>
  </p>
  <section v-if="mode !== 'tray' || settings.trayShowChart" class="today-chart">
    <h1>
      {{ t('今日使用') }} <span v-if="demo" class="sample-label">{{ t('示例') }}</span>
    </h1>
    <Chart :samples="display.samples" :compact="mode === 'tray'" />
  </section>
  <div v-if="mode !== 'tray'" class="today-footer">
    <span>{{ demo ? t('示例仅用于预览，不会写入使用记录') : t('本机记录 · 每分钟更新曲线') }}</span
    ><span>{{ t('休息次数', { count: today.breaks }) }}</span>
  </div>
</template>
