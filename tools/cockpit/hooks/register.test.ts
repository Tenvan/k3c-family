import { expect, mock, test } from 'claude-code/testing'

import { THEME, TINT, spinnerText, contextTip, limitTip, cacheTip, RISKY_GIT_CONFIG, paceText, markdownStats, shortHash, statsLines, toolInfoLines, cacheHitPct, cacheLeftMs, pace, breakdownTitle, contextColor, oneLine, parseRepo, parseSteps, repoRows, repoShort, svgSource, toBreakdown, breakdownBarSvg, ringSvg, spinner, title, titleText, toolKind, toolTarget } from './core'

const H = 3600_000
const MEASURE = {
  context: { tokens: 50_000, window: 200_000, percent: 25 },
  rateLimits: [{ kind: 'five_hour', percentUsed: 80, resetsAt: new Date(4 * H).toISOString() }],
  cost: { usd: 1.5 },
  changed: [],
}
const BAND = { plugin: 'tenvan-cc', component: 'AbovePrompt', props: { hasSurvey: false, isWorking: false, maxRows: 3, bodyColumns: 120, scroll: { offset: 0, bodyRows: 3 }, view: {} } } as const

const SUMMARY = { role: 'user' as const, text: 'Zusammenfassung', toolUses: [] }

test('Nach /compact liest das Band den Kontext neu, ohne auf die nächste Messung zu warten', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('session.compact', () => ({ messages: [SUMMARY] }))
  on('session.usage', () => ({ value: { startedAt: 0, context: { window: 200_000, tokens: 20_000, percent: 10 }, rateLimits: [] } }) as never)
  await $.session.measure({ ...MEASURE, context: { tokens: 180_000, window: 200_000, percent: 90 } })
  await $.session.compact({ trigger: 'manual', messages: [SUMMARY, SUMMARY] })
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  for (let i = 0; i < 100 && !JSON.stringify(await band.find({ key: 'compact-pill' })).includes(TINT.success); i++) await Promise.resolve()
  expect(JSON.stringify(await band.find({ key: 'compact-pill' }))).toContain(`"backgroundColor":"${TINT.success}"`)
})

test('Takt: 80 % nach 1 von 5 Stunden liegt 60 Punkte vor', () => {
  expect(pace({ kind: 'five_hour', percentUsed: 80, resetsAt: new Date(4 * H).toISOString() }, 0)).toBe(60)
  expect(pace({ kind: 'spend_limit', percentUsed: 80 }, 0)).toBe(undefined)
})

test('Cache: Subscription hält 1h, sonst 5m', () => {
  expect(cacheLeftMs(null, true, 0)).toBe(0)
  expect(cacheLeftMs(0, true, 10 * 60_000)).toBe(50 * 60_000)
  expect(cacheLeftMs(0, false, 10 * 60_000)).toBe(0)
})

test('Band zeichnet nur auf desktop, Details öffnet das Panel', async ($, on) => {
  mock.clock(on, { now: H })
  const opened: string[] = []
  on('ui.open', ($, e) => {
    opened.push(e.id)
    return { value: { isPlaced: true as const } }
  })
  on('session.measure', () => ({ changed: [] }))
  // Steht für das Engine-eigene Band: leer.
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('ui.render', { component: 'SessionMode' }, () => ({ type: 'Text' as const, text: '' }) as never)
  await $.session.measure(MEASURE)

  // Das Terminal zeichnet dasselbe Band, nur ohne Svg: Kontext als Textbalken statt Ring.
  const terminal = await $.ui.mount({ ...BAND, surface: 'terminal' })
  expect((await terminal.find({ text: /Kontext ▰/ }))?.text).toContain('25%')
  expect(await terminal.find({ type: 'Svg' })).toBe(undefined)

  const desktop = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect((await desktop.find({ text: /5h 80%/ }))?.text).toContain('5h 80%')
  expect(await desktop.find({ key: 'details' })).toBe(undefined)

  // Details steht mit dem Logo in der Fußzeile rechts unter dem Prompt (SessionMode).
  const hint = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'SessionMode', props: { modes: [] } })
  await hint.press({ key: 'details' })
  expect(opened).toEqual(['tenvan-cc'])
})

test('tool.check bleibt beobachtend: ein deny von unten geht unverändert durch', async ($, on) => {
  on('tool.check', () => ({ decision: 'deny', reason: 'Regel' }))
  const verdict = await $.tool.check({ tool: 'Bash', input: { command: 'rm -rf /' } })
  expect(verdict.decision).toBe('deny')
})

test('Panel zeichnet alle Abschnitte auf desktop', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  await $.session.measure(MEASURE)
  const pane = await $.ui.mount({
    plugin: 'tenvan-cc',
    surface: 'desktop',
    component: 'Pane',
    requestId: 'tenvan-cc',
    props: { title: 'Cockpit', isFocused: false, bodyColumns: 60, placement: 'dock', scroll: { offset: 0, bodyRows: 40 }, view: {} },
  })
  for (const title of ['Übersicht', 'Letzter Turn', 'GATE · Freigaben', 'Subagents', 'Log']) {
    expect(await pane.find({ text: title })).toBeDefined()
  }
  expect((await pane.find({ text: /40 Punkte schneller als die Zeit/ }))?.text).toContain('Reset')
})

test('parseSteps: JSON aus Freitext, max. 3, Label erst ab 80 Zeichen gekürzt, prompt-Fallback, Begründung', () => {
  const steps = parseSteps('Hier: [{"label":"Tests schreiben","why":"sichert den Parser ab"},{"label":"Eine sehr lange Beschriftung über dreißig Zeichen","prompt":"x"},{"label":""},{"label":"C"},{"label":"D"},{"label":"E"},{"label":"F"}]')
  expect(steps).toHaveLength(3)
  expect(steps[0]?.why).toBe('sichert den Parser ab')
  expect(steps[1]?.why).toBe(undefined)
  expect(steps[0]).toEqual({ label: 'Tests schreiben', prompt: 'Tests schreiben', why: 'sichert den Parser ab' })
  expect(steps[1]?.label).toBe('Eine sehr lange Beschriftung über dreißig Zeichen')
  expect(parseSteps('kein json')).toEqual([])
})

const STEP_REPLY = '[{"label":"Tests ergänzen","why":"Parser ist ungetestet","prompt":"Ergänze Tests für parseSteps."}]'
const TURN = { answer: 'Fertig.', reason: 'answer', durationMs: 1000, isAborted: false, turnId: 't1' } as const

test('Folge-Schritt: Button füllt das Eingabefeld, erst Enter sendet und leert die Vorschläge', async ($, on) => {
  mock.clock(on, { now: H })
  const filled: string[] = []
  const asked: string[] = []
  on('model.complete', ($, e) => (asked.push(e.prompt), { value: { isAnswered: true, text: STEP_REPLY, usage: { input_tokens: 1, output_tokens: 1, cache_read_input_tokens: 0, cache_creation_input_tokens: 0 } } }))
  on('session.usage', () => ({ value: { startedAt: 0, context: { window: 200_000 }, rateLimits: [] } }))
  on('turn.complete', () => ({ text: 'Fertig.' }))
  on('prompt.fill', ($, e) => {
    filled.push(e.text)
    return { isFilled: true, box: { text: e.text, cursor: e.text.length } }
  })
  on('prompt.submit', ($, e) => ({ text: e.text }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await $.turn.complete(TURN)
  // Der Vorschlag läuft im Hintergrund weiter; warten, bis der Button gezeichnet ist.
  for (let i = 0; i < 100 && !(await band.find({ key: 'step-0' })); i++) await Promise.resolve()

  expect(JSON.stringify(await band.find({ key: 'step-0' }))).toContain('"hotkey":"1"')
  // Begründung als Tooltip am Info-Symbol, nicht als Zeile.
  expect(JSON.stringify(await band.findAll({ type: 'Svg' }))).toContain('Parser ist ungetestet')
  // ↻ holt neue Vorschläge und nennt dem Modell die bisherigen.
  await band.press({ key: 'steps-refresh' })
  for (let i = 0; i < 100 && asked.length < 2; i++) await Promise.resolve()
  expect(asked[1]).toContain('Schon vorgeschlagen')
  await band.press({ key: 'step-0' })
  expect(filled).toEqual(['Ergänze Tests für parseSteps.'])
  expect(await band.find({ key: 'step-0' })).not.toBe(undefined)
  // Enter der Person: prompt.submit aus dem Eingabefeld startet den Turn.
  await $.prompt.submit({ text: filled[0] } as never)
  expect(await band.find({ key: 'step-0' })).toBe(undefined)
  // Ohne Vorschläge bleibt die Zeile mit „↻ Neue“ stehen.
  expect(await band.find({ type: 'Text', text: 'keine Vorschläge' })).not.toBe(undefined)
  expect(await band.find({ type: 'Button', key: 'steps-refresh' })).not.toBe(undefined)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ type: 'Text', text: /Agent arbeitet/ })).not.toBe(undefined)
})

test('Folge-Schritte abschaltbar über die Option nextSteps', { options: { nextSteps: false } }, async ($, on) => {
  let calls = 0
  on('model.complete', () => {
    calls += 1
    return { value: { isAnswered: true, text: STEP_REPLY, usage: { input_tokens: 0, output_tokens: 0, cache_read_input_tokens: 0, cache_creation_input_tokens: 0 } } }
  })
  on('session.usage', () => ({ value: { startedAt: 0, context: { window: 200_000 }, rateLimits: [] } }))
  on('turn.complete', () => ({ text: 'Fertig.' }))
  await $.turn.complete(TURN)
  expect(calls).toBe(0)
})

const USAGE0 = { input_tokens: 0, output_tokens: 0, cache_read_input_tokens: 0, cache_creation_input_tokens: 0 }
const PANE_PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 60, placement: 'dock', scroll: { offset: 0, bodyRows: 40 }, view: {} } as const

test('cacheHitPct: Anteil Cache-Lesen an der Eingabe', () => {
  expect(cacheHitPct({ ...USAGE0, input_tokens: 10, cache_read_input_tokens: 90 })).toBe(90)
  expect(cacheHitPct(USAGE0)).toBe(null)
})

test('Ab 85 % Kontext tönt das Band Kompaktieren rot; der Knopf schreibt /compact ins Eingabefeld', async ($, on) => {
  mock.clock(on, { now: H })
  const filled: string[] = []
  on('session.measure', () => ({ changed: [] }))
  on('prompt.fill', ($, e) => {
    filled.push(e.text)
    return { isFilled: true, box: { text: e.text, cursor: 0 } }
  })
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  await $.session.measure({ ...MEASURE, context: { tokens: 180_000, window: 200_000, percent: 90 } })
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect(JSON.stringify(await band.find({ key: 'compact-pill' }))).toContain(`"backgroundColor":"${TINT.error}"`)
  await band.press({ key: 'compact' })
  for (let i = 0; i < 100 && filled.length === 0; i++) await Promise.resolve()
  expect(filled).toEqual(['/compact'])
})

test('turn.step: Trefferquote des Haupt-Requests erscheint im Panel', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('turn.step', async function* () {
    return { turnId: 't1', index: 0, answer: '', toolUses: [], stopReason: 'end_turn', usage: { ...USAGE0, input_tokens: 4, cache_read_input_tokens: 96, model: 'm' } }
  })
  await $.session.measure(MEASURE)
  for await (const _ of $.turn.step({ turnId: 't1', index: 0, model: 'm', messageCount: 1 })) {
    // Stream leer durchlaufen
  }
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect((await pane.find({ text: /Trefferquote 96 %/ }))?.text).toContain('Trefferquote 96 %')
})

test('Abgebrochener Turn: kein Vorschlag, Agent-Status über agent.list abgeglichen', async ($, on) => {
  mock.clock(on, { now: H })
  let calls = 0
  on('model.complete', () => {
    calls += 1
    return { value: { isAnswered: true, text: '[]', usage: USAGE0 } }
  })
  on('agent.spawn', () => ({ model: 'm', agentId: 'a1' }))
  on('agent.list', () => ({ value: [{ id: 'a1', description: 'Suche', type: 'Explore', status: 'completed' as const }] }))
  on('session.usage', () => ({ value: { startedAt: 0, context: { window: 200_000 }, rateLimits: [] } }))
  on('turn.complete', () => ({ text: 'Fertig.' }))
  await $.agent.spawn({ tool_use_id: 'u1', prompt: 'p', description: 'Suche', subagentType: 'Explore', provider: { plugin: 'test', tier: 'user' }, parentModel: 'm', background: true, fork: false })
  await $.turn.complete({ ...TURN, reason: 'aborted', isAborted: true })
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ text: /✓ fertig/ })).toBeDefined()
  expect(calls).toBe(0)
})

test('title: Version ohne Build-Stempel', () => {
  expect(title(null)).toBe('Tenvan CC')
  expect(titleText(null)).toBe('Tenvan CC')
  expect(title({ version: '0.3.0', builtAt: new Date(2026, 9, 12, 9, 5).getTime() })).toBe('Tenvan CC (Version 0.3.0)')
})

test('Titelleiste in Band und Panel mit Version aus plugin.json', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  on('fs.read', (_, e) => ({ value: String(e.path).endsWith('logo.svg') ? '<?xml version="1.0"?><!-- x --><svg xmlns="http://www.w3.org/2000/svg"/>' : '{ "version": "9.9.9" }' }))
  on('fs.stat', () => ({ value: { kind: 'file' as const, size: 1, mtimeMs: new Date(2026, 9, 12, 14, 30).getTime(), isLink: false } }))
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  await $.session.measure(MEASURE)
  const expected = 'Tenvan CC (Version 9.9.9)'
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect((await band.find({ type: 'Text', text: expected }))?.text).toBe(expected)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect((await pane.find({ type: 'Text', text: expected }))?.text).toBe(expected)
})

test('Kompaktieren ist immer sichtbar, Legende im Tooltip der Limit-Ringe (Panel ohne Legende)', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  await $.session.measure(MEASURE)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect(await band.find({ key: 'compact' })).not.toBe(undefined)
  const leg = JSON.stringify((await band.findAll({ type: 'Svg' })).find(x => JSON.stringify(x).includes('5-Stunden-Limit')))
  expect(leg).toContain('🟩 im Plan – ')
  expect(leg).toContain('"isInteractive":true')
  expect((await band.findAll({ type: 'Svg' })).some(x => JSON.stringify(x).includes('im Plan · zu schnell'))).toBe(false)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ type: 'Text', text: 'Legende' })).toBe(undefined)
})

test('toolKind: Datei, Shell, Sonstige', () => {
  expect([toolKind('Edit'), toolKind('Bash'), toolKind('WebFetch')]).toEqual(['file', 'shell', 'other'])
})

test('Protokoll: Prompt gekürzt, Ziel des Tool-Calls, Pfade auf zwei Segmente', () => {
  expect(oneLine('a\n  b   c', 80)).toBe('a b c')
  expect(oneLine('x'.repeat(100), 10)).toBe('xxxxxxxxx…')
  expect(toolTarget({ file_path: ['C:', 'W', 'bridge', 'appconfig.go'].join(String.fromCharCode(92)) })).toBe('bridge/appconfig.go')
  expect(toolTarget({ pattern: 'erpapi' })).toBe('erpapi')
  expect(toolTarget(undefined)).toBe('')
})

test('Spinner: wechselt das Zeichen je Frame', () => {
  expect(spinner(0)).not.toBe(spinner(1))
})

test('Panel zeigt „Agent arbeitet“ solange ein Turn läuft', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  on('fs.read', () => ({ value: '{ "version": "1" }' }))
  on('fs.stat', () => ({ value: { kind: 'file' as const, size: 1, mtimeMs: 1, isLink: false } }))
  on('prompt.submit', () => ({ text: 'los' }))
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  await $.prompt.submit({ text: 'los' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ type: 'Text', text: /Agent arbeitet/ })).not.toBe(undefined)
})

test('Log: nur die letzten 10 Einträge, Prompt auf 80 Zeichen gekürzt, Quelle you', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('prompt.submit', () => ({ text: '' }))
  for (let i = 1; i <= 12; i++) await $.prompt.submit({ text: `prompt-${String(i).padStart(2, '0')}` } as never)
  await $.prompt.submit({ text: `lang-${'x'.repeat(200)}` } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  expect(await pane.find({ type: 'Text', text: /prompt-03/ })).toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /prompt-04/ })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /prompt-12/ })).not.toBe(undefined)
  const long = await pane.find({ type: 'Text', text: /^lang-x+…$/ })
  expect(long?.text?.length).toBe(80)
  expect(await pane.find({ type: 'Text', text: /^you/ })).not.toBe(undefined)
})

test('Ringdiagramm: Bogenlänge nach Prozent, Farbe nach Füllstand', () => {
  const full = ringSvg(100, '#fff')
  const half = ringSvg(50, '#fff')
  expect(full).toContain('stroke-dasharray="69.12 69.12"')
  expect(half).toContain('stroke-dasharray="34.56 69.12"')
  expect(ringSvg(0, '#fff')).not.toContain('stroke-dasharray')
  expect([contextColor(10), contextColor(55), contextColor(75), contextColor(85)]).toEqual(['success', 'accent', 'warning', 'error'])
})

test('Band zeichnet Ringe für Kontext und Limits', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  await $.session.measure(MEASURE)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect(await band.find({ type: 'Svg' })).not.toBe(undefined)
})

const BREAKDOWN = {
  categories: [
    { name: 'Messages', tokens: 288_800, kind: 'used' },
    { name: 'MCP tools (deferred)', tokens: 95_800, kind: 'deferred' },
    { name: 'Free space', tokens: 621_900, kind: 'free' },
    { name: 'Leer', tokens: 0, kind: 'used' },
  ],
  totalTokens: 347_200,
  rawMaxTokens: 1_000_000,
  percentage: 35,
}

test('toBreakdown: ohne deferred und leere Zeilen, Titel nach Vorbild', () => {
  const b = toBreakdown(BREAKDOWN)
  expect(b.rows.map(r => r.name)).toEqual(['Messages', 'Free space'])
  expect(b.rows[0]?.percent).toBe(28.9)
  expect(breakdownTitle(b)).toBe('Kontextfenster 347k / 1.0M (35 %)')
})

test('Band: Ringe tragen die Tooltips (interaktives Svg im Leerlauf)', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('session.usage', () => ({ value: { startedAt: 0, context: { window: 1_000_000, tokens: 347_200, percent: 35, breakdown: BREAKDOWN }, rateLimits: [] } }) as never)
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  await $.session.measure(MEASURE)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  const alt = async () => JSON.stringify(await band.find({ type: 'Svg', text: /Kontextfenster 347k/ }))
  for (let i = 0; i < 3000 && (await alt()) === undefined; i++) await Promise.resolve()
  expect(await band.find({ type: 'Svg' })).not.toBe(undefined)
  const dump = JSON.stringify(await band.find({ type: 'Svg' }))
  expect(dump).toContain('alt')
})

test('toBreakdown: belegt absteigend, Puffer davor, Free space immer unten; Balken mit einem Segment je Zeile', () => {
  const b = toBreakdown({
    categories: [
      { name: 'Free space', tokens: 500_000, kind: 'free' },
      { name: 'Klein', tokens: 10_000, kind: 'used' },
      { name: 'Puffer', tokens: 45_000, kind: 'buffer' },
      { name: 'Groß', tokens: 300_000, kind: 'used' },
    ],
    totalTokens: 355_000,
    rawMaxTokens: 1_000_000,
    percentage: 35,
  })
  expect(b.rows.map(r => r.name)).toEqual(['Groß', 'Klein', 'Puffer', 'Free space'])
  expect(breakdownBarSvg(b).match(/<rect x=/g)?.length).toBe(4)
})

test('Tooltips: Kontext, Limit und Cache als Klartext, Titel im Svg maskiert', () => {
  const b = toBreakdown(BREAKDOWN)
  expect(contextTip({ contextTokens: 347_200, contextWindow: 1_000_000, contextPercent: 35 }, b)).toBe(
    ['Kontextfenster 347k / 1.0M (35 %)', '🟦🟦🟦  Messages: 289k (28.9 %)', '⬜⬜⬜⬜⬜⬜  Free space: 622k (62.2 %)', 'Kompaktieren wird ab 85 % empfohlen.'].join('\n'),
  )
  expect(contextTip({ contextTokens: 1000, contextWindow: 200_000, contextPercent: 1 }, null)).toContain('Kontextfenster 1k / 200k (1 %)')
  const now = new Date(2026, 9, 12, 9, 0).getTime()
  const tip = limitTip({ kind: 'five_hour', percentUsed: 80, resetsAt: new Date(now + 4 * H).toISOString() }, now)
  expect(tip).toContain('5-Stunden-Limit: 80 % verbraucht')
  expect(tip).toContain('in 4h 0m')
  expect(cacheTip(0, 3600_000, null, null, now)).toContain('kalt')
  expect(cacheTip(600_000, 3600_000, 96, now - 60_000, now)).toContain('Trefferquote des letzten Requests: 96 %')
  expect(ringSvg(50, '#fff', 28, 'a < b & c')).toContain('<title>a &lt; b &amp; c</title>')
})

test('svgSource: ohne XML-Kopf und Kommentare, null bei Nicht-SVG', () => {
  expect(svgSource('<?xml version="1.0"?>\n<!-- c -->\n<svg a="1"/>')).toBe('<svg a="1"/>')
  expect(svgSource('{ "version": "1" }')).toBe(null)
})

test('Repo-Infozeile: Branch, Worktree, Commit, Änderungen, Upstream, Ordner', () => {
  const linked = parseRepo({ top: 'C:/W/repo/.claude/worktrees/x1', gitDir: 'C:/W/repo/.git/worktrees/x1', commonDir: 'C:/W/repo/.git', branch: 'claude/x1', sha: 'abc1234', status: ' M a\n?? b', counts: '2\t1', prefix: 'plugins/' })
  expect(linked).toMatchObject({ name: 'x1', isLinked: true, dirty: 2, behind: 2, ahead: 1, subdir: 'plugins' })
  expect(repoShort(linked)).toBe('⎇ claude/x1')
  expect(repoRows(linked)).toEqual([
    ['Branch', 'claude/x1'],
    ['Worktree', 'x1 (verknüpft)'],
    ['Commit', 'abc1234'],
    ['Änderungen', '2 geänderte Dateien'],
    ['Upstream', '↑1 voraus · ↓2 hinterher'],
    ['Ordner', 'C:/W/repo/.claude/worktrees/x1'],
    ['Unterordner', 'plugins'],
  ])
  const main = parseRepo({ top: 'C:/W/repo', gitDir: 'C:/W/repo/.git', commonDir: 'C:/W/repo/.git', branch: '', sha: 'abc1234', status: '', counts: '', prefix: '' })
  expect(repoShort(main)).toBe('⎇ detached')
  expect(repoRows(main).map(r => r[1])).toEqual(['detached', 'repo (Hauptcheckout)', 'abc1234', 'sauber', 'kein Upstream', 'C:/W/repo'])
  // Riskante Repo-Konfiguration: Status wird nicht ausgeführt, die Zeile sagt es.
  const untrusted = parseRepo({ top: 'C:/W/repo', gitDir: 'C:/W/repo/.git', commonDir: 'C:/W/repo/.git', branch: 'x', sha: 'abc1234', status: null, counts: '', prefix: '' })
  expect(untrusted.dirty).toBe(null)
  expect(repoRows(untrusted)[3]?.[1]).toContain('nicht geprüft')
  const re = new RegExp(RISKY_GIT_CONFIG)
  expect(['filter.lfs.clean', 'core.fsmonitor', 'diff.x.textconv', 'diff.external', 'alias.st', 'include.path', 'includeif.gitdir:/x/.path', 'extensions.worktreeconfig', 'gpg.program', 'gpg.ssh.program', 'log.showsignature'].every(k => re.test(k))).toBe(true)
  expect(['core.autocrlf', 'user.name', 'remote.origin.url'].some(k => re.test(k))).toBe(false)
})

test('markdownStats: Codeblöcke, Diffs, Mermaid, Formeln, Tabellen, Links', () => {
  const text = ['Text mit [Link](https://a.de) und https://b.de', '```ts', 'x', '```', '```diff', '-a', '+b', '```', '```mermaid', 'graph TD', '```', '$$', 'x', '$$', '| a | b |', '|---|---|', '| 1 | 2 |'].join('\n')
  expect(markdownStats(text)).toEqual({ code: 1, diff: 1, mermaid: 1, math: 1, tables: 1, links: 2 })
  expect(statsLines(markdownStats('nur Text'))).toEqual([])
  expect(statsLines(markdownStats(text))[1]).toBe('Codeblöcke 1 · Diffs 1 · Mermaid 1 · Formeln 1 · Tabellen 1 · Links 2')
})

test('toolInfoLines: Art, Ziel, Status, Umfang einer Änderung', () => {
  expect(toolInfoLines('Edit', { file_path: 'a/b/c.go', old_string: 'x\ny', new_string: 'z' }, 'fertig')).toEqual(['Edit (Datei) · fertig', 'Ziel: b/c.go', 'Umfang: −2 / +1 Zeilen'])
  expect(toolInfoLines('Bash', { command: 'ls' }, 'läuft')).toEqual(['Bash (Shell) · läuft', 'Ziel: ls'])
  expect(shortHash('a')).toBe(shortHash('a'))
  expect(shortHash('a')).not.toBe(shortHash('b'))
})

test('paceText: in Worten statt +/- Zahl', () => {
  expect(paceText(60)).toBe('60 Punkte schneller als die Zeit')
  expect(paceText(-12)).toBe('12 Punkte Reserve')
  expect(paceText(0)).toBe('genau im Plan')
})

test('Panel: Karte Kontextfenster nennt den Grund, wenn die Engine keine Aufteilung liefert', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('session.usage', () => ({ value: { startedAt: 0, context: { window: 1_000_000, tokens: 1, percent: 1 }, rateLimits: [] } }) as never)
  await $.session.measure(MEASURE)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  for (let i = 0; i < 3000 && !(await pane.find({ type: 'Text', text: /keine Aufteilung/ })); i++) await Promise.resolve()
  expect(await pane.find({ type: 'Text', text: /keine Aufteilung/ })).not.toBe(undefined)
})

test('Arbeitszeile: Phase in Token-Farbe, Schritt und Zähler; Platzhalter Working entfällt; abschaltbar', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('prompt.submit', () => ({ text: 'los' }))
  await $.prompt.submit({ text: 'los' } as never)
  const props = { word: 'Creating notes.md', message: null, suffix: '…', mode: 'thinking' as const }
  const line = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Spinner', props })
  const phase = await line.find({ type: 'Text', text: /Denkt nach/ })
  expect(JSON.stringify(phase)).toContain(THEME.info)
  expect(await line.find({ type: 'Text', text: 'Creating notes.md' })).not.toBe(undefined)
  expect(await line.find({ type: 'Text', text: /0 Tool-Calls/ })).not.toBe(undefined)
  expect(spinnerText('Working', null)).toBe('')
  expect(spinnerText('x', 'Kompaktiert')).toBe('Kompaktiert')
})

test('Dauerzeile: Fertig nach Dauer in Erfolgsfarbe', async ($, on) => {
  const line = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'terminal', component: 'TurnDuration', props: { word: 'Baked', durationMs: 64_000 } })
  expect(JSON.stringify(await line.find({ type: 'Text', text: '✓ Fertig' }))).toContain(THEME.success)
  expect(await line.find({ type: 'Text', text: / nach 1m 4s/ })).not.toBe(undefined)
})

test('Band: Ring-Tooltips aus, solange ein Subagent läuft', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('agent.spawn', () => ({ model: 'm', agentId: 'a1' }))
  await $.session.measure(MEASURE)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  // Kontext-Ring (25 %): im Leerlauf interaktiv mit Tooltip als alt, sonst nur Prozent
  const ring = async () => JSON.stringify((await band.findAll({ type: 'Svg' })).find(x => /"alt":"(Kontextfenster|25 %)/.test(JSON.stringify(x))))
  expect(await ring()).toContain('"isInteractive":true')
  expect(await ring()).toContain('"alt":"Kontextfenster 50k / 200k (25 %)"')

  await $.agent.spawn({ tool_use_id: 'u1', prompt: 'p', description: 'Suche', subagentType: 'Explore', provider: { plugin: 'test', tier: 'user' }, parentModel: 'm', background: true, fork: false })
  for (let i = 0; i < 3000 && (await ring()).includes('isInteractive'); i++) await Promise.resolve()
  expect(await ring()).not.toContain('isInteractive')
  expect(await ring()).toContain('"alt":"25 %"')
  // Limit-Ring mit Legende im Tooltip ist dann ebenfalls nur Bild
  const leg = JSON.stringify((await band.findAll({ type: 'Svg' })).find(x => JSON.stringify(x).includes('"alt":"80 %"')))
  expect(leg).not.toContain('"isInteractive":true')
})

test('Farben: Band und Panel nutzen THEME, keinen Theme-Schlüssel claude', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('agent.spawn', () => ({ model: 'm', agentId: 'a1' }))
  on('prompt.submit', () => ({ text: 'los' }))
  await $.session.measure(MEASURE)
  await $.agent.spawn({ tool_use_id: 'u1', prompt: 'p', description: 'Suche', subagentType: 'Explore', provider: { plugin: 'test', tier: 'user' }, parentModel: 'm', background: true, fork: false })
  await $.prompt.submit({ text: 'los' } as never)
  const band = JSON.stringify(await (await $.ui.mount({ ...BAND, surface: 'desktop', })).find({ type: 'Box' }))
  const pane = JSON.stringify(await (await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })).find({ type: 'Box' }))
  for (const tree of [band, pane]) {
    expect(tree).not.toContain('"claude"')
    expect(tree).toContain(THEME.accent)
  }
  expect(pane).toContain('Agent arbeitet')
})

test('Verzeichniswächter: Wechsel aus dem Session-Ordner wird zur Rückfrage, danach Warnung im Band', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('session.cwd', () => ({ value: 'C:\\WORKSPACE\\k3c' }) as never)
  on('session.root', () => ({ value: 'C:\\WORKSPACE\\k3c' }) as never)
  on('env.get', () => ({ value: 'C:\\Users\\r' }) as never)
  on('tool.check', () => ({ decision: 'allow' }))
  on('tool.call', () => ({ result: { stdout: '', stderr: '', interrupted: false } }) as never)
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  const away = await $.tool.check({ tool: 'mcp__ccd_directory__change_directory', input: { path: 'C:\\WORKSPACE\\ErpApi' } } as never)
  expect(away.decision).toBe('ask')
  expect(away.reason).toContain('ErpApi')
  const inside = await $.tool.check({ tool: 'Bash', input: { command: 'cd src && ls' } } as never)
  expect(inside.decision).toBe('allow')
  await $.session.measure(MEASURE)
  await $.tool.call({ tool: 'Bash', command: 'cd /c/WORKSPACE/ErpApi && git status' } as never)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  for (let i = 0; i < 200 && !(await band.find({ key: 'dir-warn' })); i++) await Promise.resolve()
  expect(await band.find({ type: 'Text', text: /außerhalb des Session-Ordners: C:\\WORKSPACE\\ErpApi/ })).not.toBe(undefined)
})
