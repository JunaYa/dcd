import { afterEach, expect, it, vi } from 'vitest'
import { defaults } from '../src/eye/model.ts'
import { loadBreakFont } from '../src/theme/breakTypography.ts'

afterEach(() => vi.unstubAllGlobals())
it('preserves system typography and no motion for existing settings', () => {
  expect(defaults.breakFont).toBe('system')
  expect(defaults.breakPixelAnimation).toBe(false)
})
it('loads pixel typography before use and reports missing font assets', async () => {
  const load = vi.fn().mockResolvedValue([{}])
  vi.stubGlobal('document', { fonts: { load } })
  await loadBreakFont('system')
  expect(load).not.toHaveBeenCalled()
  await loadBreakFont('pixel')
  expect(load).toHaveBeenCalledWith('24px "DCD Pixel"')
  load.mockResolvedValue([])
  await expect(loadBreakFont('pixel')).rejects.toThrow('像素字体加载失败')
})
