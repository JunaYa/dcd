import assert from 'node:assert/strict'
import { readFileSync, readdirSync } from 'node:fs'
import { it } from 'vitest'
import { resolveLanguage, languages } from '../src/i18n/locale.ts'

const messages = JSON.parse(
  readFileSync(new URL('../src/i18n/messages.json', import.meta.url), 'utf8'),
)
it('resolves supported system languages, regions, ordered preferences and fallback', () => {
  for (const [tags, expected] of [
    [['zh-Hans-CN'], 'zh'],
    [['zh-TW'], 'zh'],
    [['JA_jp'], 'ja'],
    [['ko-KR'], 'ko'],
    [['fr-CA'], 'fr'],
    [['de-DE'], 'de'],
    [['ar-SA'], 'ar'],
    [['es-ES', 'fr-FR'], 'fr'],
    [['es-ES'], 'en'],
    [[], 'en'],
  ])
    assert.equal(resolveLanguage(tags), expected)
})
it('every message has seven translations with matching interpolation parameters', () => {
  const placeholders = (text) => [...text.matchAll(/\{(\w+)\}/g)].map((match) => match[1]).sort()
  for (const [key, translations] of Object.entries(messages)) {
    for (const { value } of languages) {
      assert.ok(translations[value]?.trim(), `${key}: missing ${value}`)
      assert.deepEqual(
        placeholders(translations[value]),
        placeholders(translations.en),
        `${key}: ${value} parameters`,
      )
    }
  }
})
it('all literal translation calls in application sources exist in the catalog', () => {
  const root = new URL('../src/', import.meta.url)
  for (const file of readdirSync(root, { recursive: true }).filter((file) =>
    /\.(vue|ts)$/.test(file),
  )) {
    const source = readFileSync(new URL(file, root), 'utf8')
    for (const match of source.matchAll(/\bt\(['"]([^'"]+)['"]/g))
      assert.ok(messages[match[1]], `${file}: missing ${match[1]}`)
  }
})
