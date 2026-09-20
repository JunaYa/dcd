import { computed } from 'vue'
import { t } from '~/i18n'
import { useMonitor } from './useMonitor'

export function useBreakPresentation() {
  const { settings, remaining } = useMonitor()
  const countdown = computed(
    () =>
      `${String(Math.floor(remaining.value / 60)).padStart(2, '0')}:${String(remaining.value % 60).padStart(2, '0')}`,
  )
  const overlayStyle = computed(() => ({
    '--overlay-opacity': String(settings.value.overlayOpacity / 100),
    '--overlay-blur': `${settings.value.overlayBlur}px`,
  }))
  const breakMessage = computed(() =>
    settings.value.message && settings.value.message !== 'Take a break'
      ? settings.value.message
      : t('休息一下'),
  )
  const backgroundStyle = computed(() =>
    settings.value.background === 'custom' && settings.value.backgroundImage
      ? {
          backgroundImage: `linear-gradient(#0005, #0005), url("${settings.value.backgroundImage}")`,
        }
      : {},
  )
  return { countdown, overlayStyle, breakMessage, backgroundStyle }
}
