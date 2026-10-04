<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import Icon from '~/components/eye/Icon.vue'
const { error, snapshot } = useMonitor()
function dismiss() {
  error.value = ''
  snapshot.value.storageError = null
  snapshot.value.shortcutError = null
}
</script>

<template>
  <div
    v-if="error || snapshot.storageError || snapshot.shortcutError"
    class="error-banner"
    role="alert"
  >
    <span>{{
      error || snapshot.shortcutError || t('保存失败', { error: snapshot.storageError ?? '' })
    }}</span
    ><button :aria-label="t('关闭错误提示')" @click="dismiss">
      <Icon name="close" />
    </button>
  </div>
</template>
