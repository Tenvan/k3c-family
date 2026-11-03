import { expect, mock, test } from 'claude-code/testing'
import type { TestBody } from 'claude-code/testing'

import { ciT } from './ci-ui'

// Reiter CI/CD mit GitHub-Remote gegen Mock-Daten (kein Netzwerk, kein gh, keine schreibenden Aufrufe gegen GitHub): Workflow-Runs und
// Jobs lesen, Job-Log, Steuerung (re-run, cancel) mit den erwarteten `gh api`-Aufrufen, fehlende GitHub-Entsprechungen ausgeblendet,
// Polling mit Hinweis. GitLab-Gegenstück: ci-panel.test.ts; Aufbereitung der Antworten: ci-github.test.ts.

const PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 160, placement: 'dock', scroll: { offset: 0, bodyRows: 60 }, view: {} } as const
type Engine = Parameters<TestBody>[0]
type On = Parameters<TestBody>[1]
type View = { find(q: { type: 'Text'; text: string | RegExp } | { type: 'Button'; key: string }): Promise<unknown>; press(t: { key: string }): Promise<unknown> }

const REPO = 'Tenvan/k3c-family'
const ESC = String.fromCharCode(27)

const run = (id: number, status: string, conclusion: string | null, branch: string, extra: Record<string, unknown> = {}) => ({
  id, name: 'CI', display_title: 'Export vereinfachen', status, conclusion, head_branch: branch, head_sha: `${id}abcdef0123`,
  html_url: `https://github.com/${REPO}/actions/runs/${id}`, event: 'push', created_at: '2026-11-02T10:00:00Z',
  run_started_at: '2026-11-02T10:00:00Z', updated_at: '2026-11-02T10:02:05Z', actor: { login: 'ralf' }, ...extra,
})
const job = (id: number, name: string, status: string, conclusion: string | null, steps: unknown[] = []) => ({
  id, name, status, conclusion, html_url: `https://github.com/${REPO}/actions/runs/1/job/${id}`, workflow_name: 'CI',
  started_at: '2026-11-02T10:00:00Z', completed_at: '2026-11-02T10:01:05Z', steps,
})
const step = (name: string, status: string, conclusion: string | null) => ({ name, status, conclusion, started_at: '2026-11-02T10:00:00Z', completed_at: '2026-11-02T10:00:07Z' })

type World = { runs: unknown[]; jobs: Record<number, unknown[]>; logs: Record<number, string>; artifacts: Record<number, unknown[]>; calls: string[][]; toasts: string[]; opens: number; fail: boolean; failRerunFailed: boolean }

function world(): World {
  return {
    runs: [run(302, 'in_progress', null, 'feature/x'), run(301, 'completed', 'failure', 'main'), run(300, 'completed', 'success', 'feature/x')],
    jobs: {
      302: [job(1, 'build', 'completed', 'success', [step('Cache laden', 'completed', 'success')]), job(2, 'test', 'in_progress', null, [step('Checkout', 'completed', 'success'), step('Run tests', 'in_progress', null)]), job(3, 'deploy', 'queued', null)],
      301: [job(9, 'test', 'completed', 'failure', [step('Checkout', 'completed', 'success'), step('Run tests', 'completed', 'failure')])],
      300: [job(7, 'build', 'completed', 'success', [step('Checkout', 'completed', 'success')])],
    },
    logs: { 9: `﻿2026-11-02T10:00:00.1Z ##[group]Run npm test\n2026-11-02T10:00:01Z ${ESC}[31;1mTest failed: expected 3 got 4${ESC}[0m\n2026-11-02T10:00:02Z ##[endgroup]\n2026-11-02T10:00:03Z ##[error]Process completed with exit code 1.\n` },
    artifacts: { 301: [{ id: 51, name: 'test-report', size_in_bytes: 2_621_440, expired: false }, { id: 52, name: 'old-dist', size_in_bytes: 10, expired: true }] },
    calls: [],
    toasts: [],
    opens: 0,
    fail: false,
    failRerunFailed: false,
  }
}

const out = (stdout: string, exitCode = 0, stderr = '') => ({ value: { exitCode, stdout, stderr, isStdoutTruncated: false, isStderrTruncated: false } }) as never

function wire(on: On, w: World, saved?: Record<string, unknown>) {
  const clock = mock.clock(on, { now: 1 })
  on('session.measure', () => ({ changed: [] }))
  on('store.get', (_$, e) => ({ value: (e as { key: string }).key === 'settings' ? saved : undefined }) as never)
  on('store.set', () => ({ value: undefined }) as never)
  on('ui.open', () => {
    w.opens++
    return { value: { isPlaced: true as const } }
  })
  on('ui.toast', (_$, e) => {
    w.toasts.push((e as { text: string }).text)
    return { value: undefined } as never
  })
  on('process.run', (_$, e) => {
    const argv = (e as { argv: readonly string[] }).argv
    if (argv[0] === 'git') {
      if (argv.includes('get-url')) return out(`git@github.com:${REPO}.git\n`)
      if (argv.includes('--show-current')) return out('feature/x\n')
      return out('')
    }
    if (argv[0] === 'glab') return out('', 1, 'glab darf bei GitHub nie aufgerufen werden')
    if (argv[0] !== 'gh') return out('')
    w.calls.push([...argv])
    if (argv[1] !== 'api') return out('[]')
    if (w.fail) return out('', 1, 'gh: HTTP 401 Bad credentials')
    const path = (argv[argv.length - 1] ?? '').replace(`repos/${REPO}/actions/`, '')
    if (argv.includes('POST')) return path.endsWith('/rerun-failed-jobs') && w.failRerunFailed ? out('', 1, 'gh: HTTP 403 nothing to re-run') : out('{}')
    if (/^runs\?/.test(path)) return out(JSON.stringify({ total_count: w.runs.length, workflow_runs: w.runs }))
    const jobs = /^runs\/(\d+)\/jobs/.exec(path)
    if (jobs) return out(JSON.stringify({ total_count: 0, jobs: w.jobs[Number(jobs[1])] ?? [] }))
    const detail = /^runs\/(\d+)$/.exec(path)
    if (detail) return out(JSON.stringify((w.runs as { id: number }[]).find(p => p.id === Number(detail[1])) ?? {}))
    const arts = /^runs\/(\d+)\/artifacts/.exec(path)
    if (arts) return out(JSON.stringify({ total_count: 0, artifacts: w.artifacts[Number(arts[1])] ?? [] }))
    const log = /^jobs\/(\d+)\/logs$/.exec(path)
    if (log) return out(w.logs[Number(log[1])] ?? '')
    return out('{}')
  })
  on('session.model', () => ({ value: 'claude-opus-5-5' }) as never)
  on('session.version', () => ({ value: { version: '2.1.295' } }) as never)
  on('session.start', ($, e) => ({ cwd: e.cwd }))
  on('command.register', ($, e) => ({ value: { command: e.name } }))
  on('fs.read', () => ({ value: '{ "version": "1" }' }) as never)
  on('http.fetch', () => ({ value: { status: 404, headers: {}, text: '' } }) as never)
  return clock
}

async function waitFor(done: () => Promise<boolean>) {
  for (let i = 0; i < 3000 && !(await done()); i++) await Promise.resolve()
}
const waitText = async (v: View, text: string | RegExp) => {
  await waitFor(async () => (await v.find({ type: 'Text', text })) !== undefined)
  return v.find({ type: 'Text', text })
}
const waitButton = async (v: View, key: string) => {
  await waitFor(async () => (await v.find({ type: 'Button', key })) !== undefined)
  return v.find({ type: 'Button', key })
}
// Schreibende Aufrufe als Pfad unter `repos/<owner>/<name>/actions/`.
const posts = (w: World) => w.calls.filter(c => c.includes('POST')).map(c => (c[c.length - 1] ?? '').replace(`repos/${REPO}/actions/`, ''))

async function openCi($: Engine, surface: 'desktop' | 'terminal' = 'desktop') {
  await $.command.run({ command: 'cockpit-ci', args: '' } as never)
  return $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc', props: PROPS })
}

test('CI/CD GitHub: Workflow-Runs, Jobs unter dem Workflow, Schritte laufender und fehlgeschlagener Jobs, nur lesende Aufrufe', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  expect(await waitText(pane, /Run #302/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'Workflow-Runs' })).not.toBe(undefined)
  for (const word of ['in_progress|running', 'failed', 'success', 'build', 'test', 'deploy', 'Export vereinfachen']) expect(await waitText(pane, new RegExp(word))).not.toBe(undefined)
  // Workflow-Name als Gruppe, Dauer 125 s aus den Zeitstempeln, Auslöser.
  expect(await waitText(pane, 'CI')).not.toBe(undefined)
  expect(await waitText(pane, /Dauer 2:05 · @ralf/)).not.toBe(undefined)
  // Schritte des laufenden Jobs; die des grünen Jobs bleiben zugeklappt.
  expect(await waitText(pane, '◌ Run tests')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: '✓ Checkout' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: '✓ Cache laden' })).toBe(undefined)
  // Der Link zum Run ist ein Markdown-Element; geprüft wird sein Text je Plattform.
  expect([ciT('de', 'ci.open', 'github'), ciT('de', 'ci.open', 'gitlab'), ciT('en', 'ci.head', 'github')]).toEqual(['↗ In GitHub', '↗ In GitLab', 'Workflow runs'])
  expect(posts(w)).toEqual([])
  expect(w.calls.some(c => c.includes('api') && c.includes('--hostname') && c.includes('github.com'))).toBe(true)
  expect(w.calls.some(c => c[0] === 'glab')).toBe(false)
})

test('CI/CD GitHub: Run wählen, fehlgeschlagener Schritt rot markiert, Log ohne Zeitstempel, Befehle und Escape-Folgen', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Run #302/)
  await pane.press({ key: 'ci-pick-301' })
  expect(await waitText(pane, /Run #301/)).not.toBe(undefined)
  expect(await waitText(pane, '✗ Run tests')).not.toBe(undefined)
  await waitButton(pane, 'ci-j-log-9')
  await pane.press({ key: 'ci-j-log-9' })
  expect(await waitText(pane, 'Test failed: expected 3 got 4')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: '▸ Run npm test' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: '✗ Process completed with exit code 1.' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /\u001b|\d{4}-\d{2}-\d{2}T/ })).toBe(undefined)
  expect(w.calls.some(c => (c[c.length - 1] ?? '').endsWith('/actions/jobs/9/logs'))).toBe(true)
  await pane.press({ key: 'ci-log-close' })
  await waitFor(async () => (await pane.find({ type: 'Text', text: 'Test failed: expected 3 got 4' })) === undefined)
  expect(await pane.find({ type: 'Text', text: 'Test failed: expected 3 got 4' })).toBe(undefined)
})

test('CI/CD GitHub: Steuerung re-run und cancel; play, Job-Abbruch und neuer Run fehlen, cancel fragt erst nach', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitButton(pane, 'ci-p-cancel')
  // Kein Äquivalent auf GitHub: nicht angezeigt statt simuliert.
  for (const key of ['ci-j-play-3', 'ci-j-cancel-2', 'ci-j-cancel-3', 'ci-new']) expect(await pane.find({ type: 'Button', key })).toBe(undefined)
  // Run abbrechen: erster Klick fragt, zweiter führt aus.
  await pane.press({ key: 'ci-p-cancel' })
  await waitFor(async () => JSON.stringify(await pane.find({ type: 'Button', key: 'ci-p-cancel' })).includes('Wirklich abbrechen?'))
  expect(posts(w)).toEqual([])
  await pane.press({ key: 'ci-p-cancel' })
  await waitFor(async () => posts(w).length >= 1)
  expect(posts(w)).toEqual(['runs/302/cancel'])
  // Fehlgeschlagener Run: einzelnen Job und die fehlgeschlagenen Jobs neu starten.
  await pane.press({ key: 'ci-pick-301' })
  await waitButton(pane, 'ci-j-retry-9')
  await pane.press({ key: 'ci-j-retry-9' })
  await waitFor(async () => posts(w).length >= 2)
  expect(posts(w)[1]).toBe('jobs/9/rerun')
  await waitButton(pane, 'ci-p-retry')
  await pane.press({ key: 'ci-p-retry' })
  await waitFor(async () => posts(w).length >= 3)
  expect(posts(w)[2]).toBe('runs/301/rerun-failed-jobs')
  expect(w.toasts.some(t => /Job test wiederholen: erledigt/.test(t))).toBe(true)
  expect(w.toasts.some(t => /Run #301: fehlgeschlagene Jobs neu starten: erledigt/.test(t))).toBe(true)
})

test('CI/CD GitHub: lehnt GitHub rerun-failed-jobs ab, startet der Reiter alle Jobs neu', async ($, on) => {
  const w = world()
  w.failRerunFailed = true
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Run #302/)
  await pane.press({ key: 'ci-pick-301' })
  await waitButton(pane, 'ci-p-retry')
  await pane.press({ key: 'ci-p-retry' })
  await waitFor(async () => posts(w).length >= 2)
  expect(posts(w)).toEqual(['runs/301/rerun-failed-jobs', 'runs/301/rerun'])
})

test('CI/CD GitHub: Takt meldet den Statuswechsel des Branch-Runs und öffnet den Reiter bei Fehlschlag', async ($, on) => {
  const w = world()
  const clock = wire(on, w)
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  await clock.advance(60_000)
  expect(w.toasts.filter(t => /Run #/.test(t))).toEqual([])
  const opens = w.opens
  w.runs = [run(302, 'completed', 'failure', 'feature/x'), run(301, 'completed', 'failure', 'main'), run(300, 'completed', 'success', 'feature/x')]
  w.jobs[302] = [job(1, 'build', 'completed', 'success'), job(2, 'test', 'completed', 'failure'), job(3, 'deploy', 'completed', 'skipped')]
  await clock.advance(15_000)
  await waitFor(async () => w.toasts.some(t => /Run #302 \(feature\/x\): fehlgeschlagen/.test(t)))
  expect(w.toasts.some(t => /Run #302 \(feature\/x\): fehlgeschlagen/.test(t))).toBe(true)
  expect(w.toasts.some(t => /Job test \(Run #302\) fehlgeschlagen/.test(t))).toBe(true)
  expect(w.opens).toBeGreaterThan(opens)
  expect(w.toasts.some(t => /#301/.test(t))).toBe(false)
  expect(posts(w)).toEqual([])
})

test('CI/CD GitHub: Fehler beim Laden steht im Reiter, die letzten Runs bleiben; Terminal ohne Svg', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($, 'terminal')
  await waitText(pane, /Run #302/)
  expect(await pane.find({ type: 'Svg' })).toBe(undefined)
  w.fail = true
  await pane.press({ key: 'ci-reload' })
  expect(await waitText(pane, /Workflow-Runs nicht ladbar: gh: HTTP 401 Bad credentials/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Der letzte Stand bleibt stehen/ })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Run #302/ })).not.toBe(undefined)
})

test('CI/CD GitHub: Kopfzeile nennt Backend und Repo, Run zeigt Zeitraum, Auslöser und Ersteller', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Run #302/)
  expect(await waitText(pane, 'github.com/Tenvan/k3c-family')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'GitHub' })).not.toBe(undefined)
  // Zeitraum im lokalen Format (Start läuft, kein Ende), Auslöser „Push“, Ersteller @ralf.
  expect(await waitText(pane, /02\.11\.2026 \d\d:\d\d · Auslöser: Push · von @ralf/)).not.toBe(undefined)
  // Job-Zeile: Start – Ende des abgeschlossenen Jobs.
  expect(await waitText(pane, /02\.11\.2026 \d\d:\d\d – \d\d:\d\d/)).not.toBe(undefined)
})

test('CI/CD GitHub: Artefakte des Runs mit Größe, abgelaufene markiert; kein Download-Aufruf, nur lesende gh-Aufrufe', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Run #302/)
  expect(await pane.find({ type: 'Text', text: /Artefakte/ })).toBe(undefined)
  await pane.press({ key: 'ci-pick-301' })
  expect(await waitText(pane, /Artefakte \(2\)/)).not.toBe(undefined)
  expect(await waitText(pane, /test-report/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /2,5 MB/ })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: 'abgelaufen' })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Das Cockpit lädt keine Dateien|das Cockpit lädt keine Dateien/ })).not.toBe(undefined)
  // Nie `gh run download`, nie ein Token, nie schreibend.
  expect(w.calls.some(c => c.includes('download') || c.includes('token') || c[1] === 'auth')).toBe(false)
  expect(posts(w)).toEqual([])
})
