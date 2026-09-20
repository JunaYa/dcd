import { computed, reactive } from 'vue'
import { describe, expect, it } from 'vitest'
import { emptySnapshot } from '../src/eye/model'
import { applySnapshot } from '../src/pages/main/composables/snapshot'

function fixture() {
  const state = emptySnapshot()
  state.days = { '2026-09-20': { seconds: 60, peakSeconds: 0, breaks: 1 } }
  state.samples = [[600, 20]]
  return state
}

describe('monitor snapshot updates', () => {
  it('clock updates do not invalidate chart and history calculations', () => {
    const state = reactive(fixture())
    let calculations = 0
    const chart = computed(() => {
      calculations++
      return { days: state.days, samples: state.samples }
    })
    const first = chart.value
    applySnapshot(state, { ...fixture(), now: state.now + 1, fatigue: 25 })
    expect(chart.value).toBe(first)
    expect(calculations).toBe(1)
    expect(state.fatigue).toBe(25)
  })

  it('updates changed history without replacing unchanged curve samples', () => {
    const state = reactive(fixture())
    const samples = state.samples
    const incoming = fixture()
    incoming.days['2026-09-20'].seconds = 120
    applySnapshot(state, incoming)
    expect(state.days['2026-09-20'].seconds).toBe(120)
    expect(state.samples).toBe(samples)
    incoming.samples = [
      [600, 30],
      [601, 40],
    ]
    applySnapshot(state, incoming)
    expect(state.samples).toEqual(incoming.samples)
  })

  it('handles midnight, removed history and settings changes', () => {
    const state = reactive(fixture())
    const incoming = emptySnapshot()
    incoming.date = '2026-09-21'
    incoming.settings.mainTheme = 'light'
    applySnapshot(state, incoming)
    expect(state.days).toEqual({})
    expect(state.samples).toEqual([])
    expect(state.date).toBe('2026-09-21')
    expect(state.settings.mainTheme).toBe('light')
  })
})
