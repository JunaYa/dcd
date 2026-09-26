<script setup lang="ts">
import { computed } from 'vue'
import { useDocumentVisibility } from '@vueuse/core'
import type { CSSProperties } from 'vue'
import type { PetPack, PetState } from './model'
const props = defineProps<{ pack: PetPack; state: PetState; size: number }>()
const visibility = useDocumentVisibility()
const style = computed(() => {
  const animation = props.pack.states[props.state]
  const scale = Math.max(
    1,
    Math.floor(props.size / Math.max(props.pack.frameWidth, props.pack.frameHeight)),
  )
  const width = scale * props.pack.frameWidth
  const height = scale * props.pack.frameHeight
  return {
    width: `${width}px`,
    height: `${height}px`,
    backgroundImage: `url("${animation.image}")`,
    backgroundSize: `${width * animation.frames}px ${height}px`,
    '--pet-end': `${-width * animation.frames}px`,
    animation: `pet-frames ${animation.frames / animation.fps}s steps(${animation.frames}) infinite`,
    animationPlayState: visibility.value === 'hidden' ? 'paused' : 'running',
  } as CSSProperties
})
</script>
<template>
  <span
    :key="`${state}-${pack.states[state].image}`"
    class="pixel-pet"
    :style="style"
    aria-hidden="true"
  />
</template>
<style>
.pixel-pet {
  display: block;
  flex-shrink: 0;
  background-repeat: no-repeat;
  image-rendering: pixelated;
  pointer-events: none;
}
@keyframes pet-frames {
  to {
    background-position-x: var(--pet-end);
  }
}
@media (prefers-reduced-motion: reduce) {
  .pixel-pet {
    animation: none !important;
  }
}
</style>
