<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { formatDuration, locale, t } from '~/i18n'
import Icon from '~/components/eye/Icon.vue'
import { useMonitor } from '../composables/useMonitor'

const { native, snapshot, settings, paused, action, inform, report } = useMonitor()
const menu = ref<'more' | 'pause' | null>(null)
const moreButton = ref<HTMLButtonElement>()
const pauseButton = ref<HTMLButtonElement>()
const today = computed(() => snapshot.value.days[snapshot.value.date])
const points = computed(() =>
  snapshot.value.samples
    .map(([minute, fatigue]) => `${2 + (minute / 1440) * 324},${62 - (fatigue / 100) * 60}`)
    .join(' '),
)
const status = computed(() =>
  !settings.value.reminders
    ? t('提醒已关闭')
    : paused.value
      ? t('暂停至', {
          time: new Date(snapshot.value.pausedUntil * 1000).toLocaleTimeString(locale.value, {
            hour: '2-digit',
            minute: '2-digit',
          }),
        })
      : t('提醒正常'),
)
function closeMenu(restoreFocus = false) {
  const trigger = menu.value === 'more' ? moreButton.value : pauseButton.value
  menu.value = null
  if (restoreFocus) trigger?.focus()
}
async function run(name: string, minutes?: number) {
  closeMenu(true)
  await action(name, minutes)
}
async function share() {
  closeMenu(true)
  try {
    const text = t('摘要', {
      duration: formatDuration(today.value?.seconds ?? 0),
      fatigue: snapshot.value.fatigue,
    })
    if (native) await writeText(text)
    else await navigator.clipboard.writeText(text)
    inform(t('今日摘要已复制，可以粘贴分享'))
  } catch (cause) {
    report(cause)
  }
}
function onPointer(event: PointerEvent) {
  if (!(event.target as HTMLElement).closest('.tray-disclosure')) closeMenu()
}
function onKey(event: KeyboardEvent) {
  if (event.key === 'Escape' && menu.value) {
    event.preventDefault()
    closeMenu(true)
  }
}
function onFocus(event: FocusEvent) {
  const trigger = menu.value === 'more' ? moreButton.value : pauseButton.value
  if (!trigger?.parentElement?.contains(event.target as Node)) closeMenu()
}
onMounted(() => {
  document.addEventListener('pointerdown', onPointer)
  document.addEventListener('keydown', onKey)
  document.addEventListener('focusin', onFocus)
})
onUnmounted(() => {
  document.removeEventListener('pointerdown', onPointer)
  document.removeEventListener('keydown', onKey)
  document.removeEventListener('focusin', onFocus)
})
</script>

<template>
  <div class="tray-panel">
    <header class="tray-heading">
      <strong>{{ t('appName') }}</strong>
      <div class="tray-disclosure">
        <button
          ref="moreButton"
          class="icon-button"
          :aria-label="t('更多')"
          :aria-expanded="menu === 'more'"
          aria-controls="tray-more"
          @click="menu = menu === 'more' ? null : 'more'"
        >
          <svg class="icon" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
            <circle cx="5" cy="12" r="1.8" />
            <circle cx="12" cy="12" r="1.8" />
            <circle cx="19" cy="12" r="1.8" />
          </svg>
        </button>
        <div v-if="menu === 'more'" id="tray-more" class="tray-popover">
          <button @click="run('settings')"><Icon name="settings" />{{ t('设置') }}</button>
          <button @click="share"><Icon name="share" />{{ t('分享 DCD') }}</button>
          <button @click="run('quit')"><Icon name="power" />{{ t('退出') }}</button>
        </div>
      </div>
    </header>
    <section class="tray-summary" :aria-label="t('疲劳值')">
      <div class="tray-fatigue">
        <strong>{{ snapshot.fatigue }}<small>%</small></strong
        ><span>{{ t('疲劳值') }}</span>
      </div>
      <p>
        {{ t('今日时长') }} <span>{{ formatDuration(today?.seconds ?? 0) }}</span>
      </p>
    </section>
    <div v-if="settings.trayShowChart" class="tray-trend">
      <svg v-if="points" viewBox="0 0 328 64" role="img" :aria-label="t('今日疲劳值变化折线图')">
        <path d="M2 63H326" stroke="var(--color-border-subtle)" />
        <polyline
          :points="points"
          fill="none"
          stroke="var(--color-accent)"
          stroke-width="1.5"
          stroke-linejoin="round"
          stroke-linecap="round"
        />
      </svg>
      <p v-else>{{ t('使用后将显示今日趋势') }}</p>
    </div>
    <div class="tray-primary-actions">
      <button class="button primary" @click="run('break')">
        <Icon name="eye" />{{ t('休息一下') }}
      </button>
      <div class="tray-disclosure">
        <button
          ref="pauseButton"
          class="button tray-pause"
          :aria-expanded="menu === 'pause'"
          aria-controls="tray-pause-options"
          @click="paused ? run('resume') : (menu = menu === 'pause' ? null : 'pause')"
        >
          <Icon name="moon" />{{ paused ? t('恢复提醒') : t('暂停提醒')
          }}<span v-if="!paused" aria-hidden="true">⌄</span>
        </button>
        <div
          v-if="menu === 'pause'"
          id="tray-pause-options"
          class="tray-popover tray-pause-options"
        >
          <button @click="run('pause', 30)">{{ t('30分钟') }}</button>
          <button @click="run('pause', 60)">{{ t('1小时') }}</button>
          <button @click="run('pause', 0)">{{ t('直到明天') }}</button>
        </div>
      </div>
    </div>
    <footer class="tray-footer">
      <span :title="status"
        ><i class="status-dot" :class="{ paused: paused || !settings.reminders }" /><span
          class="tray-status-text"
          >{{ status }}</span
        ></span
      >
      <button @click="run('main')">{{ t('打开主窗口') }}<span aria-hidden="true">↗</span></button>
    </footer>
  </div>
</template>

<style scoped>
.tray-panel {
  display: grid;
  gap: var(--space-4);
}
.tray-heading,
.tray-fatigue,
.tray-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
}
.tray-heading {
  height: 40px;
}
.tray-heading strong {
  font-size: var(--text-small);
  font-weight: 600;
}
.tray-disclosure {
  position: relative;
}
.tray-summary p {
  margin: var(--space-2) 0 0;
  color: var(--color-text-muted);
  font-size: var(--text-small);
}
.tray-summary p span {
  color: var(--color-text-secondary);
  margin-inline-start: var(--space-2);
}
.tray-fatigue strong {
  font-size: 44px;
  line-height: 1.1;
  letter-spacing: -0.04em;
  font-variant-numeric: tabular-nums;
}
.tray-fatigue small {
  font-size: 24px;
  margin-inline-start: 2px;
}
.tray-fatigue > span {
  color: var(--color-text-muted);
  font-size: var(--text-small);
}
.tray-trend {
  height: 64px;
}
.tray-trend svg {
  display: block;
  width: 100%;
  height: 64px;
}
.tray-trend p {
  margin: 0;
  height: 64px;
  display: grid;
  place-items: center;
  color: var(--color-text-muted);
  font-size: var(--text-small);
}
.tray-primary-actions {
  display: grid;
  gap: var(--space-2);
}
.tray-primary-actions .button {
  width: 100%;
  min-height: 44px;
  gap: var(--space-2);
}
.tray-pause {
  background: transparent;
  border-color: var(--color-border-subtle);
}
.tray-footer {
  min-height: 40px;
  border-top: 1px solid var(--color-border-subtle);
  font-size: 12px;
}
.tray-footer > span {
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--color-text-muted);
  white-space: nowrap;
}
.tray-footer .status-dot {
  flex-shrink: 0;
}
.tray-footer button {
  min-height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 4px;
  padding-inline: 4px;
}
.tray-popover {
  position: absolute;
  inset-inline-end: 0;
  top: calc(100% + 4px);
  width: 200px;
  padding: 4px;
  border-radius: var(--radius-md);
  background: var(--color-surface-raised);
  box-shadow: var(--shadow-popover);
  border: 1px solid var(--color-border-subtle);
  z-index: var(--z-dropdown);
}
.tray-popover button {
  width: 100%;
  min-height: 40px;
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: var(--space-2);
  text-align: start;
}
.tray-pause-options {
  top: auto;
  bottom: calc(100% + 4px);
  width: 100%;
}
@media (hover: hover) and (pointer: fine) {
  .tray-popover button:hover,
  .tray-footer button:hover {
    background: var(--color-hover);
  }
}
.tray-status-text {
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
