import { expect, mock, test } from 'claude-code/testing'

// Diagnose (Kontext-Inspektor, Agent-Profiler, Blast-Radius): Verdrahtung in register.tsx – Meldungen, Reiter mit Unterreitern auf
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

// Warten, bis das Panel nach einem Klick neu gezeichnet hat (Bedingung erfüllt oder 200 Mikroschritte).
const settle = async (done: () => Promise<boolean>) => {
  for (let i = 0; i < 200 && !(await done()); i++) await Promise.resolve()
}

for (const surface of ['desktop', 'terminal'] as const) {
  test(`Diagnose (${surface}): Reiter mit Unterreitern Hinweise, Profil, Blast-Radius; Inspektor unter Verbrauch`, async ($, on) => {
    stubs(on)
    await $.command.run({ command: 'cockpit-profiling', args: '' } as never)
    const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
    // Hinweise (Standard-Unterreiter): Meldungen, Tipps, Wächter und die Ampel-Legende.
    expect(await pane.find({ type: 'Text', text: 'Meldungen' })).not.toBe(undefined)
    expect(await pane.find({ type: 'Text', text: 'Ampel:' })).not.toBe(undefined)
    for (const key of ['diag-sub-log', 'diag-sub-profile', 'diag-sub-blast']) expect(await pane.find({ type: 'Button', key })).not.toBe(undefined)
    if (surface === 'terminal') expect(await pane.find({ type: 'Svg' })).toBe(undefined)
    // Profil als eigener Unterreiter.
    await pane.press({ key: 'diag-sub-profile' })
    await settle(async () => (await pane.find({ type: 'Button', key: 'card-fold-diag-profile' })) !== undefined)
    expect(await pane.find({ type: 'Text', text: /^(Zeitverteilung|Überblick|Ziele)/ })).not.toBe(undefined)
    // Blast-Radius: Neu berechnen im Kartenkopf, zugeklappt verschwindet er.
    await pane.press({ key: 'diag-sub-blast' })
    await settle(async () => (await pane.find({ type: 'Button', key: 'diag-refresh' })) !== undefined)
    expect(await pane.find({ type: 'Button', key: 'diag-refresh' })).not.toBe(undefined)
    await pane.press({ key: 'card-fold-diag-blast' })
    await settle(async () => (await pane.find({ type: 'Button', key: 'diag-refresh' })) === undefined)
    expect(await pane.find({ type: 'Button', key: 'diag-refresh' })).toBe(undefined)
    // Kontext-Inspektor gehört zum Verbrauch.
    await pane.press({ key: 'pane-tabs-usage' })
    await settle(async () => (await pane.find({ type: 'Button', key: 'diag-agents' })) !== undefined)
    expect(await pane.find({ type: 'Text', text: 'Kontext-Inspektor' })).not.toBe(undefined)
  })

  test(`Diagnose (${surface}): Kurzinfos unter Hinweise, im Band nur Gelb und Rot; Band-Schalter wirkt nur aufs Band`, async ($, on) => {
    stubs(on)
    await $.session.measure(MEASURE)
    const band = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'AbovePrompt', props: BAND_PROPS })
    // 25 % Kontext ist grün: keine Meldungszeile, keine Knöpfe.
    expect(await band.find({ key: 'alerts' })).toBe(undefined)
    expect(await band.find({ type: 'Button', key: 'diag-inspect' })).toBe(undefined)
    await $.command.run({ command: 'cockpit-profiling', args: '' } as never)
    const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
    expect(JSON.stringify(await pane.find({ key: 'hint-diag-inspect' }))).toContain('Kontext')
    await pane.press({ key: 'pane-tabs-set' })
    await settle(async () => (await pane.find({ type: 'Button', key: 'set-inspectBand' })) !== undefined)
    await pane.press({ key: 'set-inspectBand' })
    await settle(async () => JSON.stringify(await pane.find({ type: 'Button', key: 'set-inspectBand' })).includes('○'))
    expect(JSON.stringify(await pane.find({ type: 'Button', key: 'set-inspectBand' }))).toContain('○ Aus')
    // Der Schalter „Kontext-Inspektor im Band“ gilt nur fürs Band; Diagnose → Hinweise zeigt jede Meldung.
    await pane.press({ key: 'pane-tabs-diag' })
    await settle(async () => (await pane.find({ key: 'card-hints' })) !== undefined)
    expect(JSON.stringify(await pane.find({ key: 'hint-diag-inspect' }))).toContain('Kontext')
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

test('Profiler: Subagent aus agent.spawn und turn.complete statt classic.* (sec-default leitet die an uns vorbei)', async ($, on) => {
  stubs(on)
  on('agent.spawn', () => ({ model: 'm', agentId: 'a1' }) as never)
  on('agent.list', () => ({ value: [] }) as never)
  on('turn.start', () => ({ turnId: 't1' }))
  on('turn.complete', () => ({ text: '' }))
  on('turn.step', async function* () {
    return { turnId: 't1', index: 0, answer: '', toolUses: [], stopReason: 'end_turn', usage: { ...USAGE0, input_tokens: 4, model: 'm' } }
  })
  on('prompt.submit', ($, e) => ({ text: e.text }))
  await $.session.measure(MEASURE)
  await $.turn.start({ turnId: 't1' } as never)
  for await (const _ of $.turn.step({ turnId: 't1', index: 0, model: 'm', messageCount: 1 })) {
    // Stream leer durchlaufen
  }
  await $.agent.spawn({ subagentType: 'planner', description: 'Plan', tool_use_id: 'u9', background: false, fork: false } as never)
  await $.turn.complete({ answer: '', reason: 'answer', durationMs: 500, isAborted: false, turnId: 's1', agentId: 'a1' } as never)
  await $.turn.complete({ answer: '', reason: 'answer', durationMs: 1000, isAborted: false, turnId: 't1' } as never)
  const report = await $.command.run({ command: 'cockpit-profil', args: 'text' } as never)
  expect(JSON.stringify(report)).toContain('planner')
})
