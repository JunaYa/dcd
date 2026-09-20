<script setup lang="ts">
import type { Bar } from '~/eye/model'
import { computed } from 'vue'
import { t } from '~/i18n'

const props = withDefaults(
  defineProps<{
    samples?: [number, number][]
    bars?: Bar[]
    metric?: 'seconds' | 'peakSeconds'
    compact?: boolean
  }>(),
  { metric: 'seconds' },
)
const width = computed(() =>
  props.bars ? Math.max(900, props.bars.length * 80) : props.compact ? 520 : 1000,
)
const left = 48
const right = computed(() => width.value - 38)
const top = 32
const bottom = props.compact ? 200 : 310
const max = computed(() => {
  if (!props.bars)
    return 100
  const values = props.bars.map(
    bar => bar[props.metric] / (props.metric === 'seconds' ? 3600 : 60),
  )
  const floor = props.metric === 'seconds' ? 15 : 300
  return Math.max(floor, Math.ceil(Math.max(...values, 0) / 5) * 5)
})
const points = computed(() =>
  (props.samples || []).map(
    ([minute, fatigue]) =>
      `${left + (minute / 1440) * (right.value - left)},${bottom - (fatigue / 100) * (bottom - top)}`,
  ),
)
const area = computed(() =>
  !points.value.length
    ? ''
    : `M${points.value[0].split(',')[0]},${bottom} L${points.value.join(' L')} L${points.value.at(-1)?.split(',')[0]},${bottom} Z`,
)
const grid = computed(() =>
  props.bars ? [0, 1 / 3, 2 / 3, 1] : [0, 0.2, 0.4, 0.6, 0.8, 1],
)
const tickStep = computed(() =>
  Math.max(1, Math.ceil((props.bars?.length || 0) / 16)),
)
</script>

<template>
  <div class="chart-scroll" :class="{ compact }">
    <svg
      class="usage-chart"
      :style="bars ? { minWidth: `${Math.max(550, bars.length * 65)}px` } : {}"
      :viewBox="`0 0 ${width} ${bottom + 45}`"
      role="img"
      :aria-label="
        bars
          ? t('柱状图', { metric: t(metric === 'seconds' ? '使用时长' : '疲劳峰值时长') })
          : t('今日疲劳值变化折线图')
      "
    >
      <defs>
        <linearGradient id="fatigue-fill" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="var(--color-accent)" stop-opacity=".32" />
          <stop offset="100%" stop-color="var(--color-accent)" stop-opacity=".025" />
        </linearGradient>
      </defs>
      <g v-for="fraction in grid" :key="fraction">
        <line
          :x1="left"
          :x2="right"
          :y1="bottom - fraction * (bottom - top)"
          :y2="bottom - fraction * (bottom - top)"
          class="grid-line"
        />
        <text
          :x="bars ? right + 7 : left - 8"
          :y="bottom - fraction * (bottom - top) + 5"
          :text-anchor="bars ? 'start' : 'end'"
        >
          {{
            bars ? Math.round(max * fraction) : `${Math.round(fraction * 100)}%`
          }}
        </text>
      </g>
      <template v-if="bars">
        <text :x="right" y="15" text-anchor="end">
          {{ t('时长单位', { unit: t(metric === 'seconds' ? '小时' : '分钟') }) }}
        </text>
        <g v-for="(bar, index) in bars" :key="bar.key">
          <line
            :x1="left + (index / bars.length) * (right - left)"
            :x2="left + (index / bars.length) * (right - left)"
            :y1="top"
            :y2="bottom"
            class="grid-line vertical"
          />
          <rect
            :x="left + ((index + 0.15) / bars.length) * (right - left)"
            :y="
              bottom
                - (bar[metric] / (metric === 'seconds' ? 3600 : 60) / max)
                * (bottom - top)
            "
            :width="((right - left) / bars.length) * 0.7"
            :height="
              (bar[metric] / (metric === 'seconds' ? 3600 : 60) / max)
                * (bottom - top)
            "
            rx="2"
            fill="var(--color-accent)"
          >
            <title>
              {{ bar.label }}：{{
                (bar[metric] / (metric === "seconds" ? 3600 : 60)).toFixed(1)
              }}{{ metric === "seconds" ? t('小时') : t('分钟') }}
            </title>
          </rect>
          <text
            v-if="index % tickStep === 0"
            :x="left + ((index + 0.5) / bars.length) * (right - left)"
            :y="bottom + 25"
            text-anchor="middle"
          >
            {{ bar.label }}
          </text>
        </g>
      </template>
      <template v-else>
        <path v-if="area" :d="area" fill="url(#fatigue-fill)" />
        <polyline
          v-if="points.length"
          :points="points.join(' ')"
          fill="none"
          stroke="var(--color-accent)"
          stroke-width="2"
          stroke-linejoin="round"
        />
        <circle
          v-if="points.length"
          :cx="points[points.length - 1].split(',')[0]"
          :cy="points[points.length - 1].split(',')[1]"
          r="4"
          fill="var(--color-accent)"
        />
        <text
          v-for="hour in 13"
          :key="hour"
          :x="left + ((hour - 1) / 12) * (right - left)"
          :y="bottom + 25"
          text-anchor="middle"
        >
          {{ (hour - 1) * 2 }}
        </text>
      </template>
      <text
        v-if="bars ? bars.every((bar) => bar[metric] === 0) : !samples?.length"
        :x="width / 2"
        :y="(top + bottom) / 2"
        text-anchor="middle"
        class="empty-chart"
      >
        {{
          bars ? t('此时间段暂无记录') : t('开始使用电脑后，将在这里记录今日使用情况')
        }}
      </text>
    </svg>
  </div>
</template>
