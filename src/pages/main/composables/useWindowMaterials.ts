import { invoke } from '@tauri-apps/api/core'
import { nextTick, onMounted, onUnmounted, watch } from 'vue'
import type { Monitor } from './useMonitor'

export function useWindowMaterials({ native, mode, page, loading, settings }: Monitor) {
  if (!native || mode !== 'main') return
  let frame = 0
  let disposed = false
  let inFlight = false
  let pending = false
  let last = ''
  const root = document.documentElement
  const resize = new ResizeObserver(schedule)
  const appearance = new MutationObserver(schedule)

  function schedule() {
    cancelAnimationFrame(frame)
    frame = requestAnimationFrame(update)
  }
  async function update() {
    if (disposed) return
    if (inFlight) {
      pending = true
      return
    }
    const elements = [
      document.querySelector('.sidebar'),
      document.querySelector('.today-toolbar, .page-heading'),
    ]
    const regions = elements
      .filter((element): element is Element => Boolean(element))
      .map((element) => {
        const rect = element.getBoundingClientRect()
        const top = Math.max(0, rect.top)
        const bottom = Math.min(innerHeight, rect.bottom)
        return {
          x: rect.left,
          y: top,
          width: rect.width,
          height: Math.max(0, bottom - top),
          radius: Number.parseFloat(getComputedStyle(element).borderTopLeftRadius) || 0,
        }
      })
    const args = {
      regions,
      dark: root.dataset.theme === 'dark',
      followSystem: settings.value.mainTheme === 'system',
      viewport: { width: innerWidth, height: innerHeight },
    }
    const signature = JSON.stringify(args)
    if (signature === last) return
    inFlight = true
    try {
      const material = await invoke<string>('update_materials', args)
      if (!disposed) {
        root.dataset.material = material
        last = signature
      }
    } catch (error) {
      root.dataset.material = 'solid'
      console.error('Unable to update native window materials', error)
    } finally {
      inFlight = false
      if (pending && !disposed) {
        pending = false
        schedule()
      }
    }
  }
  async function observeLayout() {
    await nextTick()
    if (disposed) return
    resize.disconnect()
    document
      .querySelectorAll('.app-shell, .sidebar, .today-toolbar, .page-heading')
      .forEach((element) => resize.observe(element))
    schedule()
  }
  function refresh() {
    last = ''
    schedule()
  }
  onMounted(() => {
    observeLayout()
    appearance.observe(root, { attributes: true, attributeFilter: ['data-theme', 'dir'] })
    window.addEventListener('resize', schedule)
    window.addEventListener('focus', refresh)
    document.addEventListener('scroll', schedule, true)
    document.addEventListener('visibilitychange', refresh)
  })
  watch([page, loading], observeLayout, { flush: 'post' })
  watch(() => settings.value.mainTheme, schedule, { flush: 'post' })
  onUnmounted(() => {
    disposed = true
    cancelAnimationFrame(frame)
    resize.disconnect()
    appearance.disconnect()
    window.removeEventListener('resize', schedule)
    window.removeEventListener('focus', refresh)
    document.removeEventListener('scroll', schedule, true)
    document.removeEventListener('visibilitychange', refresh)
    delete root.dataset.material
  })
}
