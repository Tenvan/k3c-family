import { expect, mock, test } from 'claude-code/testing'

import { THEME, effectiveEffort, effortColor, handoffMarkdown, handoffPays, isTrivialPrompt, limitAlert } from './core'

const H = 3600_000
const MEASURE = {
  context: { tokens: 50_000, window: 200_000, percent: 25 },
  rateLimits: [{ kind: 'five_hour', percentUsed: 30, resetsAt: new Date(4 * H).toISOString() }],
  cost: { usd: 1.5 },
  changed: [],
}
const BAND = { plugin: 'tenvan-cc', component: 'AbovePrompt', props: { hasSurvey: false, isWorking: false, maxRows: 3, bodyColumns: 120, scroll: { offset: 0, bodyRows: 3 }, view: {} } } as const
const USAGE0 = { input_tokens: 1, output_tokens: 1, cache_read_input_tokens: 0, cache_creation_input_tokens: 0, model: 'claude-opus-5-5' }

test('Auto-Effort: triviale Prompts, Limit-Alarm, Sparmodus, Handoff-Schwelle', () => {
  expect(isTrivialPrompt('ok')).toBe(true)
  expect(isTrivialPrompt('ja, weiter')).toBe(true)
  expect(isTrivialPrompt('/compact')).toBe(true)
  expect(isTrivialPrompt('Baue die Paginierung in die Auftragsliste ein')).toBe(false)
  expect(limitAlert([{ kind: 'five_hour', percentUsed: 50 }])).toBe(null)
  expect(limitAlert([{ kind: 'five_hour', percentUsed: 50 }, { kind: 'seven_day', percentUsed: 84 }])?.kind).toBe('seven_day')
  expect(effectiveEffort('high', true, [{ kind: 'five_hour', percentUsed: 81 }])).toBe('low')
  expect(effectiveEffort('high', false, [{ kind: 'five_hour', percentUsed: 81 }])).toBe('high')
  expect(handoffPays(70, 0)).toBe(true)
  expect(handoffPays(70, 1000)).toBe(false)
  expect(handoffPays(40, 0)).toBe(false)
  expect([effortColor('low'), effortColor('medium'), effortColor('high'), effortColor(null)]).toEqual([THEME.warning, THEME.success, THEME.accent, undefined])
})

test('Handoff-Markdown: leere Abschnitte fallen weg, Git immer', () => {
  const md = handoffMarkdown({ summary: '', lastPrompt: 'Baue X', touched: ['a/b.ts'], steps: [], git: 'abc123 fix' })
  expect(md).toContain('## Letzter Auftrag')
  expect(md).toContain('- a/b.ts')
  expect(md).not.toContain('## Zusammenfassung')
  expect(md).toContain('abc123 fix')
})

test('Auto-Effort: Knopf schaltet ein, Haiku-Stufe geht in den Haupt-Request, Handänderung schaltet aus', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('prompt.submit', ($, e) => ({ text: e.text }))
  // Das Band zeichnet die Bänder darunter mit: hier antwortet niemand sonst auf AbovePrompt.
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  let judged = 0
  on('model.classify', () => {
    judged += 1
    return { value: 'low' }
  })
  const sent: unknown[] = []
  on('turn.step', async function* ($, e) {
    sent.push(e.effort)
    return { turnId: 't1', index: 0, answer: '', toolUses: [], stopReason: 'end_turn', usage: USAGE0 }
  })
  const step = async (effort: 'high' | 'medium') => {
    for await (const _ of $.turn.step({ turnId: 't1', index: 0, model: 'claude-opus-5-5', effort, messageCount: 1 })) {
      // Stream leer durchlaufen
    }
  }
  await $.session.measure(MEASURE)
  await step('high')
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect(await band.find({ type: 'Text', text: /High · Opus 5\.5/ })).not.toBe(undefined)

  await band.press({ key: 'auto' })
  for (let i = 0; i < 200 && (await band.find({ key: 'auto', text: 'Auto ✓' })) === undefined; i++) await Promise.resolve()
  expect(await band.find({ key: 'auto', text: 'Auto ✓' })).not.toBe(undefined)

  await $.prompt.submit({ text: 'ok' } as never)
  expect(judged).toBe(0)
  await $.prompt.submit({ text: 'Erkläre kurz, was die Funktion parseRepo macht' } as never)
  expect(judged).toBe(1)
  for (let i = 0; i < 200 && (await band.find({ type: 'Text', text: 'Haiku: low' })) === undefined; i++) await Promise.resolve()
  expect(await band.find({ type: 'Text', text: 'Haiku: low' })).not.toBe(undefined)
  await step('high')
  expect(sent.at(-1)).toBe('low')

  // Effort der Session weicht ab: von Hand geändert, Auto geht aus und der Request bleibt unverändert.
  await step('medium')
  expect(sent.at(-1)).toBe('medium')
  for (let i = 0; i < 200 && (await band.find({ key: 'auto', text: 'Auto' })) === undefined; i++) await Promise.resolve()
  expect(await band.find({ key: 'auto', text: 'Auto' })).not.toBe(undefined)
})
