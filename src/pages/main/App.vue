<script setup lang="ts">
import { invoke, isTauri } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { save as saveDialog } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { computed, nextTick, onMounted, onUnmounted, ref } from 'vue'
import Chart from '~/components/eye/Chart.vue'
import Icon from '~/components/eye/Icon.vue'
import Toggle from '~/components/eye/Toggle.vue'
import {
  aggregate,
  dateKey,
  defaults,
  demoSnapshot,
  duration,
  emptySnapshot,
  type Settings,
  type Snapshot,
} from '~/eye/model'

const native = isTauri()
const mode = new URLSearchParams(location.search).get('mode') || 'main'
const page = ref('today')
const navigation = [
  { id: 'today', label: '今日', icon: 'sun' },
  { id: 'analysis', label: '分析', icon: 'chart' },
  { id: 'rules', label: '规则', icon: 'clock' },
  { id: 'settings', label: '设置', icon: 'settings' },
]
const snapshot = ref<Snapshot>(emptySnapshot())
const settings = ref<Settings>({ ...defaults })
const baseline = ref(JSON.stringify(defaults))
const dirty = computed(() => JSON.stringify(settings.value) !== baseline.value)
const loading = ref(native)
const saving = ref(false)
const error = ref('')
const toast = ref('')
const demo = ref(false)
const demoData = demoSnapshot()
const display = computed(() => (demo.value ? demoData : snapshot.value))
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
  if (!previous)
    return null
  const delta = Math.round((today.value.seconds / previous - 1) * 100)
  return `${delta > 0 ? '+' : ''}${delta}%`
})
const range = ref(8)
const grouping = ref('day')
const ranges = [
  { value: 8, label: '过去7天' },
  { value: 29, label: '过去28天' },
  { value: 91, label: '过去90天' },
  { value: 365, label: '过去12个月' },
  { value: 0, label: '全部时间' },
]
const groups = [
  { value: 'day', label: '按天' },
  { value: 'week', label: '按周' },
  { value: 'month', label: '按月' },
]
const bars = computed(() =>
  aggregate(display.value.days, range.value, grouping.value),
)
const pauseOpen = ref(false)
const paused = computed(() => snapshot.value.pausedUntil > snapshot.value.now)
const pauseLabel = computed(() =>
  paused.value
    ? `提醒已暂停至 ${new Date(snapshot.value.pausedUntil * 1000).toLocaleString('zh-CN', { month: 'numeric', day: 'numeric', hour: '2-digit', minute: '2-digit' })}`
    : '',
)
const remaining = computed(() =>
  Math.max(0, Math.ceil(snapshot.value.breakUntil - snapshot.value.now)),
)
const countdown = computed(
  () =>
    `${String(Math.floor(remaining.value / 60)).padStart(2, '0')}:${String(remaining.value % 60).padStart(2, '0')}`,
)
const dialog = ref<HTMLDialogElement>()
const dialogKind = ref<'help' | 'break'>('help')
const dialogOpener = ref<HTMLElement | null>(null)
const backgroundStyle = computed(() =>
  settings.value.background === 'custom' && settings.value.backgroundImage
    ? {
        backgroundImage: `linear-gradient(#0005, #0005), url("${settings.value.backgroundImage}")`,
      }
    : {},
)
const unlisteners: UnlistenFn[] = []
let interval: ReturnType<typeof setInterval> | undefined
let toastTimer: ReturnType<typeof setTimeout> | undefined
let fetching = false
let settingsGeneration = 0
let disposed = false

function inform(message: string) {
  toast.value = message
  clearTimeout(toastTimer)
  toastTimer = setTimeout(() => {
    toast.value = ''
  }, 3500)
}
function report(cause: unknown) {
  error.value = String(cause)
}
async function refresh() {
  if (!native || fetching)
    return
  fetching = true
  const generation = settingsGeneration
  try {
    const data = await invoke<Snapshot>('eye_snapshot')
    snapshot.value = data
    if (!dirty.value && !saving.value && generation === settingsGeneration) {
      settings.value = { ...data.settings }
      baseline.value = JSON.stringify(data.settings)
    }
    loading.value = false
  }
  catch (cause) {
    report(cause)
  }
  finally {
    fetching = false
  }
}
function validate() {
  const s = settings.value
  if (
    ![s.workMinutes, s.breakMinutes, s.breakSeconds, s.repeatMinutes].every(
      Number.isInteger,
    )
    || s.workMinutes < 1
    || s.workMinutes > 240
    || s.breakMinutes < 0
    || s.breakMinutes > 60
    || s.breakSeconds < 0
    || s.breakSeconds > 59
    || s.breakMinutes * 60 + s.breakSeconds < 1
    || s.repeatMinutes < 1
    || s.repeatMinutes > 60
  ) {
    throw new Error(
      '请填写有效时长：工作 1–240 分钟，休息至少 1 秒，提醒间隔 1–60 分钟。',
    )
  }
  if (!s.trayIcon && !s.dockIcon)
    throw new Error('请至少保留状态栏或程序坞入口。')
}
async function saveSettings() {
  if (saving.value)
    return
  try {
    validate()
    saving.value = true
    settingsGeneration++
    error.value = ''
    const data = { ...settings.value }
    if (native)
      await invoke('eye_save_settings', { settings: data })
    else localStorage.setItem('eye-preview-settings', JSON.stringify(data))
    baseline.value = JSON.stringify(data)
    snapshot.value.settings = data
    inform('设置已保存')
  }
  catch (cause) {
    report(cause)
  }
  finally {
    saving.value = false
    settingsGeneration++
  }
}
async function toggle(key: keyof Settings, value: boolean) {
  settings.value = { ...settings.value, [key]: value }
  await saveSettings()
}
async function action(name: string, minutes?: number) {
  error.value = ''
  pauseOpen.value = false
  try {
    if (native) {
      await invoke('eye_action', { action: name, minutes: minutes ?? null })
      await refresh()
    }
    else {
      const now = Date.now() / 1000
      if (name === 'break') {
        snapshot.value.breakUntil
          = now + settings.value.breakMinutes * 60 + settings.value.breakSeconds
        snapshot.value.now = now
        openDialog('break')
      }
      if (name === 'skip') {
        snapshot.value.breakUntil = 0
        closeDialog()
      }
      if (name === 'pause') {
        const tomorrow = new Date()
        tomorrow.setHours(24, 0, 0, 0)
        snapshot.value.pausedUntil
          = minutes === 0
            ? tomorrow.getTime() / 1000
            : now + (minutes || 30) * 60
      }
      if (name === 'resume')
        snapshot.value.pausedUntil = 0
      if (name === 'main' || name === 'settings') {
        if (mode === 'tray') {
          window.open(
            `${location.pathname}#${name === 'settings' ? 'settings' : 'today'}`,
            '_blank',
            'noopener',
          )
        }
        else {
          page.value = name === 'settings' ? 'settings' : 'today'
        }
      }
      if (name === 'quit')
        inform('桌面应用可通过此按钮退出；当前为浏览器预览')
    }
    if (name === 'pause')
      inform('提醒已暂停，使用时长继续记录')
    if (name === 'resume')
      inform('提醒已恢复')
  }
  catch (cause) {
    report(cause)
  }
}
function openDialog(kind: 'help' | 'break') {
  dialogOpener.value = document.activeElement as HTMLElement
  dialogKind.value = kind
  nextTick(() => dialog.value?.showModal())
}
function closeDialog() {
  dialog.value?.close()
  dialogOpener.value?.focus()
}
function cancelDialog(event: Event) {
  if (dialogKind.value === 'break') {
    event.preventDefault()
    if (settings.value.allowSkip)
      action('skip')
  }
}
async function windowAction(name: 'close' | 'minimize' | 'toggleMaximize') {
  if (!native) {
    inform('窗口控制仅在桌面应用中可用')
    return
  }
  try {
    await getCurrentWindow()[name]()
  }
  catch (cause) {
    report(cause)
  }
}
async function share() {
  const text = `Eye Monitor · 今日使用 ${duration(today.value.seconds)}，疲劳值 ${display.value.fatigue}%。记得让双眼休息一下。`
  try {
    if (native)
      await writeText(text)
    else await navigator.clipboard.writeText(text)
    inform('今日摘要已复制，可以粘贴分享')
  }
  catch (cause) {
    report(cause)
  }
}
async function exportCsv() {
  const rows = [
    ['日期', '使用时长（分钟）', '疲劳峰值时长（分钟）', '完成休息次数'],
    ...bars.value.map(b => [
      b.key,
      (b.seconds / 60).toFixed(1),
      (b.peakSeconds / 60).toFixed(1),
      b.breaks,
    ]),
  ]
  const csv = `\uFEFF${rows.map(row => row.join(',')).join('\r\n')}`
  const filename = `eye-monitor-${demo.value ? '示例-' : ''}${dateKey()}.csv`
  try {
    if (native) {
      const path = await saveDialog({
        defaultPath: filename,
        filters: [{ name: 'CSV', extensions: ['csv'] }],
      })
      if (!path)
        return
      await writeTextFile(path, csv)
    }
    else {
      const url = URL.createObjectURL(
        new Blob([csv], { type: 'text/csv;charset=utf-8' }),
      )
      const link = document.createElement('a')
      link.href = url
      link.download = filename
      link.click()
      setTimeout(() => URL.revokeObjectURL(url), 1000)
    }
    inform('已导出当前筛选范围的 CSV')
  }
  catch (cause) {
    report(cause)
  }
}
async function upload(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file)
    return
  try {
    if (
      !['image/png', 'image/jpeg', 'image/webp'].includes(file.type)
      || file.size > 3 * 1024 * 1024
    ) {
      throw new Error('请选择不超过 3 MB 的 PNG、JPEG 或 WebP 图片。')
    }
    const data = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader()
      reader.onload = () => resolve(String(reader.result))
      reader.onerror = () => reject(new Error('无法读取图片'))
      reader.readAsDataURL(file)
    })
    settings.value.backgroundImage = data
    settings.value.background = 'custom'
    await saveSettings()
  }
  catch (cause) {
    report(cause)
  }
  finally {
    input.value = ''
  }
}
function keydown(event: KeyboardEvent) {
  if (event.key === 'Escape')
    pauseOpen.value = false
}
function clickOutside(event: MouseEvent) {
  if (!(event.target as HTMLElement).closest('.pause-control'))
    pauseOpen.value = false
}
onMounted(async () => {
  document.addEventListener('keydown', keydown)
  document.addEventListener('click', clickOutside)
  if (native) {
    await refresh()
    const nav = await listen<string>('eye-navigate', (event) => {
      page.value = event.payload
    })
    const warn = await listen<string>('eye-warning', event =>
      inform(event.payload))
    if (disposed) {
      nav()
      warn()
      return
    }
    unlisteners.push(nav, warn)
  }
  else {
    try {
      const saved = localStorage.getItem('eye-preview-settings')
      if (saved) {
        settings.value = { ...defaults, ...JSON.parse(saved) }
        baseline.value = JSON.stringify(settings.value)
      }
    }
    catch {
      report('浏览器预览设置无法读取，请重新保存。')
    }
    if (navigation.some(item => item.id === location.hash.slice(1)))
      page.value = location.hash.slice(1)
  }
  interval = setInterval(() => {
    if (native) {
      refresh()
    }
    else {
      snapshot.value.now = Date.now() / 1000
      if (snapshot.value.breakUntil && remaining.value === 0) {
        snapshot.value.breakUntil = 0
        closeDialog()
        inform('休息完成，欢迎回来')
      }
    }
  }, 1000)
})
onUnmounted(() => {
  disposed = true
  clearInterval(interval)
  clearTimeout(toastTimer)
  unlisteners.forEach(fn => fn())
  document.removeEventListener('keydown', keydown)
  document.removeEventListener('click', clickOutside)
})
</script>

<template>
  <div
    v-if="mode === 'break'"
    class="break-screen"
    :class="[
      `background-${settings.background}`,
      { 'break-window': settings.reminderStyle !== 'fullscreen' },
    ]"
    :style="backgroundStyle"
  >
    <Icon name="eye" />
    <h1>{{ settings.message || "休息一下" }}</h1>
    <p>移开视线，看看远处，让双眼放松。</p>
    <div class="countdown">
      {{ countdown }}
    </div>
    <span class="break-caption">{{
      remaining > 0 ? "休息倒计时" : "休息已完成"
    }}</span>
    <button
      v-if="settings.allowSkip"
      class="button break-skip"
      @click="action('skip')"
    >
      跳过本次休息
    </button>
  </div>
  <div v-else class="app-shell" :class="[{ 'tray-app': mode === 'tray' }]">
    <aside v-if="mode !== 'tray'" class="sidebar">
      <div class="window-drag" data-tauri-drag-region>
        <div class="traffic-lights">
          <button
            class="red"
            aria-label="关闭窗口"
            @click="windowAction('close')"
          /><button
            class="yellow"
            aria-label="最小化"
            @click="windowAction('minimize')"
          /><button
            class="green"
            aria-label="切换最大化"
            @click="windowAction('toggleMaximize')"
          />
        </div>
      </div>
      <nav aria-label="主导航">
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
        <span class="status-dot" :class="{ paused }" />{{
          paused ? "提醒已暂停" : "Eye Monitor"
        }}
      </div>
    </aside>
    <main
      class="main-content"
      :class="[{ 'analysis-page': page === 'analysis' }]"
    >
      <div v-if="mode !== 'tray'" class="title-drag" data-tauri-drag-region />
      <div v-if="!native && mode !== 'tray'" class="preview-note">
        浏览器预览 · 系统计时在桌面应用中运行
      </div>
      <div
        v-if="error || snapshot.storageError"
        class="error-banner"
        role="alert"
      >
        <span>{{ error || `数据保存失败：${snapshot.storageError}` }}</span><button
          aria-label="关闭错误提示"
          @click="
            error = '';
            snapshot.storageError = null;
          "
        >
          <Icon name="close" />
        </button>
      </div>
      <div v-if="loading" class="loading-state" role="status">
        正在读取本机记录…
      </div>
      <template v-else-if="page === 'today' || mode === 'tray'">
        <header v-if="mode !== 'tray'" class="today-toolbar">
          <div class="page-heading">
            <div>
              <h1>今日</h1>
              <p>留意用眼节奏，给双眼一点休息。</p>
            </div>
          </div>
          <button
            class="demo-button"
            :class="{ active: demo }"
            :aria-pressed="demo"
            @click="demo = !demo"
          >
            {{ demo ? "示例数据 · 返回实时" : "预览示例数据" }}
          </button>
          <button
            class="button primary break-action"
            title="手动开始一次休息"
            aria-label="手动开始一次休息"
            @click="action('break')"
          >
            <Icon name="eye" /><span>休息一下</span>
          </button>
          <div class="pause-control">
            <button
              class="icon-button"
              :class="{ active: paused }"
              title="暂停提醒"
              aria-label="暂停提醒"
              :aria-expanded="pauseOpen"
              @click.stop="pauseOpen = !pauseOpen"
            >
              <Icon name="moon" />
            </button>
            <div v-if="pauseOpen" class="pause-menu">
              <strong>暂停通知</strong><button @click="action('pause', 30)">
                30分钟
              </button><button @click="action('pause', 60)">
                1小时
              </button><button @click="action('pause', 0)">
                直到明天
              </button><button @click="action('resume')">
                取消暂停
              </button>
            </div>
          </div>
          <button
            class="icon-button"
            title="使用说明"
            aria-label="使用说明"
            @click="openDialog('help')"
          >
            <Icon name="help" />
          </button>
        </header>
        <div class="overview" :class="[{ 'tray-overview': mode === 'tray' }]">
          <div class="stat-cards">
            <section class="stat-card fatigue-card">
              <h2>疲劳值</h2>
              <strong>{{ display.fatigue }}%</strong>
            </section>
            <section class="stat-card duration-card">
              <h2>今日时长</h2>
              <div>
                <strong>{{ duration(today.seconds) }}</strong><span v-if="change" class="badge" title="与昨日全天相比">{{
                  change
                }}</span>
              </div>
            </section>
          </div>
          <div v-if="mode === 'tray'" class="tray-actions">
            <button @click="action('break')">
              <Icon name="eye" />手动开始一次休息
            </button>
            <div class="pause-control">
              <button
                :class="{ active: pauseOpen }"
                :aria-expanded="pauseOpen"
                @click.stop="pauseOpen = !pauseOpen"
              >
                <Icon name="moon" />{{ paused ? "提醒已暂停" : "暂停提醒" }}
              </button>
              <div v-if="pauseOpen" class="pause-menu">
                <strong>暂停通知</strong><button @click="action('pause', 30)">
                  30分钟
                </button><button @click="action('pause', 60)">
                  1小时
                </button><button @click="action('pause', 0)">
                  直到明天
                </button><button @click="action('resume')">
                  取消暂停
                </button>
              </div>
            </div>
            <button @click="action('main')">
              <Icon name="window" />打开主窗口
            </button><button @click="action('settings')">
              <Icon name="settings" />设置
            </button><button @click="share">
              <Icon name="share" />分享 Eye Monitor
            </button><button @click="action('quit')">
              <Icon name="power" />退出
            </button>
          </div>
        </div>
        <p v-if="paused" class="pause-status">
          {{ pauseLabel }}<button @click="action('resume')">
            恢复提醒
          </button>
        </p>
        <section class="today-chart">
          <h1>今日使用 <span v-if="demo" class="sample-label">示例</span></h1>
          <Chart :samples="display.samples" :compact="mode === 'tray'" />
        </section>
        <div v-if="mode !== 'tray'" class="today-footer">
          <span>{{
            demo
              ? "示例仅用于预览，不会写入使用记录"
              : "本机记录 · 每分钟更新曲线"
          }}</span><span>已完成 {{ today.breaks }} 次休息</span>
        </div>
      </template>
      <template v-else-if="page === 'analysis'">
        <header class="page-heading">
          <div>
            <h1>使用分析</h1>
            <p>回顾使用时长与疲劳趋势。</p>
          </div>
        </header>
        <div class="analysis-controls">
          <div class="filter-row">
            <Icon name="calendar" />
            <div class="segmented" aria-label="统计范围">
              <button
                v-for="item in ranges"
                :key="item.value"
                :class="{ selected: range === item.value }"
                :aria-pressed="range === item.value"
                @click="range = item.value"
              >
                {{ item.label }}
              </button>
            </div>
          </div>
          <div class="filter-row">
            <Icon name="list" />
            <div class="segmented" aria-label="分组方式">
              <button
                v-for="item in groups"
                :key="item.value"
                :class="{ selected: grouping === item.value }"
                :aria-pressed="grouping === item.value"
                @click="grouping = item.value"
              >
                {{ item.label }}
              </button>
            </div>
            <button
              class="button export-button"
              title="导出 CSV"
              aria-label="导出 CSV"
              @click="exportCsv"
            >
              <Icon name="download" />
            </button>
          </div>
        </div>
        <div class="analysis-meta">
          <span>{{ demo ? "正在查看示例数据" : "使用记录仅保存在本机" }}</span><button
            class="demo-button"
            :aria-pressed="demo"
            @click="demo = !demo"
          >
            {{ demo ? "返回实时数据" : "预览示例数据" }}
          </button>
        </div>
        <section class="analysis-chart">
          <h1>
            使用时长
            <Icon name="info" title="当前筛选范围内的实际电脑使用时长" />
          </h1>
          <Chart :bars="bars" />
        </section>
        <section class="analysis-chart">
          <h1>
            疲劳峰值时长
            <Icon name="info" title="疲劳值达到 100% 后仍在使用电脑的时长" />
          </h1>
          <Chart :bars="bars" metric="peakSeconds" />
        </section>
      </template>
      <form
        v-else-if="page === 'rules'"
        class="preferences rules-page"
        @submit.prevent="saveSettings"
      >
        <header class="page-heading">
          <div>
            <h1>休息规则</h1>
            <p>设定适合自己的工作与休息节奏。</p>
          </div>
        </header>
        <div class="setting-row rule-row">
          <label for="work">规则</label>
          <div class="rule-inputs">
            <span>工作</span><input
              id="work"
              v-model.number="settings.workMinutes"
              type="number"
              min="1"
              max="240"
              required
              aria-label="工作分钟"
            ><span>分钟后，休息</span><input
              v-model.number="settings.breakMinutes"
              type="number"
              min="0"
              max="60"
              required
              aria-label="休息分钟"
            ><span>分钟</span><input
              v-model.number="settings.breakSeconds"
              type="number"
              min="0"
              max="59"
              required
              aria-label="休息秒数"
            ><span>秒</span>
          </div>
        </div>
        <div class="setting-row rule-row">
          <label for="repeat">提醒间隔</label>
          <div class="rule-inputs">
            <span>若疲劳值一直处于100%，每隔</span><input
              id="repeat"
              v-model.number="settings.repeatMinutes"
              type="number"
              min="1"
              max="60"
              required
            ><span>分钟提醒一次</span>
          </div>
        </div>
        <div class="setting-row">
          <span>在弹窗出现前通知提醒</span><Toggle
            :model-value="settings.preNotify"
            label="在弹窗出现前通知提醒"
            :disabled="saving"
            @update:model-value="toggle('preNotify', $event)"
          />
        </div>
        <div class="setting-row">
          <span>休息结束时播放声音</span><Toggle
            :model-value="settings.sound"
            label="休息结束时播放声音"
            :disabled="saving"
            @update:model-value="toggle('sound', $event)"
          />
        </div>
        <div class="setting-row">
          <span>观影模式
            <Icon
              name="help"
              title="开启后，无键盘和鼠标操作时仍持续计时。关闭时，空闲一分钟后暂停累计并恢复疲劳值。"
            /></span><Toggle
            :model-value="settings.movieMode"
            label="观影模式"
            :disabled="saving"
            @update:model-value="toggle('movieMode', $event)"
          />
        </div>
        <div class="setting-row contact-row">
          <span>关于应用</span>
          <div>
            Eye Monitor<span>工作有节奏，休息有提醒</span><small>疲劳值按连续使用时长估算</small>
          </div>
        </div>
        <div v-if="dirty" class="save-row">
          <span>有未保存的更改</span><button class="button primary" type="submit" :disabled="saving">
            {{ saving ? "保存中…" : "保存规则" }}
          </button>
        </div>
      </form>
      <form
        v-else-if="page === 'settings'"
        class="preferences"
        @submit.prevent="saveSettings"
      >
        <header class="page-heading">
          <div>
            <h1>设置</h1>
            <p>调整应用行为和提醒方式。</p>
          </div>
        </header>
        <section>
          <h2>系统集成</h2>
          <div class="setting-row">
            <span>开机时启动</span><Toggle
              :model-value="settings.autostart"
              label="开机时启动"
              :disabled="saving"
              @update:model-value="toggle('autostart', $event)"
            />
          </div>
          <div class="setting-row">
            <span>显示状态栏图标</span><Toggle
              :model-value="settings.trayIcon"
              label="显示状态栏图标"
              :disabled="saving"
              @update:model-value="toggle('trayIcon', $event)"
            />
          </div>
          <div class="setting-row">
            <span>在状态栏中显示今日时长</span><Toggle
              :model-value="settings.trayTime"
              label="在状态栏中显示今日时长"
              :disabled="saving"
              @update:model-value="toggle('trayTime', $event)"
            />
          </div>
          <div class="setting-row">
            <span>显示程序坞图标</span><Toggle
              :model-value="settings.dockIcon"
              label="显示程序坞图标"
              :disabled="saving"
              @update:model-value="toggle('dockIcon', $event)"
            />
          </div>
        </section>
        <section>
          <h2>休息提醒</h2>
          <div class="setting-row">
            <span>开启提醒</span><Toggle
              :model-value="settings.reminders"
              label="开启提醒"
              :disabled="saving"
              @update:model-value="toggle('reminders', $event)"
            />
          </div>
          <div class="setting-row">
            <label for="style">提醒方式</label><select
              id="style"
              v-model="settings.reminderStyle"
              @change="saveSettings"
            >
              <option value="fullscreen">
                桌面浮层
              </option>
              <option value="window">
                窗口提醒
              </option>
            </select>
          </div>
          <div class="setting-row">
            <label for="message">休息提醒内容</label><input
              id="message"
              v-model="settings.message"
              class="message-input"
              maxlength="120"
              placeholder="Take a break"
            >
          </div>
          <div class="setting-row">
            <span>在弹窗中展示跳过按钮
              <Icon
                name="help"
                title="关闭后需要完成倒计时才能结束休息"
              /></span><Toggle
              :model-value="settings.allowSkip"
              label="在弹窗中展示跳过按钮"
              :disabled="saving"
              @update:model-value="toggle('allowSkip', $event)"
            />
          </div>
          <div class="setting-row">
            <span>休息浮层背景</span>
          </div>
          <div class="background-options">
            <label
              v-for="option in [
                { id: 'system', label: '跟随系统' },
                { id: 'light', label: '浅色' },
                { id: 'dark', label: '深色' },
                { id: 'custom', label: '自定义' },
              ]"
              :key="option.id"
            ><input
              v-model="settings.background"
              type="radio"
              :value="option.id"
              name="background"
              @change="saveSettings"
            ><span
              class="swatch"
              :class="option.id"
              :style="
                option.id === 'custom' && settings.backgroundImage
                  ? { backgroundImage: `url(${settings.backgroundImage})` }
                  : {}
              "
            />{{ option.label }}</label><label class="button upload-button">上传<input
              type="file"
              accept="image/png,image/jpeg,image/webp"
              @change="upload"
            ></label><button
              class="button"
              type="button"
              @click="
                saveSettings().then(() => {
                  if (!dirty) action('break');
                })
              "
            >
              测试
            </button>
          </div>
        </section>
        <div v-if="dirty" class="save-row">
          <span>有未保存的更改</span><button class="button primary" type="submit" :disabled="saving">
            {{ saving ? "保存中…" : "保存设置" }}
          </button>
        </div>
      </form>
    </main>
  </div>
  <div v-if="toast" class="toast" role="status">
    <Icon name="check" />{{ toast }}
  </div>
  <dialog
    ref="dialog"
    :class="
      dialogKind === 'break'
        ? `break-dialog background-${settings.background}`
        : 'help-dialog'
    "
    :style="dialogKind === 'break' ? backgroundStyle : {}"
    @cancel="cancelDialog"
  >
    <template v-if="dialogKind === 'help'">
      <button
        class="dialog-close icon-button"
        aria-label="关闭说明"
        @click="closeDialog"
      >
        <Icon name="close" />
      </button><Icon name="eye" />
      <h2>给双眼一点休息时间</h2>
      <p>
        使用电脑时，疲劳值会按工作时长逐渐上升。达到 100% 时，应用会提醒你休息。
      </p>
      <p>
        默认工作 25 分钟，休息 5 分钟。空闲超过 1
        分钟后停止累计，疲劳值逐渐恢复；观影模式会持续计时。
      </p>
      <p>
        暂停提醒期间仍会记录使用时长。数据仅保存在本机，分析页面支持导出 CSV。
      </p>
      <button class="button primary" @click="closeDialog">
        知道了
      </button>
    </template>
    <template v-else>
      <Icon name="eye" />
      <h1>{{ settings.message || "休息一下" }}</h1>
      <p>移开视线，看看远处，让双眼放松。</p>
      <div class="countdown">
        {{ countdown }}
      </div>
      <p class="break-caption">
        浏览器休息预览
      </p>
      <button v-if="settings.allowSkip" class="button" @click="action('skip')">
        跳过本次休息
      </button>
    </template>
  </dialog>
</template>
