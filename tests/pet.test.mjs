import { describe, expect, it } from 'vitest'
import { defaultPet, parsePetPack, petState } from '../src/pet/model'

describe('pet state', () => {
  it('prioritizes rest, celebrates completed breaks and follows the tired threshold', () => {
    expect(petState(100, false, 20)).toBe('ready')
    expect(petState(60, false, 20)).toBe('working')
    expect(petState(20, false, 20)).toBe('tired')
    expect(petState(20, true, 20, true)).toBe('resting')
    expect(petState(100, false, 20, true)).toBe('celebrate')
    expect(petState(35, false, 40)).toBe('tired')
  })
})
describe('pet packs', () => {
  it('roundtrips all five builtin animations', () => {
    expect(parsePetPack(JSON.stringify(defaultPet))).toEqual(defaultPet)
  })
  it('rejects missing states, invalid dimensions, remote images and unsafe formats', () => {
    for (const change of [
      (p) => {
        delete p.states.resting
      },
      (p) => {
        p.frameWidth = 9999
      },
      (p) => {
        p.states.working.frames = 3
      },
      (p) => {
        p.states.ready.fps = 0
      },
      (p) => {
        p.states.ready.image = 'https://example.com/a.png'
      },
      (p) => {
        p.states.ready.image = 'data:image/svg+xml,<svg/>'
      },
    ]) {
      const pack = structuredClone(defaultPet)
      change(pack)
      expect(() => parsePetPack(JSON.stringify(pack))).toThrow()
    }
    expect(() => parsePetPack('null')).toThrow()
    expect(() => parsePetPack('x'.repeat(4 * 1024 * 1024 + 1))).toThrow()
  })
})
