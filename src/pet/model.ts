import builtin from './default.json'

export const petStates = ['ready', 'working', 'tired', 'resting', 'celebrate'] as const
export type PetState = (typeof petStates)[number]
export interface PetAnimation {
  image: string
  frames: number
  fps: number
}
export interface PetPack {
  version: 1
  name: string
  frameWidth: number
  frameHeight: number
  states: Record<PetState, PetAnimation>
}
export const defaultPet = builtin as PetPack
export const maxPackBytes = 4 * 1024 * 1024
const invalid = () => new Error('宠物素材无效')
const integer = (value: unknown, min: number, max: number): value is number =>
  Number.isInteger(value) && Number(value) >= min && Number(value) <= max

export function pngSize(image: string): [number, number] {
  if (!/^data:image\/png;base64,[A-Za-z0-9+/]+={0,2}$/.test(image) || image.length > maxPackBytes)
    throw invalid()
  const bytes = Uint8Array.from(atob(image.slice(22)), (c) => c.charCodeAt(0))
  if (
    bytes.length < 33 ||
    [137, 80, 78, 71, 13, 10, 26, 10].some((byte, i) => bytes[i] !== byte) ||
    String.fromCharCode(...bytes.slice(12, 16)) !== 'IHDR'
  )
    throw invalid()
  const view = new DataView(bytes.buffer)
  return [view.getUint32(16), view.getUint32(20)]
}
export function parsePetPack(source: string): PetPack {
  if (source.length > maxPackBytes) throw invalid()
  let pack: PetPack
  try {
    pack = JSON.parse(source)
  } catch {
    throw invalid()
  }
  if (
    !pack ||
    pack.version !== 1 ||
    typeof pack.name !== 'string' ||
    !pack.name.trim() ||
    pack.name.length > 40 ||
    !integer(pack.frameWidth, 8, 128) ||
    !integer(pack.frameHeight, 8, 128) ||
    !pack.states
  )
    throw invalid()
  const states = {} as Record<PetState, PetAnimation>
  for (const state of petStates) {
    const animation = pack.states[state]
    if (
      !animation ||
      typeof animation.image !== 'string' ||
      !integer(animation.frames, 1, 32) ||
      !integer(animation.fps, 1, 24)
    )
      throw invalid()
    const [width, height] = pngSize(animation.image)
    if (width !== pack.frameWidth * animation.frames || height !== pack.frameHeight) throw invalid()
    states[state] = { image: animation.image, frames: animation.frames, fps: animation.fps }
  }
  return {
    version: 1,
    name: pack.name.trim(),
    frameWidth: pack.frameWidth,
    frameHeight: pack.frameHeight,
    states,
  }
}
export async function decodePetPack(pack: PetPack) {
  await Promise.all(
    petStates.map(async (state) => {
      const image = new Image()
      image.src = pack.states[state].image
      await image.decode()
      if (
        image.naturalWidth !== pack.frameWidth * pack.states[state].frames ||
        image.naturalHeight !== pack.frameHeight
      )
        throw invalid()
    }),
  )
}
export function petState(
  energy: number,
  resting: boolean,
  tiredAt: number,
  celebrate = false,
): PetState {
  if (resting) return 'resting'
  if (celebrate) return 'celebrate'
  if (energy <= tiredAt) return 'tired'
  return energy >= 95 ? 'ready' : 'working'
}
