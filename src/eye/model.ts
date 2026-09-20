export interface Settings {
  workMinutes: number
  breakMinutes: number
  breakSeconds: number
  repeatMinutes: number
  preNotify: boolean
  sound: boolean
  movieMode: boolean
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
}
export const defaults: Settings = {
  workMinutes: 25,
  breakMinutes: 5,
  breakSeconds: 0,
  repeatMinutes: 1,
  preNotify: true,
  sound: true,
  movieMode: false,
  autostart: false,
  trayIcon: true,
  trayTime: true,
  dockIcon: true,
  reminders: true,
  reminderStyle: 'fullscreen',
  message: 'Take a break',
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
  }
}
export function demoSnapshot(): Snapshot {
  const snapshot = emptySnapshot()
  const hours = [0, 0, 0, 8.3, 8.25, 6.9, 10, 9.73]
  hours.forEach((hour, index) => {
    const date = new Date()
    date.setDate(date.getDate() - (7 - index))
    snapshot.days[dateKey(date)] = {
      seconds: hour * 3600,
      peakSeconds: hour * 840,
      breaks: Math.round(hour * 2),
    }
  })
  const samples: [number, number][] = []
  for (let minute = 540; minute <= 1125; minute += 25) {
    samples.push([minute, 0], [minute + 19, 100], [minute + 24, 100])
  }
  samples.push(
    [1130, 0],
    [1290, 0],
    [1315, 100],
    [1330, 100],
    [1331, 0],
    [1360, 0],
    [1370, 52],
  )
  snapshot.samples = samples
  snapshot.fatigue = 52
  return snapshot
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
