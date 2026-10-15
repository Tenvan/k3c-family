import { expect, mock, test } from 'claude-code/testing'

import { THEME, TINT, turnBarsText } from './core'

// Jede Anzeige zeichnet auch im Terminal: dieselben Daten und Texte, ohne Svg (die Elementtabelle des Terminals hat keins).

const F = '```'
const NOW = Date.UTC(2026, 9, 13, 10)
const T = { plugin: 'tenvan-cc', surface: 'terminal' } as const
const PANE_PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 100, placement: 'inline', scroll: { offset: 0, bodyRows: 40 }, view: {} } as const
const BAND = { ...T, component: 'AbovePrompt', props: { hasSurvey: false, isWorking: false, maxRows: 10, bodyColumns: 100, scroll: { offset: 0, bodyRows: 10 }, view: {} } } as const
const MEASURE = {
  context: { tokens: 50_000, window: 200_000, percent: 25 },
  rateLimits: [{ kind: 'five_hour', percentUsed: 40, resetsAt: new Date(NOW + 3600_000).toISOString() }],
  cost: { usd: 1.5 },
  changed: [],
}

test('Turn-Bilanz als Blockzeichen: Höhe nach Kosten, teuer ohne Edit als Warnung', () => {
  const bars = turnBarsText([
    { usd: 0.05, tools: 2, edits: 1, durationMs: 1 },
    { usd: 0.4, tools: 9, edits: 0, durationMs: 1 },
  ])
  expect(bars.map(b => b.glyph).join('')).toBe('▂█')
  expect(bars.map(b => b.isWaste)).toEqual([false, true])
})

test('Terminal: Band mit Textbalken, Reset-Zeit am Limit, ohne Legende', async ($, on) => {
  mock.clock(on, { now: NOW })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  await $.session.measure(MEASURE)
  const band = await $.ui.mount(BAND)
  expect((await band.find({ text: /Kontext ▰/ }))?.text).toContain('25%')
  // Kein Ring-Tooltip im Terminal: die Reset-Zeit steht am Limit, die Legende entfällt.
  expect((await band.find({ text: /^5h 40%/ }))?.text).toMatch(/5h 40% · ↻ \d+h \d+m \(\S+ \d\d:\d\d\)/)
  expect(await band.find({ type: 'Text', text: 'im Plan' })).toBe(undefined)
  expect(JSON.stringify(await band.find({ type: 'Box' }))).toContain(TINT.accent)
  expect(JSON.stringify(await band.find({ type: 'Box' }))).toContain(THEME.accentFill)
  expect(await band.find({ type: 'Button', key: 'compact' })).not.toBe(undefined)
  expect(await band.find({ type: 'Svg' })).toBe(undefined)
})

test('Terminal: alle Panels zeichnen im Dev-Mode ohne Svg', async ($, on) => {
  mock.clock(on, { now: NOW })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  await $.command.run({ command: 'cockpit-dev', args: '' } as never)
  const settle = async (done: () => Promise<boolean>) => {
    for (let i = 0; i < 100 && !(await done()); i++) await Promise.resolve()
  }

  const main = await $.ui.mount({ ...T, component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  await settle(async () => (await main.find({ type: 'Text', text: ' DEV · Mockdaten ' })) !== undefined)
  expect(await main.find({ type: 'Text', text: 'Arbeitsverzeichnis' })).not.toBe(undefined)
  expect(await main.find({ type: 'Text', text: 'GATE · Freigaben' })).not.toBe(undefined)
  // Turn-Bilanz und Verlauf der Subagents ohne Svg: Blockzeichen bzw. Dauer als Text.
  expect(await main.find({ type: 'Text', text: /^[▁▂▃▄▅▆▇█]$/ })).not.toBe(undefined)
  expect(await main.find({ type: 'Svg' })).toBe(undefined)
  // Übersicht als Raster: Bezeichnung eigene Spalte, Balken und dreistellige Prozente fluchten.
  expect(await main.find({ type: 'Text', text: /^[▰▱]{10}  62% · / })).not.toBe(undefined)
  expect(await main.find({ type: 'Text', text: /^[▰▱]{10}  41% · / })).not.toBe(undefined)

  const summary = await $.ui.mount({ ...T, component: 'Pane', requestId: 'tenvan-cc-summary', props: PANE_PROPS })
  expect(await summary.find({ type: 'Button', key: 'cu-refresh' })).not.toBe(undefined)

  const settings = await $.ui.mount({ ...T, component: 'Pane', requestId: 'tenvan-cc-settings', props: PANE_PROPS })
  expect(await settings.find({ type: 'Button', key: 'set-cardLog' })).not.toBe(undefined)

  const mr = await $.ui.mount({ ...T, component: 'Pane', requestId: 'tenvan-cc-mr', props: PANE_PROPS })
  expect(await mr.find({ type: 'Button', key: 'mr-a-rebase' })).not.toBe(undefined)
  // Eigene MRs im selben Repo: Status je MR; !42 lässt sich (auto-)mergen, !17 hat einen Konflikt und nur Rebase.
  expect(await mr.find({ type: 'Button', key: 'mine-merge-42' })).not.toBe(undefined)
  expect(await mr.find({ type: 'Button', key: 'mine-rebase-17' })).not.toBe(undefined)
  expect(await mr.find({ type: 'Button', key: 'mine-merge-17' })).toBe(undefined)
  expect(await mr.findAll({ type: 'Text', text: 'Konflikt' })).toHaveLength(1)

  const foot = await $.ui.mount({ ...T, component: 'SessionMode', props: { modes: [] } as never })
  expect(JSON.stringify(await foot.find({ type: 'Button', key: 'mr-foot' }))).toContain('!42')
})

test('Terminal: Formel als Unicode-Text, Mermaid bleibt der Markdown-Block der Engine', async ($, on) => {
  on('ui.render', { component: 'AssistantMessage' }, () => ({ type: 'Box' as const }))
  const math = await $.ui.mount({ ...T, component: 'AssistantMessage', props: { text: '$$\n\\alpha^2\n$$', isFirstOfReply: true } })
  expect(await math.find({ type: 'Text', text: 'Formel' })).not.toBe(undefined)
  expect(await math.find({ type: 'Text', text: 'α²' })).not.toBe(undefined)
  const mermaid = await $.ui.mount({ ...T, component: 'AssistantMessage', props: { text: `${F}mermaid\ngraph TD\nA-->B\n${F}`, isFirstOfReply: true } })
  expect(await mermaid.find({ type: 'Text', text: 'Mermaid · flowchart' })).toBe(undefined)
})
