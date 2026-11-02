// Tipps zu Prompt-Cache, Kontext und Limits (Idee: Dividend, Cache Bell, idle-compact): meldet sich nur, wenn eine Zahl eine
// Handlung wert ist, und nennt die Zahl dahinter. Rein lesend, jede Zahl kommt aus dem, was die Engine ohnehin meldet.
// Rein, ohne `$`.

import type { Limit, UiLang } from '../types'
import { COMPACT_AT, pace } from './core'
import type { Rate } from './rating'

export const T = {
  // Rückschreiben in den Cache ab so vielen Tokens in einer Antwort
  writeTokens: 40_000,
  // Kompaktieren vor Ablauf lohnt ab diesem Kontext, in den letzten Minuten des 1-h-Caches
  compactTokens: 80_000,
  compactWindowMs: 10 * 60_000,
  // kalter Cache kostet erst ab dieser Größe nennenswert
  coldTokens: 40_000,
  context: 60,
  contextTokens: 120_000,
  limit: 80,
  limitHard: 95,
  paceFrom: 50,
  paceOver: 10,
} as const

export type TipId = 'limitHard' | 'ctxFull' | 'cacheWrite' | 'cacheCold' | 'compactSoon' | 'limitWarn' | 'limitPace' | 'ctxHalf'
export type Tip = { id: TipId; tone: 'warn' | 'bad'; vars: Record<string, string | number> }
export type TipInput = {
  now: number
  isSubscription: boolean
  lastAt: number | null
  contextTokens?: number
  contextPercent?: number
  // Tokens, die die letzte Hauptantwort in den Cache geschrieben hat
  lastWrite?: number
  limits: readonly Limit[]
}

const LABEL: Record<string, string> = { five_hour: '5h', seven_day: '7d' }
const limitName = (l: Limit): string => LABEL[l.kind] ?? l.kind

// Alle zutreffenden Tipps, wichtigster zuerst.
export function tipsOf(i: TipInput): Tip[] {
  const out: Tip[] = []
  const tokens = i.contextTokens ?? 0
  const ttl = i.isSubscription ? 3600_000 : 300_000
  const left = i.lastAt === null ? 0 : Math.max(0, i.lastAt + ttl - i.now)
  const worst = [...i.limits].sort((a, b) => b.percentUsed - a.percentUsed)[0]
  if (worst !== undefined && worst.percentUsed >= T.limitHard) out.push({ id: 'limitHard', tone: 'bad', vars: { name: limitName(worst), n: Math.round(worst.percentUsed) } })
  if ((i.contextPercent ?? 0) >= COMPACT_AT) out.push({ id: 'ctxFull', tone: 'bad', vars: { n: Math.round(i.contextPercent ?? 0) } })
  if ((i.lastWrite ?? 0) >= T.writeTokens) out.push({ id: 'cacheWrite', tone: 'warn', vars: { n: Math.round((i.lastWrite ?? 0) / 1000) } })
  if (i.lastAt !== null && left === 0 && tokens >= T.coldTokens) out.push({ id: 'cacheCold', tone: 'warn', vars: { n: Math.round(tokens / 1000) } })
  if (i.isSubscription && tokens >= T.compactTokens && left > 0 && left <= T.compactWindowMs) out.push({ id: 'compactSoon', tone: 'warn', vars: { min: Math.max(1, Math.round(left / 60_000)), n: Math.round(tokens / 1000) } })
  if (worst !== undefined && worst.percentUsed >= T.limit && worst.percentUsed < T.limitHard) out.push({ id: 'limitWarn', tone: 'warn', vars: { name: limitName(worst), n: Math.round(worst.percentUsed) } })
  for (const l of i.limits) {
    const p = pace(l, i.now)
    if (l.percentUsed >= T.paceFrom && l.percentUsed < T.limit && p !== undefined && p > T.paceOver) {
      out.push({ id: 'limitPace', tone: 'warn', vars: { name: limitName(l), n: Math.round(l.percentUsed), d: p } })
      break
    }
  }
  const ctx = i.contextPercent ?? 0
  if (ctx < COMPACT_AT && (ctx >= T.context || tokens >= T.contextTokens)) out.push({ id: 'ctxHalf', tone: 'warn', vars: { n: Math.round(ctx), t: Math.round(tokens / 1000) } })
  return out
}

// ── Texte ──

const de = {
  'tip.limitHard': '{name}-Limit bei {n} %: committe und schreibe eine Statusnotiz, bevor du an die Wand läufst.',
  'tip.ctxFull': 'Kontext bei {n} %: jeder Request liest alles erneut. Jetzt /compact oder neu beginnen.',
  'tip.cacheWrite': 'Die letzte Antwort hat {n}k Tokens neu in den Cache geschrieben. Üblich nach Pause über der Cache-Zeit, Modell-/Effort-Wechsel oder geändertem Anfang (neuer MCP-Server, Tool, Skill).',
  'tip.cacheCold': 'Der Cache ist abgelaufen: die nächste Nachricht schreibt {n}k Tokens neu. Vorher kompaktieren ist günstiger.',
  'tip.compactSoon': 'Cache läuft in ~{min} min ab, Kontext {n}k Tokens: jetzt kompaktieren, solange es noch günstig ist.',
  'tip.limitWarn': '{name}-Limit bei {n} %: größere Aufträge zurückstellen oder auf ein kleineres Modell wechseln.',
  'tip.limitPace': '{name}-Limit bei {n} % und {d} Punkte schneller als der Takt: bei diesem Tempo reicht das Fenster nicht bis zum Reset.',
  'tip.ctxHalf': 'Kontext bei {n} % ({t}k Tokens): bei einem natürlichen Einschnitt (Commit, grüner Test) kompaktieren.',
  'tip.band': 'Tipp: {text}',
  'tip.short.limitHard': '{name} {n} %',
  'tip.short.ctxFull': 'Kontext {n} %',
  'tip.short.cacheWrite': 'Cache neu {n}k',
  'tip.short.cacheCold': 'Cache kalt',
  'tip.short.compactSoon': 'Cache ~{min} min',
  'tip.short.limitWarn': '{name} {n} %',
  'tip.short.limitPace': '{name} zu schnell',
  'tip.short.ctxHalf': 'Kontext {n} %',
  'tip.cardTitle': 'Tipps',
  'tip.none': 'Im Moment nichts zu tun: Cache, Kontext und Limits sind unauffällig.',
  'tip.hint': 'Nur Lesen. Schwellen: Cache-Schreiben ab {w}k Tokens, Kompaktieren vor Ablauf ab {c}k Tokens in den letzten {m} min, Kontext ab {x} %, Limit ab {l} %.',
} as const
const en = {
  'tip.limitHard': '{name} limit at {n}%: commit and write a status note before you hit the wall.',
  'tip.ctxFull': 'Context at {n}%: every request re-reads everything. Run /compact now or start over.',
  'tip.cacheWrite': 'The last reply wrote {n}k tokens into the cache again. Usual after a break past the cache time, a model/effort change or a changed start (new MCP server, tool, skill).',
  'tip.cacheCold': 'The cache has expired: the next message rewrites {n}k tokens. Compacting first is cheaper.',
  'tip.compactSoon': 'Cache expires in ~{min} min, context {n}k tokens: compact now while it is still cheap.',
  'tip.limitWarn': '{name} limit at {n}%: postpone big jobs or switch to a smaller model.',
  'tip.limitPace': '{name} limit at {n}% and {d} points ahead of pace: at this rate the window will not last until the reset.',
  'tip.ctxHalf': 'Context at {n}% ({t}k tokens): compact at a natural break (commit, green test).',
  'tip.band': 'Tip: {text}',
  'tip.short.limitHard': '{name} {n}%',
  'tip.short.ctxFull': 'context {n}%',
  'tip.short.cacheWrite': 'cache rewritten {n}k',
  'tip.short.cacheCold': 'cache cold',
  'tip.short.compactSoon': 'cache ~{min} min',
  'tip.short.limitWarn': '{name} {n}%',
  'tip.short.limitPace': '{name} too fast',
  'tip.short.ctxHalf': 'context {n}%',
  'tip.cardTitle': 'Tips',
  'tip.none': 'Nothing to do right now: cache, context and limits look fine.',
  'tip.hint': 'Read-only. Thresholds: cache write from {w}k tokens, compact before expiry from {c}k tokens in the last {m} min, context from {x}%, limit from {l}%.',
} as const satisfies Record<keyof typeof de, string>
export const TIP_TEXT = { de, en } as const
type Key = keyof typeof de

export function tx(L: UiLang, key: Key, vars?: Record<string, string | number>): string {
  const text: string = TIP_TEXT[L][key]
  return vars === undefined ? text : text.replace(/\{(\w+)\}/g, (m, name: string) => (name in vars ? String(vars[name]) : m))
}

export const tipText = (tip: Tip, L: UiLang): string => tx(L, `tip.${tip.id}`, tip.vars)
export const tipShort = (tip: Tip, L: UiLang): string => tx(L, `tip.short.${tip.id}`, tip.vars)

// ── Anzeige ──

export type TipSection = { title: string; rows: { label: string; value: string; tone?: 'ok' | 'warn' | 'bad' | 'dim'; rate?: Rate }[]; note?: string }

export function bandRate(tips: readonly Tip[]): Rate | undefined {
  const first = tips[0]
  return first === undefined ? undefined : first.tone === 'bad' ? 'bad' : 'poor'
}

export function bandText(tips: readonly Tip[], L: UiLang): string {
  const first = tips[0]
  return first === undefined ? '' : tx(L, 'tip.band', { text: tipShort(first, L) })
}

export function paneView(tips: readonly Tip[], L: UiLang): TipSection[] {
  const note = tx(L, 'tip.hint', { w: T.writeTokens / 1000, c: T.compactTokens / 1000, m: T.compactWindowMs / 60_000, x: T.context, l: T.limit })
  if (tips.length === 0) return [{ title: tx(L, 'tip.cardTitle'), rows: [{ label: '', value: tx(L, 'tip.none'), tone: 'ok' }], note }]
  return [{ title: tx(L, 'tip.cardTitle'), rows: tips.map(x => ({ label: tipShort(x, L), value: tipText(x, L), tone: x.tone })), note }]
}
