import type { BreakFont } from '../theme/breakTypography'
import { resolveLanguage } from '../i18n/locale.ts'

export type Theme = 'system' | 'light' | 'dark'

export interface Settings {
  shortcutMain: string
  shortcutBreak: string
  shortcutSkip: string
  shortcutPause: string
  shortcutClock: string
  petEnabled: boolean
  petName: string
  petSize: number
  petPosition: 'top' | 'bottom'
  petShowEnergy: boolean
  petShowOnBreak: boolean
  petTiredThreshold: number
  petPack: string
  language: string
  accentColor: string
  mainTheme: Theme
  trayTheme: Theme
  trayShowChart: boolean
  overlayOpacity: number
  overlayBlur: number
  breakFont: BreakFont
  breakPixelAnimation: boolean
  workMinutes: number
  breakMinutes: number
  breakSeconds: number
  repeatMinutes: number
  preNotify: boolean
  sound: boolean
  movieMode: boolean
  pauseMedia: boolean
  autostart: boolean
  trayIcon: boolean
  trayTime: boolean
  dockIcon: boolean
  reminders: boolean
  reminderStyle: 'fullscreen' | 'window'
  message: string
  allowSkip: boolean
  background: 'system' | 'light' | 'dark' | 'custom'
  backgroundImage: string
}
export interface Day {
  seconds: number
  peakSeconds: number
  breaks: number
}
export interface Snapshot {
  settings: Settings
  days: Record<string, Day>
  date: string
  samples: [number, number][]
  workSeconds: number
  pausedUntil: number
  breakUntil: number
  fatigue: number
  now: number
  storageError: string | null
  shortcutError: string | null
}
export const defaults: Settings = {
  shortcutMain: 'CmdOrCtrl+Shift+A',
  shortcutBreak: 'CmdOrCtrl+Shift+B',
  shortcutSkip: 'CmdOrCtrl+Shift+S',
  shortcutPause: 'CmdOrCtrl+Shift+P',
  shortcutClock: 'CmdOrCtrl+Shift+L',
  petEnabled: false,
  petName: 'Mori',
  petSize: 96,
  petPosition: 'top',
  petShowEnergy: true,
  petShowOnBreak: true,
  petTiredThreshold: 20,
  petPack: '',
  language: resolveLanguage(typeof navigator === 'undefined' ? ['en'] : navigator.languages),
  accentColor: '#8842b6',
  mainTheme: 'dark',
  trayTheme: 'dark',
  trayShowChart: true,
  overlayOpacity: 82,
  overlayBlur: 16,
  breakFont: 'system',
  breakPixelAnimation: false,
  workMinutes: 25,
  breakMinutes: 5,
  breakSeconds: 0,
  repeatMinutes: 1,
  preNotify: true,
  sound: true,
  movieMode: false,
  pauseMedia: false,
  autostart: false,
  trayIcon: true,
  trayTime: true,
  dockIcon: true,
  reminders: true,
  reminderStyle: 'fullscreen',
  message: '',
  allowSkip: true,
  background: 'system',
  backgroundImage: '',
}
export function dateKey(date = new Date()) {
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
}
export function duration(seconds: number) {
  const minutes = Math.floor(seconds / 60)
  return minutes >= 60
    ? `${Math.floor(minutes / 60)}小时 ${minutes % 60}分`
    : `${minutes}分`
}
export function emptySnapshot(): Snapshot {
  return {
    settings: { ...defaults },
    days: {},
    date: dateKey(),
    samples: [],
    workSeconds: 0,
    pausedUntil: 0,
    breakUntil: 0,
    fatigue: 0,
    now: Date.now() / 1000,
    storageError: null,
    shortcutError: null,
  }
}
export interface Bar {
  key: string
  label: string
  seconds: number
  peakSeconds: number
  breaks: number
}
export function aggregate(
  days: Record<string, Day>,
  range: number,
  group: string,
  today = new Date(),
): Bar[] {
  const start = new Date(
    today.getFullYear(),
    today.getMonth(),
    today.getDate(),
  )
  const keys = Object.keys(days).sort()
  const count
    = range
      || (keys.length
        ? Math.ceil(
          (start.getTime() - new Date(`${keys[0]}T00:00:00`).getTime())
          / 86400000,
        ) + 1
        : 8)
  start.setDate(start.getDate() - Math.max(count - 1, 0))
  const result = new Map<string, Bar>()
  for (let date = new Date(start); date.getTime() <= today.getTime(); date = new Date(date.getFullYear(), date.getMonth(), date.getDate() + 1)) {
    const key = dateKey(date)
    const bucket = new Date(date)
    if (group === 'week')
      bucket.setDate(bucket.getDate() - ((bucket.getDay() + 6) % 7))
    const id = group === 'month' ? key.slice(0, 7) : dateKey(bucket)
    const label
      = group === 'month'
        ? `${bucket.getFullYear()}年${bucket.getMonth() + 1}月`
        : `${bucket.getMonth() + 1}月${bucket.getDate()}日${group === 'week' ? '起' : ` 周${'日一二三四五六'[bucket.getDay()]}`}`
    const bar = result.get(id) || {
      key: id,
      label,
      seconds: 0,
      peakSeconds: 0,
      breaks: 0,
    }
    const day = days[key]
    if (day) {
      bar.seconds += day.seconds
      bar.peakSeconds += day.peakSeconds
      bar.breaks += day.breaks
    }
    result.set(id, bar)
  }
  return [...result.values()]
}
