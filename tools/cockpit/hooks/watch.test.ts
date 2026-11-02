import { expect, mock, test } from 'claude-code/testing'

import { MAX_ERRORS, bandText as errBand, errorText, newErrors, noteError, recentCount, reportText as errReport } from './errors'
import { LIMITS, newGuard, noteCall, notePrompt, noteSpawn, shouldHold } from './guard'
import { T, tipsOf } from './tips'
import { EMPTY_TREE, MAX_RESTORE, MAX_STEPS, addStep, newTimeline, noFilterArgs, parseNameStatus, restorePlan, shownSteps, snapOf } from './timeline'

// Fehler-Eingang, Nutzungswächter, Cache-Tipps und Zeitreise: die Logik ohne `$`, dazu die Verdrahtung in register.tsx.

const H = Date.UTC(2026, 9, 28, 10)
const MIN = 60_000

test('Fehler-Eingang: erste Zeile, Steuerzeichen raus, gleicher Fehler zählt hoch statt eine Zeile zu füllen', () => {
  expect(errorText({ text: 'Exit code 1\nfatal: not a git repository\nmehr' })).toBe('fatal: not a git repository')
  expect(errorText({ deny: 'nicht erlaubt' })).toBe('nicht erlaubt')
  expect(errorText({})).toBe('')
  const st = newErrors()
  const e = { at: H, tool: 'Bash', target: 'npm test', kind: 'error' as const, text: 'boom', agent: false }
  noteError(st, e)
  noteError(st, { ...e, at: H + 1_000 })
  noteError(st, { ...e, tool: 'Read', target: 'a.ts' })
  expect(st.list.map(x => [x.tool, x.times])).toEqual([['Read', 1], ['Bash', 2]])
  expect(recentCount(st, H + 2_000)).toBe(3)
  expect(recentCount(st, H + 11 * MIN)).toBe(0)
})

test('Fehler-Eingang: höchstens 30 Einträge, Band erst bei jungen Fehlern, Bericht nennt Ziel und Text', () => {
  const st = newErrors()
  for (let i = 0; i < 40; i++) noteError(st, { at: H, tool: 'Bash', target: `cmd ${i}`, kind: i % 2 === 0 ? 'error' : 'denied', text: `t${i}`, agent: false })
  expect(st.list.length).toBe(MAX_ERRORS)
  expect(errBand(st, H, 'de')).toBe('Fehler: 30 in 10 min')
  expect(errBand(st, H + 20 * MIN, 'de')).toBe('')
  expect(errBand(newErrors(), H, 'de')).toBe('')
  expect(errReport(st, H, 'de')).toContain('cmd 39 – t39')
})

test('Wächter: Subagents gleichzeitig und je Minute, dieselbe Regel höchstens alle 5 Minuten', () => {
  const st = newGuard()
  expect(noteSpawn(st, H, 2)).toEqual([])
  const a = noteSpawn(st, H + 1_000, LIMITS.agents - 1)
  expect(a.map(x => x.rule)).toEqual(['agents'])
  expect(noteSpawn(st, H + 2_000, LIMITS.agents)).toEqual([])
  expect(noteSpawn(st, H + 6 * MIN, LIMITS.agents).map(x => x.rule)).toEqual(['agents'])
  const burst = newGuard()
  let seen: string[] = []
  for (let i = 0; i < LIMITS.burst; i++) seen = noteSpawn(burst, H + i * 1_000, 0).map(x => x.rule)
  expect(seen).toEqual(['burst'])
})

test('Wächter: halten nur mit Einstellung und ab der harten Grenze; schwerer Prompt; gleicher Aufruf in Folge', () => {
  const st = newGuard()
  expect(shouldHold(st, LIMITS.hold, false)).toBe(false)
  expect(shouldHold(st, LIMITS.hold - 1, true)).toBe(false)
  expect(shouldHold(st, LIMITS.hold, true)).toBe(true)
  expect(st.held).toBe(1)
  expect(notePrompt(st, H, 70)).toBe(null)
  expect(notePrompt(st, H, undefined)).toBe(null)
  expect(notePrompt(st, H, 90)?.rule).toBe('heavy')
  const calls = newGuard()
  const args = { command: 'npm test', cwd: '.' }
  expect(noteCall(calls, H, 'Bash', args, 'npm test')).toBe(null)
  expect(noteCall(calls, H, 'Bash', { cwd: '.', command: 'npm test' }, 'npm test')).toBe(null)
  expect(noteCall(calls, H, 'Bash', args, 'npm test')?.rule).toBe('retry')
  const other = newGuard()
  noteCall(other, H, 'Bash', args, '')
  noteCall(other, H, 'Bash', { command: 'ls' }, '')
  expect(noteCall(other, H, 'Bash', args, '')).toBe(null)
})

test('Tipps: Cache neu geschrieben, kalt, bald abgelaufen, Kontext und Limits mit Schwellen', () => {
  const base = { now: H, isSubscription: true, lastAt: H - 5 * MIN, limits: [] }
  expect(tipsOf({ ...base, contextTokens: 10_000, contextPercent: 5 })).toEqual([])
  expect(tipsOf({ ...base, lastWrite: T.writeTokens }).map(x => x.id)).toEqual(['cacheWrite'])
  expect(tipsOf({ ...base, lastAt: H - 2 * 3600_000, contextTokens: 50_000, contextPercent: 25 }).map(x => x.id)).toEqual(['cacheCold'])
  expect(tipsOf({ ...base, lastAt: H - 55 * MIN, contextTokens: 90_000, contextPercent: 45 }).map(x => x.id)).toEqual(['compactSoon'])
  // Ohne Subscription (5 min Cache) gibt es kein „kompaktieren vor Ablauf“
  expect(tipsOf({ ...base, isSubscription: false, lastAt: H - 4 * MIN, contextTokens: 90_000, contextPercent: 45 }).map(x => x.id)).toEqual([])
  expect(tipsOf({ ...base, contextTokens: 130_000, contextPercent: 65 }).map(x => x.id)).toEqual(['ctxHalf'])
  expect(tipsOf({ ...base, contextTokens: 170_000, contextPercent: 88 })[0]?.id).toBe('ctxFull')
  const limit = (percentUsed: number) => [{ kind: 'five_hour', percentUsed, resetsAt: new Date(H + 3 * 3600_000).toISOString() }]
  expect(tipsOf({ ...base, limits: limit(96) })[0]?.id).toBe('limitHard')
  expect(tipsOf({ ...base, limits: limit(82) })[0]?.id).toBe('limitWarn')
  // 60 % verbraucht, aber erst 40 % des Fensters um: 20 Punkte zu schnell
  expect(tipsOf({ ...base, limits: [{ kind: 'five_hour', percentUsed: 60, resetsAt: new Date(H + 3 * 3600_000).toISOString() }] })[0]?.id).toBe('limitPace')
})

test('Zeitreise: Schritte gekappt, Start als 0, Name-Status lesen, Zurück lässt neue Dateien liegen', () => {
  const st = newTimeline()
  for (let i = 0; i < MAX_STEPS + 5; i++) addStep(st, { at: H + i, tool: i % 2 === 0 ? 'Edit' : 'Read', target: `f${i}`, ok: i !== 3 })
  expect(st.steps.length).toBe(MAX_STEPS)
  expect(st.steps[0]?.n).toBe(6)
  expect(shownSteps(st)[0]?.n).toBe(MAX_STEPS + 5)
  expect(st.steps.find(x => x.n === MAX_STEPS)?.kind).toBe('read')
  st.base = 'b'.repeat(40)
  st.steps[0]!.snap = 'a'.repeat(40)
  expect(snapOf(st, 0)).toBe('b'.repeat(40))
  expect(snapOf(st, 6)).toBe('a'.repeat(40))
  expect(snapOf(st, 7)).toBe('')
  const nul = String.fromCharCode(0)
  const changes = parseNameStatus(`M${nul}src/a.ts${nul}A${nul}neu.ts${nul}D${nul}weg.ts${nul}`)
  expect(changes).toEqual([{ status: 'M', path: 'src/a.ts' }, { status: 'A', path: 'neu.ts' }, { status: 'D', path: 'weg.ts' }])
  expect(restorePlan(changes)).toEqual({ restore: ['src/a.ts', 'weg.ts'], keep: ['neu.ts'], tooMany: false })
  const many = Array.from({ length: MAX_RESTORE + 1 }, (_, i) => ({ status: 'M' as const, path: `f${i}` }))
  expect(restorePlan(many).tooMany).toBe(true)
  expect(parseNameStatus('')).toEqual([])
})

const PANE_PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 120, placement: 'dock', scroll: { offset: 0, bodyRows: 60 }, view: {} } as const
const BAND_PROPS = { hasSurvey: false, isWorking: false, maxRows: 10, bodyColumns: 120, scroll: { offset: 0, bodyRows: 10 }, view: {} } as const
const MEASURE = { context: { tokens: 50_000, window: 200_000, percent: 25 }, rateLimits: [], cost: { usd: 1.5 }, changed: [] }

const stubs = (on: Parameters<typeof mock.clock>[0]) => {
  mock.clock(on, { now: H })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', () => ({ value: undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => ({ value: { isPlaced: true as const } }))
  on('ui.render', { component: 'AbovePrompt' }, () => ({ type: 'Box' as const }))
}

test('Fehler-Eingang: Bericht nennt den Fehler; das Band meldet erst ab Gelb (3 in 10 min)', async ($, on) => {
  stubs(on)
  on('tool.call', () => ({ isError: true, text: 'Exit code 2\nsrc/x.ts: boom', result: { stdout: '', stderr: 'boom', interrupted: false } }) as never)
  await $.session.measure(MEASURE)
  await $.tool.call({ tool: 'Bash', command: 'npm run build', tool_use_id: 'e1' } as never)
  const report = JSON.stringify(await $.command.run({ command: 'cockpit-fehler', args: 'text' } as never))
  expect(report).toContain('src/x.ts: boom')
  expect(report).toContain('Bash')
  const band = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'AbovePrompt', props: BAND_PROPS })
  // Ein Fehler ist grün: keine Meldungszeile.
  expect(await band.find({ key: 'alerts' })).toBe(undefined)
  await $.tool.call({ tool: 'Read', file_path: 'a.ts', tool_use_id: 'e2' } as never)
  await $.tool.call({ tool: 'Grep', pattern: 'x', tool_use_id: 'e3' } as never)
  for (let i = 0; i < 3000 && !(await band.find({ key: 'alerts' })); i++) await Promise.resolve()
  expect(JSON.stringify(await band.find({ key: 'alerts' }))).toContain('Fehler: 3 in 10 min')
  expect(await band.find({ type: 'Button', key: 'alerts-more' })).not.toBe(undefined)
})

test('Panel: Tipps und Wächter unter Hinweise; Fehler-Eingang und Zeitreise im Protokoll', async ($, on) => {
  stubs(on)
  await $.command.run({ command: 'cockpit-zeitreise', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PANE_PROPS })
  // /cockpit-zeitreise öffnet Diagnose → Protokoll mit Filter Snapshots.
  expect(await pane.find({ type: 'Text', text: 'Noch kein Schritt aufgezeichnet.' })).not.toBe(undefined)
  for (const key of ['log-filter-all', 'log-filter-errors', 'log-filter-gate']) expect(await pane.find({ type: 'Button', key })).not.toBe(undefined)
  await pane.press({ key: 'log-filter-errors' })
  for (let i = 0; i < 3000 && !(await pane.find({ type: 'Text', text: 'Bisher kein Fehler in dieser Session.' })); i++) await Promise.resolve()
  expect(await pane.find({ type: 'Text', text: 'Bisher kein Fehler in dieser Session.' })).not.toBe(undefined)
  await pane.press({ key: 'diag-sub-hints' })
  for (let i = 0; i < 3000 && !(await pane.find({ key: 'card-hints' })); i++) await Promise.resolve()
  for (const title of ['Meldungen', 'Tipps', 'Nutzungswächter']) expect(await pane.find({ type: 'Text', text: title })).not.toBe(undefined)
  const report = JSON.stringify(await $.command.run({ command: 'cockpit-zeitreise', args: 'text' } as never))
  expect(report).toContain('Zeitreise: 0 Schritte')
})

test('Zeitreise: add und restore laufen ohne Filter, verdächtige Filternamen sperren', () => {
  const args = noFilterArgs(['filter.lfs.clean', 'filter.lfs.smudge', 'filter.lfs.process', 'filter.lfs.required', 'Filter.Evil.Clean', 'user.name'])
  expect(args).toEqual([
    '-c', 'filter.lfs.clean=', '-c', 'filter.lfs.smudge=', '-c', 'filter.lfs.process=', '-c', 'filter.lfs.required=false',
    '-c', 'filter.Evil.clean=', '-c', 'filter.Evil.smudge=', '-c', 'filter.Evil.process=', '-c', 'filter.Evil.required=false',
    `--attr-source=${EMPTY_TREE}`,
  ])
  // Ohne Filter bleibt der leere Baum als Attributquelle (schaltet .gitattributes im Arbeitsbaum ab).
  expect(noFilterArgs([])).toEqual([`--attr-source=${EMPTY_TREE}`])
  // Namen, die sich nicht sicher als -c übergeben lassen, sperren den Snapshot.
  expect(noFilterArgs(['filter.a=b.clean'])).toBe(null)
  expect(noFilterArgs(['filter.a b.clean'])).toBe(null)
})
