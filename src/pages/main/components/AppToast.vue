<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import Icon from '~/components/eye/Icon.vue'
const { toast } = useMonitor()
</script>

<template>
  <div v-if="toast" class="toast" role="status"><Icon name="check" />{{ toast }}</div>
</template>
