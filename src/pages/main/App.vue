<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'
import Button from '~/components/Button.vue'

async function onSkip() {
  await invoke('stop_timer')
  await invoke('hide_main_window')
}

const minutes = ref(0)
const seconds = ref(0)

onMounted(() => {
  const interval = setInterval(() => {
    seconds.value++
    if (seconds.value === 60) {
      minutes.value++
      seconds.value = 0
    }
  }, 1000)

  return () => clearInterval(interval)
})
</script>

<template>
  <div class="w-full h-screen bg-red-100 flex flex-col items-center justify-center gap-8 select-none">
    <div class="text-2xl font-normal text-gray-600 text-center">
      专注时间30分钟了，请注意休息
    </div>
    <!-- countdown  min sec -->
    <div class="flex flex-row items-center justify-center gap-8">
      <div class="text-4xl font-normal text-gray-900 text-center w-full flex flex-row items-baseline justify-center gap-2">
        <div class="min-w-[54px] w-[54px] text-right rounded-md bg-blue-300 text-5xl p-2 text-white">
          {{ minutes }}
        </div>
        <div class="text-left text-[#333333]">
          分
        </div>
      </div>
      <div class="text-4xl font-normal text-gray-900 text-center w-full flex flex-row items-baseline justify-center gap-2">
        <div class="min-w-[54px] w-[54px] text-right rounded-md bg-blue-300 text-5xl p-2 text-white">
          {{ seconds }}
        </div>
        <div class="text-left text-[#333333]">
          秒
        </div>
      </div>
    </div>
    <div class="flex flex-row items-center justify-center gap-4">
      <Button class-name="btn-solid" anim @click="onSkip">
        Skip
      </Button>
    </div>
  </div>
</template>

<style scoped>
</style>
