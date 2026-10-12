import { expect, mock, test } from 'claude-code/testing'

import { AGENT_STEPS_WARN, LOOP_EDITS, LOOP_ERRORS, THEME, compactFocus, detectLoop, isAgentHeavy, isWasteful, mcpDiet, mcpKey, turnBarsSvg } from './core'

const H = 3600_000
const MEASURE = { context: { tokens: 50_000, window: 200_000, percent: 25 }, rateLimits: [], cost: { usd: 1.5 }, changed: [] }
const BAND = { plugin: 'orgamax-cockpit', component: 'AbovePrompt', props: { hasSurvey: false, isWorking: false, maxRows: 3, bodyColumns: 120, scroll: { offset: 0, bodyRows: 3 }, view: {} } } as const
const PANE_PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 144, placement: 'dock', scroll: { offset: 0, bodyRows: 60 }, view: {} } as const
const SPAWN = { tool_use_id: 'u1', prompt: 'p', description: 'Suche', subagentType: 'Explore', provider: { plugin: 'test', tier: 'user' }, parentModel: 'm', background: true, fork: false } as const

test('compactFocus: Ziel, geänderte Dateien und offene Schritte; leer ohne Wissen', () => {
  const f = compactFocus('Baue die Bremse ein', ['C:\\p\\hooks\\core.ts'], [{ label: 'Tests' }, { label: 'Doku' }])
  expect(f).toBe('Behalte für die weitere Arbeit: Ziel „Baue die Bremse ein“ · geänderte Dateien hooks/core.ts · offene Schritte Tests; Doku')
  expect(compactFocus('', [], [])).toBe('')
})

test('detectLoop: wiederholte Fehler gewinnen vor wiederholten Edits; Einzelfälle sind keine Schleife', () => {
  const err = (key: string) => ({ key, isError: true, isEdit: false })
  const edit = (key: string) => ({ key, isError: false, isEdit: true })
  expect(detectLoop([err('Bash → npm test'), err('Read → a.ts')])).toBe(null)
  expect(detectLoop(Array.from({ length: LOOP_ERRORS }, () => err('Bash → npm test')))).toEqual({ kind: 'error', target: 'Bash → npm test', n: LOOP_ERRORS })
  const edits = Array.from({ length: LOOP_EDITS }, () => edit('Edit → a.ts'))
  expect(detectLoop(edits)).toEqual({ kind: 'edit', target: 'Edit → a.ts', n: LOOP_EDITS })
  // Fehler außerhalb des Fensters der letzten 8 Aufrufe zählen nicht mehr.
  expect(detectLoop([...Array.from({ length: LOOP_ERRORS }, () => err('x')), ...Array.from({ length: 8 }, () => edit(`f${Math.random()}`))])).toBe(null)
})

test('mcpDiet: nur geladene, nie aufgerufene Server ab 500 Tokens, teuerste zuerst', () => {
  const tools = [
    { name: 'mcp__plugin_serena_serena__find_symbol', serverName: 'plugin:serena:serena', tokens: 900, isLoaded: true },
    { name: 'mcp__clickup__a', serverName: 'clickup', tokens: 3000, isLoaded: true },
    { name: 'mcp__clickup__b', serverName: 'clickup', tokens: 1000, isLoaded: true },
    { name: 'mcp__tiny__a', serverName: 'tiny', tokens: 100, isLoaded: true },
    { name: 'mcp__lazy__a', serverName: 'lazy', tokens: 5000, isLoaded: false },
  ]
  expect(mcpKey('mcp__plugin_serena_serena__find_symbol')).toBe('plugin_serena_serena')
  expect(mcpKey('Read')).toBe(null)
  expect(mcpDiet(tools, [])).toEqual([
    { server: 'clickup', tokens: 4000, tools: 2 },
    { server: 'plugin:serena:serena', tokens: 900, tools: 1 },
  ])
  expect(mcpDiet(tools, ['plugin_serena_serena', 'clickup'])).toEqual([])
})

test('Turn-Bilanz: teurer Turn ohne Edit in Warnfarbe', () => {
  expect(isWasteful({ usd: 0.4, tools: 9, edits: 0, durationMs: 1 })).toBe(true)
  expect(isWasteful({ usd: 0.4, tools: 9, edits: 1, durationMs: 1 })).toBe(false)
  expect(isWasteful({ tools: 9, edits: 0, durationMs: 1 })).toBe(false)
  const svg = turnBarsSvg([{ usd: 0.1, tools: 1, edits: 1, durationMs: 1 }, { usd: 0.4, tools: 9, edits: 0, durationMs: 1 }])
  expect(svg.match(/<rect/g)?.length).toBe(2)
  expect(svg).toContain(THEME.warning)
})

test('isAgentHeavy: viele Tool-Calls oder lange Laufzeit', () => {
  expect(isAgentHeavy({ steps: AGENT_STEPS_WARN, since: 0 }, 0)).toBe(true)
  expect(isAgentHeavy({ steps: 1, since: 0 }, 11 * 60_000)).toBe(true)
  expect(isAgentHeavy({ steps: 1, since: 0 }, 60_000)).toBe(false)
})

test('Band: Kompaktieren schreibt /compact mit Fokus ins Eingabefeld', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('prompt.submit', ($, e) => ({ text: e.text }))
  const filled: string[] = []
  on('prompt.fill', ($, e) => {
    filled.push(e.text)
    return { isFilled: true, box: { text: e.text, cursor: e.text.length } }
  })
  await $.session.measure(MEASURE)
  await $.prompt.submit({ text: 'Baue die Bremse ein' } as never)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  // Ohne bisherigen Turn ist der Cache kalt: der nächste Turn liest den ganzen Kontext neu.
  expect(await band.find({ text: /kalt · 50k neu/ })).not.toBe(undefined)
  await band.press({ key: 'compact' })
  for (let i = 0; i < 100 && filled.length === 0; i++) await Promise.resolve()
  expect(filled[0]).toBe('/compact Behalte für die weitere Arbeit: Ziel „Baue die Bremse ein“')
})

test('Band: wiederholter Fehler zeigt Schleifen-Warnung, der Knopf schlägt einen anderen Ansatz vor', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  on('tool.call', () => ({ result: {}, isError: true }) as never)
  const filled: string[] = []
  on('prompt.fill', ($, e) => {
    filled.push(e.text)
    return { isFilled: true, box: { text: e.text, cursor: 0 } }
  })
  await $.session.measure(MEASURE)
  for (let i = 0; i < LOOP_ERRORS; i++) await $.tool.call({ tool: 'Bash', tool_use_id: `b${i}`, input: { command: 'npm test' } } as never)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  expect(await band.find({ text: /Schleife\? Bash → npm test ist 3× fehlgeschlagen/ })).not.toBe(undefined)
  await band.press({ key: 'loop-fix' })
  for (let i = 0; i < 100 && filled.length === 0; i++) await Promise.resolve()
  expect(filled[0]).toContain('anderen Ansatz')
})

test('Agent-Bremse: teurer Subagent bekommt den Knopf, danach werden seine Tool-Calls abgelehnt', async ($, on) => {
  mock.clock(on, { now: H })
  on('agent.spawn', () => ({ model: 'm', agentId: 'a1' }))
  on('tool.call', () => ({ result: {} }) as never)
  await $.agent.spawn(SPAWN)
  for (let i = 0; i < AGENT_STEPS_WARN; i++) await $.tool.call({ tool: 'Read', tool_use_id: `r${i}`, input: { file_path: 'a.ts' }, agentId: 'a1' } as never)
  const pane = await $.ui.mount({ plugin: 'orgamax-cockpit', surface: 'desktop', component: 'Pane', requestId: 'orgamax-cockpit', props: PANE_PROPS })
  expect(await pane.find({ text: /teuer: 40 Tool-Calls/ })).not.toBe(undefined)
  await pane.press({ key: 'brake-a1' })
  for (let i = 0; i < 100 && !(await pane.find({ text: 'gebremst' })); i++) await Promise.resolve()
  const ran = (await $.tool.call({ tool: 'Read', tool_use_id: 'r-x', input: { file_path: 'a.ts' }, agentId: 'a1' } as never)) as { deny?: string }
  expect(ran.deny).toContain('gebremst')
  // Der Haupt-Agent ist nicht betroffen.
  const main = (await $.tool.call({ tool: 'Read', tool_use_id: 'r-y', input: { file_path: 'a.ts' } } as never)) as { deny?: string }
  expect(main.deny).toBe(undefined)
})

test('Features: Knopf hinter der Legende schreibt /cockpit-features ins Eingabefeld', async ($, on) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
  const filled: string[] = []
  on('prompt.fill', ($, e) => {
    filled.push(e.text)
    return { isFilled: true, box: { text: e.text, cursor: 0 } }
  })
  await $.session.measure(MEASURE)
  const band = await $.ui.mount({ ...BAND, surface: 'desktop' })
  await band.press({ key: 'features' })
  for (let i = 0; i < 100 && filled.length === 0; i++) await Promise.resolve()
  expect(filled).toEqual(['/cockpit-features'])
})

test('/cockpit-features liest die Übersicht in der eingestellten Sprache', async ($, on) => {
  const paths: string[] = []
  on('fs.read', ($, e) => {
    paths.push(e.path)
    return { value: '🛩️ Features' }
  }) as never
  on('store.get', () => ({ value: undefined }) as never)
  const out = await $.command.run({ command: 'cockpit-features', args: '' } as Parameters<typeof $.command.run>[0])
  expect(out.text).toBe('🛩️ Features')
  // Die Engine normalisiert den Pfad (unter Windows mit Backslashes).
  expect(paths.some(p => p.replace(/\\/g, '/').endsWith('assets/features.de.md'))).toBe(true)
})

