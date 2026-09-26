export const breakFonts = ['system', 'serif', 'mono', 'pixel'] as const
export type BreakFont = (typeof breakFonts)[number]

export async function loadBreakFont(font: BreakFont) {
  if (font === 'pixel') {
    const faces = await document.fonts.load('24px "DCD Pixel"')
    if (!faces.length) throw new Error('像素字体加载失败')
  }
}
