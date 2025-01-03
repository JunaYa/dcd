<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app'
import { onMounted, ref } from 'vue'
import logo from '~/assets/logo.png'

const appVersion = ref('')

onMounted(async () => {
  appVersion.value = await getVersion()
})
</script>

<template>
  <div class="startup p-8">
    <div class="mb-4 flex items-center justify-center">
      <img :src="logo" alt="logo" class="logo relative h-18 w-18">
    </div>
    <div class="text-center text-base">
      Descktop Countdown 版本: V{{ appVersion }}
    </div>
  </div>
</template>

<style scoped>
.logo {
  filter: drop-shadow(0 0 1em #747bff);
}
</style>
