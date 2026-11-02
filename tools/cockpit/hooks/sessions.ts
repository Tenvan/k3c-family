// Reiter Sessions: liest die Status-Datei der Schaltzentrale (`<UserHome>/.deltra/claude/sessions.json`) und macht daraus Zeilen für die
// Session-Übersicht und die Freigabe-Inbox. Rein ohne `$`, damit Parser und Sortierung direkt testbar sind; das Mod liest
// die Datei nur, es schreibt nie.
import type { ApprovalAction, ApprovalRow, SessionRow, SessionStand, SessionsView } from '../types'

// Standardpfad relativ zum Benutzerordner; die Einstellung `sessionsFile` hat Vorrang. Der Benutzerordner kommt zur Laufzeit
// aus der Umgebung (`USERPROFILE`, sonst `HOME`), nie hartcodiert.
export const SESSIONS_FILE_REL = '.deltra/claude/sessions.json'

export function sessionsFilePath(override: unknown, home: string): string {
  if (typeof override === 'string' && override.trim() !== '') return override.trim()
  const base = home.replace(/[\\/]+$/, '').replace(/\\/g, '/')
  return base === '' ? SESSIONS_FILE_REL : `${base}/${SESSIONS_FILE_REL}`
}

export const STANDS: readonly SessionStand[] = ['läuft', 'wartet', 'fertig', 'archiviert']
export const ACTIONS: readonly ApprovalAction[] = ['commit', 'push', 'löschen', 'archivieren', 'sonstiges']

// Sortierung der Übersicht: wer wartet, steht oben; danach läuft, fertig, archiviert (je Gruppe in Dateireihenfolge).
const STAND_ORDER: Record<SessionStand | '', number> = { wartet: 0, läuft: 1, fertig: 2, archiviert: 3, '': 4 }

const isObj = (v: unknown): v is Record<string, unknown> => typeof v === 'object' && v !== null && !Array.isArray(v)

// Ein Feld als Text; Zahlen und Wahrheitswerte werden gedruckt, alles andere (fehlend, null, Objekt) ist leer.
const text = (v: unknown): string => (typeof v === 'string' ? v.trim() : typeof v === 'number' || typeof v === 'boolean' ? String(v) : '')

// Platzhalter für „niemand“: leer und die gängigen Striche.
const NOBODY = /^(|[-–—]|niemand|keiner|none)$/i
export const isWaiting = (waitingFor: string, stand: string): boolean => stand === 'wartet' || !NOBODY.test(waitingFor.trim())

export const standOf = (raw: string): SessionStand | '' => STANDS.find(s => s === raw.trim().toLowerCase()) ?? ''
export const actionOf = (raw: string): ApprovalAction => ACTIONS.find(a => a === raw.trim().toLowerCase()) ?? 'sonstiges'

// Lesefehler und Formatfehler als Klartext-Schlüssel für die Anzeige (`sessions.err.<kind>`).
export type SessionsErrorKind = 'missing' | 'empty' | 'json' | 'shape'
export type SessionsResult = { ok: true; view: SessionsView } | { ok: false; kind: SessionsErrorKind; detail: string }

// `raw` = Dateiinhalt, `null` = Datei nicht lesbar (`detail` nennt dann die Ursache des Lesens).
export function parseSessions(raw: string | null, detail = ''): SessionsResult {
  if (raw === null) return { ok: false, kind: 'missing', detail }
  const src = (raw.charCodeAt(0) === 0xfeff ? raw.slice(1) : raw).trim()
  if (src === '') return { ok: false, kind: 'empty', detail: '' }
  let data: unknown
  try {
    data = JSON.parse(src)
  } catch (err) {
    return { ok: false, kind: 'json', detail: err instanceof Error ? err.message : String(err) }
  }
  if (!isObj(data)) return { ok: false, kind: 'shape', detail: Array.isArray(data) ? 'Array' : typeof data }

  const issues: string[] = []
  const list = (key: 'sessions' | 'approvals'): unknown[] => {
    const v = data[key]
    if (v === undefined) issues.push(`${key}: fehlt`)
    else if (!Array.isArray(v)) issues.push(`${key}: kein Array`)
    else return v
    return []
  }
  const updated = text(data.updated)
  if (updated === '') issues.push('updated: fehlt')

  const sessions: SessionRow[] = []
  list('sessions').forEach((x, i) => {
    if (!isObj(x)) return void issues.push(`sessions[${i}]: kein Objekt`)
    const missing = (['id', 'repo', 'branch', 'state'] as const).filter(k => text(x[k]) === '')
    if (missing.length > 0) issues.push(`sessions[${i}]: ${missing.join(', ')} fehlt`)
    const stateRaw = text(x.state)
    if (stateRaw !== '' && standOf(stateRaw) === '') issues.push(`sessions[${i}]: state „${stateRaw}“ unbekannt`)
    sessions.push({ id: text(x.id), title: text(x.title), repo: text(x.repo), branch: text(x.branch), state: standOf(stateRaw), stateRaw, waitingFor: text(x.waitingFor), note: text(x.note) })
  })

  const approvals: ApprovalRow[] = []
  list('approvals').forEach((x, i) => {
    if (!isObj(x)) return void issues.push(`approvals[${i}]: kein Objekt`)
    const missing = (['id', 'action', 'detail'] as const).filter(k => text(x[k]) === '')
    if (missing.length > 0) issues.push(`approvals[${i}]: ${missing.join(', ')} fehlt`)
    const actionRaw = text(x.action)
    approvals.push({ id: text(x.id), action: actionOf(actionRaw), actionRaw, detail: text(x.detail), since: text(x.since) })
  })

  // Sortierung stabil über den Index: Array.prototype.sort ist es seit ES2019, der Index macht es für den Test explizit.
  const rank = (s: SessionRow) => (isWaiting(s.waitingFor, s.state) ? -1 : STAND_ORDER[s.state])
  const ordered = sessions.map((s, i) => ({ s, i })).sort((a, b) => rank(a.s) - rank(b.s) || a.i - b.i).map(x => x.s)
  return { ok: true, view: { updated, sessions: ordered, approvals, issues } }
}
