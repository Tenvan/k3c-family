import { expect, mock, test } from 'claude-code/testing'

// Profiling (Kontext-Inspektor, Agent-Profiler, Blast-Radius): Verdrahtung in register.tsx – Band-Kurzinfos, ein Panel mit drei Karten auf
// Desktop und Terminal, Befehle, Schalter. Die Logik der Module testen inspect.test.ts, profiler.test.ts und blast.test.ts.

const H = Date.UTC(2026, 9, 20, 10)
const PANE_PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 120, placement: 'dock', scroll: { offset: 0, bodyRows: 40 }, view: {} } as const
const BAND_PROPS = { hasSurvey: false, isWorking: false, maxRows: 10, bodyColumns: 120, scroll: { offset: 0, bodyRows: 10 }, view: {} } as const
const MEASURE = {
  context: { tokens: 50_000, window: 200_000, percent: 25 },
  rateLimits: [],
  cost: { usd: 1.5 },
  changed: [],
}
const USAGE0 = { input_tokens: 0, output_tokens: 0, cache_read_input_tokens: 0, cache_creation_input_tokens: 0 }
const stubs = (on: Parameters<typeof mock.clock>[0]) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
}

for (const surface of ['desktop', 'terminal'] as const) {
  test(`Profiling (${surface}): ein Panel mit drei Karten, Knöpfe und Falten`, async ($, on) => {
    stubs(on)
    const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc-profiling', props: PANE_PROPS })
    for (const title of ['Kontext-Inspektor', 'Agent-Profiler', 'Blast-Radius']) expect(await pane.find({ type: 'Text', text: title })).not.toBe(undefined)
    expect(await pane.find({ type: 'Text', text: /^(Zeitverteilung|Überblick|Ziele)/ })).not.toBe(undefined)
    for (const key of ['diag-agents', 'diag-refresh', 'card-fold-diag-profile']) expect(await pane.find({ type: 'Button', key })).not.toBe(undefined)
    if (surface === 'terminal') expect(await pane.find({ type: 'Svg' })).toBe(undefined)
    await pane.press({ key: 'card-fold-diag-blast' })
    for (let i = 0; i < 200 && (await pane.find({ type: 'Button', key: 'diag-refresh' })) !== undefined; i++) await Promise.resolve()
    expect(await pane.find({ type: 'Button', key: 'diag-refresh' })).toBe(undefined)
  })

  test(`Diagnose (${surface}): Band zeigt Kurzinfos als Knöpfe, Einstellung blendet sie aus`, async ($, on) => {
    stubs(on)
    await $.session.measure(MEASURE)
    const band = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'AbovePrompt', props: BAND_PROPS })
    expect(await band.find({ type: 'Button', key: 'diag-inspect' })).not.toBe(undefined)
    expect(JSON.stringify(await band.find({ key: 'diag-inspect-pill' }))).toContain('Kontext')

    const page = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc-settings', props: PANE_PROPS })
    await page.press({ key: 'set-inspectBand' })
    for (let i = 0; i < 200 && (await band.find({ type: 'Button', key: 'diag-inspect' })) !== undefined; i++) await Promise.resolve()
    expect(await band.find({ type: 'Button', key: 'diag-inspect' })).toBe(undefined)
  })
}

test('Diagnose: Ereignisse laufen durch die Hooks, Band und Befehle zeigen das Ergebnis', async ($, on) => {
  stubs(on)
  on('tool.call', () => ({ result: { stdout: 'ok', stderr: '', interrupted: false } }) as never)
  on('turn.step', async function* () {
    return { turnId: 't1', index: 0, answer: '', toolUses: [], stopReason: 'end_turn', usage: { ...USAGE0, input_tokens: 4, cache_read_input_tokens: 96, model: 'm' } }
  })
  on('turn.start', () => ({ turnId: 't1' }))
  on('turn.complete', () => ({ text: 'Fertig.' }))
  on('session.append', () => ({}) as never)
  on('prompt.submit', ($, e) => ({ text: e.text }))
  await $.session.measure(MEASURE)
  await $.turn.start({ turnId: 't1' } as never)
  for await (const _ of $.turn.step({ turnId: 't1', index: 0, model: 'm', messageCount: 1 })) {
    // Stream leer durchlaufen
  }
  await $.tool.call({ tool: 'Bash', command: 'npm test', tool_use_id: 'u1' } as never)
  await $.turn.complete({ answer: 'Fertig.', reason: 'answer', durationMs: 1000, isAborted: false, turnId: 't1' } as never)

  const report = await $.command.run({ command: 'cockpit-profil', args: 'text' } as never)
  expect(JSON.stringify(report)).toContain('Bash')
  const inspect = await $.command.run({ command: 'cockpit-kontext', args: 'text' } as never)
  expect(JSON.stringify(inspect)).toContain('Kontext')
  const help = await $.command.run({ command: 'cockpit-blast', args: 'help' } as never)
  expect(JSON.stringify(help)).toContain('cockpit-blast')
  const bad = await $.command.run({ command: 'cockpit-blast', args: 'watch ../../etc/passwd' } as never)
  expect(JSON.stringify(bad)).toContain('Ungültiger Pfad')
})
