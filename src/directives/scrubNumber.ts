import type { ObjectDirective } from 'vue'

export function scrubValue(start: number, pixels: number, min: number, max: number, step: number) {
  const value = start + Math.trunc(pixels / 8) * step
  return Math.min(max, Math.max(min, Number(value.toFixed(8))))
}

const cleanups = new WeakMap<HTMLInputElement, () => void>()

export const vScrubNumber: ObjectDirective<HTMLInputElement> = {
  mounted(input) {
    let gesture:
      | { pointer: number; x: number; value: number; original: string; dragged: boolean }
      | undefined
    function write(value: string) {
      input.value = value
      input.dispatchEvent(new Event('input', { bubbles: true }))
    }
    function finish(cancel = false) {
      if (!gesture) return
      const current = gesture
      gesture = undefined
      input.classList.remove('scrubbing')
      if (input.hasPointerCapture(current.pointer)) input.releasePointerCapture(current.pointer)
      if (cancel && current.dragged) write(current.original)
      if (!cancel && current.dragged && input.value !== current.original) {
        input.dispatchEvent(new Event('change', { bubbles: true }))
      }
    }
    function down(event: PointerEvent) {
      if (
        event.pointerType !== 'mouse' ||
        event.button !== 0 ||
        input.disabled ||
        input.readOnly ||
        !Number.isFinite(input.valueAsNumber)
      )
        return
      gesture = {
        pointer: event.pointerId,
        x: event.clientX,
        value: input.valueAsNumber,
        original: input.value,
        dragged: false,
      }
      input.setPointerCapture(event.pointerId)
    }
    function move(event: PointerEvent) {
      if (!gesture || gesture.pointer !== event.pointerId) return
      if (input.disabled) {
        finish(true)
        return
      }
      const delta = event.clientX - gesture.x
      if (!gesture.dragged && Math.abs(delta) < 6) return
      gesture.dragged = true
      event.preventDefault()
      input.classList.add('scrubbing')
      const min = input.min === '' ? -Infinity : Number(input.min)
      const max = input.max === '' ? Infinity : Number(input.max)
      const step = Number(input.step) > 0 ? Number(input.step) : 1
      const value = scrubValue(gesture.value, delta, min, max, step)
      if (String(value) !== input.value) write(String(value))
    }
    function up(event: PointerEvent) {
      if (gesture?.pointer === event.pointerId) finish()
    }
    function cancel() {
      finish(true)
    }
    function key(event: KeyboardEvent) {
      if (event.key === 'Escape' && gesture) {
        event.preventDefault()
        event.stopPropagation()
        finish(true)
      }
    }
    input.classList.add('scrub-number')
    input.addEventListener('pointerdown', down)
    input.addEventListener('pointermove', move)
    input.addEventListener('pointerup', up)
    input.addEventListener('pointercancel', cancel)
    input.addEventListener('lostpointercapture', cancel)
    input.addEventListener('keydown', key)
    window.addEventListener('blur', cancel)
    cleanups.set(input, () => {
      finish(true)
      input.removeEventListener('pointerdown', down)
      input.removeEventListener('pointermove', move)
      input.removeEventListener('pointerup', up)
      input.removeEventListener('pointercancel', cancel)
      input.removeEventListener('lostpointercapture', cancel)
      input.removeEventListener('keydown', key)
      window.removeEventListener('blur', cancel)
      input.classList.remove('scrub-number')
    })
  },
  beforeUnmount(input) {
    cleanups.get(input)?.()
    cleanups.delete(input)
  },
}
