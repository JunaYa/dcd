import { invoke, isTauri } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { computed, inject, onMounted, onUnmounted, provide, ref, type InjectionKey } from 'vue'
import { defaults, emptySnapshot, type Settings, type Snapshot } from '~/eye/model'
import { t } from '~/i18n'
import { applySnapshot, sameSettings } from './snapshot'
const monitorKey: InjectionKey<Monitor> = Symbol('monitor')
export type Monitor = ReturnType<typeof createMonitor>
export function provideMonitor(initialSnapshot?: Snapshot) {
  const monitor = createMonitor(initialSnapshot)
  provide(monitorKey, monitor)
  return monitor
}
export function useMonitor() {
  const monitor = inject(monitorKey)
  if (!monitor) throw new Error('Monitor must be provided by the window root')
  return monitor
}
function createMonitor(initialSnapshot?: Snapshot) {
  const native = isTauri()
  const mode = new URLSearchParams(location.search).get('mode') || 'main'
  const page = ref('today')
  const browserBreak = ref(false)
  const snapshot = ref<Snapshot>(initialSnapshot ?? emptySnapshot())
  const settings = ref<Settings>({ ...defaults, ...initialSnapshot?.settings })
  const baseline = ref(JSON.stringify(settings.value))
  const dirty = computed(() => JSON.stringify(settings.value) !== baseline.value)
  const loading = ref(native && !initialSnapshot)
  const saving = ref(false)
  const error = ref('')
  const toast = ref('')
  const paused = computed(() => snapshot.value.pausedUntil > snapshot.value.now)
  const remaining = computed(() =>
    Math.max(0, Math.ceil(snapshot.value.breakUntil - snapshot.value.now)),
  )
  const unlisteners: UnlistenFn[] = []
  let interval: ReturnType<typeof setInterval> | undefined
  let toastTimer: ReturnType<typeof setTimeout> | undefined
  let fetching = false
  let settingsGeneration = 0
  let disposed = false

  function inform(message: string) {
    if (disposed) return
    toast.value = message
    clearTimeout(toastTimer)
    toastTimer = setTimeout(() => {
      toast.value = ''
    }, 3500)
  }
  function report(cause: unknown) {
    if (disposed) return
    error.value = t(String(cause))
  }
  async function refresh() {
    if (!native || fetching || disposed) return
    fetching = true
    const generation = settingsGeneration
    try {
      const data = await invoke<Snapshot>('eye_snapshot')
      if (disposed) return
      applySnapshot(snapshot.value, data)
      if (
        !dirty.value &&
        !saving.value &&
        generation === settingsGeneration &&
        !sameSettings(settings.value, data.settings)
      ) {
        settings.value = { ...data.settings }
        baseline.value = JSON.stringify(data.settings)
      }
      loading.value = false
    } catch (cause) {
      report(cause)
    } finally {
      fetching = false
    }
  }
  function validate() {
    const s = settings.value
    if (
      ![s.workMinutes, s.breakMinutes, s.breakSeconds, s.repeatMinutes].every(Number.isInteger) ||
      s.workMinutes < 1 ||
      s.workMinutes > 240 ||
      s.breakMinutes < 0 ||
      s.breakMinutes > 60 ||
      s.breakSeconds < 0 ||
      s.breakSeconds > 59 ||
      s.breakMinutes * 60 + s.breakSeconds < 1 ||
      s.repeatMinutes < 1 ||
      s.repeatMinutes > 60
    ) {
      throw new Error(t('请填写有效时长：工作 1–240 分钟，休息至少 1 秒，提醒间隔 1–60 分钟。'))
    }
    if (!s.trayIcon && !s.dockIcon) throw new Error(t('请至少保留状态栏或程序坞入口。'))
  }
  async function saveSettings() {
    if (saving.value) return
    try {
      validate()
      saving.value = true
      settingsGeneration++
      error.value = ''
      const data = { ...settings.value }
      if (native) await invoke('eye_save_settings', { settings: data })
      else localStorage.setItem('eye-preview-settings', JSON.stringify(data))
      baseline.value = JSON.stringify(data)
      snapshot.value.settings = data
      inform(t('设置已保存'))
    } catch (cause) {
      report(cause)
    } finally {
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
    try {
      if (native) {
        await invoke('eye_action', { action: name, minutes: minutes ?? null })
        await refresh()
      } else {
        const now = Date.now() / 1000
        if (name === 'break') {
          snapshot.value.breakUntil =
            now + settings.value.breakMinutes * 60 + settings.value.breakSeconds
          snapshot.value.now = now
          browserBreak.value = true
        }
        if (name === 'skip') {
          snapshot.value.breakUntil = 0
          browserBreak.value = false
        }
        if (name === 'pause') {
          const tomorrow = new Date()
          tomorrow.setHours(24, 0, 0, 0)
          snapshot.value.pausedUntil =
            minutes === 0 ? tomorrow.getTime() / 1000 : now + (minutes || 30) * 60
        }
        if (name === 'resume') snapshot.value.pausedUntil = 0
        if (name === 'main' || name === 'settings') {
          if (mode === 'tray') {
            window.open(
              `${location.pathname}#${name === 'settings' ? 'settings' : 'today'}`,
              '_blank',
              'noopener',
            )
          } else {
            page.value = name === 'settings' ? 'settings' : 'today'
          }
        }
        if (name === 'quit') inform(t('桌面应用可通过此按钮退出；当前为浏览器预览'))
      }
      if (name === 'pause') inform(t('提醒已暂停，使用时长继续记录'))
      if (name === 'resume') inform(t('提醒已恢复'))
    } catch (cause) {
      report(cause)
    }
  }

  function tick() {
    if (document.hidden || disposed) return
    if (native) {
      void refresh()
    } else {
      snapshot.value.now = Date.now() / 1000
      if (snapshot.value.breakUntil && remaining.value === 0) {
        snapshot.value.breakUntil = 0
        browserBreak.value = false
        inform(t('休息完成，欢迎回来'))
      }
    }
  }
  function updateVisibility() {
    clearInterval(interval)
    if (!document.hidden && !disposed) {
      tick()
      interval = setInterval(tick, 1000)
    }
  }
  async function subscribe<T>(name: string, handler: (payload: T) => void) {
    const unlisten = await listen<T>(name, (event) => {
      if (!disposed) handler(event.payload)
    })
    if (disposed) unlisten()
    else unlisteners.push(unlisten)
  }
  onMounted(async () => {
    document.addEventListener('visibilitychange', updateVisibility)
    if (!document.hidden) interval = setInterval(tick, 1000)
    if (native) {
      try {
        await subscribe<string>('eye-navigate', (value) => {
          page.value = value
        })
        if (disposed) return
        await subscribe<string>('eye-warning', (value) => inform(t(value)))
        if (!initialSnapshot) await refresh()
      } catch (cause) {
        if (!disposed) report(cause)
      }
    } else {
      try {
        const saved = localStorage.getItem('eye-preview-settings')
        if (saved) {
          settings.value = { ...defaults, ...JSON.parse(saved) }
          baseline.value = JSON.stringify(settings.value)
        }
      } catch {
        report('浏览器预览设置无法读取，请重新保存。')
      }
      if (['today', 'analysis', 'rules', 'settings'].includes(location.hash.slice(1)))
        page.value = location.hash.slice(1)
    }
  })
  onUnmounted(() => {
    disposed = true
    clearInterval(interval)
    clearTimeout(toastTimer)
    unlisteners.forEach((fn) => fn())
    document.removeEventListener('visibilitychange', updateVisibility)
  })
  return {
    native,
    mode,
    page,
    snapshot,
    settings,
    dirty,
    loading,
    saving,
    error,
    toast,
    paused,
    remaining,
    browserBreak,
    inform,
    report,
    saveSettings,
    toggle,
    action,
  }
}
