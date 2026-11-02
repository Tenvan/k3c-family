import { expect, mock, test } from 'claude-code/testing'
import type { TestBody } from 'claude-code/testing'

// Reiter CI/CD im Panel gegen Mock-Daten (kein Netzwerk, kein glab): Pipelines und Jobs lesen, Job-Log, Aktionen (retry, cancel,
// play, neue Pipeline) mit den erwarteten glab-Aufrufen, Rückfrage bei Folgenreichem, Polling mit Hinweis und Auto-Öffnen.
// Aufbereitung der Antworten: ci.test.ts.

const PROPS = { title: 'Cockpit', isFocused: false, bodyColumns: 160, placement: 'dock', scroll: { offset: 0, bodyRows: 60 }, view: {} } as const
type Engine = Parameters<TestBody>[0]
type On = Parameters<TestBody>[1]
type View = { find(q: { type: 'Text'; text: string | RegExp } | { type: 'Button'; key: string }): Promise<unknown>; press(t: { key: string }): Promise<unknown> }

const pipe = (id: number, status: string, ref: string, extra: Record<string, unknown> = {}) => ({
  id, status, ref, sha: `${id}abcdef0123`, web_url: `https://gitlab.deltra.de/consulting/x/-/pipelines/${id}`, source: 'push',
  created_at: '2026-11-02T10:00:00Z', updated_at: '2026-11-02T10:05:00Z', ...extra,
})
const job = (id: number, name: string, stage: string, status: string, extra: Record<string, unknown> = {}) => ({
  id, name, stage, status, web_url: `j${id}`, duration: 65, allow_failure: false, commit: { title: 'Export vereinfachen' }, ...extra,
})

type World = { pipelines: unknown[]; jobs: Record<number, unknown[]>; traces: Record<number, string>; calls: string[][]; toasts: string[]; opens: number; fail: boolean }
const ESC = String.fromCharCode(27)

function world(): World {
  return {
    pipelines: [pipe(202, 'running', 'feature/x'), pipe(201, 'failed', 'main'), pipe(200, 'success', 'feature/x')],
    jobs: {
      202: [job(1, 'build', 'build', 'success'), job(2, 'test', 'test', 'running'), job(3, 'deploy', 'deploy', 'manual')],
      201: [job(9, 'test', 'test', 'failed')],
      200: [job(7, 'build', 'build', 'success')],
    },
    traces: { 9: `${ESC}[31;1mTest failed: expected 3 got 4${ESC}[0m\nJob failed: exit code 1\n` },
    calls: [],
    toasts: [],
    opens: 0,
    fail: false,
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
      if (argv.includes('get-url')) return out('git@gitlab.deltra.de:consulting/x.git\n')
      if (argv.includes('--show-current')) return out('feature/x\n')
      return out('')
    }
    if (argv[0] !== 'glab') return out('')
    w.calls.push([...argv])
    if (w.fail) return out('', 1, 'glab: 401 Unauthorized')
    const path = argv.find(a => a.startsWith('projects/:id/') || a === 'projects/:id' || a.startsWith('merge_requests') || a === 'user') ?? ''
    if (argv.includes('POST')) return out('{}')
    if (/^projects\/:id\/pipelines\?/.test(path)) return out(JSON.stringify(w.pipelines))
    const jobs = /^projects\/:id\/pipelines\/(\d+)\/jobs/.exec(path)
    if (jobs) return out(JSON.stringify(w.jobs[Number(jobs[1])] ?? []))
    const detail = /^projects\/:id\/pipelines\/(\d+)$/.exec(path)
    if (detail) return out(JSON.stringify({ ...(w.pipelines as { id: number }[]).find(p => p.id === Number(detail[1])), duration: 125, user: { username: 'ralf' } }))
    const trace = /^projects\/:id\/jobs\/(\d+)\/trace$/.exec(path)
    if (trace) return out(w.traces[Number(trace[1])] ?? '')
    return out(path === 'projects/:id' ? '{}' : '[]')
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
const posts = (w: World) => w.calls.filter(c => c.includes('POST')).map(c => c.filter(a => a !== 'glab' && a !== 'api' && a !== '-X' && a !== 'POST').join(' '))

async function openCi($: Engine, surface: 'desktop' | 'terminal' = 'desktop') {
  await $.command.run({ command: 'cockpit-ci', args: '' } as never)
  return $.ui.mount({ plugin: 'tenvan-cc', surface, component: 'Pane', requestId: 'tenvan-cc', props: PROPS })
}

test('CI/CD: Pipelines, Jobs je Stage und Kopf der Branch-Pipeline aus glab-Antworten', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  expect(await waitText(pane, /Pipeline #202/)).not.toBe(undefined)
  for (const word of ['running', 'failed', 'success', 'build', 'test', 'deploy', 'manual', 'Export vereinfachen']) expect(await waitText(pane, new RegExp(word))).not.toBe(undefined)
  // Dauer 125 s aus dem Detail, Auslöser dazu.
  expect(await waitText(pane, /Dauer 2:05 · @ralf/)).not.toBe(undefined)
  // Lesend: bisher kein schreibender Aufruf.
  expect(posts(w)).toEqual([])
})

test('CI/CD: Pipeline wählen lädt deren Jobs, Log zeigt aufbereitete letzte Zeilen und schließt per Klick', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Pipeline #202/)
  await pane.press({ key: 'ci-pick-201' })
  expect(await waitText(pane, /Pipeline #201/)).not.toBe(undefined)
  await waitButton(pane, 'ci-j-log-9')
  await pane.press({ key: 'ci-j-log-9' })
  expect(await waitText(pane, 'Test failed: expected 3 got 4')).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /\u001b/ })).toBe(undefined)
  await pane.press({ key: 'ci-log-close' })
  await waitFor(async () => (await pane.find({ type: 'Text', text: 'Test failed: expected 3 got 4' })) === undefined)
  expect(await pane.find({ type: 'Text', text: 'Test failed: expected 3 got 4' })).toBe(undefined)
})

test('CI/CD: Steuerung ruft glab mit den richtigen Endpunkten auf; Abbrechen und neue Pipeline fragen erst nach', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitButton(pane, 'ci-j-play-3')
  // play (manueller Job) und retry (fehlgeschlagene Pipeline) laufen direkt.
  await pane.press({ key: 'ci-j-play-3' })
  await waitFor(async () => posts(w).length >= 1)
  expect(posts(w)).toEqual(['projects/:id/jobs/3/play'])
  await waitFor(async () => (await pane.find({ type: 'Button', key: 'ci-p-cancel' })) !== undefined)
  // abbrechen: erster Klick fragt, zweiter führt aus.
  await pane.press({ key: 'ci-p-cancel' })
  await waitFor(async () => (await pane.find({ type: 'Button', key: 'ci-p-cancel' })) !== undefined)
  expect(JSON.stringify(await pane.find({ type: 'Button', key: 'ci-p-cancel' }))).toContain('Wirklich abbrechen?')
  expect(posts(w)).toEqual(['projects/:id/jobs/3/play'])
  await pane.press({ key: 'ci-p-cancel' })
  await waitFor(async () => posts(w).length >= 2)
  expect(posts(w)[1]).toBe('projects/:id/pipelines/202/cancel')
  // neue Pipeline für den Branch: ebenfalls mit Rückfrage, Ref als Feld.
  await pane.press({ key: 'ci-new' })
  await waitFor(async () => JSON.stringify(await pane.find({ type: 'Button', key: 'ci-new' })).includes('Pipeline für feature/x starten?'))
  expect(JSON.stringify(await pane.find({ type: 'Button', key: 'ci-new' }))).toContain('Pipeline für feature/x starten?')
  expect(posts(w).length).toBe(2)
  await pane.press({ key: 'ci-new' })
  await waitFor(async () => posts(w).length >= 3)
  expect(posts(w)[2]).toBe('projects/:id/pipeline -f ref=feature/x')
  // Job wiederholen und Pipeline neu starten an der fehlgeschlagenen Pipeline.
  await pane.press({ key: 'ci-pick-201' })
  await waitButton(pane, 'ci-j-retry-9')
  await pane.press({ key: 'ci-j-retry-9' })
  await waitFor(async () => posts(w).length >= 4)
  expect(posts(w)[3]).toBe('projects/:id/jobs/9/retry')
  await waitButton(pane, 'ci-p-retry')
  await pane.press({ key: 'ci-p-retry' })
  await waitFor(async () => posts(w).length >= 5)
  expect(posts(w)[4]).toBe('projects/:id/pipelines/201/retry')
  expect(w.toasts.some(t => /Job test wiederholen: erledigt/.test(t))).toBe(true)
})

test('CI/CD: Takt meldet den Statuswechsel der Branch-Pipeline und öffnet den Reiter bei Fehlschlag', async ($, on) => {
  const w = world()
  const clock = wire(on, w)
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  await clock.advance(60_000)
  // Der erste Abruf meldet nichts (kein Vorher).
  expect(w.toasts.filter(t => /Pipeline #/.test(t))).toEqual([])
  const opens = w.opens
  w.pipelines = [pipe(202, 'failed', 'feature/x'), pipe(201, 'failed', 'main'), pipe(200, 'success', 'feature/x')]
  w.jobs[202] = [job(1, 'build', 'build', 'success'), job(2, 'test', 'test', 'failed'), job(3, 'deploy', 'deploy', 'manual')]
  await clock.advance(15_000)
  await waitFor(async () => w.toasts.some(t => /Pipeline #202 \(feature\/x\): fehlgeschlagen/.test(t)))
  expect(w.toasts.some(t => /Pipeline #202 \(feature\/x\): fehlgeschlagen/.test(t))).toBe(true)
  expect(w.toasts.some(t => /Job test \(Pipeline #202\) fehlgeschlagen/.test(t))).toBe(true)
  expect(w.opens).toBeGreaterThan(opens)
  // Fremde Branches lösen nichts aus (Pipeline 201 auf main ist unverändert).
  expect(w.toasts.some(t => /#201/.test(t))).toBe(false)
})

test('CI/CD: abgeschaltete Hinweise und kein Auto-Öffnen, Terminal ohne Svg', async ($, on) => {
  const w = world()
  const clock = wire(on, w, { ciNotify: false, ciAutoOpen: false })
  await $.session.start({ cwd: '.', surface: 'desktop', isInteractive: true })
  await clock.advance(60_000)
  const polls = () => w.calls.filter(c => c.some(a => a.startsWith('projects/:id/pipelines'))).length
  const calls = polls()
  w.pipelines = [pipe(202, 'failed', 'feature/x')]
  await clock.advance(60_000)
  // Ohne Hinweise und ohne Reiter pollt das Cockpit gar nicht erst.
  expect(polls()).toBe(calls)
  expect(w.toasts.filter(t => /Pipeline #/.test(t))).toEqual([])
  const pane = await openCi($, 'terminal')
  expect(await waitText(pane, /Pipeline #202/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Svg' })).toBe(undefined)
})

test('CI/CD: Fehler beim Laden steht im Reiter, die letzten Pipelines bleiben', async ($, on) => {
  const w = world()
  wire(on, w)
  const pane = await openCi($)
  await waitText(pane, /Pipeline #202/)
  w.fail = true
  await pane.press({ key: 'ci-reload' })
  expect(await waitText(pane, /Pipelines nicht ladbar: glab: 401 Unauthorized/)).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Der letzte Stand bleibt stehen/ })).not.toBe(undefined)
  expect(await pane.find({ type: 'Text', text: /Pipeline #202/ })).not.toBe(undefined)
})

test('CI/CD: Git-Panel bietet den Einstieg in den Reiter', async ($, on) => {
  const w = world()
  wire(on, w)
  // Dev-Mode: der Git-Reiter zeichnet den Mock-MR samt Aktionen, ohne dass ein echtes Repo nötig ist.
  await $.command.run({ command: 'cockpit-dev', args: '' } as never)
  await $.command.run({ command: 'cockpit-git', args: '' } as never)
  const pane = await $.ui.mount({ plugin: 'tenvan-cc', surface: 'desktop', component: 'Pane', requestId: 'tenvan-cc', props: PROPS })
  await waitButton(pane, 'mr-a-ci')
  await pane.press({ key: 'mr-a-ci' })
  await waitFor(async () => (await pane.find({ type: 'Button', key: 'pane-tabs-ci' })) === undefined)
  expect(await pane.find({ type: 'Button', key: 'pane-tabs-ci' })).toBe(undefined)
  expect(await waitText(pane, /Pipeline #202/)).not.toBe(undefined)
})
