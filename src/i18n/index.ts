import { ref } from 'vue'
import { type Language, resolveLanguage } from './locale'

import messages from './messages.json'

export { languages, resolveLanguage } from './locale'
export const locale = ref<Language>(resolveLanguage(typeof navigator === 'undefined' ? ['en'] : navigator.languages))
export function t(key: string, values: Record<string, string | number> = {}): string {
  const entry = messages[key as keyof typeof messages]
  const text = entry?.[locale.value] ?? entry?.en ?? key
  return text.replace(/\{(\w+)\}/g, (match, name) => String(values[name] ?? match))
}
export function formatDuration(seconds: number): string {
  const minutes = Math.floor(seconds / 60)
  const number = new Intl.NumberFormat(locale.value)
  return minutes >= 60
    ? `${number.format(Math.floor(minutes / 60))} ${t('小时')} ${number.format(minutes % 60)} ${t('分钟')}`
    : `${number.format(minutes)} ${t('分钟')}`
}
export function formatBarDate(key: string, grouping: string): string {
  return new Intl.DateTimeFormat(locale.value, grouping === 'month'
    ? { year: 'numeric', month: 'short' }
    : { month: 'short', day: 'numeric', ...(grouping === 'day' ? { weekday: 'short' as const } : {}) })
    .format(new Date(`${key.length === 7 ? `${key}-01` : key}T12:00:00`))
}
