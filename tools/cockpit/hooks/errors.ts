// Fehler-Eingang (Idee: Error Inbox aus den Community-Mods): die letzten fehlgeschlagenen oder abgelehnten Tool-Aufrufe der
// Session an einem Ort, mit Wiederholungszähler. Nur Beobachtung, nichts wird gespeichert (Speicher der Session).
// Rein, ohne `$`: die Hooks in register.tsx rufen `noteError`/`noteCall`, Anzeige und Bericht kommen von hier.

import type { UiLang } from '../types'
import { oneLine } from './core'
import { rate, type Rate } from './rating'

export const MAX_ERRORS = 30
export const TEXT_MAX = 160
// Fehler der letzten Minuten fließen in die Ampel ein.
export const RECENT_MS = 10 * 60_000

export type ErrKind = 'error' | 'denied'
export type ErrEntry = { n: number; at: number; tool: string; target: string; kind: ErrKind; text: string; agent: boolean; times: number; id?: string }
export type ErrState = { list: ErrEntry[]; seq: number; calls: number }

export const newErrors = (): ErrState => ({ list: [], seq: 0, calls: 0 })

// Jeder Tool-Aufruf zählt, damit „n von m“ im Bericht steht.
export function noteCall(st: ErrState): void {
  st.calls++
}

const rec = (v: unknown): Record<string, unknown> => (typeof v === 'object' && v !== null ? (v as Record<string, unknown>) : {})

// Erste aussagekräftige Zeile des Fehlers: `deny`-Grund, sonst Text oder Ergebnis des Aufrufs; Steuerzeichen raus, gekürzt.
export function errorText(answer: unknown): string {
  const a = rec(answer)
  const raw = typeof a.deny === 'string' ? a.deny : typeof a.text === 'string' ? a.text : typeof a.result === 'string' ? a.result : typeof rec(a.result).stderr === 'string' ? String(rec(a.result).stderr) : ''
  const line = raw
    .split(/\r?\n/)
    .map(l => l.replace(/[\u0000-\u001f\u007f]/g, ' ').trim())
    .find(l => l !== '' && !/^(exit code|error:?)\s*\d*$/i.test(l))
  return oneLine(line ?? raw, TEXT_MAX)
}

// Neuer Eintrag; derselbe Fehler (Tool, Ziel, Text) direkt nach sich selbst zählt nur hoch statt eine Zeile zu füllen.
export function noteError(st: ErrState, e: { at: number; tool: string; target: string; kind: ErrKind; text: string; agent: boolean; id?: string }): void {
  const last = st.list[0]
  if (last !== undefined && last.tool === e.tool && last.target === e.target && last.kind === e.kind && last.text === e.text) {
    last.times++
    last.at = e.at
    // Der jüngste Aufruf trägt die Wiederholungen (Protokoll hängt sie an seine Zeile).
    if (e.id !== undefined) last.id = e.id
    return
  }
  st.seq++
  st.list.unshift({ n: st.seq, ...e, times: 1 })
  if (st.list.length > MAX_ERRORS) st.list.length = MAX_ERRORS
}

export const recentCount = (st: ErrState, now: number): number => st.list.filter(x => now - x.at <= RECENT_MS).reduce((n, x) => n + x.times, 0)

// ── Texte ──

const de = {
  'err.band': 'Fehler: {n} in 10 min',
  'err.cardTitle': 'Fehler-Eingang',
  'err.title.summary': 'Überblick',
  'err.title.list': 'Letzte Fehler',
  'err.label.total': 'Fehler gesamt',
  'err.label.recent': 'letzte 10 min',
  'err.label.calls': 'Aufrufe',
  'err.label.top': 'häufigstes Tool',
  'err.kind.error': 'Fehler',
  'err.kind.denied': 'abgelehnt',
  'err.times': '×{n}',
  'err.agent': 'Subagent',
  'err.none': 'Bisher kein Fehler in dieser Session.',
  'err.hint': 'Nur diese Session, im Speicher; nach einem Reload beginnt die Liste von vorn. Abgelehnt = Freigabe verweigert oder Wächter.',
  'err.report.head': 'Fehler-Eingang: {n} Einträge, {c} Aufrufe',
} as const
const en = {
  'err.band': 'Errors: {n} in 10 min',
  'err.cardTitle': 'Error inbox',
  'err.title.summary': 'Overview',
  'err.title.list': 'Latest errors',
  'err.label.total': 'errors in total',
  'err.label.recent': 'last 10 min',
  'err.label.calls': 'calls',
  'err.label.top': 'most frequent tool',
  'err.kind.error': 'error',
  'err.kind.denied': 'denied',
  'err.times': '×{n}',
  'err.agent': 'subagent',
  'err.none': 'No error in this session so far.',
  'err.hint': 'This session only, in memory; the list starts over after a reload. Denied = permission refused or guard.',
  'err.report.head': 'Error inbox: {n} entries, {c} calls',
} as const satisfies Record<keyof typeof de, string>
export const ERR_TEXT = { de, en } as const
type Key = keyof typeof de

export function tx(L: UiLang, key: Key, vars?: Record<string, string | number>): string {
  const text: string = ERR_TEXT[L][key]
  return vars === undefined ? text : text.replace(/\{(\w+)\}/g, (m, name: string) => (name in vars ? String(vars[name]) : m))
}

// ── Anzeige ──

export type ErrSection = { title: string; rows: { label: string; value: string; tone?: 'ok' | 'warn' | 'bad' | 'dim'; rate?: Rate }[]; note?: string }

// Ampel nach der Zahl der Fehler in den letzten Minuten (Grenzen 0 / 2 / 5 wie bei „unaufgelöst“).
export function bandRate(st: ErrState, now: number): Rate | undefined {
  const n = recentCount(st, now)
  return n === 0 ? undefined : rate('unresolved', n)
}

export function bandText(st: ErrState, now: number, L: UiLang): string {
  const n = recentCount(st, now)
  return n === 0 ? '' : tx(L, 'err.band', { n })
}

export function topTool(st: ErrState): string {
  const by = new Map<string, number>()
  for (const x of st.list) by.set(x.tool, (by.get(x.tool) ?? 0) + x.times)
  return [...by.entries()].sort((a, b) => b[1] - a[1])[0]?.[0] ?? ''
}

const clock = (at: number): string => {
  const d = new Date(at)
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

export function entryLine(x: ErrEntry, L: UiLang): string {
  const mark = x.kind === 'denied' ? `${tx(L, 'err.kind.denied')} · ` : ''
  const times = x.times > 1 ? ` ${tx(L, 'err.times', { n: x.times })}` : ''
  const who = x.agent ? ` (${tx(L, 'err.agent')})` : ''
  return `${mark}${x.target === '' ? '' : `${x.target} – `}${x.text === '' ? tx(L, 'err.kind.error') : x.text}${times}${who}`
}

export function paneView(st: ErrState, now: number, L: UiLang): ErrSection[] {
  if (st.list.length === 0) return [{ title: tx(L, 'err.title.summary'), rows: [{ label: '', value: tx(L, 'err.none'), tone: 'dim' }] }]
  const total = st.list.reduce((n, x) => n + x.times, 0)
  const recent = recentCount(st, now)
  return [
    {
      title: tx(L, 'err.title.summary'),
      rows: [
        { label: tx(L, 'err.label.total'), value: String(total) },
        { label: tx(L, 'err.label.recent'), value: String(recent), rate: recent === 0 ? 'great' : rate('unresolved', recent) },
        { label: tx(L, 'err.label.calls'), value: String(st.calls), tone: 'dim' },
        { label: tx(L, 'err.label.top'), value: topTool(st) },
      ],
    },
    {
      title: tx(L, 'err.title.list'),
      rows: st.list.map(x => ({ label: `${clock(x.at)} ${x.tool}`, value: entryLine(x, L), tone: x.kind === 'denied' ? ('warn' as const) : ('bad' as const) })),
      note: tx(L, 'err.hint'),
    },
  ]
}

export function reportText(st: ErrState, now: number, L: UiLang): string {
  const lines = [tx(L, 'err.report.head', { n: st.list.length, c: st.calls })]
  for (const s of paneView(st, now, L)) {
    lines.push('', s.title)
    for (const r of s.rows) lines.push(r.label === '' ? `  ${r.value}` : `  ${r.label}: ${r.value}`)
  }
  return lines.join('\n')
}
