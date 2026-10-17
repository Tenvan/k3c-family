import { expect, mock, test } from 'claude-code/testing'

import { THEME, clampTiles } from './core'

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
  on('store.set', () => ({ value: undefined }) as never)

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
  on('store.set', () => ({ value: undefined }) as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
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
  // Begründung als Tooltip am Info-Symbol (Desktop), nicht als eigene Zeile.
  expect(JSON.stringify(await band.findAll({ type: 'Svg' }))).toContain('Vorschlag von Claude')
  expect(await band.find({ type: 'Text', text: 'Vorschlag von Claude' })).toBe(undefined)
  expect(await band.find({ type: 'Button', key: 'steps-refresh' })).not.toBe(undefined)
})

test('Zusammenfassung: Aktualisieren sendet deutschen Prompt und zeigt das Ergebnis', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('store.delete', () => ({ value: undefined }) as never)
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
  on('store.set', () => ({ value: undefined }) as never)
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

test('Einstellungsseite: „Alle aus” einer Gruppe und Zurücksetzen', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  const main = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await main.find({ type: 'Text', text: 'Übersicht' })).not.toBe(undefined)

  // Gruppen in Reihenfolge des ersten Schalters: Band, Checkliste, Während Claude arbeitet, Effort & Kosten, Panel …
  await page.press({ key: 'grp-off-4' })
  for (let i = 0; i < 3000 && (await main.find({ type: 'Text', text: 'Übersicht' })) !== undefined; i++) await Promise.resolve()
  expect(await waitFind(page, /^0\/\d+ an$/)).not.toBe(undefined)
  for (const card of ['Übersicht', 'Log', 'Letzter Turn']) expect(await main.find({ type: 'Text', text: card })).toBe(undefined)

  await page.press({ key: 'set-reset' })
  expect(await waitFind(page, /^\d+\/\d+ an$/)).not.toBe(undefined)
  // Effort & Kosten: Auto-Effort und Sparmodus sind standardmäßig aus
  expect(await page.find({ type: 'Text', text: '0/2 an' })).not.toBe(undefined)
  expect(await waitFind(main, 'Übersicht')).not.toBe(undefined)
})

test('Panel: Modellzeile unter dem Titel mit Modell und Claude-Code-Version', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
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

test('Dev-Mode: /cockpit-dev füllt das Panel mit Mockdaten und schaltet zurück', async ($, on) => {
  mock.clock(on, { now: Date.UTC(2026, 9, 13, 10) })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  const settle = async (done: () => Promise<boolean>) => {
    for (let i = 0; i < 100 && !(await done()); i++) await Promise.resolve()
  }
  const badge = () => pane.find({ type: 'Text', text: ' DEV · Mockdaten ' })
  expect(await badge()).toBe(undefined)
  expect(JSON.stringify(await $.command.run({ command: 'cockpit-dev', args: '' } as never))).toContain('Dev-Mode an')
  await settle(async () => (await badge()) !== undefined)
  expect(await badge()).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'feature/dev-mock' })).not.toBe(undefined)
  expect(JSON.stringify(await pane.find({ type: 'Button', key: 'wd-mr-open' }))).toContain('!42')
  // Kachel „Build und Release-Check“: 10 Zeilen Befehl + Ergebnis, 7 sichtbar, Rest als Hinweis.
  expect(await pane.find({ type: 'Text', text: '… +3 Zeilen' })).not.toBe(undefined)
  await $.command.run({ command: 'cockpit-dev', args: '' } as never)
  await settle(async () => (await badge()) === undefined)
  expect(await badge()).toBe(undefined)
})

test('MR-Panel und Fußzeilen-Knopf im Dev-Mode', async ($, on) => {
  mock.clock(on, { now: Date.UTC(2026, 9, 13, 10) })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  await $.command.run({ command: 'cockpit-dev', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-mr', props: { ...PANE_PROPS, title: 'Cockpit · Merge Requests' } })
  expect(await pane.find({ type: 'Text', text: 'Offene Threads' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Button', key: 'mr-a-rebase' })).not.toBe(undefined)
  // Klick-Rückmeldung: der gedrückte Knopf zeigt „◌ … …“, solange seine Aktion läuft (mindestens 600 ms).
  await pane.press({ key: 'mr-a-refresh' })
  for (let i = 0; i < 100 && !JSON.stringify(await pane.find({ type: 'Button', key: 'mr-a-refresh' })).includes('◌'); i++) await Promise.resolve()
  expect(JSON.stringify(await pane.find({ type: 'Button', key: 'mr-a-refresh' }))).toContain('◌')
  const foot = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'SessionMode', props: { modes: [] } as never })
  expect(JSON.stringify(await foot.find({ type: 'Button', key: 'mr-foot' }))).toContain('!42')
  expect(await foot.find({ type: 'Button', key: 'details' })).not.toBe(undefined)
})

test('Subagent-Kacheln: Standard 6, Stepper in Zweierschritten mit Grenzen 2 bis 12', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  expect([clampTiles(undefined), clampTiles(5), clampTiles(0), clampTiles(99)]).toEqual([6, 6, 2, 12])
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  expect(await waitFind(page, '6')).not.toBe(undefined)
  await page.press({ key: 'tiles-plus' })
  expect(await waitFind(page, '8')).not.toBe(undefined)
  for (let i = 0; i < 6; i++) await page.press({ key: 'tiles-minus' })
  expect(await waitFind(page, '2')).not.toBe(undefined)
})

test('Einstellungen: „Neu laden“ übernimmt, was eine andere Session gespeichert hat', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  let saved: unknown
  on('store.get', () => ({ value: saved }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  const main = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await waitFind(main, 'Log')).not.toBe(undefined)
  saved = { cardLog: false }
  await page.press({ key: 'set-reload' })
  for (let i = 0; i < 3000 && (await main.find({ type: 'Text', text: 'Log' })) !== undefined; i++) await Promise.resolve()
  expect(await main.find({ type: 'Text', text: 'Log' })).toBe(undefined)
})

test('Klapp-Status: Einklappen wird im Speicher abgelegt und beim Start gelesen', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  const stored = new Map<string, unknown>()
  on('store.get', (_$, e) => ({ value: stored.get((e as { key: string }).key) }) as never)
  on('store.set', (_$, e) => {
    stored.set((e as { key: string }).key, (e as { value: unknown }).value)
    return { value: undefined } as never
  })
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await waitFind(pane, 'Leer.')).not.toBe(undefined)
  await pane.press({ key: 'card-fold-cardLog' })
  for (let i = 0; i < 100 && !stored.has('collapsed'); i++) await Promise.resolve()
  expect(stored.get('collapsed')).toEqual(['cardLog'])
})

test('Einstellungen: „Neu laden“ meldet das Ergebnis als Toast', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  let saved: unknown
  on('store.get', () => ({ value: saved }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  const toasts: string[] = []
  on('ui.toast', (_$, e) => {
    toasts.push((e as { text: string }).text)
    return { value: undefined } as never
  })
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  await page.press({ key: 'set-reload' })
  for (let i = 0; i < 200 && toasts.length < 1; i++) await Promise.resolve()
  expect(toasts).toEqual(['Einstellungen sind aktuell'])
  saved = { cardLog: false }
  await page.press({ key: 'set-reload' })
  for (let i = 0; i < 200 && toasts.length < 2; i++) await Promise.resolve()
  expect(toasts[1]).toBe('Einstellungen neu geladen')
})

test('Arbeitsverzeichnis: ein Git-Fehler lässt die Kachel stehen, nur „kein Repository“ blendet sie aus', async ($, on) => {
  const clock = mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  let mode: 'ok' | 'broken' | 'none' = 'ok'
  on('process.run', (_$, e) => {
    const argv = (e as { argv: readonly string[] }).argv
    const top = argv.includes('--show-toplevel')
    const r = (exitCode: number, stdout: string, stderr = '') => ({ value: { exitCode, stdout, stderr, isStdoutTruncated: false, isStderrTruncated: false } }) as never
    if (mode === 'none') return r(128, '', 'fatal: not a git repository (or any of the parent directories): .git')
    if (mode === 'broken' && top) return r(128, '', 'fatal: Unable to read current working directory')
    return r(0, top ? 'C:/WORKSPACE/demo' : '')
  })
  on('session.model', () => ({ value: 'claude-opus-5-5' }) as never)
  on('session.version', () => ({ value: { version: '2.1.295' } }) as never)
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  on('fs.read', () => ({ value: '{ "version": "1" }' }) as never)
  on('http.fetch', () => ({ value: { status: 404, headers: {}, text: '' } }) as never)
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await waitFind(pane, 'Arbeitsverzeichnis')).not.toBe(undefined)
  mode = 'broken'
  await clock.advance(30_000)
  expect(await pane.find({ type: 'Text', text: 'Arbeitsverzeichnis' })).not.toBe(undefined)
  mode = 'none'
  await clock.advance(30_000)
  for (let i = 0; i < 100 && (await pane.find({ type: 'Text', text: 'Arbeitsverzeichnis' })); i++) await Promise.resolve()
  expect(await pane.find({ type: 'Text', text: 'Arbeitsverzeichnis' })).toBe(undefined)
})

test('Einstellungen: ▾/▸ klappt jede Gruppe zu und auf', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  expect(await page.find({ type: 'Button', key: 'set-cardLog' })).not.toBe(undefined)
  await page.press({ key: 'set-fold-panel' })
  for (let i = 0; i < 100 && (await page.find({ type: 'Button', key: 'set-cardLog' })); i++) await Promise.resolve()
  expect(await page.find({ type: 'Button', key: 'set-cardLog' })).toBe(undefined)
  expect(JSON.stringify(await page.find({ type: 'Button', key: 'set-fold-panel' }))).toContain('▸')
  await page.press({ key: 'set-fold-panel' })
  for (let i = 0; i < 100 && !(await page.find({ type: 'Button', key: 'set-cardLog' })); i++) await Promise.resolve()
  expect(await page.find({ type: 'Button', key: 'set-cardLog' })).not.toBe(undefined)
})

test('Einstellungen: „Alle einklappen“ und „Alle ausklappen“ schalten alle Karten', async ($, on) => {
  mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  const page = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc-settings', props: SETTINGS_PROPS })
  await page.press({ key: 'set-fold-all' })
  for (let i = 0; i < 100 && (await page.find({ type: 'Button', key: 'set-cardLog' })); i++) await Promise.resolve()
  expect(await page.find({ type: 'Button', key: 'set-cardLog' })).toBe(undefined)
  expect(await page.find({ type: 'Button', key: 'lang-de' })).toBe(undefined)
  await page.press({ key: 'set-unfold-all' })
  for (let i = 0; i < 100 && !(await page.find({ type: 'Button', key: 'set-cardLog' })); i++) await Promise.resolve()
  expect(await page.find({ type: 'Button', key: 'lang-de' })).not.toBe(undefined)
})

test('Arbeitsverzeichnis: verknüpfte Sessions aus der Desktop-App mit Status und Branch', async ($, on) => {
  const clock = mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('process.run', (_$, e) => {
    const top = (e as { argv: readonly string[] }).argv.includes('--show-toplevel')
    return { value: { exitCode: 0, stdout: top ? 'C:/WORKSPACE/demo' : '', stderr: '', isStdoutTruncated: false, isStderrTruncated: false } } as never
  })
  const rows = [{ sessionId: 'local_1', title: 'Sitzungsleiste anbinden', name: 'sl', branch: 'claude/sl', cwd: 'C:/w', isRunning: true, isArchived: false }]
  on('mcp.call', (_$, e) => {
    const c = e as { server: string; tool: string }
    return { value: { content: [{ type: 'text', text: c.tool === 'list_sessions' ? JSON.stringify(rows) : '{}' }], isError: false } } as never
  })
  on('session.model', () => ({ value: 'claude-opus-5-5' }) as never)
  on('session.version', () => ({ value: { version: '2.1.295' } }) as never)
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  on('fs.read', () => ({ value: '{ "version": "1" }' }) as never)
  on('http.fetch', () => ({ value: { status: 404, headers: {}, text: '' } }) as never)
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  await clock.advance(30_000)
  expect(await waitFind(pane, 'Verknüpfte Sessions · 1')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Sitzungsleiste anbinden' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'läuft' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /⎇ claude\/sl/ })).not.toBe(undefined)
})
