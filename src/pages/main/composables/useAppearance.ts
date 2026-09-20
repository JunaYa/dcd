import { useMediaQuery } from '@vueuse/core'
import { computed, onUnmounted, watch, watchEffect } from 'vue'
import { locale, resolveLanguage, t } from '~/i18n'
import { accentTokens } from '~/theme/accent'
import type { Monitor } from './useMonitor'

export function useAppearance({ mode, settings }: Monitor) {
  const systemDark = useMediaQuery('(prefers-color-scheme: dark)')
  const activeTheme = computed(() => {
    const theme =
      mode === 'break'
        ? settings.value.background
        : mode === 'tray'
          ? settings.value.trayTheme
          : settings.value.mainTheme
    return theme === 'system'
      ? systemDark.value
        ? 'dark'
        : 'light'
      : theme === 'light'
        ? 'light'
        : 'dark'
  })
  watch(
    () => settings.value.language,
    (language) => {
      locale.value = resolveLanguage([language])
    },
    { immediate: true, flush: 'sync' },
  )
  watchEffect(() => {
    document.documentElement.lang = locale.value
    document.documentElement.dir = locale.value === 'ar' ? 'rtl' : 'ltr'
    document.title = t('appName')
  })
  watchEffect(() => {
    document.documentElement.style.setProperty(
      '--overlay-opacity',
      String(settings.value.overlayOpacity / 100),
    )
  })
  watchEffect(() => {
    const tokens = accentTokens(settings.value.accentColor, activeTheme.value === 'dark')
    for (const [name, value] of Object.entries(tokens)) {
      document.documentElement.style.setProperty(name, value)
    }
  })
  let frame = 0
  watch(
    activeTheme,
    (theme) => {
      const root = document.documentElement
      cancelAnimationFrame(frame)
      root.classList.add('no-transitions')
      root.dataset.theme = theme
      frame = requestAnimationFrame(() => {
        frame = requestAnimationFrame(() => root.classList.remove('no-transitions'))
      })
    },
    { immediate: true, flush: 'sync' },
  )
  onUnmounted(() => {
    cancelAnimationFrame(frame)
    document.documentElement.classList.remove('no-transitions')
  })
}
