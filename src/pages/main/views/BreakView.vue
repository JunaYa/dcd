<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import Icon from '~/components/eye/Icon.vue'
import { useBreakPresentation } from '../composables/useBreakPresentation'
const { settings, remaining, action } = useMonitor()
const { countdown, breakMessage, backgroundStyle, overlayStyle } = useBreakPresentation()
</script>

<template>
  <div
    class="break-screen"
    :style="overlayStyle"
    :class="[
      `background-${settings.background}`,
      { 'break-window': settings.reminderStyle !== 'fullscreen' },
    ]"
  >
    <div class="break-backdrop" :style="backgroundStyle" aria-hidden="true" />
    <div class="break-content">
      <Icon name="eye" />
      <h1>{{ breakMessage }}</h1>
      <p>{{ t('移开视线，看看远处，让双眼放松。') }}</p>
      <div class="countdown">
        {{ countdown }}
      </div>
      <span class="break-caption">{{ remaining > 0 ? t('休息倒计时') : t('休息已完成') }}</span>
      <button v-if="settings.allowSkip" class="button break-skip" @click="action('skip')">
        {{ t('跳过本次休息') }}
      </button>
    </div>
  </div>
</template>
