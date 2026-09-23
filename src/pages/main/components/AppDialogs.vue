<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { nextTick, ref, watch } from 'vue'
import Icon from '~/components/eye/Icon.vue'
import { useBreakPresentation } from '../composables/useBreakPresentation'
const { settings, browserBreak, action } = useMonitor()
const { countdown, breakMessage, backgroundStyle } = useBreakPresentation()
const dialog = ref<HTMLDialogElement>()
const dialogKind = ref<'help' | 'break' | null>(null)
let dialogOpener: HTMLElement | null = null
function openDialog(kind: 'help' | 'break') {
  dialogOpener = document.activeElement as HTMLElement
  dialogKind.value = kind
  nextTick(() => dialog.value?.showModal())
}
function closeDialog() {
  dialog.value?.close()
  dialogKind.value = null
  dialogOpener?.focus()
}
function cancelDialog(event: Event) {
  if (dialogKind.value === 'break') {
    event.preventDefault()
    if (settings.value.allowSkip) action('skip')
  } else {
    event.preventDefault()
    closeDialog()
  }
}
watch(browserBreak, (active) => (active ? openDialog('break') : closeDialog()))
defineExpose({ openHelp: () => openDialog('help') })
</script>

<template>
  <dialog
    ref="dialog"
    :class="
      dialogKind === 'break' ? `break-dialog background-${settings.background}` : 'help-dialog'
    "
    :style="dialogKind === 'break' ? backgroundStyle : {}"
    @cancel="cancelDialog"
  >
    <template v-if="dialogKind === 'help'">
      <button class="dialog-close icon-button" :aria-label="t('关闭说明')" @click="closeDialog">
        <Icon name="close" /></button
      ><Icon name="eye" />
      <h2>{{ t('给双眼一点休息时间') }}</h2>
      <p>
        {{ t('帮助计时') }}
      </p>
      <p>
        {{ t('帮助规则') }}
      </p>
      <p>
        {{ t('帮助隐私') }}
      </p>
      <button class="button primary" @click="closeDialog">
        {{ t('知道了') }}
      </button>
    </template>
    <template v-else-if="dialogKind === 'break'">
      <div class="break-content">
        <Icon name="eye" />
        <div class="break-copy">
          <h1>{{ breakMessage }}</h1>
          <p>{{ t('移开视线，看看远处，让双眼放松。') }}</p>
        </div>
        <div class="break-timer">
          <div class="countdown">{{ countdown }}</div>
          <span class="break-caption">{{ t('浏览器休息预览') }}</span>
        </div>
        <button v-if="settings.allowSkip" class="button break-skip" @click="action('skip')">
          {{ t('跳过本次休息') }}
        </button>
      </div>
    </template>
  </dialog>
</template>
