// Checkliste im Band (Idee: glanceflow, kein Code übernommen): ein Job je Prompt, seine Schritte aus Claudes eigener
// Aufgabenliste (TodoWrite, Desktop-Aufgabenliste `update_plan`, TaskCreate/TaskUpdate), Status, Restzeit, Verlauf und
// Berichte. Rein beobachtend: kein Prompt-Einschub, keine eigenen Tools, kein deny. Reine Funktionen ohne `$`.
import type { ActKind, Job, JobPhase, JobRecord, JobStep, JobStepStatus, UiLang } from '../types'
import { duration, oneLine } from './core'
import { t, type Key } from './i18n'

// Ohne neues Zeichen (Tool-Aufruf, Model-Request) so lange gilt Claude als „hängt?“.
export const STUCK_MS = 180_000
// Erst ab dieser Dauer meldet „Fertig“ sich mit Ton oder Hinweis.
export const LONG_JOB_MS = 60_000
export const HISTORY_DAYS = 30
// Ab so vielen Schritten faltet das Band erledigte und spätere Schritte in je eine Zeile.
export const FOLD_AT = 5
export const HISTORY_PREFIX = 'glance:'
const RECORDS_PER_DAY = 200
const FILES_PER_STEP = 12
const FILES_PER_JOB = 50
// Obergrenze der Schritte je Job: eine ausufernde Aufgabenliste darf Band, Atom und Verlauf nicht aufblähen.
export const MAX_STEPS = 100

export function newJob(id: string, prompt: string, cwd: string, now: number): Job {
  return { id, title: jobTitle(prompt), cwd, startedAt: now, phase: 'working', question: '', ask: '', steps: [], act: null, signAt: now, tools: 0, files: [] }
}

// Erste aussagekräftige Zeile eines Berichts: Anrede („Ok Boss 🤠“), Überschriften-Zeichen und Markdown weg.
function firstLine(text: string): string {
  const lines = text
    .split(/\r?\n/)
    .map(l => l.replace(/^[\s>#*\-|]+/, '').replace(/[*_`]/g, '').trim())
    .filter(l => l !== '' && !/^ok boss\b/i.test(l) && !/^-+$/.test(l))
  return lines[0] ?? ''
}

/**
 * Titel eines Jobs aus dem Prompt. Nachrichten, die keine Person getippt hat, bekommen statt der XML-Hülle einen lesbaren
 * Titel mit Symbol (sprachneutral, der Titel wird gespeichert): ↩ Rückmeldung eines Subagents, ✉ Nachricht einer anderen
 * Session, ⚙ Benachrichtigung einer Hintergrundaufgabe.
 */
export function jobTitle(prompt: string, max = 70): string {
  const text = prompt.trim()
  const agent = /^<agent-message from="([^"]+)">([\s\S]*?)(?:<\/agent-message>|$)/.exec(text)
  if (agent) {
    const body = agent[2] ?? ''
    const report = /The report follows:\s*([\s\S]*)$/.exec(body)?.[1] ?? body
    return oneLine(`↩ ${agent[1]}: ${firstLine(report)}`, max)
  }
  const peer = /^<cross-session-message from="([^"]+)"[^>]*>([\s\S]*?)(?:<\/cross-session-message>|$)/.exec(text)
  if (peer) return oneLine(`✉ ${peer[1]}: ${firstLine(peer[2] ?? '')}`, max)
  if (text.startsWith('<task-notification>')) {
    const summary = /<summary>([\s\S]*?)<\/summary>/.exec(text)?.[1]
    return oneLine(`⚙ ${summary === undefined ? 'Task' : firstLine(summary)}`, max)
  }
  return oneLine(text, max)
}

// ---- Plan aus Claudes Aufgabenliste ----

export const PLAN_TOOLS: ReadonlySet<string> = new Set(['TodoWrite', 'mcp__ccd_turn__update_plan', 'TaskCreate', 'TaskUpdate'])

const statusOf = (s: unknown): JobStepStatus => (s === 'completed' ? 'done' : s === 'in_progress' ? 'active' : 'upcoming')
const str = (v: unknown): string => (typeof v === 'string' ? v.trim() : '')

// Schritt mit neuem Status; Start- und Endzeit setzt der erste Wechsel nach aktiv bzw. erledigt.
function stepAt(old: JobStep | undefined, name: string, status: JobStepStatus, now: number, ref?: string): JobStep {
  const base: JobStep = old ?? { name, status: 'upcoming', files: [] }
  const startedAt = base.startedAt ?? (status === 'upcoming' ? undefined : now)
  const doneAt = status === 'done' ? (base.doneAt ?? now) : undefined
  return {
    name,
    status,
    files: base.files,
    ...(ref ?? base.ref ? { ref: ref ?? base.ref } : {}),
    ...(startedAt === undefined ? {} : { startedAt }),
    ...(doneAt === undefined ? {} : { doneAt }),
  }
}

// Ganze Liste ersetzen; ein Schritt mit gleichem Namen behält Zeiten und Dateien.
function replaceAll(job: Job, items: readonly { name: string; status: JobStepStatus }[], now: number): Job {
  const steps = items.filter(i => i.name !== '').slice(0, MAX_STEPS).map(i => stepAt(job.steps.find(s => s.name === i.name), i.name, i.status, now))
  return { ...job, steps }
}

const setStatus = (job: Job, index: number, status: JobStepStatus, now: number): Job => {
  const old = job.steps[index]
  if (old === undefined) return job
  return { ...job, steps: job.steps.map((s, i) => (i === index ? stepAt(old, old.name, status, now) : s)) }
}

// Nummer einer neuen Aufgabe aus der Antwort von TaskCreate („Task #3 created …“ oder ein Objekt mit id).
function taskIdOf(result: unknown, fallback: string): string {
  if (typeof result === 'string') return /#(\d+)/.exec(result)?.[1] ?? fallback
  const r = (result ?? {}) as Record<string, unknown>
  const task = (r.task ?? {}) as Record<string, unknown>
  const id = r.taskId ?? r.id ?? task.id
  return typeof id === 'string' || typeof id === 'number' ? String(id) : fallback
}

/** Neuer Plan nach einem Aufruf von Claudes Aufgabenliste; null, wenn der Aufruf keine ist. */
export function planUpdate(job: Job, tool: string, input: unknown, result: unknown, now: number): Job | null {
  if (!PLAN_TOOLS.has(tool)) return null
  const o = (input ?? {}) as Record<string, unknown>
  const listOf = (v: unknown) => (Array.isArray(v) ? v.map(x => (x ?? {}) as Record<string, unknown>) : null)
  if (tool === 'TodoWrite') {
    const todos = listOf(o.todos)
    return todos === null ? null : replaceAll(job, todos.map(x => ({ name: oneLine(str(x.content), 120), status: statusOf(x.status) })), now)
  }
  if (tool === 'mcp__ccd_turn__update_plan') {
    const items = listOf(o.items)
    if (items !== null) return replaceAll(job, items.map(x => ({ name: oneLine(str(x.content), 120), status: statusOf(x.status) })), now)
    let next = job
    // Teil-Updates: `advance` schließt den aktiven Schritt und startet den nächsten offenen; `done`/`start` sind 1-basiert.
    if (o.advance === true) {
      const active = next.steps.findIndex(s => s.status === 'active')
      if (active >= 0) next = setStatus(next, active, 'done', now)
      const upcoming = next.steps.findIndex(s => s.status === 'upcoming')
      if (upcoming >= 0) next = setStatus(next, upcoming, 'active', now)
    }
    if (Array.isArray(o.done)) for (const n of o.done) if (typeof n === 'number') next = setStatus(next, n - 1, 'done', now)
    if (typeof o.start === 'number') next = setStatus(next, o.start - 1, 'active', now)
    return next
  }
  if (tool === 'TaskCreate') {
    const name = oneLine(str(o.subject) || str(o.activeForm) || str(o.description), 120)
    if (name === '') return null
    if (job.steps.length >= MAX_STEPS) return job
    const ref = taskIdOf(result, String(job.steps.length + 1))
    return { ...job, steps: [...job.steps, stepAt(undefined, name, 'upcoming', now, ref)] }
  }
  // TaskUpdate
  const ref = str(String(o.taskId ?? ''))
  const index = job.steps.findIndex(s => s.ref === ref)
  if (index < 0) return null
  if (o.status === 'deleted') return { ...job, steps: job.steps.filter((_, i) => i !== index) }
  const old = job.steps[index] as JobStep
  const name = oneLine(str(o.subject), 120) || old.name
  const status = typeof o.status === 'string' ? statusOf(o.status) : old.status
  return { ...job, steps: job.steps.map((s, i) => (i === index ? stepAt(old, name, status, now) : s)) }
}

// ---- Was Claude gerade tut ----

const ACT: Record<string, ActKind> = {
  Read: 'read', NotebookRead: 'read',
  Grep: 'search', Glob: 'search', LS: 'search', ToolSearch: 'search',
  Edit: 'edit', Write: 'edit', MultiEdit: 'edit', NotebookEdit: 'edit',
  Bash: 'shell', PowerShell: 'shell', BashOutput: 'shell', Monitor: 'shell',
  WebFetch: 'web', WebSearch: 'web',
  Agent: 'agent', Task: 'agent', Workflow: 'agent',
  TodoWrite: 'plan', TaskCreate: 'plan', TaskUpdate: 'plan', mcp__ccd_turn__update_plan: 'plan',
  AskUserQuestion: 'ask',
}

export function actOf(tool: string): ActKind {
  const known = ACT[tool]
  if (known !== undefined) return known
  // MCP-Tools nach ihrem Namen grob einordnen (Serena, context-mode, Browser …).
  if (/find|search|symbol|overview|grep|list_dir|query/i.test(tool)) return 'search'
  if (/read|get_page|fetch/i.test(tool)) return /fetch|navigate|page/i.test(tool) ? 'web' : 'read'
  if (/replace|insert|write|edit|rename/i.test(tool)) return 'edit'
  if (/execute|run|shell/i.test(tool)) return 'shell'
  return 'other'
}

/** Ein Lebenszeichen: Claude ist wieder in Bewegung; „hängt?“ löst sich damit auf. */
export const touch = (job: Job, now: number): Job => ({ ...job, signAt: now })

/** Tool-Aufruf der Hauptschleife: Aktivität zählen (gleiche Art hintereinander zählt hoch). */
export function noteAct(job: Job, tool: string, now: number): Job {
  const kind = actOf(tool)
  const n = job.act?.kind === kind ? job.act.n + 1 : 1
  return { ...job, act: { kind, n }, tools: job.tools + 1, signAt: now }
}

const shortPath = (path: string): string => path.replace(/\\/g, '/').split('/').filter(Boolean).slice(-2).join('/')

/** Geänderte Datei dem laufenden Schritt (sonst dem Job) zuordnen; nur Namen, keine Inhalte. */
export function noteFile(job: Job, path: string): Job {
  const name = shortPath(path)
  if (name === '') return job
  const add = (list: readonly string[], max: number) => (list.includes(name) ? [...list] : [...list, name].slice(-max))
  const active = job.steps.findIndex(s => s.status === 'active')
  const steps = active < 0 ? job.steps : job.steps.map((s, i) => (i === active ? { ...s, files: add(s.files, FILES_PER_STEP) } : s))
  return { ...job, steps, files: add(job.files, FILES_PER_JOB) }
}

// ---- Ende eines Turns ----

// Markdown-Reste weg, damit „**Soll Claude …?**“ als Frage erkannt wird.
const plain = (s: string): string => s.replace(/[*_`>#]/g, '').replace(/\s+/g, ' ').trim()

/**
 * Die Frage, mit der Claudes Antwort endet: der letzte Satz mit „?“ in den letzten drei Zeilen (eine Grußzeile darf
 * danach stehen). Codeblöcke zählen nicht. Leer, wenn die Antwort nichts fragt.
 */
export function lastQuestion(answer: string, max = 160): string {
  const text = answer.replace(/```[\s\S]*?```/g, ' ')
  const lines = text.split(/\r?\n/).map(plain).filter(l => l !== '')
  for (const line of lines.slice(-3).reverse()) {
    const sentences = line.match(/[^.!?]*\?/g)
    const last = sentences?.[sentences.length - 1]?.trim()
    if (last !== undefined && last.length > 3) return oneLine(last, max)
  }
  return ''
}

/** Job nach dem Turn: Pause, Abbruch, Rückfrage (Desktop-Abschlussfrage zuerst) oder fertig. */
export function finishJob(job: Job, answer: string, isAborted: boolean, isPaused: boolean, now: number): Job {
  const question = job.ask !== '' ? job.ask : isAborted ? '' : lastQuestion(answer)
  const phase: JobPhase = isPaused ? 'paused' : isAborted ? 'stopped' : question !== '' ? 'needsYou' : 'done'
  return { ...job, phase, question, endedAt: now, signAt: now }
}

/**
 * Zeigt das Band „hängt?“: Claude arbeitet, aber seit STUCK_MS kam kein Lebenszeichen. `isBusy`: ein Tool der Hauptschleife
 * läuft gerade (langer Build, offene Freigabe), dann nie.
 */
export const isStuck = (job: Job, now: number, isBusy = false): boolean => !isBusy && job.phase === 'working' && now - job.signAt >= STUCK_MS

// Phase, wie das Band sie zeigt: `stuck` ist abgeleitet, nicht gespeichert.
export type ShownPhase = JobPhase | 'stuck'
export const shownPhase = (job: Job, now: number, isBusy = false): ShownPhase => (isStuck(job, now, isBusy) ? 'stuck' : job.phase)

/** Sichtbar ist ein Job mit Plan, mit Tool-Arbeit oder einer Rückfrage; eine schnelle Antwort ohne Tools nicht. */
export const isWorthShowing = (job: Job): boolean => job.steps.length > 0 || job.tools > 0 || job.phase === 'needsYou' || job.cmd === true

// ---- Fortschritt und Restzeit ----

export function progressOf(job: Job): { done: number; total: number } {
  return { done: job.steps.filter(s => s.status === 'done').length, total: job.steps.length }
}

const timed = (s: JobStep): number | null => (s.startedAt !== undefined && s.doneAt !== undefined ? s.doneAt - s.startedAt : null)

/** Mittlere Schrittdauer aus dem Verlauf dieses Projekts, sobald drei Jobs mit gemessenen Schritten vorliegen. */
export function avgStepMs(records: readonly JobRecord[], cwd: string): number | null {
  const mine = records.filter(r => r.cwd === cwd && r.steps.some(s => s.ms !== undefined))
  if (mine.length < 3) return null
  const all = mine.flatMap(r => r.steps.flatMap(s => (s.ms === undefined ? [] : [s.ms])))
  return all.length === 0 ? null : all.reduce((a, b) => a + b, 0) / all.length
}

/**
 * Restzeit: mittlere Dauer der erledigten Schritte (ab zwei), sonst der Verlauf, mal offene Schritte, minus die Zeit im
 * laufenden Schritt. `longer`: der laufende Schritt dauert schon doppelt so lang wie üblich, dann keine Zahl.
 */
export function etaOf(job: Job, now: number, historyAvg: number | null): { ms: number; longer: boolean } | null {
  const { done, total } = progressOf(job)
  if (total === 0 || done === total || job.phase !== 'working') return null
  const own = job.steps.flatMap(s => (timed(s) === null ? [] : [timed(s) as number]))
  const per = own.length >= 2 ? own.reduce((a, b) => a + b, 0) / own.length : historyAvg
  if (per === null) return null
  const active = job.steps.find(s => s.status === 'active')
  const inActive = active?.startedAt === undefined ? 0 : now - active.startedAt
  if (inActive > 2 * per) return { ms: 0, longer: true }
  return { ms: Math.max(0, per * (total - done) - Math.min(inActive, per)), longer: false }
}

/** Erwartete Dauer eines noch offenen Schritts (Plan-Karte), wie in etaOf. */
export function stepGuessMs(job: Job, historyAvg: number | null): number | null {
  const own = job.steps.flatMap(s => (timed(s) === null ? [] : [timed(s) as number]))
  return own.length >= 2 ? own.reduce((a, b) => a + b, 0) / own.length : historyAvg
}

export type StepRow = { kind: 'step'; step: JobStep; index: number } | { kind: 'fold'; n: number; isDone: boolean }

/**
 * Schritte fürs Band: bis FOLD_AT alle; darüber erledigte als eine Zeile, dann der laufende (oder nächste) und einer
 * danach, der Rest als „… n weitere“. Die Plan-Karte im Panel zeigt immer alle.
 */
export function visibleSteps(steps: readonly JobStep[], max = FOLD_AT): StepRow[] {
  const all: StepRow[] = steps.map((step, index) => ({ kind: 'step', step, index }))
  if (steps.length <= max) return all
  const done = steps.filter(s => s.status === 'done').length
  if (done === steps.length) return [{ kind: 'fold', n: done, isDone: true }]
  const focus = Math.max(0, steps.findIndex(s => s.status !== 'done'))
  const shown = steps.slice(focus, focus + 2).map((step, i): StepRow => ({ kind: 'step', step, index: focus + i }))
  const after = steps.length - focus - shown.length
  return [
    ...(done > 0 ? [{ kind: 'fold', n: done, isDone: true } as const] : []),
    ...shown,
    ...(after > 0 ? [{ kind: 'fold', n: after, isDone: false } as const] : []),
  ]
}

/** Auftrag für „Weiter“: dieselbe Aufgabe mit den offenen Schritten. */
export function continuePrompt(job: Job, lang: UiLang): string {
  const open = job.steps.filter(s => s.status !== 'done').map(s => s.name)
  return t(lang, 'glance.continuePrompt', { title: job.title, open: open.length === 0 ? '' : t(lang, 'glance.openSteps', { list: open.join('; ') }) })
}

// ---- Verlauf ----

const pad = (n: number) => String(n).padStart(2, '0')
/** Lokaler Tag „JJJJ-MM-TT“. */
export function dayKey(ts: number): string {
  const d = new Date(ts)
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`
}
export const storeKey = (day: string): string => `${HISTORY_PREFIX}${day}`
/** Tag um n Tage verschoben (lokal, mittags gerechnet, damit die Sommerzeit nicht verrutscht). */
export function shiftDay(day: string, n: number): string {
  const [y, m, d] = day.split('-').map(Number)
  return dayKey(new Date(y ?? 1970, (m ?? 1) - 1, (d ?? 1) + n, 12).getTime())
}
/** Verlaufsschlüssel älter als HISTORY_DAYS (zum Löschen). */
export function staleKeys(keys: readonly string[], today: string): string[] {
  const oldest = shiftDay(today, -HISTORY_DAYS)
  return keys.filter(k => k.startsWith(HISTORY_PREFIX) && k.slice(HISTORY_PREFIX.length) < oldest)
}

/** Job als Verlaufseintrag; ein laufender oder pausierter Job zählt als gestoppt. */
export function toRecord(job: Job, now: number): JobRecord {
  const outcome = job.phase === 'done' || job.phase === 'needsYou' ? job.phase : 'stopped'
  return {
    id: job.id,
    title: job.title,
    cwd: job.cwd,
    startedAt: job.startedAt,
    endedAt: job.endedAt ?? now,
    outcome,
    question: job.question,
    tools: job.tools,
    steps: job.steps.map(s => ({ name: s.name, done: s.status === 'done', ...(timed(s) === null ? {} : { ms: timed(s) as number }) })),
    files: job.files,
  }
}

export function recordsOf(v: unknown): JobRecord[] {
  return Array.isArray(v) ? v.filter((r): r is JobRecord => typeof r === 'object' && r !== null && typeof (r as JobRecord).id === 'string' && Array.isArray((r as JobRecord).steps)) : []
}

/** Eintrag ersetzen (gleiche id) oder anhängen. */
export function addRecord(list: readonly JobRecord[], rec: JobRecord): JobRecord[] {
  const rest = list.filter(r => r.id !== rec.id)
  return [...rest, rec].slice(-RECORDS_PER_DAY)
}

// ---- Berichte (Klartext zum Kopieren; keine Tokens, Modelle oder Dateinamen) ----

const dateLabel = (day: string, lang: UiLang): string => {
  const [y, m, d] = day.split('-')
  return lang === 'de' ? `${d}.${m}.${y}` : day
}
const projectName = (cwd: string): string => cwd.replace(/\\/g, '/').split('/').filter(Boolean).pop() ?? cwd
const spent = (rs: readonly JobRecord[]): number => rs.reduce((a, r) => a + Math.max(0, r.endedAt - r.startedAt), 0)
// Schnelle Fragen ohne Tools gehören in keinen Bericht.
const counts = (r: JobRecord): boolean => r.steps.length > 0 || r.tools > 0

export function dayReport(records: readonly JobRecord[], day: string, cwd: string, lang: UiLang): string {
  const rs = records.filter(r => r.cwd === cwd && counts(r))
  const head = t(lang, 'glance.r.dayHead', { project: projectName(cwd), date: dateLabel(day, lang) })
  if (rs.length === 0) return `${head}\n\n${t(lang, 'glance.r.none')}`
  const done = rs.flatMap(r => {
    const steps = r.steps.filter(s => s.done).map(s => s.name)
    return steps.length === 0 && r.outcome !== 'done' ? [] : [`• ${r.title}${steps.length === 0 ? '' : `: ${steps.join('; ')}`}`]
  })
  const open = rs.flatMap(r => {
    const steps = r.steps.filter(s => !s.done).map(s => s.name)
    return r.outcome === 'done' || steps.length === 0 ? [] : [`• ${r.title}: ${steps.join('; ')}`]
  })
  const decide = rs.flatMap(r => (r.outcome === 'needsYou' && r.question !== '' ? [`• ${r.title}: ${r.question}`] : []))
  const block = (key: Key, lines: string[]) => (lines.length === 0 ? [] : ['', t(lang, key), ...lines])
  return [head, ...block('glance.r.done', done), ...block('glance.r.open', open), ...block('glance.r.decide', decide), '', t(lang, 'glance.r.time', { d: duration(spent(rs)), n: rs.length })].join('\n')
}

/** Woche über alle Projekte: die sieben Tage bis `endDay`. `byDay`: Verlauf je Tag (Schlüssel JJJJ-MM-TT). */
export function weekReport(byDay: Readonly<Record<string, readonly JobRecord[]>>, endDay: string, lang: UiLang): string {
  const days = Array.from({ length: 7 }, (_, i) => shiftDay(endDay, i - 6))
  const head = t(lang, 'glance.r.weekHead', { from: dateLabel(days[0] as string, lang), to: dateLabel(endDay, lang) })
  const rs = days.flatMap(d => (byDay[d] ?? []).filter(counts))
  if (rs.length === 0) return `${head}\n\n${t(lang, 'glance.r.none')}`
  const steps = rs.reduce((a, r) => a + r.steps.filter(s => s.done).length, 0)
  const projects = new Set(rs.map(r => r.cwd)).size
  const perDay = days.map(d => ({ d, n: (byDay[d] ?? []).filter(counts).length }))
  const busiest = perDay.reduce((a, b) => (b.n > a.n ? b : a))
  const weekday = t(lang, `glance.day.${new Date(`${busiest.d}T12:00:00`).getDay()}` as Key)
  const biggest = [...rs].sort((a, b) => b.steps.length - a.steps.length).filter(r => r.steps.length > 0).slice(0, 3)
  return [
    head,
    '',
    t(lang, 'glance.r.weekSteps', { steps, jobs: rs.length, done: rs.filter(r => r.outcome === 'done').length }),
    t(lang, 'glance.r.weekTime', { d: duration(spent(rs)), p: projects }),
    t(lang, 'glance.r.busiest', { day: weekday, n: busiest.n }),
    ...(biggest.length === 0 ? [] : ['', t(lang, 'glance.r.biggest'), ...biggest.map(r => `• ${r.title} ${t(lang, 'glance.r.stepsN', { n: r.steps.length })}`)]),
  ].join('\n')
}
