<script setup lang="ts" generic="T extends string | number">
import { computed, ref } from 'vue'

const props = defineProps<{ options: { value: T; label: string }[]; label: string }>()
const model = defineModel<T>({ required: true })
const animate = ref(false)
const selectedIndex = computed(() =>
  props.options.findIndex((option) => option.value === model.value),
)

function select(value: T, event: MouseEvent) {
  animate.value = event.detail > 0
  model.value = value
}
</script>

<template>
  <div
    class="segmented"
    role="group"
    :aria-label="label"
    :class="{ 'segmented-animated': animate }"
    :style="{ '--segment-count': options.length, '--segment-index': selectedIndex }"
  >
    <span v-if="selectedIndex >= 0" class="segmented-track" aria-hidden="true">
      <span class="segmented-indicator" />
    </span>
    <button
      v-for="option in options"
      :key="option.value"
      type="button"
      :class="{ selected: model === option.value }"
      :aria-pressed="model === option.value"
      @click="select(option.value, $event)"
    >
      {{ option.label }}
    </button>
  </div>
</template>
