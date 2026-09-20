<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { vScrubNumber } from '~/directives/scrubNumber'
import Icon from '~/components/eye/Icon.vue'
import Toggle from '~/components/eye/Toggle.vue'
const { settings, saving, dirty, saveSettings, toggle } = useMonitor()
</script>

<template>
  <form class="preferences rules-page" @submit.prevent="saveSettings">
    <header class="page-heading">
      <div>
        <h1>{{ t('休息规则') }}</h1>
        <p>{{ t('设定适合自己的工作与休息节奏。') }}</p>
      </div>
    </header>
    <div class="setting-row rule-row">
      <label for="work">{{ t('规则') }}</label>
      <div class="rule-inputs">
        <span>{{ t('工作') }}</span
        ><input
          id="work"
          v-model.number="settings.workMinutes"
          v-scrub-number
          type="number"
          :disabled="saving"
          :title="t('左右拖动调整数值，点击可输入；Esc 取消拖动。')"
          min="1"
          max="240"
          required
          :aria-label="t('工作分钟')"
        /><span>{{ t('分钟后，休息') }}</span
        ><input
          v-model.number="settings.breakMinutes"
          v-scrub-number
          type="number"
          :disabled="saving"
          :title="t('左右拖动调整数值，点击可输入；Esc 取消拖动。')"
          min="0"
          max="60"
          required
          :aria-label="t('休息分钟')"
        /><span>{{ t('分钟') }}</span
        ><input
          v-model.number="settings.breakSeconds"
          v-scrub-number
          type="number"
          :disabled="saving"
          :title="t('左右拖动调整数值，点击可输入；Esc 取消拖动。')"
          min="0"
          max="59"
          required
          :aria-label="t('休息秒数')"
        /><span>{{ t('秒') }}</span>
      </div>
    </div>
    <div class="setting-row rule-row">
      <label for="repeat">{{ t('提醒间隔') }}</label>
      <div class="rule-inputs">
        <span>{{ t('若疲劳值一直处于100%，每隔') }}</span
        ><input
          id="repeat"
          v-model.number="settings.repeatMinutes"
          v-scrub-number
          type="number"
          :disabled="saving"
          :title="t('左右拖动调整数值，点击可输入；Esc 取消拖动。')"
          min="1"
          max="60"
          required
        /><span>{{ t('分钟提醒一次') }}</span>
      </div>
    </div>
    <div class="setting-row">
      <span>{{ t('在弹窗出现前通知提醒') }}</span
      ><Toggle
        :model-value="settings.preNotify"
        :label="t('在弹窗出现前通知提醒')"
        :disabled="saving"
        @update:model-value="toggle('preNotify', $event)"
      />
    </div>
    <div class="setting-row">
      <span>{{ t('休息结束时播放声音') }}</span
      ><Toggle
        :model-value="settings.sound"
        :label="t('休息结束时播放声音')"
        :disabled="saving"
        @update:model-value="toggle('sound', $event)"
      />
    </div>
    <div class="setting-row">
      <span
        >{{ t('观影模式') }}
        <Icon
          name="help"
          :title="
            t('开启后，无键盘和鼠标操作时仍持续计时。关闭时，空闲一分钟后暂停累计并恢复疲劳值。')
          " /></span
      ><Toggle
        :model-value="settings.movieMode"
        :label="t('观影模式')"
        :disabled="saving"
        @update:model-value="toggle('movieMode', $event)"
      />
    </div>
    <div class="setting-row contact-row">
      <span>{{ t('关于应用') }}</span>
      <div>
        {{ t('appName') }}<span>{{ t('工作有节奏，休息有提醒') }}</span
        ><small>{{ t('疲劳值按连续使用时长估算') }}</small>
      </div>
    </div>
    <div v-if="dirty" class="save-row">
      <span>{{ t('有未保存的更改') }}</span
      ><button class="button primary" type="submit" :disabled="saving">
        {{ saving ? t('保存中…') : t('保存规则') }}
      </button>
    </div>
  </form>
</template>
