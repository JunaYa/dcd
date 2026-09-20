<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { computed } from 'vue'
import { languages } from '~/i18n'
import Icon from '~/components/eye/Icon.vue'
import Toggle from '~/components/eye/Toggle.vue'
const { settings, saving, dirty, saveSettings, toggle, action, report } = useMonitor()
const themeOptions = computed(() => [
  { value: 'system', label: t('跟随系统') },
  { value: 'light', label: t('浅色') },
  { value: 'dark', label: t('深色') },
])
const messageInput = computed({
  get: () => (settings.value.message === 'Take a break' ? '' : settings.value.message),
  set: (value: string) => {
    settings.value.message = value
  },
})
async function upload(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  try {
    if (
      !['image/png', 'image/jpeg', 'image/webp'].includes(file.type) ||
      file.size > 3 * 1024 * 1024
    ) {
      throw new Error(t('请选择不超过 3 MB 的 PNG、JPEG 或 WebP 图片。'))
    }
    const data = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader()
      reader.onload = () => resolve(String(reader.result))
      reader.onerror = () => reject(new Error(t('无法读取图片')))
      reader.readAsDataURL(file)
    })
    settings.value.backgroundImage = data
    settings.value.background = 'custom'
    await saveSettings()
  } catch (cause) {
    report(cause)
  } finally {
    input.value = ''
  }
}
</script>

<template>
  <form class="preferences" @submit.prevent="saveSettings">
    <header class="page-heading">
      <div>
        <h1>{{ t('设置') }}</h1>
        <p>{{ t('调整应用行为和提醒方式。') }}</p>
      </div>
    </header>
    <section>
      <h2>{{ t('语言') }}</h2>
      <p class="section-description">
        {{ t('语言说明') }}
      </p>
      <div class="setting-row">
        <label for="language">{{ t('语言') }}</label>
        <select id="language" v-model="settings.language" :disabled="saving" @change="saveSettings">
          <option
            v-for="language in languages"
            :key="language.value"
            :value="language.value"
            :lang="language.value"
          >
            {{ language.label }}
          </option>
        </select>
      </div>
    </section>
    <section>
      <h2>{{ t('外观') }}</h2>
      <p class="section-description">
        {{ t('各个窗口独立设置，修改后自动保存。') }}
      </p>
      <div class="setting-row">
        <label for="main-theme">{{ t('主窗口主题') }}</label>
        <select
          id="main-theme"
          v-model="settings.mainTheme"
          :disabled="saving"
          @change="saveSettings"
        >
          <option v-for="option in themeOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
      </div>
      <div class="setting-row">
        <label for="tray-theme">{{ t('托盘菜单主题') }}</label>
        <select
          id="tray-theme"
          v-model="settings.trayTheme"
          :disabled="saving"
          @change="saveSettings"
        >
          <option v-for="option in themeOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
      </div>
      <div class="setting-row">
        <span>{{ t('托盘菜单显示今日曲线') }}</span>
        <Toggle
          :model-value="settings.trayShowChart"
          :label="t('托盘菜单显示今日曲线')"
          :disabled="saving"
          @update:model-value="toggle('trayShowChart', $event)"
        />
      </div>
    </section>
    <section>
      <h2>{{ t('系统集成') }}</h2>
      <div class="setting-row">
        <span>{{ t('开机时启动') }}</span
        ><Toggle
          :model-value="settings.autostart"
          :label="t('开机时启动')"
          :disabled="saving"
          @update:model-value="toggle('autostart', $event)"
        />
      </div>
      <div class="setting-row">
        <span>{{ t('显示状态栏图标') }}</span
        ><Toggle
          :model-value="settings.trayIcon"
          :label="t('显示状态栏图标')"
          :disabled="saving"
          @update:model-value="toggle('trayIcon', $event)"
        />
      </div>
      <div class="setting-row">
        <span>{{ t('在状态栏中显示今日时长') }}</span
        ><Toggle
          :model-value="settings.trayTime"
          :label="t('在状态栏中显示今日时长')"
          :disabled="saving"
          @update:model-value="toggle('trayTime', $event)"
        />
      </div>
      <div class="setting-row">
        <span>{{ t('显示程序坞图标') }}</span
        ><Toggle
          :model-value="settings.dockIcon"
          :label="t('显示程序坞图标')"
          :disabled="saving"
          @update:model-value="toggle('dockIcon', $event)"
        />
      </div>
    </section>
    <section>
      <h2>{{ t('休息提醒') }}</h2>
      <div class="setting-row">
        <span>{{ t('开启提醒') }}</span
        ><Toggle
          :model-value="settings.reminders"
          :label="t('开启提醒')"
          :disabled="saving"
          @update:model-value="toggle('reminders', $event)"
        />
      </div>
      <div class="setting-row">
        <label for="style">{{ t('提醒方式') }}</label
        ><select id="style" v-model="settings.reminderStyle" @change="saveSettings">
          <option value="fullscreen">
            {{ t('桌面浮层') }}
          </option>
          <option value="window">
            {{ t('窗口提醒') }}
          </option>
        </select>
      </div>
      <div class="setting-row">
        <label for="message">{{ t('休息提醒内容') }}</label
        ><input
          id="message"
          v-model="messageInput"
          class="message-input"
          maxlength="120"
          :placeholder="t('休息一下')"
        />
      </div>
      <div class="setting-row">
        <span
          >{{ t('在弹窗中展示跳过按钮') }}
          <Icon name="help" :title="t('关闭后需要完成倒计时才能结束休息')" /></span
        ><Toggle
          :model-value="settings.allowSkip"
          :label="t('在弹窗中展示跳过按钮')"
          :disabled="saving"
          @update:model-value="toggle('allowSkip', $event)"
        />
      </div>
    </section>
    <section>
      <h2>{{ t('休息浮层外观') }}</h2>
      <p class="section-description">
        {{ t('独立于主窗口和托盘菜单，也用于窗口提醒。') }}
      </p>
      <div class="setting-row">
        <span>{{ t('背景主题') }}</span>
      </div>
      <div class="background-options">
        <label
          v-for="option in [
            { id: 'system', label: t('跟随系统') },
            { id: 'light', label: t('浅色') },
            { id: 'dark', label: t('深色') },
            { id: 'custom', label: t('自定义') },
          ]"
          :key="option.id"
          ><input
            v-model="settings.background"
            type="radio"
            :value="option.id"
            name="background"
            @change="saveSettings"
          /><span
            class="swatch"
            :class="option.id"
            :style="
              option.id === 'custom' && settings.backgroundImage
                ? { backgroundImage: `url(${settings.backgroundImage})` }
                : {}
            "
          />{{ option.label }}</label
        ><label class="button upload-button"
          >{{ t('上传')
          }}<input type="file" accept="image/png,image/jpeg,image/webp" @change="upload" /></label
        ><button
          class="button"
          type="button"
          @click="
            saveSettings().then(() => {
              if (!dirty) action('break')
            })
          "
        >
          {{ t('测试') }}
        </button>
      </div>
      <div class="setting-row">
        <label for="overlay-opacity">{{ t('桌面遮罩浓度') }}</label>
        <div class="range-control">
          <input
            id="overlay-opacity"
            v-model.number="settings.overlayOpacity"
            type="range"
            min="50"
            max="100"
            :disabled="saving"
            @change="saveSettings"
          />
          <output for="overlay-opacity">{{ settings.overlayOpacity }}%</output>
        </div>
      </div>
      <div class="setting-row">
        <label for="overlay-blur">{{ t('桌面背景模糊') }}</label>
        <div class="range-control">
          <input
            id="overlay-blur"
            v-model.number="settings.overlayBlur"
            type="range"
            min="0"
            max="40"
            :disabled="saving"
            @change="saveSettings"
          />
          <output for="overlay-blur">{{ settings.overlayBlur }}</output>
        </div>
      </div>
    </section>
    <div v-if="dirty" class="save-row">
      <span>{{ t('有未保存的更改') }}</span
      ><button class="button primary" type="submit" :disabled="saving">
        {{ saving ? t('保存中…') : t('保存设置') }}
      </button>
    </div>
  </form>
</template>
