<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{ value: number }>()
const digits = computed(() => String(props.value).padStart(3, '\u00a0').split(''))
</script>

<template>
  <span class="rolling-number" :aria-label="String(value)">
    <span
      v-for="(digit, index) in digits"
      :key="digits.length - index"
      class="rolling-number-digit"
      :class="{ 'is-empty': digit === '\u00a0' }"
      aria-hidden="true"
    >
      <Transition name="number-roll">
        <span :key="digit" class="rolling-number-value">{{ digit }}</span>
      </Transition>
    </span>
  </span>
</template>

<style scoped>
.rolling-number {
  display: inline-flex;
  overflow: hidden;
  vertical-align: bottom;
  font-variant-numeric: tabular-nums;
}
.rolling-number-digit {
  display: grid;
  width: 1ch;
  flex-shrink: 0;
  overflow: hidden;
  perspective: 180px;
}
.rolling-number-digit.is-empty {
  width: 0;
}
.rolling-number-value {
  grid-area: 1 / 1;
  text-align: center;
  transform-origin: center bottom;
}
.number-roll-enter-active {
  transition: transform 240ms cubic-bezier(0.32, 0.72, 0, 1), opacity 240ms ease-out;
}
.number-roll-leave-active {
  transition: transform 180ms ease-out, opacity 180ms ease-out;
}
.number-roll-enter-from {
  transform: translateY(70%) rotateX(-35deg);
  opacity: 0;
}
.number-roll-leave-to {
  transform: translateY(-45%) rotateX(25deg);
  opacity: 0;
}
@media (prefers-reduced-motion: reduce) {
  .number-roll-enter-active,
  .number-roll-leave-active {
    transition: none;
  }
  .number-roll-enter-from,
  .number-roll-leave-to {
    transform: none;
    opacity: 1;
  }
  .number-roll-leave-active {
    display: none;
  }
}
</style>
