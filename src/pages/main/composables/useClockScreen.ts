import { computed, onMounted, onUnmounted, ref, useTemplateRef } from 'vue'
import { useMonitor } from './useMonitor'

export function useClockScreen() {
  const { native, settings, action, error } = useMonitor()
  const now = ref(new Date())
  const closing = ref(false)
  const closeButton = useTemplateRef<HTMLButtonElement>('clockClose')
  const time = computed(() =>
    new Intl.DateTimeFormat(settings.value.language, {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hourCycle: 'h23',
    }).format(now.value),
  )
  const date = computed(() =>
    new Intl.DateTimeFormat(settings.value.language, {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      weekday: 'long',
    }).format(now.value),
  )
  let timer: ReturnType<typeof setInterval> | undefined
  function update() {
    now.value = new Date()
  }
  async function close() {
    if (closing.value) return
    closing.value = true
    try {
      await action('close-clock')
    } finally {
      closing.value = false
    }
  }
  function onKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && !event.repeat) {
      event.preventDefault()
      void close()
    }
  }
  onMounted(() => {
    timer = setInterval(update, 1000)
    document.addEventListener('visibilitychange', update)
    window.addEventListener('keydown', onKeydown)
    closeButton.value?.focus({ preventScroll: true })
  })
  onUnmounted(() => {
    clearInterval(timer)
    document.removeEventListener('visibilitychange', update)
    window.removeEventListener('keydown', onKeydown)
  })

  return { native, settings, now, time, date, error, closing, close }
}
