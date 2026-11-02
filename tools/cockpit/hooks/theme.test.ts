import { expect, test } from 'claude-code/testing'

import { CARD_CYCLE, THEME, TINT, applyScheme, cycleTint, desktopModeLight, themed, wantsLight } from './theme'
import { DECISION_COLOR } from './core'
import { CODE } from './skins'
import { MODES } from './ui'

test('Theme: hell und zurück schaltet Tokens und abgeleitete Tabellen am Ort um', () => {
  const dark = { accent: THEME.accent, tint: TINT.neutral, card: cycleTint(0) }
  expect(applyScheme(true)).toBe(true)
  expect(THEME.accent).not.toBe(dark.accent)
  expect(TINT.neutral).not.toBe(dark.tint)
  expect(CARD_CYCLE[0]).toBe(cycleTint(0))
  expect(cycleTint(0)).not.toBe(dark.card)
  expect(DECISION_COLOR.you).toBe(THEME.accent)
  expect(CODE.bg).toBe(THEME.codeBg)
  expect(MODES.pane.chip).toBe(THEME.chip)
  expect(applyScheme(true)).toBe(false)
  expect(applyScheme(false)).toBe(true)
  expect(THEME.accent).toBe(dark.accent)
  expect(DECISION_COLOR.you).toBe(dark.accent)
})

test('Theme: themed rechnet beim Wechsel neu, die Referenz bleibt', () => {
  const table = themed(() => ({ c: THEME.error }))
  const before = table.c
  applyScheme(true)
  expect(table.c).toBe(THEME.error)
  expect(table.c).not.toBe(before)
  applyScheme(false)
  expect(table.c).toBe(before)
})

test('Theme: Darstellung hell/dunkel gilt immer, auto folgt dem Host-Theme', () => {
  expect(wantsLight('hell', 'dark')).toBe(true)
  expect(wantsLight('dunkel', 'light')).toBe(false)
  expect(wantsLight('auto', 'light-daltonized')).toBe(true)
  expect(wantsLight('auto', 'dark')).toBe(false)
  expect(wantsLight('auto', undefined)).toBe(false)
})

test('Theme: Desktop-Modus aus config.json, system über die Windows-Registry', () => {
  const reg = (v: string) => `
    AppsUseLightTheme    REG_DWORD    ${v}
`
  expect(desktopModeLight('{"userThemeMode":"light"}', undefined)).toBe(true)
  expect(desktopModeLight('{"userThemeMode":"dark"}', reg('0x1'))).toBe(false)
  expect(desktopModeLight('{"userThemeMode":"system"}', reg('0x1'))).toBe(true)
  expect(desktopModeLight('{"userThemeMode":"system"}', reg('0x0'))).toBe(false)
  expect(desktopModeLight('{"userThemeMode":"system"}', undefined)).toBe(undefined)
  expect(desktopModeLight('kaputt', reg('0x1'))).toBe(undefined)
})
