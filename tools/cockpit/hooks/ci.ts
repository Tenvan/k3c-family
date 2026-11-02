// CI/CD auf GitLab: API-Antworten (`glab api`) in kleine Zustände übersetzen, Statuswechsel als Ereignisse melden, Job-Logs
// aufbereiten. Rein, ohne `$`; die Aufrufe stehen in forge.ts, Zustand und Anzeige in register.tsx und ci-ui.tsx.
// Mechanik (Entscheidung): Polling statt Webhook. Ein Webhook bräuchte einen vom GitLab-Server erreichbaren Empfänger auf dem
// Arbeitsplatz (NAT/Firewall); `glab api` nutzt die vorhandene glab-Anmeldung, das Cockpit sieht nie ein Token.

import type { CiJob, CiPipeline } from '../types'

export type CiEvent = 'pipelineSuccess' | 'pipelineFailed' | 'pipelineCanceled' | 'pipelineManual' | 'jobFailed'

type Obj = Record<string, unknown>
const obj = (v: unknown): Obj => (typeof v === 'object' && v !== null ? (v as Obj) : {})
// Alles aus der GitLab-API (Namen, Refs, Titel, Logs) ist fremder Text: Steuerzeichen und Escape-Folgen (CSI, OSC, DCS, APC, PM,
// SOS) fliegen raus, bevor er in Text oder Toasts landet, damit er das Terminal nicht umschalten oder Titel/Links setzen kann.
const ESC = String.fromCharCode(27)
const BEL = String.fromCharCode(7)
const STRING_SEQ = new RegExp(`${ESC}[\\]P^_X][\\s\\S]*?(?:${BEL}|${ESC}\\\\|$)`, 'g')
const CSI_SEQ = new RegExp(`${ESC}\\[[0-?]*[ -/]*[@-~]`, 'g')
const OTHER_ESC = new RegExp(`${ESC}[\\s\\S]?`, 'g')
const CONTROL = new RegExp('[\\u0000-\\u0008\\u000b-\\u001f\\u007f-\\u009f]', 'g')
export const clean = (v: string): string => v.replace(STRING_SEQ, '').replace(CSI_SEQ, '').replace(OTHER_ESC, '').replace(CONTROL, '')
const str = (v: unknown): string => (typeof v === 'string' ? clean(v) : '')
const num = (v: unknown): number => (typeof v === 'number' && Number.isFinite(v) ? v : 0)

export const PIPELINES_MAX = 15
export const LOG_LINES = 60

const ACTIVE = new Set(['created', 'waiting_for_resource', 'preparing', 'pending', 'running', 'scheduled'])
export const isActive = (status: string): boolean => ACTIVE.has(status)
export const isFinished = (status: string): boolean => !ACTIVE.has(status) && status !== 'manual'

// Ton eines Status für Farbe und Badge.
export const ciTone = (status: string): 'success' | 'error' | 'warning' | 'accent' | 'neutral' =>
  status === 'success' ? 'success'
    : status === 'failed' ? 'error'
      : ACTIVE.has(status) ? 'accent'
        : status === 'manual' ? 'warning'
          : 'neutral'

export function toPipelines(list: unknown): CiPipeline[] {
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => typeof x.id === 'number').map(x => ({
    id: x.id as number,
    status: str(x.status),
    ref: str(x.ref),
    sha: str(x.sha).slice(0, 8),
    url: str(x.web_url),
    source: str(x.source),
    createdAt: str(x.created_at),
    updatedAt: str(x.updated_at),
    durationS: Math.round(num(x.duration)),
    user: str(obj(x.user).username),
    title: '',
  }))
}

// Detail einer Pipeline (Dauer, Auslöser) über die Listenwerte legen.
export const withDetail = (p: CiPipeline, detail: unknown): CiPipeline => {
  const d = toPipelines([detail])[0]
  return d === undefined ? p : { ...p, durationS: d.durationS, user: d.user || p.user, status: d.status || p.status }
}

export function toJobs(list: unknown): CiJob[] {
  if (!Array.isArray(list)) return []
  return list.map(obj).filter(x => typeof x.id === 'number').map(x => ({
    id: x.id as number,
    name: str(x.name),
    stage: str(x.stage),
    status: str(x.status),
    url: str(x.web_url),
    durationS: Math.round(num(x.duration)),
    allowFailure: x.allow_failure === true,
    manual: str(x.status) === 'manual',
    startedAt: str(x.started_at),
  }))
}

// Commit-Titel der Pipeline steht an den Jobs (`commit.title`), nicht an der Pipeline.
export function commitTitle(jobs: unknown): string {
  if (!Array.isArray(jobs)) return ''
  const first = jobs.map(obj).find(j => str(obj(j.commit).title) !== '')
  return first === undefined ? '' : str(obj(first.commit).title)
}

// Jobs nach Stage gruppiert; Stages in der Reihenfolge ihres ersten (kleinsten) Jobs, Jobs je Stage nach Id.
// ponytail: Stage-Reihenfolge aus den Job-Ids abgeleitet, nicht aus `.gitlab-ci.yml`; bei abweichenden `needs` kann sie abweichen.
export function byStage(jobs: readonly CiJob[]): { stage: string; jobs: CiJob[] }[] {
  const order = new Map<string, number>()
  for (const j of jobs) order.set(j.stage, Math.min(order.get(j.stage) ?? Infinity, j.id))
  const stages = [...order.entries()].sort((a, b) => a[1] - b[1]).map(([s]) => s)
  return stages.map(stage => ({ stage, jobs: jobs.filter(j => j.stage === stage).sort((a, b) => a.id - b.id) }))
}

// Aktionen je Job: wiederholen (beendet, nicht manuell), abbrechen (läuft oder wartet), starten (manuell).
export const jobCan = (j: CiJob): { retry: boolean; cancel: boolean; play: boolean; log: boolean } => ({
  retry: ['failed', 'success', 'canceled'].includes(j.status),
  cancel: ['running', 'pending', 'created', 'preparing', 'waiting_for_resource', 'scheduled'].includes(j.status),
  play: j.status === 'manual',
  log: !['created', 'manual', 'skipped'].includes(j.status),
})

// Aktionen je Pipeline: Fehlgeschlagene neu starten, Laufende abbrechen.
export const pipelineCan = (p: CiPipeline): { retry: boolean; cancel: boolean } => ({
  retry: p.status === 'failed' || p.status === 'canceled',
  cancel: isActive(p.status),
})

// Welche Pipeline der Reiter zeigt: die gewählte (wenn es sie noch gibt), sonst die neueste des Branches, sonst die neueste.
export function pickPipeline(list: readonly CiPipeline[], sel: number, branch: string): CiPipeline | null {
  return list.find(p => p.id === sel) ?? list.find(p => p.ref === branch) ?? list[0] ?? null
}

// Polling: alle 15 s, solange etwas läuft, sonst jede Minute.
export const ciPollDelayMs = (list: readonly CiPipeline[]): number => (list.some(p => isActive(p.status)) ? 15_000 : 60_000)

// Wechsel zwischen zwei Abfragen: nur Pipelines, die es in beiden gibt (der erste Abruf meldet nichts); nur die des
// Branches (`mine`: Ref gleich Branch oder Id der MR-Pipeline) sind Anlass für Hinweise.
export function ciEvents(prev: readonly CiPipeline[] | null, next: readonly CiPipeline[], mine: (p: CiPipeline) => boolean): { event: CiEvent; pipeline: CiPipeline }[] {
  if (prev === null) return []
  const out: { event: CiEvent; pipeline: CiPipeline }[] = []
  for (const p of next) {
    const was = prev.find(x => x.id === p.id)
    if (was === undefined || was.status === p.status || !mine(p)) continue
    if (p.status === 'success') out.push({ event: 'pipelineSuccess', pipeline: p })
    else if (p.status === 'failed') out.push({ event: 'pipelineFailed', pipeline: p })
    else if (p.status === 'canceled') out.push({ event: 'pipelineCanceled', pipeline: p })
    else if (p.status === 'manual') out.push({ event: 'pipelineManual', pipeline: p })
  }
  return out
}

// Jobs, die zwischen zwei Abfragen derselben Pipeline fehlgeschlagen sind (ohne erlaubte Fehlschläge).
export function jobEvents(prev: readonly CiJob[] | null, next: readonly CiJob[]): CiJob[] {
  if (prev === null) return []
  return next.filter(j => j.status === 'failed' && !j.allowFailure && prev.some(x => x.id === j.id && x.status !== 'failed'))
}

// Fußzeile der Liste: „5 min“, „2 h“, „3 d“ seit einem ISO-Zeitpunkt.
export function ago(iso: string, now: number): string {
  const t = Date.parse(iso)
  if (!Number.isFinite(t)) return '–'
  const s = Math.max(0, Math.round((now - t) / 1000))
  return s < 90 ? `${s} s` : s < 5400 ? `${Math.round(s / 60)} min` : s < 129_600 ? `${Math.round(s / 3600)} h` : `${Math.round(s / 86_400)} d`
}

// Dauer in Sekunden als „1:05“ oder „1:02:03“; 0 = unbekannt.
export function clock(sec: number): string {
  if (!(sec > 0)) return '–'
  const h = Math.floor(sec / 3600)
  const m = Math.floor((sec % 3600) / 60)
  const s = sec % 60
  const pad = (n: number) => String(n).padStart(2, '0')
  return h > 0 ? `${h}:${pad(m)}:${pad(s)}` : `${m}:${pad(s)}`
}

// Job-Log für die Anzeige: ANSI-Farben und GitLab-Abschnittsmarker (`section_start:…`) entfernt, Wagenrücklauf-Überschreibungen
// aufgelöst, leere Zeilen am Ende weg, nur die letzten `max` Zeilen.
const SECTION = /section_(?:start|end):\d+:[\w.-]+(?:\[[^\]]*\])?/g
export function tailLog(raw: string, max = LOG_LINES): { lines: string[]; truncated: boolean } {
  const all = raw
    .split('\n')
    .map(l => (l.includes('\r') ? (l.split('\r').filter(x => x !== '').pop() ?? '') : l))
    .map(l => clean(l.replace(SECTION, '')).trimEnd())
  while (all.length > 0 && all[all.length - 1] === '') all.pop()
  return { lines: all.slice(-max), truncated: all.length > max }
}
