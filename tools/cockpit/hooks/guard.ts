// Nutzungswächter (Idee: agent-usage-guard): erkennt die drei Dinge, die ein Nutzungsfenster am schnellsten leeren, und meldet
// sie, bevor es teuer wird: ein Schwall von Subagents, ein Prompt auf ein fast volles Kontextfenster, derselbe Tool-Aufruf
// immer wieder. Standard ist Warnen (Band, Panel, Hinweis); nur mit der Einstellung „Wächter hält Agents an“ lehnt er den
// Start eines weiteren Subagents ab, wenn zu viele laufen. Rein, ohne `$`.

import type { UiLang } from '../types'
import { oneLine } from './core'
import type { Rate } from './rating'

export const LIMITS = {
  // gleichzeitig laufende Subagents: ab hier Warnung, ab HOLD wird (mit Einstellung) abgelehnt
  agents: 8,
  hold: 12,
  // neue Subagents je Minute
  burst: 10,
  // Prompt bei so vollem Kontextfenster (Prozent): jeder Request liest alles erneut
  heavy: 85,
  // gleiche Aufrufe hintereinander
  retries: 3,
  // dieselbe Regel meldet sich höchstens alle 5 Minuten erneut
  quietMs: 5 * 60_000,
  // Meldungen bleiben so lange im Band
  showMs: 10 * 60_000,
} as const

export type GuardRule = 'agents' | 'burst' | 'heavy' | 'retry'
export type GuardAlert = { rule: GuardRule; at: number; n: number; detail: string }
export type GuardState = {
  spawns: number[]
  alerts: GuardAlert[]
  last: { key: string; n: number }
  said: Partial<Record<GuardRule, number>>
  held: number
}

export const newGuard = (): GuardState => ({ spawns: [], alerts: [], last: { key: '', n: 0 }, said: {}, held: 0 })

// Eine Regel meldet sich nicht öfter als `quietMs`; liefert die Meldung nur, wenn sie neu ist (dann auch ein Hinweis).
function raise(st: GuardState, rule: GuardRule, at: number, n: number, detail: string): GuardAlert | null {
  const prev = st.said[rule]
  if (prev !== undefined && at - prev < LIMITS.quietMs) return null
  st.said[rule] = at
  const alert = { rule, at, n, detail }
  st.alerts.unshift(alert)
  if (st.alerts.length > 20) st.alerts.length = 20
  return alert
}

// Neuer Subagent; `live`: wie viele laufen schon. Meldet bei zu vielen gleichzeitig und bei zu vielen je Minute.
export function noteSpawn(st: GuardState, at: number, live: number): GuardAlert[] {
  st.spawns = st.spawns.filter(x => at - x < 60_000)
  st.spawns.push(at)
  const out: GuardAlert[] = []
  if (live + 1 >= LIMITS.agents) {
    const a = raise(st, 'agents', at, live + 1, '')
    if (a !== null) out.push(a)
  }
  if (st.spawns.length >= LIMITS.burst) {
    const a = raise(st, 'burst', at, st.spawns.length, '')
    if (a !== null) out.push(a)
  }
  return out
}

// Soll ein weiterer Subagent abgelehnt werden? Nur mit der Einstellung; zählt das Ablehnen mit.
export function shouldHold(st: GuardState, live: number, enabled: boolean): boolean {
  if (!enabled || live < LIMITS.hold) return false
  st.held++
  return true
}

// Prompt auf ein fast volles Fenster: jeder weitere Request liest den ganzen Kontext erneut.
export function notePrompt(st: GuardState, at: number, contextPercent: number | undefined): GuardAlert | null {
  return contextPercent !== undefined && contextPercent >= LIMITS.heavy ? raise(st, 'heavy', at, Math.round(contextPercent), '') : null
}

const stable = (v: unknown): string => {
  if (typeof v !== 'object' || v === null) return JSON.stringify(v) ?? ''
  if (Array.isArray(v)) return `[${v.map(stable).join(',')}]`
  const o = v as Record<string, unknown>
  return `{${Object.keys(o).sort().map(k => `${JSON.stringify(k)}:${stable(o[k])}`).join(',')}}`
}

// Derselbe Aufruf (Tool und Argumente) mehrmals direkt hintereinander: eine Wiederholung ohne Aussicht auf ein anderes Ergebnis.
export function noteCall(st: GuardState, at: number, tool: string, args: unknown, label: string): GuardAlert | null {
  const key = `${tool}\u0000${stable(args)}`
  st.last = st.last.key === key ? { key, n: st.last.n + 1 } : { key, n: 1 }
  return st.last.n >= LIMITS.retries ? raise(st, 'retry', at, st.last.n, oneLine(label === '' ? tool : `${tool} ${label}`, 60)) : null
}

// Ein neuer Turn beginnt die Folge von vorn.
export function resetCalls(st: GuardState): void {
  st.last = { key: '', n: 0 }
}

// ── Texte ──

const de = {
  'guard.rule.agents': 'Viele Subagents gleichzeitig ({n}; ab {max} wird gewarnt)',
  'guard.rule.burst': 'Schwall neuer Subagents ({n} in einer Minute)',
  'guard.rule.heavy': 'Prompt auf {n} % Kontext: jeder Request liest alles erneut',
  'guard.rule.retry': 'Derselbe Aufruf {n}× hintereinander: {detail}',
  'guard.advice.agents': 'Weniger parallel starten oder den Rest nacheinander laufen lassen.',
  'guard.advice.burst': 'Eine Aufteilung in kleinere Aufträge oder ein Limit setzen, bevor das Fenster leerläuft.',
  'guard.advice.heavy': 'Erst kompaktieren (/compact) oder mit /clear neu beginnen.',
  'guard.advice.retry': 'Anderen Weg wählen statt zu wiederholen; sonst frage Claude nach der Ursache.',
  'guard.band': 'Wächter: {rule}',
  'guard.band.agents': '{n} Agents',
  'guard.band.burst': '{n} Starts/min',
  'guard.band.heavy': 'Kontext {n} %',
  'guard.band.retry': '{n}× gleicher Aufruf',
  'guard.toast': 'Wächter: {text}',
  'guard.deny': 'Wächter: Es laufen schon {n} Subagents (Grenze {max}). Starte weitere erst, wenn welche fertig sind.',
  'guard.cardTitle': 'Nutzungswächter',
  'guard.title.rules': 'Regeln',
  'guard.title.alerts': 'Letzte Meldungen',
  'guard.label.agents': 'Subagents',
  'guard.label.burst': 'Starts/min',
  'guard.label.heavy': 'Kontext',
  'guard.label.retry': 'Wiederholung',
  'guard.label.held': 'abgelehnt',
  'guard.value.agents': 'warnt ab {a} gleichzeitig, lehnt ab {h} ab (nur mit Einstellung)',
  'guard.value.burst': 'warnt ab {n} neuen Subagents in einer Minute',
  'guard.value.heavy': 'warnt bei Prompt ab {n} % Kontext',
  'guard.value.retry': 'warnt beim {n}. gleichen Aufruf in Folge',
  'guard.none': 'Noch keine Meldung.',
  'guard.hint': 'Warnt nur, solange „Wächter hält Agents an“ aus ist. Zählt nur diese Session.',
} as const
const en = {
  'guard.rule.agents': 'Many subagents at once ({n}; warns from {max})',
  'guard.rule.burst': 'Burst of new subagents ({n} in one minute)',
  'guard.rule.heavy': 'Prompt at {n}% context: every request re-reads everything',
  'guard.rule.retry': 'Same call {n}× in a row: {detail}',
  'guard.advice.agents': 'Start fewer in parallel or let the rest run one after another.',
  'guard.advice.burst': 'Split into smaller jobs or set a limit before the window runs dry.',
  'guard.advice.heavy': 'Compact first (/compact) or start over with /clear.',
  'guard.advice.retry': 'Pick a different approach instead of repeating; otherwise ask Claude for the cause.',
  'guard.band': 'Guard: {rule}',
  'guard.band.agents': '{n} agents',
  'guard.band.burst': '{n} starts/min',
  'guard.band.heavy': 'context {n}%',
  'guard.band.retry': '{n}× same call',
  'guard.toast': 'Guard: {text}',
  'guard.deny': 'Guard: {n} subagents are already running (limit {max}). Start more only once some have finished.',
  'guard.cardTitle': 'Usage guard',
  'guard.title.rules': 'Rules',
  'guard.title.alerts': 'Latest alerts',
  'guard.label.agents': 'Subagents',
  'guard.label.burst': 'Starts/min',
  'guard.label.heavy': 'Context',
  'guard.label.retry': 'Repeat',
  'guard.label.held': 'refused',
  'guard.value.agents': 'warns from {a} at once, refuses from {h} (only with the setting)',
  'guard.value.burst': 'warns from {n} new subagents in one minute',
  'guard.value.heavy': 'warns on a prompt from {n}% context',
  'guard.value.retry': 'warns on the {n}th identical call in a row',
  'guard.none': 'No alert yet.',
  'guard.hint': 'Warns only while "Guard holds agents" is off. Counts this session only.',
} as const satisfies Record<keyof typeof de, string>
export const GUARD_TEXT = { de, en } as const
type Key = keyof typeof de

export function tx(L: UiLang, key: Key, vars?: Record<string, string | number>): string {
  const text: string = GUARD_TEXT[L][key]
  return vars === undefined ? text : text.replace(/\{(\w+)\}/g, (m, name: string) => (name in vars ? String(vars[name]) : m))
}

export const alertText = (a: GuardAlert, L: UiLang): string =>
  tx(L, `guard.rule.${a.rule}`, { n: a.n, max: LIMITS.agents, detail: a.detail })

export const alertAdvice = (a: GuardAlert, L: UiLang): string => tx(L, `guard.advice.${a.rule}`)

// ── Anzeige ──

export type GuardSection = { title: string; rows: { label: string; value: string; tone?: 'ok' | 'warn' | 'bad' | 'dim'; rate?: Rate }[]; note?: string }

const fresh = (st: GuardState, now: number): GuardAlert[] => st.alerts.filter(a => now - a.at <= LIMITS.showMs)

export function bandRate(st: GuardState, now: number): Rate | undefined {
  const f = fresh(st, now)
  return f.length === 0 ? undefined : f.some(a => a.rule === 'agents' && a.n >= LIMITS.hold) ? 'bad' : 'poor'
}

export function bandText(st: GuardState, now: number, L: UiLang): string {
  const a = fresh(st, now)[0]
  return a === undefined ? '' : tx(L, 'guard.band', { rule: tx(L, `guard.band.${a.rule}`, { n: a.n }) })
}

const clock = (at: number): string => {
  const d = new Date(at)
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

export function paneView(st: GuardState, now: number, L: UiLang): GuardSection[] {
  const rules: GuardSection = {
    title: tx(L, 'guard.title.rules'),
    rows: [
      { label: tx(L, 'guard.label.agents'), value: tx(L, 'guard.value.agents', { a: LIMITS.agents, h: LIMITS.hold }) },
      { label: tx(L, 'guard.label.burst'), value: tx(L, 'guard.value.burst', { n: LIMITS.burst }) },
      { label: tx(L, 'guard.label.heavy'), value: tx(L, 'guard.value.heavy', { n: LIMITS.heavy }) },
      { label: tx(L, 'guard.label.retry'), value: tx(L, 'guard.value.retry', { n: LIMITS.retries }) },
      ...(st.held > 0 ? [{ label: tx(L, 'guard.label.held'), value: String(st.held), tone: 'warn' as const }] : []),
    ],
  }
  const alerts: GuardSection = {
    title: tx(L, 'guard.title.alerts'),
    rows:
      st.alerts.length === 0
        ? [{ label: '', value: tx(L, 'guard.none'), tone: 'dim' }]
        : st.alerts.map(a => ({ label: clock(a.at), value: `${alertText(a, L)} – ${alertAdvice(a, L)}`, tone: 'warn' as const })),
    note: tx(L, 'guard.hint'),
  }
  void now
  return [alerts, rules]
}
