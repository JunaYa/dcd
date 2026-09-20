<script setup lang="ts">
import { useMonitor } from '../composables/useMonitor'
import { t } from '~/i18n'
import { computed } from 'vue'
import { save as saveDialog } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { aggregate, dateKey } from '~/eye/model'
import { formatBarDate } from '~/i18n'
import Chart from '~/components/eye/Chart.vue'
import Icon from '~/components/eye/Icon.vue'
const { native, display, demo, inform, report } = useMonitor()
const range = defineModel<number>('range', { required: true })
const grouping = defineModel<string>('grouping', { required: true })
const ranges = computed(() => [
  { value: 8, label: t('过去7天') },
  { value: 29, label: t('过去28天') },
  { value: 91, label: t('过去90天') },
  { value: 365, label: t('过去12个月') },
  { value: 0, label: t('全部时间') },
])
const groups = computed(() => [
  { value: 'day', label: t('按天') },
  { value: 'week', label: t('按周') },
  { value: 'month', label: t('按月') },
])
const bars = computed(() =>
  aggregate(display.value.days, range.value, grouping.value).map((bar) => ({
    ...bar,
    label: formatBarDate(bar.key, grouping.value),
  })),
)
async function exportCsv() {
  const rows = [
    [t('日期'), t('使用时长（分钟）'), t('疲劳峰值时长（分钟）'), t('完成休息次数')],
    ...bars.value.map((b) => [
      b.key,
      (b.seconds / 60).toFixed(1),
      (b.peakSeconds / 60).toFixed(1),
      b.breaks,
    ]),
  ]
  const csv = `\uFEFF${rows.map((row) => row.join(',')).join('\r\n')}`
  const filename = `dcd-${demo.value ? 'demo-' : ''}${dateKey()}.csv`
  try {
    if (native) {
      const path = await saveDialog({
        defaultPath: filename,
        filters: [{ name: 'CSV', extensions: ['csv'] }],
      })
      if (!path) return
      await writeTextFile(path, csv)
    } else {
      const url = URL.createObjectURL(new Blob([csv], { type: 'text/csv;charset=utf-8' }))
      const link = document.createElement('a')
      link.href = url
      link.download = filename
      link.click()
      setTimeout(() => URL.revokeObjectURL(url), 1000)
    }
    inform(t('已导出当前筛选范围的 CSV'))
  } catch (cause) {
    report(cause)
  }
}
</script>

<template>
  <header class="page-heading">
    <div>
      <h1>{{ t('使用分析') }}</h1>
      <p>{{ t('回顾使用时长与疲劳趋势。') }}</p>
    </div>
  </header>
  <div class="analysis-controls">
    <div class="filter-row">
      <Icon name="calendar" />
      <div class="segmented" :aria-label="t('统计范围')">
        <button
          v-for="item in ranges"
          :key="item.value"
          :class="{ selected: range === item.value }"
          :aria-pressed="range === item.value"
          @click="range = item.value"
        >
          {{ item.label }}
        </button>
      </div>
    </div>
    <div class="filter-row">
      <Icon name="list" />
      <div class="segmented" :aria-label="t('分组方式')">
        <button
          v-for="item in groups"
          :key="item.value"
          :class="{ selected: grouping === item.value }"
          :aria-pressed="grouping === item.value"
          @click="grouping = item.value"
        >
          {{ item.label }}
        </button>
      </div>
      <button
        class="button export-button"
        :title="t('导出 CSV')"
        :aria-label="t('导出 CSV')"
        @click="exportCsv"
      >
        <Icon name="download" />
      </button>
    </div>
  </div>
  <div class="analysis-meta">
    <span>{{ demo ? t('正在查看示例数据') : t('使用记录仅保存在本机') }}</span
    ><button class="demo-button" :aria-pressed="demo" @click="demo = !demo">
      {{ demo ? t('返回实时数据') : t('预览示例数据') }}
    </button>
  </div>
  <section class="analysis-chart">
    <h1>
      {{ t('使用时长') }}
      <Icon name="info" :title="t('当前筛选范围内的实际电脑使用时长')" />
    </h1>
    <Chart :bars="bars" />
  </section>
  <section class="analysis-chart">
    <h1>
      {{ t('疲劳峰值时长') }}
      <Icon name="info" :title="t('疲劳值达到 100% 后仍在使用电脑的时长')" />
    </h1>
    <Chart :bars="bars" metric="peakSeconds" />
  </section>
</template>
