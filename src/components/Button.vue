<script setup lang="ts">
defineProps<{ className?: string }>()
const emit = defineEmits<{ (e: 'click'): void }>()
</script>

<template>
  <button :class="className" type="button" @click="emit('click')">
    <slot />
  </button>
</template>
