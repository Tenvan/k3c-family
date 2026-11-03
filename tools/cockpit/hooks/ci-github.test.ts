import { expect, test } from 'claude-code/testing'

import { byStage, ciCaps, ciEvents, jobCan, jobEvents, pipelineCan, tailLog } from './ci'
import { ghLogText, ghStatus, ghWithDetail, ghWorkflowName, toGhArtifacts, toGhJobs, toGhRuns } from './ci-github'
import { ciSupported } from './forge'
import { forgeRemote } from './mr'

// Übersetzung der GitHub-Actions-Antworten (`gh api`) in die gemeinsame Form, gegen Mock-Daten. Panel und Aufrufe: ci-github-panel.test.ts.

const ESC = String.fromCharCode(27)
const BEL = String.fromCharCode(7)
const RAW_RUN = {
  id: 9001, name: 'CI', display_title: 'Export vereinfachen\nzweite Zeile', status: 'completed', conclusion: 'failure', head_branch: 'feature/x',
  head_sha: '0123456789abcdef', html_url: 'https://github.com/Tenvan/k3c-family/actions/runs/9001', event: 'push',
  created_at: '2026-11-02T10:00:00Z', run_started_at: '2026-11-02T10:00:05Z', updated_at: '2026-11-02T10:02:10Z',
  actor: { login: 'ralf' }, triggering_actor: { login: 'anna' }, head_commit: { message: 'Commit-Zeile' },
}
const RAW_JOB = (id: number, name: string, status: string, conclusion: string | null, extra: Record<string, unknown> = {}) => ({
  id, name, status, conclusion, html_url: `https://github.com/Tenvan/k3c-family/actions/runs/9001/job/${id}`,
  started_at: '2026-11-02T10:00:10Z', completed_at: '2026-11-02T10:01:15Z', workflow_name: 'CI', steps: [], ...extra,
})

test('ghStatus: status und conclusion auf das GitLab-Vokabular abgebildet', () => {
  expect([
    ghStatus('completed', 'success'), ghStatus('completed', 'failure'), ghStatus('completed', 'timed_out'), ghStatus('completed', 'cancelled'),
    ghStatus('completed', 'skipped'), ghStatus('completed', 'action_required'), ghStatus('in_progress', null), ghStatus('queued', null), ghStatus('waiting', null), ghStatus(undefined, undefined),
  ]).toEqual(['success', 'failed', 'failed', 'canceled', 'skipped', 'manual', 'running', 'pending', 'waiting_for_resource', 'created'])
})

test('toGhRuns: Felder, Dauer aus den Zeitstempeln, Titel erste Zeile, Auslöser, Objekt oder Liste, ohne Id verworfen', () => {
  const [p] = toGhRuns({ workflow_runs: [RAW_RUN, { status: 'x' }] })
  expect(p).toEqual({
    id: 9001, status: 'failed', ref: 'feature/x', sha: '01234567', url: 'https://github.com/Tenvan/k3c-family/actions/runs/9001', source: 'push',
    createdAt: '2026-11-02T10:00:00Z', updatedAt: '2026-11-02T10:02:10Z', startedAt: '2026-11-02T10:00:05Z', finishedAt: '2026-11-02T10:02:10Z', durationS: 125, user: 'anna', userName: '', tag: false, title: 'Export vereinfachen',
  })
  expect(toGhRuns({ workflow_runs: [RAW_RUN, { status: 'x' }] }).length).toBe(1)
  expect(toGhRuns([RAW_RUN]).length).toBe(1)
  expect(toGhRuns({ workflow_runs: [{ ...RAW_RUN, display_title: '', triggering_actor: null }] })[0]).toMatchObject({ title: 'Commit-Zeile', user: 'ralf' })
  expect(toGhRuns(null)).toEqual([])
  expect(toGhRuns({})).toEqual([])
})

test('toGhJobs: Workflow als Stage, Schritte mit Status, kein allowFailure und kein manueller Job', () => {
  const steps = [
    { name: 'Checkout', status: 'completed', conclusion: 'success', started_at: '2026-11-02T10:00:10Z', completed_at: '2026-11-02T10:00:12Z' },
    { name: 'Test', status: 'completed', conclusion: 'failure', started_at: '2026-11-02T10:00:12Z', completed_at: '2026-11-02T10:01:12Z' },
  ]
  const jobs = toGhJobs({ jobs: [RAW_JOB(2, 'test', 'completed', 'failure', { steps }), RAW_JOB(1, 'build', 'completed', 'success', { workflow_name: undefined })] }, 'Fallback')
  expect(jobs[0]).toMatchObject({ id: 2, name: 'test', stage: 'CI', status: 'failed', durationS: 65, allowFailure: false, manual: false })
  expect(jobs[0]?.steps).toEqual([{ name: 'Checkout', status: 'success', durationS: 2 }, { name: 'Test', status: 'failed', durationS: 60 }])
  expect(jobs[1]?.stage).toBe('Fallback')
  expect(byStage(jobs).map(g => [g.stage, g.jobs.map(j => j.id)])).toEqual([['Fallback', [1]], ['CI', [2]]])
  expect(toGhJobs(null, 'x')).toEqual([])
  expect(ghWorkflowName({ name: 'CI' })).toBe('CI')
})

test('ghWithDetail: Dauer, Auslöser, Status und Titel aus dem Detail-Run', () => {
  const base = toGhRuns([{ ...RAW_RUN, status: 'in_progress', conclusion: null, display_title: '', head_commit: null }])[0]!
  expect(base.title).toBe('')
  const d = ghWithDetail(base, RAW_RUN)
  expect(d).toMatchObject({ status: 'failed', durationS: 125, title: 'Export vereinfachen' })
  expect(ghWithDetail(base, null)).toEqual(base)
})

test('Steuerung: GitHub blendet play, Job-Abbruch und neuen Run aus, retry und cancel des Runs bleiben', () => {
  expect(ciCaps('github')).toEqual({ play: false, jobCancel: false, newRun: false })
  expect(ciCaps('gitlab')).toEqual({ play: true, jobCancel: true, newRun: true })
  const run = (status: string) => toGhRuns([{ ...RAW_RUN, status: status === 'running' ? 'in_progress' : 'completed', conclusion: status === 'running' ? null : status === 'canceled' ? 'cancelled' : 'failure' }])[0]!
  expect(pipelineCan(run('failed'))).toEqual({ retry: true, cancel: false })
  expect(pipelineCan(run('canceled'))).toEqual({ retry: true, cancel: false })
  expect(pipelineCan(run('running'))).toEqual({ retry: false, cancel: true })
  const job = toGhJobs({ jobs: [RAW_JOB(1, 'x', 'completed', 'failure')] }, 'CI')[0]!
  expect(jobCan(job)).toEqual({ retry: true, cancel: false, play: false, log: true })
})

test('Ereignisse: dieselbe Logik für GitHub-Runs (Wechsel, Branch-Filter) und Jobs', () => {
  const run = (id: number, status: string, conclusion: string | null, ref = 'feature/x') => toGhRuns([{ ...RAW_RUN, id, status, conclusion, head_branch: ref }])[0]!
  const prev = [run(1, 'in_progress', null), run(2, 'in_progress', null, 'main')]
  const next = [run(1, 'completed', 'failure'), run(2, 'completed', 'success', 'main')]
  expect(ciEvents(prev, next, p => p.ref === 'feature/x').map(e => [e.event, e.pipeline.id])).toEqual([['pipelineFailed', 1]])
  expect(ciEvents(null, next, () => true)).toEqual([])
  const before = toGhJobs({ jobs: [RAW_JOB(5, 'test', 'in_progress', null)] }, 'CI')
  const after = toGhJobs({ jobs: [RAW_JOB(5, 'test', 'completed', 'failure')] }, 'CI')
  expect(jobEvents(before, after).map(j => j.id)).toEqual([5])
})

test('Sanitizing: Escape-Folgen und Steuerzeichen aus GitHub-Text und -Logs entfernt', () => {
  const evil = `${ESC}]0;gehackt${BEL}Titel${ESC}[31m rot${ESC}[0m\u0001`
  const [p] = toGhRuns({ workflow_runs: [{ ...RAW_RUN, display_title: evil, head_branch: evil, actor: { login: evil }, triggering_actor: null }] })
  for (const v of [p?.title, p?.ref, p?.user]) expect(v).toBe('Titel rot')
  const [j] = toGhJobs({ jobs: [RAW_JOB(1, evil, 'completed', 'success', { workflow_name: evil, steps: [{ name: evil, status: 'completed', conclusion: 'success' }] })] }, evil)
  expect(j?.name).toBe('Titel rot')
  expect(j?.stage).toBe('Titel rot')
  expect(j?.steps?.[0]?.name).toBe('Titel rot')
  const raw = `2026-11-02T10:00:00.1234567Z ${ESC}[31;1mfehler${ESC}[0m\n2026-11-02T10:00:01Z ${ESC}]8;;http://x${ESC}\\link${ESC}]8;;${ESC}\\\n`
  const { lines } = tailLog(ghLogText(raw))
  expect(lines.join('\n')).not.toContain(ESC)
  expect(lines[0]).toBe('fehler')
  expect(lines[1]).toBe('link')
})

test('ghLogText: Zeitstempel, BOM und Workflow-Befehle weg, Gruppen als Titel, Fehler markiert', () => {
  const raw = '﻿2026-11-02T10:00:00.1Z ##[group]Run npm test\n2026-11-02T10:00:01Z > test\n2026-11-02T10:00:02Z ##[endgroup]\n2026-11-02T10:00:03Z ##[error]Process completed with exit code 1.\n'
  expect(ghLogText(raw)).toBe('▸ Run npm test\n> test\n✗ Process completed with exit code 1.\n')
})

test('Provider-Erkennung aus dem Remote-URL: GitLab und GitHub unterstützt, Bitbucket nicht', () => {
  expect(forgeRemote('git@gitlab.deltra.de:consulting/x.git')).toEqual({ kind: 'gitlab', host: 'gitlab.deltra.de', project: 'consulting/x' })
  expect(forgeRemote('https://github.com/Tenvan/k3c-family.git')).toEqual({ kind: 'github', host: 'github.com', project: 'Tenvan/k3c-family' })
  expect(forgeRemote('git@github.com:Tenvan/k3c-family.git')?.kind).toBe('github')
  expect(ciSupported(forgeRemote('git@github.com:a/b.git'))).toBe(true)
  expect(ciSupported(forgeRemote('git@gitlab.deltra.de:a/b.git'))).toBe(true)
  expect(ciSupported(forgeRemote('ssh://git@git.deltra.de:7999/cor/x.git'))).toBe(false)
  expect(ciSupported(null)).toBe(false)
})

test('toGhRuns/toGhJobs: Start, Ende (nur abgeschlossen), Auslöser-Event und Ersteller', () => {
  const [done] = toGhRuns([RAW_RUN])
  expect(done).toMatchObject({ startedAt: '2026-11-02T10:00:05Z', finishedAt: '2026-11-02T10:02:10Z', source: 'push', user: 'anna', userName: '', tag: false })
  const [live] = toGhRuns([{ ...RAW_RUN, status: 'in_progress', conclusion: null, event: 'workflow_dispatch' }])
  expect(live).toMatchObject({ finishedAt: '', source: 'workflow_dispatch' })
  const [j1, j2] = toGhJobs({ jobs: [RAW_JOB(1, 'a', 'completed', 'success'), RAW_JOB(2, 'b', 'in_progress', null, { completed_at: null })] }, 'CI')
  expect(j1).toMatchObject({ startedAt: '2026-11-02T10:00:10Z', finishedAt: '2026-11-02T10:01:15Z' })
  expect(j2).toMatchObject({ startedAt: '2026-11-02T10:00:10Z', finishedAt: '' })
})

test('toGhArtifacts: Name, Größe, abgelaufen, Link auf die Run-Seite; fremder Text bereinigt, ohne Id oder Namen verworfen', () => {
  const run = 'https://github.com/Tenvan/k3c-family/actions/runs/9001'
  const raw = { total_count: 4, artifacts: [
    { id: 5, name: 'dist', size_in_bytes: 1_048_576, expired: false, archive_download_url: 'https://api.github.com/x/zip' },
    { id: 6, name: `evil${ESC}]0;x${BEL}.zip`, size_in_bytes: 10, expired: true },
    { name: 'ohne-id', size_in_bytes: 1 },
    { id: 7, name: '', size_in_bytes: 1 },
  ] }
  const a = toGhArtifacts(raw, run)
  expect(a.map(x => [x.id, x.job, x.name, x.size, x.expired, x.url])).toEqual([[5, 0, 'dist', 1_048_576, false, run], [6, 0, 'evil.zip', 10, true, run]])
  expect(JSON.stringify(a)).not.toContain('api.github.com')
  expect(toGhArtifacts(null, run)).toEqual([])
})

test('Bitbucket: erkannt, aber ohne CI-Adapter', () => {
  expect(forgeRemote('https://git.deltra.de/scm/cor/orga-marketplace.git')).toMatchObject({ kind: 'bitbucket', host: 'git.deltra.de', project: 'cor/orga-marketplace' })
  expect(ciSupported(forgeRemote('ssh://git@git.deltra.de:7999/cor/x.git'))).toBe(false)
})
