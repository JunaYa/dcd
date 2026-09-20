import assert from 'node:assert/strict'
import { it } from 'vitest'
import { aggregate, dateKey, duration } from '../src/eye/model.ts'

const today = new Date(2026, 8, 20, 12)
const day = { seconds: 3600, peakSeconds: 120, breaks: 2 }
it('day range includes today and fills missing days without counting older records', () => {
  const bars = aggregate({ '2026-09-19': day, '2026-09-01': day }, 3, 'day', today)
  assert.equal(bars.length, 3)
  assert.equal(bars[0].key, '2026-09-18')
  assert.equal(bars[2].key, '2026-09-20')
  assert.equal(bars.reduce((sum, bar) => sum + bar.seconds, 0), 3600)
})
it('weekly aggregation uses Monday and preserves totals', () => {
  const bars = aggregate({ '2026-09-19': day, '2026-09-20': day }, 8, 'week', today)
  assert.equal(bars[1].key, '2026-09-14')
  assert.equal(bars[1].seconds, 7200)
  assert.equal(bars[1].peakSeconds, 240)
  assert.equal(bars[1].breaks, 4)
})
it('all-time monthly aggregation spans year boundaries', () => {
  const bars = aggregate({ '2025-12-31': day, '2026-01-01': day }, 0, 'month', new Date(2026, 0, 2, 12))
  assert.deepEqual(bars.map(bar => bar.key), ['2025-12', '2026-01'])
  assert.equal(bars[0].seconds, 3600)
  assert.equal(bars[1].seconds, 3600)
})
it('empty history yields a usable empty chart', () => {
  const bars = aggregate({}, 0, 'day', today)
  assert.equal(bars.length, 8)
  assert.ok(bars.every(bar => bar.seconds === 0))
})
it('duration and date use local display values', () => {
  assert.equal(duration(35040), '9小时 44分')
  assert.equal(duration(59), '0分')
  assert.equal(dateKey(today), '2026-09-20')
})
