import { createRenderer } from 'vue'
import { afterEach, beforeEach, expect, it, vi } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { emptySnapshot } from '../src/eye/model'
import { provideMonitor } from '../src/pages/main/composables/useMonitor'

vi.mock('@tauri-apps/api/core', () => ({ isTauri: () => true, invoke: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }))

const renderer = createRenderer({
  createElement: () => ({}),
  createText: () => ({}),
  createComment: () => ({}),
  insert() {},
  remove() {},
  setText() {},
  setElementText() {},
  patchProp() {},
  parentNode: () => null,
  nextSibling: () => null,
})
let app
let monitor
let documentTarget
let unlisten
const settle = () => vi.advanceTimersByTimeAsync(0)
function mount() {
  app = renderer.createApp({
    setup() {
      monitor = provideMonitor(emptySnapshot())
      return () => null
    },
  })
  app.mount({})
}
beforeEach(() => {
  vi.useFakeTimers()
  vi.resetAllMocks()
  documentTarget = Object.assign(new EventTarget(), { hidden: false })
  vi.stubGlobal('document', documentTarget)
  vi.stubGlobal('location', new URL('http://localhost/main.html'))
  unlisten = vi.fn()
  listen.mockResolvedValue(unlisten)
  invoke.mockImplementation(async () => emptySnapshot())
})
afterEach(() => {
  app?.unmount()
  app = undefined
  vi.useRealTimers()
  vi.unstubAllGlobals()
})
it('skips polling hidden windows, refreshes on return and cleans up on unmount', async () => {
  mount()
  await settle()
  await vi.advanceTimersByTimeAsync(1000)
  expect(invoke).toHaveBeenCalledTimes(1)
  documentTarget.hidden = true
  documentTarget.dispatchEvent(new Event('visibilitychange'))
  expect(vi.getTimerCount()).toBe(0)
  await vi.advanceTimersByTimeAsync(3000)
  expect(invoke).toHaveBeenCalledTimes(1)
  documentTarget.hidden = false
  documentTarget.dispatchEvent(new Event('visibilitychange'))
  await settle()
  expect(invoke).toHaveBeenCalledTimes(2)
  app.unmount()
  app = undefined
  await vi.advanceTimersByTimeAsync(3000)
  expect(invoke).toHaveBeenCalledTimes(2)
  expect(unlisten).toHaveBeenCalledTimes(2)
})
it('unsubscribes a listener that resolves after the window has unmounted', async () => {
  let resolveListener
  listen.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        resolveListener = resolve
      }),
  )
  mount()
  app.unmount()
  app = undefined
  resolveListener(unlisten)
  await settle()
  expect(unlisten).toHaveBeenCalledOnce()
  expect(listen).toHaveBeenCalledTimes(1)
  expect(vi.getTimerCount()).toBe(0)
})
it('preserves an unsaved settings draft during refresh', async () => {
  mount()
  await settle()
  monitor.settings.value.workMinutes = 35
  await vi.advanceTimersByTimeAsync(1000)
  expect(monitor.settings.value.workMinutes).toBe(35)
  expect(monitor.dirty.value).toBe(true)
})
it('does not overwrite a completed save with an older in-flight snapshot', async () => {
  let resolveSnapshot
  invoke.mockImplementation((command) =>
    command === 'eye_snapshot'
      ? new Promise((resolve) => {
          resolveSnapshot = resolve
        })
      : Promise.resolve(),
  )
  mount()
  await settle()
  await vi.advanceTimersByTimeAsync(1000)
  monitor.settings.value.workMinutes = 40
  await monitor.saveSettings()
  resolveSnapshot(emptySnapshot())
  await settle()
  expect(monitor.settings.value.workMinutes).toBe(40)
  expect(monitor.dirty.value).toBe(false)
})
