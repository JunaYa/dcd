import { expect, it } from 'vitest'
import { accentTokens, contrastRatio, defaultAccent } from '../src/theme/accent.ts'

it('keeps extreme custom colors readable in both themes', () => {
  for (const color of [
    '#ffffff',
    '#000000',
    '#ffff00',
    '#00ff00',
    '#0000ff',
    '#ff00ff',
    defaultAccent,
  ]) {
    for (const dark of [false, true]) {
      const tokens = accentTokens(color, dark)
      expect(contrastRatio(tokens['--color-accent-strong'], '#ffffff')).toBeGreaterThanOrEqual(4.5)
      expect(
        contrastRatio(tokens['--color-accent'], dark ? '#303035' : '#ffffff'),
      ).toBeGreaterThanOrEqual(4.5)
    }
  }
})
it('uses the default palette for malformed stored colors', () => {
  expect(accentTokens('invalid', true)).toEqual(accentTokens(defaultAccent, true))
})
