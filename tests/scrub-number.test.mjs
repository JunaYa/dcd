import { expect, it } from 'vitest'
import { scrubValue } from '../src/directives/scrubNumber.ts'

it('scrubs in whole steps and ignores small pointer movement', () => {
  expect(scrubValue(25, 7, 1, 240, 1)).toBe(25)
  expect(scrubValue(25, 40, 1, 240, 1)).toBe(30)
  expect(scrubValue(25, -40, 1, 240, 1)).toBe(20)
  expect(scrubValue(1, 24, 0, 10, 0.1)).toBe(1.3)
})
it('respects numeric bounds and restores the starting value when dragged back', () => {
  expect(scrubValue(5, -1000, 1, 240, 1)).toBe(1)
  expect(scrubValue(58, 1000, 0, 59, 1)).toBe(59)
  expect(scrubValue(25, 0, 1, 240, 1)).toBe(25)
})
