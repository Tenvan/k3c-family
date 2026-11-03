// CI/CD auf GitHub: Antworten von `gh api` (Actions: Workflow-Runs und Jobs) in dieselbe Form wie bei GitLab übersetzen
// (`CiPipeline`, `CiJob`). Rein, ohne `$`; die Aufrufe stehen in forge.ts. Aller API-Text läuft durch `str` (ci.ts) und verliert
// damit Escape-Folgen und Steuerzeichen, genau wie bei GitLab.
// Statusvokabular: GitHub trennt `status` (queued, in_progress, completed …) und `conclusion` (success, failure …); hier wird
// beides auf das GitLab-Vokabular abgebildet, damit Badges, Ereignisse und Polling unverändert greifen.

import type { CiArtifact, CiJob, CiPipeline, CiStep } from '../types'
import { num, obj, str } from './ci'

export function ghStatus(status: unknown, conclusion: unknown): string {
  const s = str(status)
  const c = str(conclusion)
  if (s === 'completed') {
    if (c === 'success' || c === 'neutral') return 'success'
    if (c === 'failure' || c === 'timed_out' || c === 'startup_failure') return 'failed'
    if (c === 'cancelled' || c === 'stale') return 'canceled'
    if (c === 'skipped') return 'skipped'
    if (c === 'action_required') return 'manual'
    return 'failed'
  }
  if (s === 'in_progress') return 'running'
  if (s === 'waiting') return 'waiting_for_resource'
  if (s === 'queued' || s === 'pending' || s === 'requested') return 'pending'
  return s === '' ? 'created' : s
}

const seconds = (from: unknown, to: unknown): number => {
  const a = Date.parse(str(from))
  const b = Date.parse(str(to))
  return Number.isFinite(a) && Number.isFinite(b) && b >= a ? Math.round((b - a) / 1000) : 0
}

const firstLine = (v: unknown): string => str(v).split('\n')[0]?.trim() ?? ''

// `gh api …/actions/runs` liefert `{ workflow_runs: [...] }`; ein einzelner Run (Detail) ist das Objekt selbst.
export function toGhRuns(raw: unknown): CiPipeline[] {
  const list = Array.isArray(raw) ? raw : obj(raw).workflow_runs
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => typeof x.id === 'number').map(x => ({
    id: x.id as number,
    status: ghStatus(x.status, x.conclusion),
    ref: str(x.head_branch),
    sha: str(x.head_sha).slice(0, 8),
    url: str(x.html_url),
    source: str(x.event),
    createdAt: str(x.created_at),
    updatedAt: str(x.updated_at),
    startedAt: str(x.run_started_at) || str(x.created_at),
    // Ein Run hat kein Ende-Feld: `updated_at` ist es erst, wenn der Run abgeschlossen ist.
    finishedAt: str(x.status) === 'completed' ? str(x.updated_at) : '',
    durationS: seconds(x.run_started_at ?? x.created_at, x.updated_at),
    user: str(obj(x.triggering_actor).login) || str(obj(x.actor).login),
    userName: '',
    tag: false,
    // Anders als bei GitLab steht der Titel (PR-Titel oder Commit-Zeile) schon an der Liste.
    title: firstLine(x.display_title) || firstLine(obj(x.head_commit).message),
  }))
}

export const ghWithDetail = (p: CiPipeline, detail: unknown): CiPipeline => {
  const d = toGhRuns([detail])[0]
  return d === undefined ? p : { ...p, durationS: d.durationS, startedAt: d.startedAt || p.startedAt, finishedAt: d.finishedAt, user: d.user || p.user, status: d.status || p.status, title: d.title || p.title }
}

// Jobs eines Runs; `stage` ist hier der Workflow-Name (GitHub hat keine Stages), `steps` die Schritte des Jobs.
export function toGhJobs(raw: unknown, workflow: string): CiJob[] {
  const list = obj(raw).jobs
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => typeof x.id === 'number').map(x => {
    const steps: CiStep[] = Array.isArray(x.steps)
      ? x.steps.map(obj).map(s => ({ name: str(s.name), status: ghStatus(s.status, s.conclusion), durationS: seconds(s.started_at, s.completed_at) }))
      : []
    const status = ghStatus(x.status, x.conclusion)
    return {
      id: x.id as number,
      name: str(x.name),
      stage: str(x.workflow_name) || workflow,
      status,
      url: str(x.html_url),
      durationS: seconds(x.started_at, x.completed_at),
      allowFailure: false,
      manual: false,
      startedAt: str(x.started_at),
      finishedAt: str(x.status) === 'completed' ? str(x.completed_at) : '',
      steps,
    }
  })
}

// Artefakte eines Runs (`…/actions/runs/<id>/artifacts` → `{ artifacts: [...] }`); sie gehören dem Run, nicht einem Job. Laden lassen sie sich
// im Browser (Seite des Runs, Abschnitt „Artifacts“); die API-Adresse `archive_download_url` verlangt ein Token und bleibt ungenutzt.
export function toGhArtifacts(raw: unknown, runUrl: string): CiArtifact[] {
  const list = obj(raw).artifacts
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => typeof x.id === 'number' && str(x.name) !== '').map(x => ({
    id: x.id as number,
    job: 0,
    name: str(x.name),
    size: Math.max(0, Math.round(num(x.size_in_bytes))),
    expired: x.expired === true,
    url: runUrl,
  }))
}

// Workflow-Name des Runs (Fallback für Jobs ohne `workflow_name`).
export const ghWorkflowName = (detail: unknown): string => str(obj(detail).name)

// Job-Log von GitHub: Zeitstempel-Präfix, BOM und Workflow-Befehle (`##[group]`, `##[error]` …) weg, Gruppen als „▸ Titel“.
const STAMP = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z ?/
const COMMAND = /^##\[(group|endgroup|error|warning|notice|debug|command)\]/
export function ghLogText(raw: string): string {
  const out: string[] = []
  for (const line of raw.replace(/^﻿/, '').split('\n')) {
    const l = line.replace(/\r$/, '').replace(STAMP, '').replace(/^﻿/, '')
    const m = COMMAND.exec(l)
    if (m === null) {
      out.push(l)
    } else if (m[1] === 'endgroup') {
      continue
    } else {
      const rest = l.slice(m[0].length)
      out.push(m[1] === 'group' ? `▸ ${rest}` : m[1] === 'error' ? `✗ ${rest}` : rest)
    }
  }
  return out.join('\n')
}
