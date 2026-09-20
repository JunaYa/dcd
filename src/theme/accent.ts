export const defaultAccent = '#8842b6'
export const accentPresets = [
  defaultAccent,
  '#2563eb',
  '#0891b2',
  '#16845b',
  '#c16b12',
  '#d14343',
  '#c23b80',
]
type RGB = [number, number, number]
function rgb(hex: string): RGB {
  return [1, 3, 5].map((offset) => Number.parseInt(hex.slice(offset, offset + 2), 16)) as RGB
}
function hex(color: RGB): string {
  return `#${color.map((value) => Math.round(value).toString(16).padStart(2, '0')).join('')}`
}
function luminance(color: RGB): number {
  const linear = color.map((value) => {
    const channel = value / 255
    return channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4
  })
  return linear[0]! * 0.2126 + linear[1]! * 0.7152 + linear[2]! * 0.0722
}
export function contrastRatio(a: string, b: string): number {
  const values = [luminance(rgb(a)), luminance(rgb(b))].sort((x, y) => y - x)
  return (values[0]! + 0.05) / (values[1]! + 0.05)
}
function readable(base: string, background: string, toward: number): string {
  const source = rgb(base)
  for (let step = 0; step <= 100; step++) {
    const color = hex(source.map((channel) => channel + ((toward - channel) * step) / 100) as RGB)
    if (contrastRatio(color, background) >= 4.5) return color
  }
  return toward === 255 ? '#ffffff' : '#000000'
}
export function accentTokens(value: string, dark: boolean): Record<string, string> {
  const base = /^#[\da-f]{6}$/i.test(value) ? value : defaultAccent
  const accent = readable(base, dark ? '#303035' : '#ffffff', dark ? 255 : 0)
  return {
    '--color-accent': accent,
    '--color-accent-strong': readable(base, '#ffffff', 0),
    '--color-accent-subtle': `color-mix(in srgb, ${accent} ${dark ? 20 : 12}%, var(--color-canvas))`,
  }
}
