import { expect, test } from 'claude-code/testing'

import { ago, byStage, clean, ciEvents, ciPollDelayMs, ciTone, clock, commitTitle, jobCan, jobEvents, pickPipeline, pipelineCan, tailLog, toJobs, toPipelines, withDetail } from './ci'

const P = (id: number, status: string, ref = 'feature/x'): ReturnType<typeof toPipelines>[number] => ({ id, status, ref, sha: 'abcdef12', url: `u${id}`, source: 'push', createdAt: '2026-11-02T10:00:00Z', updatedAt: '', durationS: 0, user: '', title: '' })

const RAW_PIPE = { id: 201, status: 'failed', ref: 'main', sha: '0123456789abcdef', web_url: 'https://gitlab.deltra.de/p/-/pipelines/201', source: 'push', created_at: '2026-11-02T10:00:00Z', updated_at: '2026-11-02T10:05:00Z', user: { username: 'ralf' } }
const RAW_JOB = (id: number, name: string, stage: string, status: string, extra: Record<string, unknown> = {}) => ({ id, name, stage, status, web_url: `j${id}`, duration: 12.4, allow_failure: false, started_at: '2026-11-02T10:01:00Z', commit: { title: 'Export vereinfachen' }, ...extra })

test('toPipelines: Felder aus der GitLab-Liste, kurze SHA, ohne Id verworfen, Nicht-Array ergibt leer', () => {
  const [p] = toPipelines([RAW_PIPE, { status: 'x' }])
  expect(p).toMatchObject({ id: 201, status: 'failed', ref: 'main', sha: '01234567', user: 'ralf', url: 'https://gitlab.deltra.de/p/-/pipelines/201' })
  expect(toPipelines([RAW_PIPE, { status: 'x' }]).length).toBe(1)
  expect(toPipelines(null)).toEqual([])
  expect(toPipelines({})).toEqual([])
})

test('withDetail: Dauer und Auslöser aus der Detail-Antwort, Status wird aktualisiert', () => {
  const base = toPipelines([{ ...RAW_PIPE, status: 'running' }])[0]!
  const d = withDetail(base, { ...RAW_PIPE, status: 'failed', duration: 95.6, user: { username: 'anna' } })
  expect(d).toMatchObject({ status: 'failed', durationS: 96, user: 'anna', ref: 'main' })
  expect(withDetail(base, null)).toEqual(base)
})

test('toJobs, commitTitle, byStage: Jobs je Stage in Reihenfolge des ersten Jobs', () => {
  const raw = [RAW_JOB(30, 'deploy', 'deploy', 'manual'), RAW_JOB(20, 'test:b', 'test', 'failed'), RAW_JOB(10, 'build', 'build', 'success'), RAW_JOB(21, 'test:a', 'test', 'success', { allow_failure: true })]
  const jobs = toJobs(raw)
  expect(jobs.find(j => j.id === 30)).toMatchObject({ manual: true, durationS: 12, status: 'manual' })
  expect(jobs.find(j => j.id === 21)?.allowFailure).toBe(true)
  expect(commitTitle(raw)).toBe('Export vereinfachen')
  expect(commitTitle([])).toBe('')
  expect(byStage(jobs).map(g => [g.stage, g.jobs.map(j => j.id)])).toEqual([['build', [10]], ['test', [20, 21]], ['deploy', [30]]])
})

test('jobCan und pipelineCan: nur sinnvolle Aktionen je Status', () => {
  const j = (status: string) => toJobs([RAW_JOB(1, 'x', 's', status)])[0]!
  expect(jobCan(j('failed'))).toEqual({ retry: true, cancel: false, play: false, log: true })
  expect(jobCan(j('running'))).toEqual({ retry: false, cancel: true, play: false, log: true })
  expect(jobCan(j('manual'))).toEqual({ retry: false, cancel: false, play: true, log: false })
  expect(jobCan(j('created')).log).toBe(false)
  expect(pipelineCan(P(1, 'failed'))).toEqual({ retry: true, cancel: false })
  expect(pipelineCan(P(1, 'running'))).toEqual({ retry: false, cancel: true })
  expect(pipelineCan(P(1, 'success'))).toEqual({ retry: false, cancel: false })
})

test('ciTone und pickPipeline: Ton je Status; gewählte, sonst neueste des Branches, sonst neueste', () => {
  expect([ciTone('success'), ciTone('failed'), ciTone('running'), ciTone('manual'), ciTone('canceled')]).toEqual(['success', 'error', 'accent', 'warning', 'neutral'])
  const list = [P(5, 'success', 'main'), P(4, 'failed', 'feature/x'), P(3, 'success', 'feature/x')]
  expect(pickPipeline(list, 3, 'feature/x')?.id).toBe(3)
  expect(pickPipeline(list, 0, 'feature/x')?.id).toBe(4)
  expect(pickPipeline(list, 99, 'andere')?.id).toBe(5)
  expect(pickPipeline([], 0, 'x')).toBe(null)
})

test('ciPollDelayMs: 15 s bei laufender oder wartender Pipeline, sonst eine Minute', () => {
  expect(ciPollDelayMs([P(1, 'success'), P(2, 'running')])).toBe(15_000)
  expect(ciPollDelayMs([P(1, 'pending')])).toBe(15_000)
  expect(ciPollDelayMs([P(1, 'success'), P(2, 'manual')])).toBe(60_000)
  expect(ciPollDelayMs([])).toBe(60_000)
})

test('ciEvents: Statuswechsel nur im Branch, nicht beim ersten Abruf, nicht bei neuen Pipelines', () => {
  const mine = (p: { ref: string }) => p.ref === 'feature/x'
  const prev = [P(2, 'running'), P(1, 'running', 'main')]
  expect(ciEvents(null, [P(2, 'failed')], mine)).toEqual([])
  expect(ciEvents(prev, [P(2, 'failed'), P(1, 'success', 'main')], mine).map(e => [e.event, e.pipeline.id])).toEqual([['pipelineFailed', 2]])
  expect(ciEvents(prev, [P(2, 'success')], mine).map(e => e.event)).toEqual(['pipelineSuccess'])
  expect(ciEvents(prev, [P(2, 'canceled')], mine).map(e => e.event)).toEqual(['pipelineCanceled'])
  expect(ciEvents(prev, [P(2, 'manual')], mine).map(e => e.event)).toEqual(['pipelineManual'])
  expect(ciEvents(prev, [P(2, 'running')], mine)).toEqual([])
  expect(ciEvents([P(1, 'success')], [P(9, 'failed')], mine)).toEqual([])
})

test('jobEvents: nur neu fehlgeschlagene Jobs ohne erlaubten Fehlschlag', () => {
  const before = toJobs([RAW_JOB(1, 'a', 's', 'running'), RAW_JOB(2, 'b', 's', 'failed'), RAW_JOB(3, 'c', 's', 'running', { allow_failure: true })])
  const after = toJobs([RAW_JOB(1, 'a', 's', 'failed'), RAW_JOB(2, 'b', 's', 'failed'), RAW_JOB(3, 'c', 's', 'failed', { allow_failure: true }), RAW_JOB(4, 'd', 's', 'failed')])
  expect(jobEvents(before, after).map(j => j.id)).toEqual([1])
  expect(jobEvents(null, after)).toEqual([])
})

test('ago und clock: Zeitangaben', () => {
  const now = Date.parse('2026-11-02T12:00:00Z')
  expect(ago('2026-11-02T11:59:30Z', now)).toBe('30 s')
  expect(ago('2026-11-02T11:00:00Z', now)).toBe('60 min')
  expect(ago('2026-11-02T08:00:00Z', now)).toBe('4 h')
  expect(ago('2026-10-30T12:00:00Z', now)).toBe('3 d')
  expect(ago('', now)).toBe('–')
  expect([clock(0), clock(65), clock(3723)]).toEqual(['–', '1:05', '1:02:03'])
})

test('tailLog: ANSI und Abschnittsmarker weg, Wagenrücklauf überschreibt, nur die letzten Zeilen', () => {
  const ESC = String.fromCharCode(27)
  const raw = [`section_start:1700000000:step_script\r${ESC}[0K${ESC}[32;1mRunning${ESC}[0;m`, 'Fortschritt 10%\rFortschritt 100%', 'ok', '', ''].join('\n')
  expect(tailLog(raw)).toEqual({ lines: ['Running', 'Fortschritt 100%', 'ok'], truncated: false })
  const many = Array.from({ length: 100 }, (_, i) => `z${i}`).join('\n')
  const t = tailLog(many, 5)
  expect(t.lines).toEqual(['z95', 'z96', 'z97', 'z98', 'z99'])
  expect(t.truncated).toBe(true)
  expect(tailLog('')).toEqual({ lines: [], truncated: false })
})

test('clean: OSC (Titel, Link), DCS, CSI und Steuerzeichen aus fremdem Text entfernt, Umlaute und Tab bleiben', () => {
  const ESC = String.fromCharCode(27)
  const BEL = String.fromCharCode(7)
  const ST = ESC + String.fromCharCode(92)
  expect(clean(`a${ESC}]0;gekapert${BEL}b`)).toBe('ab')
  expect(clean(`a${ESC}]8;;http://x${ST}link${ESC}]8;;${ST}b`)).toBe('alinkb')
  expect(clean(`a${ESC}Pq data${ST}b`)).toBe('ab')
  expect(clean(`${ESC}[2J${ESC}[1;31mrot${ESC}[0m${String.fromCharCode(0)}${String.fromCharCode(8)}`)).toBe('rot')
  expect(clean('Prüfung\tok')).toBe('Prüfung\tok')
  expect(toJobs([{ id: 1, name: `build${ESC}]0;x${BEL}`, stage: 's', status: 'failed' }])[0]?.name).toBe('build')
  expect(tailLog(`zeile${ESC}]0;titel${BEL}1\n`).lines).toEqual(['zeile1'])
})
