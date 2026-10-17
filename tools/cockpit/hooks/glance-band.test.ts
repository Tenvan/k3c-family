import type { On } from 'claude-code'
import { expect, mock, test } from 'claude-code/testing'

// Checkliste (Idee: glanceflow) im Band, in der Plan-Karte und im Verlauf: gemountet auf Desktop und Terminal.

const NOW = Date.UTC(2026, 9, 13, 10)
const PROPS = { hasSurvey: false, isWorking: false, maxRows: 10, bodyColumns: 120, scroll: { offset: 0, bodyRows: 10 }, view: {} } as const
const BAND = { plugin: 'tenvan-cc', component: 'AbovePrompt', props: PROPS } as const
const PANE_PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 100, placement: 'inline', scroll: { offset: 0, bodyRows: 40 }, view: {} } as const
const TODOS = [
  { content: 'Typen anlegen', status: 'completed', activeForm: 'Lege Typen an' },
  { content: 'Band zeichnen', status: 'in_progress', activeForm: 'Zeichne Band' },
  { content: 'Tests schreiben', status: 'pending', activeForm: 'Schreibe Tests' },
]
const USAGE = { value: { startedAt: 0, context: { window: 200_000 }, rateLimits: [] } }
const TURN = { answer: 'Fertig.', reason: 'answer', durationMs: 1000, isAborted: false, turnId: 't1' } as const

const settle = async (done: () => Promise<boolean>) => {
  for (let i = 0; i < 200 && !(await done()); i++) await Promise.resolve()
}

// Gemeinsame Stubs: unter dem Plugin steht im Test nichts.
// `sent`: jeder Prompt, der unten ankommt; `saved`: jede Store-Schreibung; `failing`: Tools, die mit Fehler enden.
function stubs(on: On, o: { sent?: string[]; saved?: { key: string; value: unknown }[]; failing?: string[] } = {}) {
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('prompt.submit', ($, e) => (o.sent?.push(e.text), { text: e.text }))
  on('tool.call', ($, e) => (o.failing?.includes((e as { tool: string }).tool) ? { result: 'Fehler', isError: true } : { result: {} }) as never)
  on('turn.complete', () => ({ text: 'Fertig.' }))
  on('session.usage', () => USAGE as never)
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', ($, e) => (o.saved?.push(e), { value: undefined }) as never)
  on('store.keys', () => ({ value: [] }) as never)
}

test('Checkliste: TodoWrite zeigt die Schritte ganz oben im Band, auch ohne Messung (Desktop)', async ($, on) => {
  mock.clock(on, { now: NOW })
  stubs(on)
  await $.prompt.submit({ text: 'Baue die Checkliste ein' } as never)
  await $.tool.call({ tool: 'TodoWrite', tool_use_id: 'tw1', todos: TODOS } as never)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await settle(async () => (await band.find({ key: 'glance' })) !== undefined)
  expect(await band.find({ type: 'Text', text: 'Arbeitet' })).not.toBe(undefined)
  expect(await band.find({ type: 'Text', text: 'Baue die Checkliste ein' })).not.toBe(undefined)
  expect(await band.find({ text: /1 von 3 erledigt/ })).not.toBe(undefined)
  expect(await band.find({ text: /✓ Typen anlegen/ })).not.toBe(undefined)
  expect(await band.find({ text: /▸ Band zeichnen/ })).not.toBe(undefined)
  expect(await band.find({ text: /○ Tests schreiben/ })).not.toBe(undefined)
  expect(await band.find({ text: /Jetzt: Schritte planen/ })).not.toBe(undefined)
  expect(await band.find({ key: 'glance-pause' })).not.toBe(undefined)
  // Fortschrittsring nur auf dem Desktop.
  expect(JSON.stringify(await band.findAll({ type: 'Svg' }))).toContain('1 von 3 erledigt')
  // Ganz oben: die Checkliste steht vor der Titelzeile (Knopf „Features“).
  const tree = JSON.stringify(await band.find({ type: 'Box' }))
  expect(tree.indexOf('glance-head')).toBeGreaterThan(-1)
  expect(tree.indexOf('glance-head')).toBeLessThan(tree.indexOf('"features"'))
})

test('Checkliste: im Terminal dieselben Schritte als Text, ohne Svg', async ($, on) => {
  mock.clock(on, { now: NOW })
  stubs(on)
  await $.prompt.submit({ text: 'Baue die Checkliste ein' } as never)
  await $.tool.call({ tool: 'TodoWrite', tool_use_id: 'tw1', todos: TODOS } as never)
  const band = await $.ui.mount({ ...BAND, surface: 'terminal' })
  await settle(async () => (await band.find({ key: 'glance' })) !== undefined)
  expect(await band.find({ text: /✓ Typen anlegen/ })).not.toBe(undefined)
  expect(await band.find({ text: /▸ Band zeichnen/ })).not.toBe(undefined)
  expect(await band.find({ text: /1 von 3 erledigt/ })).not.toBe(undefined)
  expect(await band.find({ type: 'Svg' })).toBe(undefined)
})

test('Checkliste: endet die Antwort mit einer Frage, braucht Claude dich', async ($, on) => {
  mock.clock(on, { now: NOW })
  stubs(on)
  await $.prompt.submit({ text: 'Räume die Branches auf' } as never)
  await $.tool.call({ tool: 'Bash', tool_use_id: 'b1', command: 'git branch' } as never)
  await $.turn.complete({ ...TURN, answer: 'Drei Branches sind gemergt.\n\nSoll Claude die Branches löschen?' })
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await settle(async () => (await band.find({ type: 'Text', text: 'Braucht dich' })) !== undefined)
  expect(await band.find({ type: 'Text', text: 'Braucht dich' })).not.toBe(undefined)
  expect(await band.find({ text: /\? Soll Claude die Branches löschen\?/ })).not.toBe(undefined)
  // Ohne offene Schritte kein „Weiter“, keine Pause nach dem Turn.
  expect(await band.find({ key: 'glance-continue' })).toBe(undefined)
  expect(await band.find({ key: 'glance-pause' })).toBe(undefined)
  // Ein Turn ohne eigenen Prompt (Benachrichtigung eines Hintergrund-Tasks) schließt den Job nicht noch einmal ab.
  await $.turn.complete({ ...TURN, turnId: 't2', answer: 'Hintergrund-Task fertig.' })
  for (let i = 0; i < 50; i++) await Promise.resolve()
  expect(await band.find({ type: 'Text', text: 'Braucht dich' })).not.toBe(undefined)
  expect(await band.find({ type: 'Text', text: '✓ Fertig' })).toBe(undefined)
})

test('Checkliste: offene Freigabe heißt „braucht dich“, ohne Lebenszeichen „hängt?“, ein laufendes Tool nie', async ($, on) => {
  const clock = mock.clock(on, { now: NOW })
  stubs(on)
  on('tool.check', () => ({ decision: 'ask' }))
  await $.prompt.submit({ text: 'Baue die Checkliste ein' } as never)
  await $.tool.call({ tool: 'TodoWrite', tool_use_id: 'tw1', todos: TODOS } as never)
  await clock.advance(181_000)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await settle(async () => (await band.find({ type: 'Text', text: 'Hängt?' })) !== undefined)
  expect(await band.find({ type: 'Text', text: 'Hängt?' })).not.toBe(undefined)
  expect(await band.find({ key: 'glance-continue' })).not.toBe(undefined)
  // Freigabe offen: „braucht dich“ mit Grund, kein „hängt?“ und kein Weiter.
  await $.tool.check({ tool: 'Bash', tool_use_id: 'c1', input: { command: 'rm -rf build' } } as never)
  await settle(async () => (await band.find({ type: 'Text', text: 'Braucht dich' })) !== undefined)
  expect(await band.find({ type: 'Text', text: 'Braucht dich' })).not.toBe(undefined)
  expect(await band.find({ text: /\? wartet auf deine Freigabe/ })).not.toBe(undefined)
  expect(await band.find({ type: 'Text', text: 'Hängt?' })).toBe(undefined)
  expect(await band.find({ key: 'glance-continue' })).toBe(undefined)
})

test('Checkliste: fehlgeschlagene Aufgabenliste ändert den Plan nicht; ein offener Job landet beim nächsten Prompt als gestoppt im Verlauf', async ($, on) => {
  mock.clock(on, { now: NOW })
  const saved: { key: string; value: unknown }[] = []
  stubs(on, { saved, failing: ['TodoWrite'] })
  await $.prompt.submit({ text: 'Erster Auftrag' } as never)
  await $.tool.call({ tool: 'TodoWrite', tool_use_id: 'tw1', todos: TODOS } as never)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await settle(async () => (await band.find({ key: 'glance' })) !== undefined)
  expect(await band.find({ text: /ohne Aufgabenliste/ })).not.toBe(undefined)
  expect(await band.find({ text: /Typen anlegen/ })).toBe(undefined)
  await $.prompt.submit({ text: 'Zweiter Auftrag' } as never)
  await settle(async () => saved.length > 0)
  expect(saved[0]?.key).toMatch(/^glance:\d{4}-\d{2}-\d{2}$/)
  expect(JSON.stringify(saved[0]?.value)).toContain('"title":"Erster Auftrag"')
  expect(JSON.stringify(saved[0]?.value)).toContain('"outcome":"stopped"')
})

test('Checkliste: Pause bricht den Haupt-Turn ab, Weiter sendet die offenen Schritte und setzt den Job fort', async ($, on) => {
  const clock = mock.clock(on, { now: NOW })
  const aborted: string[] = []
  const sent: string[] = []
  stubs(on, { sent })
  on('turn.start', ($, e) => ({ turnId: e.turnId }))
  on('turn.abort', ($, e) => (aborted.push(e.turnId), { value: undefined }) as never)
  await $.prompt.submit({ text: 'Baue die Checkliste ein' } as never)
  await $.turn.start({ text: 'Baue die Checkliste ein', turnId: 'turn-1' })
  await $.tool.call({ tool: 'TodoWrite', tool_use_id: 'tw1', todos: TODOS } as never)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await settle(async () => (await band.find({ key: 'glance-pause' })) !== undefined)
  await band.press({ key: 'glance-pause' })
  expect(aborted).toEqual(['turn-1'])
  await $.turn.complete({ ...TURN, turnId: 'turn-1', reason: 'aborted', isAborted: true, answer: '' })
  await settle(async () => (await band.find({ text: /Pausiert/ })) !== undefined)
  expect(await band.find({ text: /Pausiert/ })).not.toBe(undefined)
  await band.press({ key: 'glance-continue' })
  await clock.advance(60)
  await settle(async () => sent.length > 1)
  expect(sent[1]).toContain('Baue die Checkliste ein')
  expect(sent[1]).toContain('Band zeichnen; Tests schreiben')
  // Derselbe Job läuft weiter: Schritte und Fortschritt bleiben.
  await settle(async () => (await band.find({ type: 'Text', text: 'Arbeitet' })) !== undefined)
  expect(await band.find({ text: /1 von 3 erledigt/ })).not.toBe(undefined)
})

test('Plan-Karte und Verlauf zeichnen auf Desktop und Terminal', async ($, on) => {
  mock.clock(on, { now: NOW })
  stubs(on)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  await $.prompt.submit({ text: 'Baue die Checkliste ein' } as never)
  await $.tool.call({ tool: 'TodoWrite', tool_use_id: 'tw1', todos: TODOS } as never)
  await $.tool.call({ tool: 'Edit', tool_use_id: 'e1', file_path: 'C:/repo/hooks/register.tsx', old_string: 'a', new_string: 'b' } as never)
  for (const surface of ['desktop', 'terminal'] as const) {
    const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
    await settle(async () => (await pane.find({ key: 'card-cardPlan' })) !== undefined)
    expect(await pane.find({ text: /▸ Band zeichnen/ })).not.toBe(undefined)
    expect(await pane.find({ text: /Dateien: hooks\/register\.tsx/ })).not.toBe(undefined)
    const history = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc-history', props: PANE_PROPS })
    await settle(async () => (await history.find({ key: 'hist-none' })) !== undefined)
    expect(await history.find({ key: 'hist-prev' })).not.toBe(undefined)
    expect(await history.find({ key: 'hist-day-report' })).not.toBe(undefined)
  }
})
