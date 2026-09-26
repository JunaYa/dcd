<script setup lang="ts">
import PixelPet from '~/pet/PixelPet.vue'
import { usePet } from '~/pet/usePet'
import { t } from '~/i18n'
import { useMonitor } from '../composables/useMonitor'
import { useBreakPresentation } from '../composables/useBreakPresentation'
defineProps<{ timer: string; caption: string; preview?: boolean }>()
const { settings, action } = useMonitor()
const { pack, energy } = usePet()
const { breakMessage } = useBreakPresentation()
</script>
<template>
  <div
    class="break-content"
    :class="{ 'with-pet': settings.petEnabled && settings.petShowOnBreak }"
  >
    <div v-if="settings.petEnabled && settings.petShowOnBreak" class="break-pet">
      <PixelPet :pack="pack" state="resting" :size="72" />
      <div v-if="settings.petShowEnergy" class="break-pet-energy">
        <meter min="0" max="100" :value="energy" :aria-label="t('能量')" />
        <span>{{ energy }}%</span>
      </div>
    </div>
    <div class="break-copy">
      <h1>{{ breakMessage }}</h1>
      <p>{{ t('移开视线，看看远处，让双眼放松。') }}</p>
    </div>
    <div class="break-timer">
      <div class="countdown">{{ timer }}</div>
      <span class="break-caption">{{ caption }}</span>
    </div>
    <component
      :is="preview ? 'span' : 'button'"
      v-if="settings.allowSkip"
      class="button break-skip"
      :type="preview ? undefined : 'button'"
      @click="!preview && action('skip')"
    >
      {{ t('跳过本次休息') }}
    </component>
  </div>
</template>

<style scoped>
.break-pet {
  display: grid;
  justify-items: center;
  gap: var(--space-2);
}
.break-pet-energy {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--color-text-muted);
  font-size: var(--text-small);
  font-variant-numeric: tabular-nums;
}
.break-pet-energy meter {
  width: 64px;
  height: 4px;
  overflow: hidden;
  border: 0;
  border-radius: var(--radius-pill);
  background: var(--color-border-subtle);
}
.break-pet-energy meter::-webkit-meter-bar {
  background: var(--color-border-subtle);
  border: 0;
  height: 4px;
}
.break-pet-energy meter::-webkit-meter-optimum-value {
  background: var(--color-accent);
}
.break-pet-energy meter::-moz-meter-bar {
  background: var(--color-accent);
}
.break-content.with-pet {
  gap: clamp(20px, 3vh, 32px);
}
@media (max-height: 640px) {
  .break-content.with-pet {
    padding-block: 20px;
    gap: 16px;
  }
}
</style>
