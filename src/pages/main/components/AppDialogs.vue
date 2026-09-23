<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { nextTick, ref, watch } from 'vue'
import Icon from '~/components/eye/Icon.vue'
import BreakContent from './BreakContent.vue'
import { useBreakPresentation } from '../composables/useBreakPresentation'
const { settings, browserBreak, action } = useMonitor()
const { countdown, backgroundStyle } = useBreakPresentation()
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
      <BreakContent :timer="countdown" :caption="t('浏览器休息预览')" />
    </template>
  </dialog>
</template>
