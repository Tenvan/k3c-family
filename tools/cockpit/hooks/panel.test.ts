import { expect, mock, test } from 'claude-code/testing'

import { THEME } from './core'

const PANE_PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 144, placement: 'dock', scroll: { offset: 0, bodyRows: 40 }, view: {} } as const
const SETTINGS_PROPS = { ...PANE_PROPS, title: 'Cockpit · Einstellungen' } as const

const MESSAGES = [
  { role: 'user' as const, text: 'Baue die Einstellungsseite', toolUses: [] },
  { role: 'assistant' as const, text: 'Ich lege die Seite an.', toolUses: [{ tool_use_id: 't1', tool: 'Write', input: { file_path: 'a/b.ts' }, text: 'ok' }] },
]

test('Einstellungsseite: Schalter blendet Karte im Panel aus', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({}) as never)

  const settings = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  expect(await settings.find({ type: 'Text', text: 'Panel' })).not.toBe(undefined)
  await settings.press({ key: 'set-cardLog' })

  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ type: 'Text', text: 'Log' })).toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Übersicht' })).not.toBe(undefined)
})

test('Panel-Karten: ▾/▸ klappt zu und auf, ✕ schaltet die Karte in den Einstellungen ab', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({}) as never)
  const pane = await $.ui.mount({ plugin: 'orgamax-cockpit', surface: 'desktop', component: 'Pane', requestId: 'orgamax-cockpit', props: PANE_PROPS })
  const settle = async (done: () => Promise<boolean>) => {
    for (let i = 0; i < 100 && !(await done()); i++) await Promise.resolve()
  }
  expect(await pane.find({ type: 'Text', text: 'Leer.' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Button', key: 'card-close-cardGate' })).not.toBe(undefined)
  await pane.press({ key: 'card-fold-cardLog' })
  await settle(async () => (await pane.find({ type: 'Text', text: 'Leer.' })) === undefined)
  expect(await pane.find({ type: 'Text', text: 'Leer.' })).toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Log' })).not.toBe(undefined)
  expect(JSON.stringify(await pane.find({ type: 'Button', key: 'card-fold-cardLog' }))).toContain('▸')
  await pane.press({ key: 'card-fold-cardLog' })
  await settle(async () => (await pane.find({ type: 'Text', text: 'Leer.' })) !== undefined)
  expect(await pane.find({ type: 'Text', text: 'Leer.' })).not.toBe(undefined)
  await pane.press({ key: 'card-close-cardLog' })
  await settle(async () => (await pane.find({ type: 'Text', text: 'Log' })) === undefined)
  expect(await pane.find({ type: 'Text', text: 'Log' })).toBe(undefined)
  const settings = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  expect(JSON.stringify(await settings.find({ type: 'Button', key: 'set-cardLog' }))).toContain('○ Aus')
})

test('Band: Claudes eigener Vorschlag trägt den Chip „CC“', async ($, on) => {
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('store.get', () => ({ value: undefined }) as never)
  on('prompt.suggest', () => ({ isShown: true }) as never)
  mock.clock(on, { now: 1 })
  // Der Test-$ ruft als Engine: origin „suggestion“ kommt so wie von Claudes eigenem Vorschlagsdienst.
  await $.prompt.suggest({ text: 'Tests laufen lassen', origin: { kind: 'suggestion' } } as Parameters<typeof $.prompt.suggest>[0])
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  for (let i = 0; i < 100 && !(await band.find({ type: 'Text', text: ' CC ' })); i++) await Promise.resolve()
  const rows = await band.findAll({ type: 'Text', text: ' CC ' })
  expect(rows.length).toBe(1)
  expect(JSON.stringify(rows[0])).toContain(THEME.chip)
  expect(await band.find({ type: 'Text', text: 'Vorschlag von Claude' })).not.toBe(undefined)
})

test('Zusammenfassung: Aktualisieren sendet deutschen Prompt und zeigt das Ergebnis', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({}) as never)
  on('store.delete', () => ({}) as never)
  on('session.id', () => ({ value: 's1' }) as never)
  on('session.messages', () => ({ value: MESSAGES }) as never)
  on('fs.read', () => ({ value: '# Anweisungen' }) as never)
  on('process.run', () => ({ value: { exitCode: 0, stdout: '', stderr: '', isStdoutTruncated: false, isStderrTruncated: false } }) as never)
  const sent: { system?: string; prompt: string }[] = []
  on('model.complete', (_, e) => {
    sent.push({ system: e.system, prompt: e.prompt })
    return { value: { isAnswered: true, text: '## Warum diese Session\n- Einstellungsseite bauen', usage: {} } } as never
  })

  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-summary', props: SETTINGS_PROPS })
  expect(await pane.find({ type: 'Text', text: /Die Zusammenfassung erscheint/ })).not.toBe(undefined)
  await pane.press({ key: 'cu-refresh' })
  for (let i = 0; i < 200 && sent.length === 0; i++) await Promise.resolve()

  expect(sent.length).toBe(1)
  expect(sent[0]?.system).toContain('Deutsch')
  expect(sent[0]?.prompt).toContain('NUTZER: Baue die Einstellungsseite')
  expect(sent[0]?.prompt).toContain('[Tool Write]')
})

test('Demo: /cockpit-demo gibt Markdown mit allen skinbaren Elementen aus', async ($, on) => {
  on('fs.read', () => ({ value: '```ts\nx\n```\n<!-- KOMPONENTEN -->\n' }) as never)
  on('store.get', () => ({ value: undefined }) as never)
  const out = await $.command.run({ command: 'cockpit-demo', args: '' } as Parameters<typeof $.command.run>[0])
  expect(out.text).toContain('```ts')
  expect(out.text).toContain('| `AssistantMessage` | Antwort des Modells (Markdown) | ✅ Chat-Skin (Code- und Tabellenkarten, Mermaid-Karten, Formelkarten) |')
  expect(out.text).toContain('| `InfoNotice` | Hinweiszeile im Transcript | ➖ nur Terminal')
  expect(out.text).toContain('| `Spinner` | Arbeitszeile während eines Turns | ✅ eigene Zeile (Design-System) |')
  expect(out.text).not.toContain('KOMPONENTEN')
})

const BAND = { plugin: 'tenvan-cc', component: 'AbovePrompt', props: { hasSurvey: false, isWorking: false, maxRows: 3, bodyColumns: 120, scroll: { offset: 0, bodyRows: 3 }, view: {} } } as const
const MEASURE = { context: { tokens: 50_000, window: 200_000, percent: 25 }, rateLimits: [], cost: { usd: 1.5 }, changed: [] }

// Gemountete Instanzen zeichnen bei Zustandsänderung neu; Suche wartet, bis der Text erscheint (oder aufgibt).
async function waitFind(view: { find(q: { type: 'Text'; text: string | RegExp }): Promise<unknown> }, text: string | RegExp) {
  for (let i = 0; i < 3000; i++) {
    const hit = await view.find({ type: 'Text', text })
    if (hit !== undefined) return hit
    await Promise.resolve()
  }
  return undefined
}

test('Sprache: lang-en schaltet Band und Panel auf Englisch', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({}) as never)
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  await $.session.measure(MEASURE)

  const settings = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  expect(await settings.find({ type: 'Text', text: 'Band über der Eingabe' })).not.toBe(undefined)
  await settings.press({ key: 'lang-en' })
  expect(await waitFind(settings, 'Band above the prompt')).not.toBe(undefined)

  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ type: 'Text', text: 'Overview' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Übersicht' })).toBe(undefined)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect(await band.find({ type: 'Text', text: /^Context/ })).not.toBe(undefined)
  expect(await band.find({ type: 'Text', text: /^Kontext/ })).toBe(undefined)
})

test('Einstellungsseite: „Alle aus“ einer Gruppe und Zurücksetzen', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({}) as never)
  const page = await $.ui.mount({ plugin: 'orgamax-cockpit', surface: 'desktop', component: 'Pane', requestId: 'orgamax-cockpit-settings', props: SETTINGS_PROPS })
  const main = await $.ui.mount({ plugin: 'orgamax-cockpit', surface: 'desktop', component: 'Pane', requestId: 'orgamax-cockpit', props: PANE_PROPS })
  expect(await main.find({ type: 'Text', text: 'Übersicht' })).not.toBe(undefined)

  // Gruppen in Reihenfolge des ersten Schalters: Band, Panel, Zusammenfassung, Chat
  await page.press({ key: 'grp-off-1' })
  for (let i = 0; i < 3000 && (await main.find({ type: 'Text', text: 'Übersicht' })) !== undefined; i++) await Promise.resolve()
  expect(await waitFind(page, '0/9 an')).not.toBe(undefined)
  for (const card of ['Übersicht', 'Log', 'Letzter Turn']) expect(await main.find({ type: 'Text', text: card })).toBe(undefined)

  await page.press({ key: 'set-reset' })
  expect(await waitFind(page, '9/9 an')).not.toBe(undefined)
  expect(await page.find({ type: 'Text', text: '7/7 an' })).not.toBe(undefined)
  expect(await waitFind(main, 'Übersicht')).not.toBe(undefined)
})

test('Panel: Modellzeile unter dem Titel mit Modell und Claude-Code-Version', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({}) as never)
  on('session.model', () => ({ value: 'claude-opus-5-5' }) as never)
  on('session.version', () => ({ value: { version: '2.1.295' } }) as never)
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  on('fs.read', () => ({ value: '{ "version": "1" }' }) as never)
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  for (let i = 0; i < 200; i++) await Promise.resolve()
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ type: 'Text', text: /Opus 5\.5 \(claude-opus-5-5\).*Claude Code 2\.1\.295/ })).not.toBe(undefined)
})
