import { computed, onUnmounted, ref, watch } from 'vue'
import { useMonitor } from '~/pages/main/composables/useMonitor'
import { defaultPet, parsePetPack, petState } from './model'
import { t } from '~/i18n'

export function usePet() {
  const { settings, snapshot, report } = useMonitor()
  const loaded = computed(() => {
    try {
      return {
        pack: settings.value.petPack ? parsePetPack(settings.value.petPack) : defaultPet,
        error: '',
      }
    } catch {
      return { pack: defaultPet, error: '宠物素材无效' }
    }
  })
  watch(
    () => loaded.value.error,
    (error) => {
      if (error) report(error)
    },
    { immediate: true },
  )
  const pack = computed(() => loaded.value.pack)
  const energy = computed(() => Math.max(0, Math.min(100, 100 - snapshot.value.fatigue)))
  const resting = computed(() => snapshot.value.breakUntil > snapshot.value.now)
  const celebrate = ref(false)
  let timer: ReturnType<typeof setTimeout> | undefined
  watch(
    () => Object.values(snapshot.value.days).reduce((sum, day) => sum + day.breaks, 0),
    (count, previous) => {
      if (count > previous) {
        celebrate.value = true
        clearTimeout(timer)
        timer = setTimeout(() => {
          celebrate.value = false
        }, 4000)
      }
    },
  )
  onUnmounted(() => clearTimeout(timer))
  const state = computed(() =>
    petState(energy.value, resting.value, settings.value.petTiredThreshold, celebrate.value),
  )
  const labels = computed(() => ({
    ready: t('元气满满'),
    working: t('陪你专注'),
    tired: t('该充充电了'),
    resting: t('慢慢恢复能量'),
    celebrate: t('休息完成，欢迎回来'),
  }))
  return { pack, energy, state, labels }
}
