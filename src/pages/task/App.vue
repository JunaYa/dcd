<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { onMounted, ref } from 'vue'
import Button from '~/components/Button.vue'

const appWindow = getCurrentWindow()

const intervalId = ref()
async function onSkip() {
  await invoke('stop_timer')
  await invoke('hide_task_window')
  reset()
}

async function getState() {
  const state = await invoke('get_state')
  console.log(state)
}

const minutes = ref(0)
const seconds = ref(0)

function start() {
  intervalId.value = setInterval(() => {
    getState()
    seconds.value++
    if (seconds.value === 60) {
      minutes.value++
      seconds.value = 0
    }
  }, 1000)
}

function reset() {
  clearInterval(intervalId.value)
  minutes.value = 0
  seconds.value = 0
}

function dragStart() {
  appWindow.startDragging()
  // no effect
  appWindow.setCursorIcon('move')
}

onMounted(() => {
  start()
  return () => clearInterval(intervalId.value)
})
</script>

<template>
  <div class="w-full h-screen animate-bg-gradient flex flex-col items-center justify-center gap-8 select-none" @mousedown="dragStart">
    <div class="text-2xl font-normal text-gray-600 text-center">
      休息一会儿
    </div>
    <!-- countdown  min sec -->
    <div class="flex flex-row items-center justify-center gap-8">
      <div class="text-4xl font-normal text-gray-900 text-center w-full flex flex-row items-baseline justify-center gap-2">
        <div class="min-w-[54px] w-[54px] text-right rounded-md bg-blue-300 text-5xl p-2 text-white">
          {{ minutes.toString().padStart(2, '0') }}
        </div>
        <div class="text-left text-[#666666]">
          分
        </div>
      </div>
      <div class="text-4xl font-normal text-gray-900 text-center w-full flex flex-row items-baseline justify-center gap-2">
        <div class="min-w-[54px] w-[54px] text-right rounded-md bg-blue-300 text-5xl p-2 text-white">
          {{ seconds.toString().padStart(2, '0') }}
        </div>
        <div class="text-left text-[#666666]">
          秒
        </div>
      </div>
    </div>
    <div class="flex flex-row items-center justify-center gap-4">
      <Button class-name="btn-solid" anim @click="onSkip">
        跳过
      </Button>
    </div>
  </div>
</template>

<style>
:root {
  /* background-color: transparent; */
}
</style>
