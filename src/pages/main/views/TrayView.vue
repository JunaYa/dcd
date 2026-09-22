<script setup lang="ts">
import { computed } from 'vue'
import { formatDuration, locale, t } from '~/i18n'
import { useMonitor } from '../composables/useMonitor'

const { snapshot, settings, paused } = useMonitor()
const today = computed(() => snapshot.value.days[snapshot.value.date])
const fatigue = computed(() => Math.min(100, Math.max(0, snapshot.value.fatigue)))
const samples = computed(() =>
  snapshot.value.samples.map(([minute, value]) => ({
    x: 2 + (minute / 1440) * 324,
    y: 62 - (Math.min(100, Math.max(0, value)) / 100) * 60,
  })),
)
const points = computed(() => samples.value.map(({ x, y }) => `${x},${y}`).join(' '))
const area = computed(() =>
  samples.value.length > 1
    ? `M${samples.value[0].x},64 L${samples.value.map(({ x, y }) => `${x},${y}`).join(' L')} L${samples.value.at(-1)!.x},64 Z`
    : '',
)
const status = computed(() =>
  !settings.value.reminders
    ? t('提醒已关闭')
    : paused.value
      ? t('暂停至', {
          time: new Date(snapshot.value.pausedUntil * 1000).toLocaleString(locale.value, {
            month: 'numeric',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
          }),
        })
      : t('提醒正常'),
)
</script>

<template>
  <div class="tray-panel">
    <header class="tray-heading">
      <strong>{{ t('appName') }}</strong>
      <span class="tray-status" :title="status"
        ><i class="status-dot" :class="{ paused: paused || !settings.reminders }" /><span>{{
          status
        }}</span></span
      >
    </header>
    <section class="tray-summary" :aria-label="t('疲劳值')">
      <div class="fatigue-ring" :title="t('疲劳值按连续使用时长估算')">
        <svg viewBox="0 0 104 104" aria-hidden="true">
          <circle cx="52" cy="52" r="47" class="ring-track" />
          <circle
            cx="52"
            cy="52"
            r="47"
            class="ring-value"
            pathLength="100"
            :stroke-dasharray="`${fatigue} 100`"
            transform="rotate(-90 52 52)"
          />
        </svg>
        <div>
          <strong>{{ fatigue }}<small>%</small></strong
          ><span>{{ t('疲劳值') }}</span>
        </div>
      </div>
      <div class="tray-metrics">
        <span class="metric-label">{{ t('今日时长') }}</span>
        <strong>{{ formatDuration(today?.seconds ?? 0) }}</strong>
        <span class="break-count">{{ t('休息次数', { count: today?.breaks ?? 0 }) }}</span>
      </div>
    </section>
    <section v-if="settings.trayShowChart" class="tray-trend" :aria-label="t('今日使用')">
      <div class="trend-caption">
        <span>{{ t('今日使用') }}</span
        ><span>0–24h</span>
      </div>
      <svg
        v-if="samples.length"
        viewBox="0 0 328 64"
        role="img"
        :aria-label="t('今日疲劳值变化折线图')"
      >
        <defs>
          <linearGradient id="tray-trend-fill" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="var(--color-accent)" stop-opacity=".18" />
            <stop offset="100%" stop-color="var(--color-accent)" stop-opacity="0" />
          </linearGradient>
        </defs>
        <path d="M2 63H326" stroke="var(--color-border-subtle)" />
        <path :d="area" fill="url(#tray-trend-fill)" />
        <polyline
          :points="points"
          fill="none"
          stroke="var(--color-accent)"
          stroke-width="1.5"
          stroke-linejoin="round"
          stroke-linecap="round"
        />
        <circle
          :cx="samples[samples.length - 1].x"
          :cy="samples[samples.length - 1].y"
          r="2"
          fill="var(--color-accent)"
        />
      </svg>
      <p v-else>{{ t('使用后将显示今日趋势') }}</p>
    </section>
    <footer>{{ t('右击托盘图标以休息或管理提醒') }}</footer>
  </div>
</template>

<style scoped>
.tray-panel {
  display: grid;
  gap: var(--space-4);
}
.tray-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  height: 20px;
  font-size: 12px;
}
.tray-heading strong {
  flex-shrink: 0;
  font-weight: 600;
}
.tray-status {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
  color: var(--color-text-muted);
}
.tray-status > span {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.status-dot {
  flex-shrink: 0;
}
.tray-summary {
  display: flex;
  align-items: center;
  gap: var(--space-6);
  min-height: 104px;
}
.fatigue-ring {
  position: relative;
  flex: 0 0 104px;
  height: 104px;
}
.fatigue-ring svg {
  width: 104px;
  height: 104px;
  pointer-events: none;
}
.fatigue-ring circle {
  fill: none;
  stroke-width: 4;
}
.ring-track {
  stroke: var(--color-border-subtle);
}
.ring-value {
  stroke: var(--color-accent);
}
.fatigue-ring > div {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
}
.fatigue-ring strong {
  font-size: 30px;
  line-height: 1.1;
  letter-spacing: -0.04em;
  font-variant-numeric: tabular-nums;
}
.fatigue-ring small {
  font-size: 15px;
  margin-inline-start: 1px;
}
.fatigue-ring span,
.metric-label {
  font-size: 12px;
  color: var(--color-text-muted);
}
.tray-metrics {
  min-width: 0;
  display: grid;
  gap: 6px;
}
.tray-metrics strong {
  font-size: 21px;
  line-height: 1.3;
  font-variant-numeric: tabular-nums;
  letter-spacing: -0.02em;
  overflow-wrap: anywhere;
}
.break-count {
  font-size: 12px;
  color: var(--color-text-secondary);
}
.tray-trend {
  height: 96px;
  border-top: 1px solid var(--color-border-subtle);
  padding-top: 10px;
}
.trend-caption {
  display: flex;
  align-items: center;
  justify-content: space-between;
  color: var(--color-text-muted);
  font-size: 11px;
  height: 20px;
}
.trend-caption > :last-child {
  direction: ltr;
  font-variant-numeric: tabular-nums;
}
.tray-trend svg {
  width: 100%;
  height: 64px;
  display: block;
  direction: ltr;
}
.tray-trend p {
  height: 64px;
  display: grid;
  place-items: center;
  text-align: center;
  font-size: 12px;
  color: var(--color-text-muted);
  margin: 0;
}
footer {
  height: 24px;
  font-size: 11px;
  line-height: 1.3;
  color: var(--color-text-muted);
  text-align: center;
  display: grid;
  place-items: center;
}
</style>
