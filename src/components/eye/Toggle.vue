<script setup lang="ts">
defineProps<{ modelValue: boolean, label: string, disabled?: boolean }>()
defineEmits<{ 'update:modelValue': [value: boolean] }>()
</script>

<template>
  <button
    type="button"
    class="toggle"
    role="switch"
    :aria-checked="modelValue"
    :aria-label="label"
    :class="{ on: modelValue }"
    :disabled="disabled"
    @click="$emit('update:modelValue', !modelValue)"
  >
    <span />
  </button>
</template>
