<script setup lang="ts">
import { computed, ref } from 'vue'
import { save as saveDialog } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { useMonitor } from '~/pages/main/composables/useMonitor'
import Toggle from '~/components/eye/Toggle.vue'
import PixelPet from './PixelPet.vue'
import { usePet } from './usePet'
import {
  decodePetPack,
  defaultPet,
  maxPackBytes,
  parsePetPack,
  petStates,
  pngSize,
  type PetState,
} from './model'
import { t } from '~/i18n'
const { native, settings, saving, saveSettings, toggle, report } = useMonitor()
const { pack, labels } = usePet()
const selected = ref<PetState>('working')
const importing = ref(false)
const busy = computed(() => saving.value || importing.value)
async function upload(event: Event, strip = false) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  importing.value = true
  try {
    if (file.size > maxPackBytes) throw new Error(t('宠物素材无效'))
    let source: string
    if (strip) {
      const image = await new Promise<string>((resolve, reject) => {
        const reader = new FileReader()
        reader.onload = () => resolve(String(reader.result))
        reader.onerror = () => reject(new Error(t('无法读取图片')))
        reader.readAsDataURL(file)
      })
      const [width, height] = pngSize(image)
      if (height !== pack.value.frameHeight || width % pack.value.frameWidth)
        throw new Error(t('宠物素材无效'))
      const next = structuredClone(pack.value)
      next.states[selected.value] = {
        ...next.states[selected.value],
        image,
        frames: width / next.frameWidth,
      }
      source = JSON.stringify(next)
    } else source = await file.text()
    const next = parsePetPack(source)
    await decodePetPack(next)
    settings.value.petPack = JSON.stringify(next)
    if (!strip) settings.value.petName = next.name
    await saveSettings()
  } catch (cause) {
    report(cause instanceof Error ? cause.message : cause)
  } finally {
    input.value = ''
    importing.value = false
  }
}
async function setSpeed(event: Event) {
  const next = structuredClone(pack.value)
  next.states[selected.value].fps = Number((event.target as HTMLInputElement).value)
  settings.value.petPack = JSON.stringify(next)
  await saveSettings()
}
async function download() {
  importing.value = true
  try {
    const exported = parsePetPack(JSON.stringify({ ...pack.value, name: settings.value.petName }))
    const content = JSON.stringify(exported, null, 2)
    if (native) {
      const path = await saveDialog({
        defaultPath: 'dcd-pet.json',
        filters: [{ name: 'JSON', extensions: ['json'] }],
      })
      if (path) await writeTextFile(path, content)
    } else {
      const url = URL.createObjectURL(new Blob([content], { type: 'application/json' }))
      const link = document.createElement('a')
      link.href = url
      link.download = 'dcd-pet.json'
      link.click()
      setTimeout(() => URL.revokeObjectURL(url), 1000)
    }
  } catch (cause) {
    report(cause)
  } finally {
    importing.value = false
  }
}
async function reset() {
  settings.value.petPack = ''
  settings.value.petName = defaultPet.name
  await saveSettings()
}
</script>
<template>
  <section class="pet-settings">
    <h2>{{ t('像素伙伴') }}</h2>
    <p class="section-description">{{ t('用一个小伙伴，感受专注与休息的节奏。') }}</p>
    <div class="setting-row">
      <span>{{ t('显示像素伙伴') }}</span
      ><Toggle
        :model-value="settings.petEnabled"
        :label="t('显示像素伙伴')"
        :disabled="busy"
        @update:model-value="toggle('petEnabled', $event)"
      />
    </div>
    <template v-if="settings.petEnabled">
      <div class="pet-custom-preview">
        <div class="pet-preview-stage">
          <PixelPet :pack="pack" :state="selected" :size="settings.petSize" />
        </div>
        <div class="pet-preview-controls">
          <label for="pet-state">{{ t('状态预览') }}</label>
          <select id="pet-state" v-model="selected" :disabled="busy">
            <option v-for="state in petStates" :key="state" :value="state">
              {{ labels[state] }}
            </option>
          </select>
          <label for="pet-fps">{{ t('动画帧率') }}</label>
          <div class="range-control">
            <input
              id="pet-fps"
              type="range"
              min="1"
              max="24"
              :value="pack.states[selected].fps"
              :disabled="busy"
              @change="setSpeed"
            /><output>{{ pack.states[selected].fps }} fps</output>
          </div>
          <label class="button upload-button"
            >{{ t('替换此状态素材')
            }}<input type="file" accept="image/png" :disabled="busy" @change="upload($event, true)"
          /></label>
        </div>
      </div>
      <p class="section-description">
        {{ t('宠物帧说明', { width: pack.frameWidth, height: pack.frameHeight }) }}
      </p>
      <div class="setting-row">
        <label for="pet-name">{{ t('伙伴名字') }}</label
        ><input
          id="pet-name"
          v-model="settings.petName"
          type="text"
          maxlength="40"
          :disabled="busy"
          @change="saveSettings"
        />
      </div>
      <div class="setting-row">
        <label for="pet-size">{{ t('伙伴大小') }}</label>
        <div class="range-control">
          <input
            id="pet-size"
            v-model.number="settings.petSize"
            type="range"
            min="48"
            max="160"
            step="8"
            :disabled="busy"
            @change="saveSettings"
          /><output>{{ settings.petSize }} px</output>
        </div>
      </div>
      <div class="setting-row">
        <label for="pet-position">{{ t('今日页面位置') }}</label
        ><select
          id="pet-position"
          v-model="settings.petPosition"
          :disabled="busy"
          @change="saveSettings"
        >
          <option value="top">{{ t('曲线上方') }}</option>
          <option value="bottom">{{ t('曲线下方') }}</option>
        </select>
      </div>
      <div class="setting-row">
        <label for="pet-tired">{{ t('低能量阈值') }}</label>
        <div class="range-control">
          <input
            id="pet-tired"
            v-model.number="settings.petTiredThreshold"
            type="range"
            min="5"
            max="50"
            :disabled="busy"
            @change="saveSettings"
          /><output>{{ settings.petTiredThreshold }}%</output>
        </div>
      </div>
      <div class="setting-row">
        <span>{{ t('显示能量条') }}</span
        ><Toggle
          :model-value="settings.petShowEnergy"
          :label="t('显示能量条')"
          :disabled="busy"
          @update:model-value="toggle('petShowEnergy', $event)"
        />
      </div>
      <div class="setting-row">
        <span>{{ t('休息浮层显示伙伴') }}</span
        ><Toggle
          :model-value="settings.petShowOnBreak"
          :label="t('休息浮层显示伙伴')"
          :disabled="busy"
          @update:model-value="toggle('petShowOnBreak', $event)"
        />
      </div>
      <div class="pet-pack-actions">
        <label class="button upload-button"
          >{{ t('导入宠物包')
          }}<input
            type="file"
            accept=".json,application/json"
            :disabled="busy"
            @change="upload($event)"
        /></label>
        <button class="button" type="button" :disabled="busy" @click="download">
          {{ t('导出宠物包') }}
        </button>
        <button class="button" type="button" :disabled="busy || !settings.petPack" @click="reset">
          {{ t('恢复内置伙伴') }}
        </button>
      </div>
      <p class="section-description">{{ t('宠物包说明') }}</p>
    </template>
  </section>
</template>
<style scoped>
.pet-settings .button {
  border-radius: var(--radius-sm);
  min-height: 40px;
}
.pet-settings .upload-button:focus-within {
  outline: 2px solid var(--color-focus);
  outline-offset: 3px;
}
.pet-custom-preview {
  display: flex;
  align-items: center;
  gap: var(--space-8);
  flex-wrap: wrap;
  padding-block: var(--space-6);
}
.pet-preview-stage {
  display: grid;
  place-items: center;
  width: 192px;
  height: 192px;
  flex-shrink: 0;
  background: var(--color-canvas);
  border-radius: var(--radius-lg);
}
.pet-preview-controls {
  display: grid;
  gap: var(--space-3);
  flex: 1;
  min-width: 200px;
  max-width: 360px;
}
.pet-preview-controls > label:not(.button) {
  font-size: var(--text-small);
  color: var(--color-text-muted);
}
.pet-pack-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
  padding-block: var(--space-5);
}
</style>
