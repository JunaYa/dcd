<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { computed } from 'vue'
import { languages } from '~/i18n'
import Icon from '~/components/eye/Icon.vue'
import Toggle from '~/components/eye/Toggle.vue'
const { settings, saving, dirty, saveSettings, toggle } = useMonitor()
const messageInput = computed({
  get: () => (settings.value.message === 'Take a break' ? '' : settings.value.message),
  set: (value: string) => {
    settings.value.message = value
  },
})
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
        <span :title="t('关闭提醒说明')">{{ t('开启提醒') }}</span
        ><Toggle
          :model-value="settings.reminders"
          :title="t('关闭提醒说明')"
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
          :title="t('关闭后需要完成倒计时才能结束休息')"
          :label="t('在弹窗中展示跳过按钮')"
          :disabled="saving"
          @update:model-value="toggle('allowSkip', $event)"
        />
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
