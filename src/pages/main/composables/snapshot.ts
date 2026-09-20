import type { Settings, Snapshot } from '~/eye/model'

export function sameSettings(current: Settings, incoming: Settings) {
  return Object.entries(current).every(([key, value]) => value === incoming[key as keyof Settings])
}

export function applySnapshot(current: Snapshot, incoming: Snapshot) {
  const daysUnchanged =
    Object.keys(current.days).length === Object.keys(incoming.days).length &&
    Object.entries(incoming.days).every(([key, day]) => {
      const previous = current.days[key]
      return (
        previous &&
        previous.seconds === day.seconds &&
        previous.peakSeconds === day.peakSeconds &&
        previous.breaks === day.breaks
      )
    })
  const samplesUnchanged =
    current.samples.length === incoming.samples.length &&
    incoming.samples.every(
      ([minute, fatigue], index) =>
        current.samples[index][0] === minute && current.samples[index][1] === fatigue,
    )
  Object.assign(current, {
    ...incoming,
    days: daysUnchanged ? current.days : incoming.days,
    samples: samplesUnchanged ? current.samples : incoming.samples,
    settings: sameSettings(current.settings, incoming.settings)
      ? current.settings
      : incoming.settings,
  })
}
