import { createRenderer, h, ref } from 'vue'
import { afterEach, beforeEach, expect, it, vi } from 'vitest'
import { useClockScreen } from '../src/pages/main/composables/useClockScreen'
import { defaults } from '../src/eye/model'

let monitor
vi.mock('../src/pages/main/composables/useMonitor', () => ({ useMonitor: () => monitor }))
let app
let elements
const renderer = createRenderer({
  createElement(tag) {
    const element = { tag, props: {}, text: '', focus: vi.fn() }
    elements.push(element)
    return element
  },
  createText: (text) => ({ text }),
  createComment: () => ({}),
  insert() {},
  remove() {},
  setText: (node, text) => {
    node.text = text
  },
  setElementText: (node, text) => {
    node.text = text
  },
  patchProp: (node, key, previous, value) => {
    node.props[key] = value
  },
  parentNode: () => null,
  nextSibling: () => null,
})
beforeEach(() => {
  vi.useFakeTimers()
  vi.setSystemTime(new Date(2026, 9, 5, 23, 59, 59))
  vi.stubGlobal('document', new EventTarget())
  vi.stubGlobal('window', new EventTarget())
  monitor = {
    settings: ref({ ...defaults, language: 'en' }),
    action: vi.fn().mockResolvedValue(),
    error: ref(''),
  }
  elements = []
  app = renderer.createApp({
    setup() {
      const clock = useClockScreen()
      return () =>
        h('main', [
          h('time', clock.time.value),
          h('p', { class: 'clock-date' }, clock.date.value),
          h('button', { ref: 'clockClose', onClick: clock.close }),
        ])
    },
  })
  app.mount({})
})
afterEach(() => {
  app?.unmount()
  vi.useRealTimers()
  vi.unstubAllGlobals()
})
it('updates time and date across midnight and clears the timer on exit', async () => {
  const time = elements.find((element) => element.tag === 'time')
  const date = elements.find((element) => element.props.class === 'clock-date')
  expect(time.text).toBe('23:59:59')
  expect(date.text).toContain('October 5')
  await vi.advanceTimersByTimeAsync(1000)
  expect(time.text).toBe('00:00:00')
  expect(date.text).toContain('October 6')
  app.unmount()
  app = undefined
  expect(vi.getTimerCount()).toBe(0)
})
it('focuses the exit button and supports button and Escape without held-key repeats', async () => {
  const button = elements.find((element) => element.tag === 'button')
  expect(button.focus).toHaveBeenCalledOnce()
  await button.props.onClick()
  expect(monitor.action).toHaveBeenCalledWith('close-clock')
  window.dispatchEvent(Object.assign(new Event('keydown'), { key: 'Escape', repeat: true }))
  expect(monitor.action).toHaveBeenCalledTimes(1)
  window.dispatchEvent(Object.assign(new Event('keydown'), { key: 'Escape', repeat: false }))
  expect(monitor.action).toHaveBeenCalledTimes(2)
  app.unmount()
  app = undefined
  window.dispatchEvent(Object.assign(new Event('keydown'), { key: 'Escape', repeat: false }))
  expect(monitor.action).toHaveBeenCalledTimes(2)
})
