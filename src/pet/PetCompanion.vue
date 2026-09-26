<script setup lang="ts">
import { ref } from 'vue'
import PixelPet from './PixelPet.vue'
import { usePet } from './usePet'
import { useMonitor } from '~/pages/main/composables/useMonitor'
import { t } from '~/i18n'
const { settings, action } = useMonitor()
const { pack, energy, state, labels } = usePet()
const expanded = ref(false)
</script>
<template>
  <section class="pet-companion" :aria-label="t('像素伙伴')">
    <button
      type="button"
      class="pet-avatar-button"
      :aria-label="t('查看伙伴能量')"
      :aria-expanded="expanded"
      @click="expanded = !expanded"
    >
      <PixelPet :pack="pack" :state="state" :size="settings.petSize" />
    </button>
    <div class="pet-details">
      <div class="pet-heading">
        <strong>{{ settings.petName }}</strong
        ><span>{{ labels[state] }}</span>
      </div>
      <div v-if="settings.petShowEnergy || expanded" class="pet-energy">
        <meter min="0" max="100" :value="energy" :aria-label="t('能量')" />
        <span>{{ energy }}%</span>
      </div>
      <p v-if="expanded" class="pet-hint">{{ t('能量跟随疲劳值恢复，跳过休息不会充满。') }}</p>
    </div>
    <button v-if="expanded" type="button" class="button pet-break-button" @click="action('break')">
      {{ t('一起休息') }}
    </button>
  </section>
</template>
<style scoped>
.pet-companion {
  display: flex;
  align-items: center;
  gap: var(--space-6);
  padding: var(--space-5) var(--space-6);
  margin-block: var(--space-6);
  background: var(--color-surface);
  border-radius: var(--radius-lg);
  min-width: 0;
  flex-wrap: wrap;
}
.pet-avatar-button {
  display: grid;
  place-items: center;
  padding: 0;
  background: transparent;
  border: 0;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition:
    background-color 150ms ease-out,
    transform 150ms ease-out;
}
.pet-avatar-button:active {
  transform: scale(0.96);
}
@media (hover: hover) {
  .pet-avatar-button:hover {
    background: var(--color-hover);
  }
}
.pet-details {
  flex: 1;
  min-width: 160px;
}
.pet-heading {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: var(--space-3);
}
.pet-heading strong {
  font-size: var(--text-section);
  overflow-wrap: anywhere;
}
.pet-heading span,
.pet-hint {
  color: var(--color-text-muted);
  font-size: var(--text-small);
}
.pet-energy {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-top: var(--space-3);
  font-variant-numeric: tabular-nums;
  font-size: var(--text-small);
  color: var(--color-text-muted);
}
meter {
  width: min(180px, 65%);
  height: 6px;
  background: var(--color-border-subtle);
  border: 0;
  border-radius: var(--radius-pill);
  overflow: hidden;
}
meter::-webkit-meter-bar {
  background: var(--color-border-subtle);
  border: 0;
  height: 6px;
}
meter::-webkit-meter-optimum-value {
  background: var(--color-success);
  border-radius: var(--radius-pill);
}
meter::-moz-meter-bar {
  background: var(--color-success);
}
.pet-hint {
  margin: var(--space-3) 0 0;
  line-height: 1.6;
}
.pet-break-button {
  flex-shrink: 0;
}
</style>
