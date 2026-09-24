<script setup lang="ts">
import { t } from '~/i18n'
import { useMonitor } from '../composables/useMonitor'
import { useBreakPresentation } from '../composables/useBreakPresentation'
defineProps<{ timer: string; caption: string; preview?: boolean }>()
const { settings, action } = useMonitor()
const { breakMessage } = useBreakPresentation()
</script>
<template>
  <div class="break-content">
    <div class="break-copy">
      <h1>{{ breakMessage }}</h1>
      <p>{{ t('移开视线，看看远处，让双眼放松。') }}</p>
    </div>
    <div class="break-timer">
      <div class="countdown">{{ timer }}</div>
      <span class="break-caption">{{ caption }}</span>
    </div>
    <component
      :is="preview ? 'span' : 'button'"
      v-if="settings.allowSkip"
      class="button break-skip"
      :type="preview ? undefined : 'button'"
      @click="!preview && action('skip')"
    >
      {{ t('跳过本次休息') }}
    </component>
  </div>
</template>
