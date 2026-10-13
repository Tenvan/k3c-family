// Reine Rechen- und Formatfunktionen, ohne `$` – direkt testbar.
import type { About, Agent, Breakdown, BreakdownRow, CallMark, CatchUp, LoopAlert, UiLang, Limit, McpSchema, Repo, ModSettings, Step, TurnRecord } from '../types'
import { t, type Key } from './i18n'

// Alle Farben des Mods, fest eingetragen (kein Lesen externer Dateien zur Laufzeit): Primitive und dunkle Rollen eines
// Dark-Design-Systems. Keine Theme-Schlüssel des Hosts ('claude', 'success' …): die färben in dessen Farben.
export const THEME = {
  accent: '#8eadfd', // --text-link / --border-focus: Akzent für Text und Rahmen
  accentFill: '#4277fc', // --action-primary: Akzent für Flächen (Ringbögen, Balken)
  info: '#8eadfd', // = accent
  success: '#89c958', // --status-success-fg
  warning: '#e8b641', // --status-warning
  error: '#fa6e6e', // --status-error-fg
  textSecondary: '#aeb2ba', // --text-secondary
  textMuted: '#8c929c', // --text-muted
  border: '#4a5565', // --border-default / --divider
  chip: '#2f3a59', // Grund der Nummern- und Typ-Chips
  max: '#82b1ff', // Hellblau für Suche und Versionsnummern
  white: '#ffffff', // reines Weiß auf dunklem Grund
  surface: '#222222', // Grund schwebender Hinweise
  violet: '#aa8ed6', // Agent-Symbol, Hinweis „Wichtig“; wie die fünfte Kategorie-Farbe
} as const

// Farbrolle → Farbe; die Rechenfunktionen liefern Rollen, gezeichnet wird mit THEME[rolle].
export type Tone = 'accent' | 'success' | 'warning' | 'error'

// Gedimmte Kartengründe im Panel statt Rahmen: je Rolle ein dunkler Ton, neutral für gewöhnliche Karten.
export const TINT: Record<Tone | 'neutral', string> = {
  neutral: '#262b33',
  accent: '#1f2a44',
  success: '#1f2c1c',
  warning: '#33290f',
  error: '#3a1f22',
}

// Gedimmte Gründe je Panel-Karte, im Wechsel kühl/warm, damit benachbarte Karten sich absetzen.
export const CARD_TINT: Record<string, string> = {
  cardWorkdir: '#232a3a', // Navy
  cardContext: '#1e2e33', // Petrol
  cardOverview: '#2a2538', // Indigo-Violett
  cardTurn: '#1f2e2a', // Tannengrün
  cardAgents: '#2d2733', // Pflaume
  cardTasks: '#1f2b36', // Stahlblau
  cardLog: '#2e2a24', // Sand
}
// Dieselben Töne als Folge für Seiten mit gleichartigen Karten (Einstellungen, Zusammenfassung): Karte i nimmt CARD_CYCLE[i % n].
export const CARD_CYCLE: readonly string[] = ['#232a3a', '#1e2e33', '#2a2538', '#1f2e2a', '#2d2733', '#1f2b36', '#2e2a24']
export const cycleTint = (i: number): string => CARD_CYCLE[i % CARD_CYCLE.length] as string

const WINDOW_MS: Record<string, number> = {
  five_hour: 5 * 3600_000,
  seven_day: 7 * 86400_000,
}

export const LIMIT_LABEL: Record<string, string> = { five_hour: '5h', seven_day: '7d' }

// Verbrauch minus verstrichener Fensteranteil in Prozentpunkten: > 0 heißt schneller als der Takt.
export function pace(limit: Limit, now: number): number | undefined {
  const span = WINDOW_MS[limit.kind]
  if (!span || !limit.resetsAt) return undefined
  const left = Math.min(span, Math.max(0, Date.parse(limit.resetsAt) - now))
  return Math.round(limit.percentUsed - (100 * (span - left)) / span)
}

export function limitColor(limit: Limit, now: number): 'error' | 'warning' | 'success' {
  if (limit.percentUsed >= 90) return 'error'
  return (pace(limit, now) ?? 0) > 10 ? 'warning' : 'success'
}

// ponytail: TTL nur geraten (Subscription 1h, sonst 5m), Cache-Misses werden nicht erkannt
export function cacheLeftMs(lastTurnAt: number | null, isSubscription: boolean, now: number): number {
  if (lastTurnAt === null) return 0
  return Math.max(0, lastTurnAt + (isSubscription ? 3600_000 : 300_000) - now)
}

// Farblegende für Band und Panel; die Farben stammen aus `limitColor` und der Kompaktieren-Schwelle.
// Der Begriff „Takt“ kommt nur in der Erklärung vor: Anwender sollen sofort sehen, was die Farbe sagt.
// Texte als Schlüssel in i18n.ts.
export const LEGEND: readonly { color: Tone; label: Key; hint: Key }[] = [
  { color: 'success', label: 'legend.ok', hint: 'legend.okHint' },
  { color: 'warning', label: 'legend.fast', hint: 'legend.fastHint' },
  { color: 'error', label: 'legend.full', hint: 'legend.fullHint' },
  { color: 'accent', label: 'legend.agents', hint: 'legend.agentsHint' },
]

// Erklärung zu den Limits, unter der Legende im Panel und im Hover-Hinweis des Bands.
export const LEGEND_NOTE: readonly Key[] = ['legend.note1', 'legend.note2', 'legend.note3']

// Takt in Worten: positiv = der Verbrauch läuft der Zeit voraus, negativ = Reserve.
export function paceText(p: number, lang: UiLang = 'de'): string {
  return p > 0 ? t(lang, 'tip.paceFast', { n: p }) : p < 0 ? t(lang, 'tip.paceReserve', { n: -p }) : t(lang, 'tip.paceOk')
}

// Ab diesem Kontext-Füllstand bietet das Band /compact an.
export const COMPACT_AT = 85

// Anteil der Eingabe-Tokens, die aus dem Prompt-Cache kamen; null ohne Eingabe.
export function cacheHitPct(u: { input_tokens: number; cache_read_input_tokens: number; cache_creation_input_tokens: number }): number | null {
  const total = u.input_tokens + u.cache_read_input_tokens + u.cache_creation_input_tokens
  return total === 0 ? null : Math.round((100 * u.cache_read_input_tokens) / total)
}

const FILE_TOOLS = new Set(['Read', 'Edit', 'Write', 'NotebookEdit', 'Glob', 'Grep'])
const SHELL_TOOLS = new Set(['Bash', 'PowerShell'])

// Werkzeugart für die Freigaben-Karte: Datei, Shell oder Sonstige.
export function toolKind(tool: string): 'file' | 'shell' | 'other' {
  return FILE_TOOLS.has(tool) ? 'file' : SHELL_TOOLS.has(tool) ? 'shell' : 'other'
}

export const DECISION_COLOR = { allow: THEME.success, you: THEME.accent, ask: THEME.warning, deny: THEME.error } as const

export function bar(percent: number, width = 10): string {
  const full = Math.round((Math.min(100, Math.max(0, percent)) / 100) * width)
  return '▰'.repeat(full) + '▱'.repeat(width - full)
}

export function tokens(n: number): string {
  return n >= 1_000_000 ? `${(n / 1_000_000).toFixed(1)}M` : n >= 1000 ? `${Math.round(n / 1000)}k` : `${n}`
}

export function usd(n: number): string {
  return `$${n.toFixed(2)}`
}

export function duration(ms: number): string {
  const s = Math.round(ms / 1000)
  if (s < 60) return `${s}s`
  const m = Math.floor(s / 60)
  return m < 60 ? `${m}m ${s % 60}s` : `${Math.floor(m / 60)}h ${m % 60}m`
}

export function clockTime(iso: string): string {
  const d = new Date(iso)
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

// Reset-Zeitpunkt eines Limits mit Wochentag: „Di 21:00“, ab 7 Tagen Abstand zusätzlich mit Datum: „Di 21:00 (20.10.)“;
// englisch „Tue 21:00 (10/20)“. Der Wochentag kommt aus dem Locale, ohne den Punkt der deutschen Kurzform.
export function resetLabel(iso: string, now: number, lang: UiLang = 'de'): string {
  const d = new Date(iso)
  const pad = (n: number) => String(n).padStart(2, '0')
  const day = d.toLocaleDateString(lang === 'de' ? 'de-DE' : 'en-US', { weekday: 'short' }).replace(/\.$/, '')
  const base = `${day} ${pad(d.getHours())}:${pad(d.getMinutes())}`
  const date = lang === 'de' ? `${pad(d.getDate())}.${pad(d.getMonth() + 1)}.` : `${pad(d.getMonth() + 1)}/${pad(d.getDate())}`
  return d.getTime() - now >= 6 * 86400_000 ? `${base} (${date})` : base
}

// Titelleiste beider Flächen: „[Logo] Tenvan CC (Version x)“.
export function titleText(about: About | null): string {
  return about ? `Tenvan CC (Version ${about.version})` : 'Tenvan CC'
}

export const title = (about: About | null): string => titleText(about)

// Svg-Quelle aus einer Logo-Datei: ohne XML-Kopf und Kommentare; null, wenn es kein <svg> ist.
export function svgSource(text: string): string | null {
  const s = text.replace(/^\s*<\?xml[^>]*\?>/, '').replace(/<!--[\s\S]*?-->/g, '').trim()
  return s.startsWith('<svg') ? s : null
}

// Konfigurationsschlüssel, über die ein Repo beim Lesen Befehle starten kann: Filter, Dateiüberwachung, Pager, Textconv,
// externer Diff, Signaturprüfung (gpg), Aliasse.
// Includes und eine Worktree-Konfiguration zählen mit: sie laden weitere Konfiguration, die diese Prüfung nicht sieht.
export const RISKY_GIT_CONFIG = '^(filter\\..*\\.(clean|smudge|process)|core\\.(fsmonitor|pager|editor|sshcommand|hookspath)|diff\\..*\\.(command|textconv)|diff\\.external|alias\\..*|include\\..*|includeif\\..*|extensions\\.worktreeconfig|gpg\\..*|log\\.showsignature)$'

// Git-Stand aus den Ausgaben der git-Befehle; `counts` ist „behind<TAB>ahead“ oder leer ohne Upstream.
export function parseRepo(o: { top: string; gitDir: string; commonDir: string; branch: string; sha: string; status: string | null; counts: string; prefix: string }): Repo {
  const norm = (p: string) => p.replace(/\\/g, '/').replace(/\/+$/, '').toLowerCase()
  const [behind, ahead] = o.counts.split(/\s+/).map(Number)
  const hasCounts = o.counts !== '' && behind !== undefined && ahead !== undefined && !Number.isNaN(behind) && !Number.isNaN(ahead)
  return {
    name: o.top.replace(/\/+$/, '').split('/').pop() ?? o.top,
    isLinked: norm(o.gitDir) !== norm(o.commonDir),
    branch: o.branch || 'detached',
    sha: o.sha,
    dirty: o.status === null ? null : o.status === '' ? 0 : o.status.split('\n').length,
    ahead: hasCounts ? (ahead ?? null) : null,
    behind: hasCounts ? (behind ?? null) : null,
    path: o.top,
    subdir: o.prefix.replace(/\/+$/, ''),
  }
}

// Kurzform fürs Band: nur der aktuelle Branch.
export const repoShort = (r: Repo): string => `⎇ ${r.branch}`

// Ausführlich fürs Panel: Beschriftung und Wert je Zeile.
export function repoRows(r: Repo, lang: UiLang = 'de'): [label: string, value: string][] {
  const sync = r.ahead === null || r.behind === null ? t(lang, 'panel.repoNoUpstream') : t(lang, 'panel.repoSync', { ahead: r.ahead, behind: r.behind })
  const changes = r.dirty === null ? t(lang, 'panel.repoUnchecked') : r.dirty === 0 ? t(lang, 'panel.repoClean') : t(lang, 'panel.repoDirty', { n: r.dirty })
  return [
    [t(lang, 'panel.repoBranch'), r.branch],
    [t(lang, 'panel.repoWorktree'), `${r.name} (${t(lang, r.isLinked ? 'panel.repoLinked' : 'panel.repoMain')})`],
    [t(lang, 'panel.repoCommit'), r.sha],
    [t(lang, 'panel.repoChanges'), changes],
    [t(lang, 'panel.repoUpstream'), sync],
    [t(lang, 'panel.repoFolder'), r.path],
    ...(r.subdir === '' ? [] : [[t(lang, 'panel.repoSubfolder'), r.subdir] as [string, string]]),
  ]
}

// Protokoll im Flightdeck-Stil: nur die letzten LOG_MAX Einträge, je mit Quelle.
export const LOG_MAX = 10
export const SRC_COLOR = { you: undefined, main: THEME.accent, gate: THEME.error, agent: THEME.warning } as const

// Einzeilig und auf `max` Zeichen gekürzt.
export function oneLine(text: string, max = 80): string {
  const t = text.replace(/\s+/g, ' ').trim()
  return t.length > max ? `${t.slice(0, max - 1)}…` : t
}

// Ziel eines Tool-Calls für das Protokoll: Pfade auf die letzten zwei Segmente, sonst Muster/Befehl.
export function toolTarget(input: unknown): string {
  // Die Argumente liegen im Event selbst (tool.call) oder unter `input` (Aufruf per $.tool.check).
  const e = (input ?? {}) as Record<string, unknown>
  const i = (e.input ?? e) as Record<string, unknown>
  const path = [i.file_path, i.path, i.notebook_path].find(x => typeof x === 'string' && x !== '')
  if (typeof path === 'string') return path.replace(/\\/g, '/').split('/').slice(-2).join('/')
  const other = [i.pattern, i.command, i.url, i.description, i.query].find(x => typeof x === 'string' && x !== '')
  return typeof other === 'string' ? oneLine(other, 60) : ''
}

// Lade-Animation, solange ein Turn läuft: Spinner-Zeichen und ein Balken, in dem ein Block hin und her läuft.
const SPINNER = '⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
export const spinner = (frame: number): string => SPINNER[frame % SPINNER.length] ?? ''

// Ringdiagramm für das Band: Bogen-Farbe je Rolle; der Akzent als Fläche in der kräftigeren Füllfarbe.
export const RING_HEX: Record<Tone, string> = { success: THEME.success, warning: THEME.warning, error: THEME.error, accent: THEME.accentFill }

// Farbrolle des Kontext-Rings: knapp ab 90 %, voll ab COMPACT_AT, sonst Akzent.
// Ampel nach Füllgrad für Ring, Text und Kompaktieren-Knopf: bis 50 % grün, bis 70 % neutral, bis COMPACT_AT gelb, darüber rot.
export function contextColor(percent: number): Tone {
  return percent >= COMPACT_AT ? 'error' : percent >= 70 ? 'warning' : percent >= 50 ? 'accent' : 'success'
}

// SVG eines Rings: grauer Grund, darüber der gefüllte Bogen ab 12 Uhr im Uhrzeigersinn.
// `tip` wird als <title> eingebettet: die Oberfläche zeigt ihn beim Überfahren als Tooltip (Svg mit isInteractive).
export function ringSvg(percent: number, color: string, size = 28, tip = ''): string {
  const r = size / 2 - 3
  const c = 2 * Math.PI * r
  const p = Math.min(100, Math.max(0, percent))
  const mid = size / 2
  const arc =
    p > 0
      ? `<circle cx="${mid}" cy="${mid}" r="${r}" fill="none" stroke="${color}" stroke-width="4" stroke-linecap="round" stroke-dasharray="${((c * p) / 100).toFixed(2)} ${c.toFixed(2)}" transform="rotate(-90 ${mid} ${mid})"/>`
      : ''
  const title = tip === '' ? '' : `<title>${tip.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')}</title>`
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 ${size} ${size}">${title}<circle cx="${mid}" cy="${mid}" r="${r}" fill="none" stroke="#888888" stroke-opacity="0.3" stroke-width="4"/>${arc}</svg>`
}

// Tooltips der Ringe: Klartext mit allem, was zum Wert gehört.
export const limitName = (kind: string, lang: UiLang = 'de'): string =>
  kind === 'five_hour' || kind === 'seven_day' ? t(lang, `tip.limit.${kind}`) : kind

export function contextTip(u: { contextTokens?: number; contextWindow: number; contextPercent?: number }, b: Breakdown | null, lang: UiLang = 'de'): string {
  const head = b !== null ? breakdownTitle(b, lang) : t(lang, 'tip.ctxTitle', { used: tokens(u.contextTokens ?? 0), window: tokens(u.contextWindow), pct: Math.round(u.contextPercent ?? 0) })
  // Balken aus farbigen Emoji-Quadraten (je angefangene 10 %): ein <title> trägt keine Grafik, Emoji schon.
  const rows = b === null ? [] : b.rows.map((r, i) => `${categoryEmoji(r, i).repeat(Math.max(1, Math.round(r.percent / 10)))}  ${r.name}: ${tokens(r.tokens)} (${r.percent.toFixed(1)} %)`)
  return [head, ...rows, t(lang, 'tip.ctxCompact', { n: COMPACT_AT })].join('\n')
}

export function limitTip(l: Limit, now: number, lang: UiLang = 'de'): string {
  const p = pace(l, now)
  return [
    t(lang, 'tip.limitUsed', { name: limitName(l.kind, lang), pct: Math.round(l.percentUsed) }),
    p === undefined ? '' : t(lang, 'tip.pace', { text: paceText(p, lang) }),
    l.resetsAt ? t(lang, 'tip.reset', { at: resetLabel(l.resetsAt, now, lang), left: duration(Math.max(0, Date.parse(l.resetsAt) - now)) }) : '',
  ]
    .filter(x => x !== '')
    .join('\n')
}

export function cacheTip(left: number, ttl: number, hit: number | null, lastAt: number | null, now: number, lang: UiLang = 'de'): string {
  return [
    left === 0 ? t(lang, 'tip.cacheCold') : t(lang, 'tip.cacheLeft', { left: duration(left) }),
    t(lang, 'tip.cacheTtl', { ttl: duration(ttl) }),
    lastAt === null ? '' : t(lang, 'tip.cacheLast', { ago: duration(now - lastAt) }),
    hit === null ? '' : t(lang, 'tip.cacheHit', { n: hit }),
  ]
    .filter(x => x !== '')
    .join('\n')
}

// Kontext-Aufteilung für den Tooltip: ohne nachgeladene (deferred) und leere Zeilen, Prozent auf eine Nachkommastelle.
// Belegte Zeilen absteigend, dann der Puffer, freier Platz immer ganz unten.
const ORDER: Record<BreakdownRow['kind'], number> = { used: 0, buffer: 1, free: 2 }
export function toBreakdown(b: {
  categories: readonly { name: string; tokens: number; kind: string }[]
  totalTokens: number
  rawMaxTokens: number
  percentage: number
}): Breakdown {
  const rows = b.categories
    .filter(c => c.kind !== 'deferred' && c.tokens > 0)
    .map(c => ({ name: c.name, tokens: c.tokens, percent: Math.round((1000 * c.tokens) / b.rawMaxTokens) / 10, kind: c.kind as BreakdownRow['kind'] }))
    .sort((a, z) => ORDER[a.kind] - ORDER[z.kind] || z.tokens - a.tokens)
  return { total: b.totalTokens, window: b.rawMaxTokens, percent: b.percentage, rows }
}

export const breakdownTitle = (b: Breakdown, lang: UiLang = 'de'): string => t(lang, 'tip.ctxTitle', { used: tokens(b.total), window: tokens(b.window), pct: b.percent })

// Farbmarke je Zeile: belegte Zeilen reihum, freier Platz und Puffer grau.
// Farben passend zu den Emoji-Quadraten: der Tooltip (<title>, nur Text) zeigt dieselbe Farbe als Emoji.
const CAT = [
  { hex: THEME.accentFill, emoji: '🟦' },
  { hex: THEME.error, emoji: '🟥' },
  { hex: THEME.success, emoji: '🟩' },
  { hex: THEME.warning, emoji: '🟨' },
  { hex: THEME.violet, emoji: '🟪' },
  { hex: '#f4900c', emoji: '🟧' },
  { hex: '#c1694f', emoji: '🟫' },
] as const
const catOf = (row: BreakdownRow, index: number) =>
  row.kind === 'free' ? { hex: THEME.border, emoji: '⬜' } : row.kind === 'buffer' ? { hex: THEME.textMuted, emoji: '⬛' } : (CAT[index % CAT.length] ?? CAT[0])
export const categoryColor = (row: BreakdownRow, index: number): string => catOf(row, index).hex
export const categoryEmoji = (row: BreakdownRow, index: number): string => catOf(row, index).emoji

// Gestapelter Balken wie im Original: je Zeile ein Segment in der Farbe der Legende, Breite nach Anteil am Fenster.
export function breakdownBarSvg(b: Breakdown, width = 600, height = 14): string {
  let x = 0
  const segs = b.rows
    .map((r, i) => {
      const w = (width * r.tokens) / b.window
      const seg = `<rect x="${x.toFixed(1)}" y="0" width="${Math.max(w, 0).toFixed(1)}" height="${height}" fill="${categoryColor(r, i)}"/>`
      x += w
      return seg
    })
    .join('')
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}"><rect width="${width}" height="${height}" rx="3" fill="#555555" fill-opacity="0.35"/><clipPath id="c"><rect width="${width}" height="${height}" rx="3"/></clipPath><g clip-path="url(#c)">${segs}</g></svg>`
}

// Einstellungen: Gruppe je Schalter; Beschriftung und Erklärung liegen in i18n.ts unter `settings.<key>` und `settings.<key>.hint`.
// Die Reihenfolge ist die der Einstellungsseite, Gruppen in der Reihenfolge ihres ersten Schalters.
export type SettingKey = Exclude<keyof ModSettings, 'lang' | 'breathKey'>
export type SettingGroup = 'band' | 'panel' | 'chat' | 'skins' | 'summary'

export const SETTING_DEFS: readonly { key: SettingKey; group: SettingGroup }[] = [
  { key: 'rings', group: 'band' },
  { key: 'limits', group: 'band' },
  { key: 'cache', group: 'band' },
  { key: 'ctxTip', group: 'band' },
  { key: 'legend', group: 'band' },
  { key: 'branch', group: 'band' },
  { key: 'steps', group: 'band' },
  { key: 'autoEffort', group: 'band' },
  { key: 'saveMode', group: 'band' },
  { key: 'breath', group: 'band' },
  { key: 'mrBand', group: 'band' },
  { key: 'mrNotify', group: 'band' },
  { key: 'mrFooter', group: 'band' },
  { key: 'cardWorkdir', group: 'panel' },
  { key: 'cardCatchUp', group: 'summary' },
  { key: 'cardOverview', group: 'panel' },
  { key: 'cardContext', group: 'panel' },
  { key: 'cardTurn', group: 'panel' },
  { key: 'cardGate', group: 'panel' },
  { key: 'cardAgents', group: 'panel' },
  { key: 'cardTasks', group: 'panel' },
  { key: 'mrPane', group: 'panel' },
  { key: 'cardLog', group: 'panel' },
  { key: 'animation', group: 'panel' },
  { key: 'chatInfo', group: 'chat' },
  { key: 'chatLines', group: 'chat' },
  { key: 'skinIcons', group: 'skins' },
  { key: 'skinQuiet', group: 'skins' },
  { key: 'skinDiff', group: 'skins' },
  { key: 'skinExpand', group: 'skins' },
  { key: 'skinPrompt', group: 'skins' },
  { key: 'skinCards', group: 'skins' },
  { key: 'skinInline', group: 'skins' },
  { key: 'skinLinks', group: 'skins' },
  { key: 'skinMermaid', group: 'skins' },
  { key: 'skinMath', group: 'skins' },
  { key: 'skinAsk', group: 'skins' },
  { key: 'skinMode', group: 'skins' },
  { key: 'skinHint', group: 'skins' },
  { key: 'skinMarkIcon', group: 'skins' },
  { key: 'skinMarkLabel', group: 'skins' },
  { key: 'catchUp', group: 'summary' },
]

export const settingLabel = (key: SettingKey, lang: UiLang): string => t(lang, `settings.${key}`)
export const settingHint = (key: SettingKey, lang: UiLang): string => t(lang, `settings.${key}.hint`)

export const DEFAULT_SETTINGS: ModSettings = {
  rings: true, legend: true, branch: true, limits: true, cache: true, ctxTip: true, steps: true, animation: true,
  cardWorkdir: true, cardCatchUp: true, cardOverview: true, cardContext: true, cardTurn: true, cardGate: true, cardAgents: true, cardTasks: true, mrPane: true, cardLog: true,
  chatInfo: true, chatLines: true, catchUp: false, lang: 'de',
  // Hervorhebungen zeichnen Antworten selbst und sind deshalb standardmäßig aus.
  skinIcons: true, skinQuiet: true, skinDiff: true, skinExpand: true, skinPrompt: true, skinCards: true, skinInline: false, skinLinks: true,
  skinMermaid: true, skinMath: true, skinAsk: true, skinMode: true, skinHint: true, skinMarkIcon: true, skinMarkLabel: true,
  // Auto-Effort ist aus, solange das Mod effortless daneben läuft: zwei Judges würden um den Effort jedes Requests streiten.
  autoEffort: false, saveMode: false,
  breath: true, breathKey: 'hrv',
  mrBand: true, mrNotify: true, mrFooter: true,
}

// $.store kann Einstellungen aus älteren Versionen liefern: fehlende Felder werden mit den Standardwerten aufgefüllt.
export const settingsOf = (saved: Partial<ModSettings> | null | undefined): ModSettings => {
  const s = { ...DEFAULT_SETTINGS, ...saved }
  return { ...s, lang: s.lang === 'en' ? 'en' : 'de', breathKey: ['hrv', 'sigh', 'box', '478'].includes(s.breathKey) ? s.breathKey : 'hrv' }
}

export const CATCH_CHUNK = 100_000
export const CATCH_KEEP = 100

type CatchMessage = { role: 'user' | 'assistant'; text: string; toolUses: readonly { tool_use_id: string; tool: string; input: unknown; text?: string }[] }

// Fingerabdruck einer Nachricht, um den Stand der Zusammenfassung im Verlauf wiederzufinden.
export const fingerprint = (m: CatchMessage): string => `${m.role}\n${m.text.slice(0, 200)}\n${m.toolUses.map(t => t.tool_use_id).join(',')}`

// Claude Code hängt System-Hinweise an Nutzer-Nachrichten; sie sind nicht die Worte des Nutzers.
const unremind = (s: string): string => s.replace(/<system-reminder>[\s\S]*?<\/system-reminder>/g, '').trim()

const clipText = (s: string, n: number): string => (s.length > n ? `${s.slice(0, n)} […]` : s)

export function formatMessage(m: CatchMessage, lang: UiLang): string {
  const lines: string[] = []
  const text = unremind(m.text)
  const who = lang === 'de' ? (m.role === 'user' ? 'NUTZER' : 'ASSISTENT') : m.role === 'user' ? 'USER' : 'ASSISTANT'
  if (text) lines.push(`${who}: ${clipText(text, 4000)}`)
  for (const t of m.toolUses) {
    lines.push(`  [${lang === 'de' ? 'Tool' : 'tool'} ${t.tool}] ${clipText(JSON.stringify(t.input) ?? '', 300)}${t.text ? ` → ${clipText(t.text, 300)}` : ''}`)
  }
  return lines.join('\n')
}

// Ab welcher Nachricht noch nichts zusammengefasst ist; nach Kompaktierung oder verschobenem Fenster über den Fingerabdruck, sonst 0.
export function startIndex(messages: readonly CatchMessage[], covered: number, marker: string): number {
  const at = covered > 0 && covered <= messages.length ? messages[covered - 1] : undefined
  if (at !== undefined && fingerprint(at) === marker) return covered
  return messages.findLastIndex(m => fingerprint(m) === marker) + 1
}

// Der nächste Block neuer Nachrichten (etwa `limit` Zeichen) und der Index hinter ihm.
export function nextChunk(messages: readonly CatchMessage[], from: number, lang: UiLang, limit = CATCH_CHUNK): { text: string; end: number } {
  let text = ''
  let end = from
  while (end < messages.length && (end === from || text.length < limit)) text += `${formatMessage(messages[end++] as CatchMessage, lang)}\n\n`
  return { text, end }
}

export function catchUpSystem(lang: UiLang, instructions: string): string {
  const head =
    lang === 'de'
      ? [
          'Du hältst eine laufende Zusammenfassung einer Claude-Code-Session aktuell. Sie erscheint in einem schmalen Seitenbereich und wird fortgeschrieben, während die Session läuft.',
          'Folge den Anweisungen unten dazu, was geschrieben wird und wie. Sie wurden für eine einmalige Zusammenfassung durch einen Agenten geschrieben, der die ganze Unterhaltung liest und Befehle ausführt. Das kannst du nicht. Du bekommst deine vorherige Zusammenfassung, die seitdem hinzugekommenen Nachrichten und den aktuellen Git-Zustand. Schreibe die vollständige neue Zusammenfassung: Behalte, was noch stimmt, schreibe um, was sich geändert hat, lass weg, was nicht mehr zählt. Die vorherige Zusammenfassung ist dein einziges Gedächtnis an frühere Nachrichten, behalte also ihre Fakten, außer neue Nachrichten ändern sie.',
          'Gewichte den Überblick: Zusammenhang, Beweggrund, Kurzfassung der Vorgehensweise und aktueller Fortschritt.',
          'Format, das die Anweisungen unten überschreibt: Der Bereich ist schmal, daher ist jeder Abschnitt eine Liste mit „- “-Stichpunkten, nie ein Absatz. Höchstens 4 Stichpunkte je Abschnitt, jeder ein oder zwei kurze Zeilen. Gib nur das Markdown der Zusammenfassung aus, beginnend mit der ersten Überschrift. Schreibe vollständig auf Deutsch, auch die Überschriften.',
        ]
      : [
          'You keep a live catch-up summary of a Claude Code session. It is shown in a narrow side pane and updated as the session goes on.',
          'Follow the instructions below for what to write and how. They were written for a one-off summary by an agent that reads the whole conversation and runs commands. You cannot. You get your previous summary, the messages added since it was written, and the current git state. Write the full updated summary: keep what is still true, rewrite what changed, drop what no longer matters. The previous summary is your only memory of earlier messages, so keep its facts unless the new messages change them.',
          'Put the weight on the high-level context, the motivation, a summary of the method, and the current progress.',
          'Format, overriding the instructions below: the pane is narrow, so every section is a list of "- " bullets, never a paragraph. At most 4 bullets per section, each one or two short lines. Output only the summary Markdown, starting with the first heading. Write entirely in English.',
        ]
  return [...head, `<instructions>\n${instructions}\n</instructions>`].join('\n\n')
}

export function catchUpPrompt(previous: string, chunk: string, git: string, lang: UiLang): string {
  const none = lang === 'de' ? '(noch keine)' : '(none yet)'
  return `<previous_summary>\n${previous || none}\n</previous_summary>\n\n<new_messages>\n${chunk}</new_messages>\n\n<git_state>\n${git}\n</git_state>`
}

// Die Zusammenfassung als einfache Zeilen für Text-Elemente. Es wird bewusst kein Markdown gerendert: die Eingabe der Zusammenfassung
// ist nicht vertrauenswürdig, so wird nichts nachgeladen oder angeklickt (Bilder, Links, HTML, Referenzen entfallen).
export type SummaryLine = { kind: 'heading' | 'bullet' | 'text'; text: string }

const plainLine = (s: string): string =>
  s
    .replace(/<[^>\n]*>/g, '')
    .replace(/!?\[([^\]]*)\]\([^)]*\)/g, '$1')
    .replace(/!?\[([^\]]*)\]\[[^\]]*\]/g, '$1')
    .replace(/(\*\*|__|`)/g, '')
    .trim()

export function summaryLines(text: string): SummaryLine[] {
  return text
    .split('\n')
    .filter(l => l.trim() !== '' && !/^\s*\[[^\]]+\]:\s*\S+/.test(l))
    .map((l): SummaryLine => {
      const h = /^\s*#{1,6}\s+(.*)$/.exec(l)
      if (h) return { kind: 'heading', text: plainLine(h[1] ?? '') }
      const b = /^\s*[-*+]\s+(.*)$/.exec(l)
      if (b) return { kind: 'bullet', text: plainLine(b[1] ?? '') }
      return { kind: 'text', text: plainLine(l) }
    })
}

// Zusammenfassung in Abschnitte je Überschrift; Zeilen vor der ersten Überschrift bilden einen Abschnitt ohne Titel.
export type SummarySection = { heading?: string; lines: SummaryLine[] }
export function summarySections(lines: readonly SummaryLine[]): SummarySection[] {
  const out: SummarySection[] = []
  for (const l of lines) {
    if (l.kind === 'heading') out.push({ heading: l.text, lines: [] })
    else if (out.length === 0) out.push({ lines: [l] })
    else out[out.length - 1]?.lines.push(l)
  }
  return out
}

// Die größten belegten Zeilen der Kontext-Aufteilung, für den kleinen Tooltip im Band.
export const topRows = (b: Breakdown, max = 3): BreakdownRow[] =>
  b.rows
    .filter(r => r.kind === 'used')
    .sort((a, z) => z.tokens - a.tokens)
    .slice(0, max)

export const CATCH0: CatchUp = { sessionId: '', summary: '', covered: 0, marker: '', updatedAt: 0, status: '' }

// Kachel eines Subagents (Flightdeck-Stil): Zeile mit Kontext, Ausgabe und Schritten; Merkmale ohne Modell und Tokens.
export const agentStats = (a: Agent, lang: UiLang = 'de'): string =>
  [a.tokensIn === undefined ? '' : `ctx ${tokens(a.tokensIn)}`, a.tokensOut === undefined ? '' : `out ${tokens(a.tokensOut)}`, t(lang, 'agents.steps', { n: a.steps ?? 0 })]
    .filter(p => p !== '')
    .join(' · ')

export const agentFlags = (a: Agent, lang: UiLang = 'de'): string =>
  [
    a.requested !== undefined && a.requested !== a.model ? t(lang, 'agents.requested', { m: a.requested }) : '',
    a.background === true ? t(lang, 'agents.background') : '',
    a.fork === true ? t(lang, 'agents.fork') : '',
    a.teammate === true ? t(lang, 'agents.teammate') : '',
    a.permission === undefined ? '' : t(lang, 'agents.permission', { p: a.permission }),
    a.cacheHit === undefined || a.cacheHit === null ? '' : t(lang, 'agents.cache', { n: a.cacheHit }),
  ]
    .filter(p => p !== '')
    .join(' · ')

// Kurzer Hash für Hover-Schlüssel von Elementen ohne eigene ID (Nachrichten).
export function shortHash(text: string): string {
  let h = 5381
  for (let i = 0; i < text.length; i++) h = ((h << 5) + h + text.charCodeAt(i)) | 0
  return (h >>> 0).toString(36)
}

const lineCount = (v: unknown): number => (typeof v === 'string' && v !== '' ? v.split('\n').length : 0)

// Zeilen für den Hover-Hinweis zu einem Tool-Aufruf: Art, Ziel, Status und bei Änderungen der Umfang.
export function toolInfoLines(tool: string, input: unknown, status: string, lang: UiLang = 'de'): string[] {
  const kind = t(lang, `common.${toolKind(tool)}`)
  const target = toolTarget(input)
  const i = (input ?? {}) as Record<string, unknown>
  const size =
    tool === 'Edit' ? t(lang, 'chat.editSize', { old: lineCount(i.old_string), new: lineCount(i.new_string) }) : tool === 'Write' ? t(lang, 'chat.lines', { n: lineCount(i.content) }) : ''
  return [`${toolText(tool)} (${kind}) · ${status}`, target === '' ? '' : t(lang, 'chat.target', { x: target }), size === '' ? '' : t(lang, 'chat.size', { x: size })].filter(l => l !== '')
}

export function pushCapped<T>(list: readonly T[], item: T, max = 50): T[] {
  return [...list, item].slice(-max)
}

// Liest bis zu `max` Vorschläge aus einer Modellantwort, die ein JSON-Array enthalten soll.
export function parseSteps(text: string, max = 3): Step[] {
  const match = /\[[\s\S]*\]/.exec(text)
  if (!match) return []
  let list: unknown
  try {
    list = JSON.parse(match[0])
  } catch {
    return []
  }
  if (!Array.isArray(list)) return []
  return list
    .filter((x): x is { label: string; prompt?: unknown; why?: unknown } => typeof x?.label === 'string' && x.label.trim() !== '')
    .slice(0, max)
    .map(x => {
      // Kürzen erst beim Zeichnen nach Bandbreite; hier nur eine Obergrenze gegen ausufernde Antworten.
      const label = x.label.trim().length > 80 ? `${x.label.trim().slice(0, 79)}…` : x.label.trim()
      const prompt = typeof x.prompt === 'string' && x.prompt.trim() ? x.prompt.trim() : x.label.trim()
      const why = typeof x.why === 'string' ? oneLine(x.why).slice(0, 90) : ''
      return why === '' ? { label, prompt } : { label, prompt, why }
    })
}

// Alle Komponenten, die ui.render anbietet (claude-code 2.1.295), und wer sie bei uns zeichnet. Für /cockpit-demo.
// `by`: 'chat' = Hover-Info (Einstellung chatInfo), 'line' = eigene Zeile (Einstellung chatLines), 'mod' = eigene Zeichnung,
// 'skin' = nur Chat-Skin, '' = noch nicht gestylt. `skins`: Einstellungen der Chat-Skins, die die Komponente zeichnen.
// Die Beschreibung („Wo“) steht in i18n.ts unter `demo.<name>`.
export const RENDER_COMPONENTS = [
  { name: 'UserMessage', by: 'skin', skins: ['skinPrompt'] },
  { name: 'AssistantMessage', by: 'chat', skins: ['skinCards', 'skinInline', 'skinLinks', 'skinMermaid', 'skinMath'] },
  { name: 'ToolUse', by: 'chat', skins: ['skinIcons', 'skinExpand'] },
  { name: 'ToolResult', by: 'chat', skins: ['skinQuiet', 'skinDiff'] },
  { name: 'ToolGroup', by: 'skin', skins: ['skinIcons'] },
  // 'terminal' = die Engine löst die Komponente auf dem Desktop nicht aus; dort gibt es nichts zu stylen.
  { name: 'ToolProgress', by: 'terminal' },
  { name: 'CommandOutput', by: 'chat', skins: ['skinCards', 'skinInline', 'skinLinks', 'skinMermaid', 'skinMath'] },
  { name: 'AskUserQuestion', by: 'skin', skins: ['skinAsk'] },
  { name: 'Spinner', by: 'line' },
  { name: 'TurnDuration', by: 'line' },
  { name: 'InfoNotice', by: 'terminal' },
  { name: 'SessionMode', by: 'skin', skins: ['skinMode'] },
  { name: 'PromptHint', by: 'skin', skins: ['skinHint'] },
  { name: 'AbovePrompt', by: 'mod' },
  { name: 'Pane', by: 'mod' },
] as const satisfies readonly { name: string; by: 'chat' | 'line' | 'mod' | 'skin' | 'terminal' | ''; skins?: readonly SettingKey[] }[]

export function demoTable(s: ModSettings, lang: UiLang = 'de'): string {
  const state = (c: (typeof RENDER_COMPONENTS)[number]) => {
    const skins: readonly SettingKey[] = 'skins' in c ? c.skins : []
    if (skins.some(k => s[k])) return t(lang, 'demo.skinOn', { what: skins.filter(k => s[k]).map(k => settingLabel(k, lang)).join(', ') })
    const by = c.by
    return t(
      lang,
      by === 'mod' ? 'demo.mod' : by === 'chat' ? (s.chatInfo ? 'demo.chatOn' : 'demo.chatOff') : by === 'line' ? (s.chatLines ? 'demo.lineOn' : 'demo.lineOff') : by === 'skin' ? 'demo.skinOff' : by === 'terminal' ? 'demo.terminal' : 'demo.open',
    )
  }
  const rows = RENDER_COMPONENTS.map(c => `| \`${c.name}\` | ${t(lang, `demo.${c.name}`)} | ${state(c)} |`)
  // Nur-Terminal-Komponenten zählen als nicht gestylt: der Desktop löst sie nicht aus.
  const open = RENDER_COMPONENTS.filter(c => (c.by as string) === '' || c.by === 'terminal').length
  const all = RENDER_COMPONENTS.length
  return [
    `## ${t(lang, 'demo.heading')}`,
    '',
    t(lang, 'demo.count', { n: all - open, m: all }),
    '',
    `| ${t(lang, 'demo.colName')} | ${t(lang, 'demo.colWhere')} | ${t(lang, 'demo.colState')} |`,
    '| --- | --- | --- |',
    ...rows,
    '',
  ].join('\n')
}

// Verlauf der Subagents (Gantt): ein Balken je Agent auf der gemeinsamen Zeitachse t0..t1.
export function timelineSvg(start: number, end: number, t0: number, t1: number, color: string, width = 360, height = 8): string {
  const span = Math.max(1, t1 - t0)
  const x = (width * Math.max(0, start - t0)) / span
  const w = Math.max(3, (width * Math.max(0, end - start)) / span)
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}"><rect width="${width}" height="${height}" rx="2" fill="#888888" fill-opacity="0.12"/><rect x="${x.toFixed(1)}" y="0" width="${Math.min(w, width - x).toFixed(1)}" height="${height}" rx="2" fill="${color}"/></svg>`
}

// Phase der Arbeitszeile: Wort und Farbrolle. Der Desktop nennt `tool-input` nie, wird wie `tool-use` behandelt.
export const SPIN_MODE: Record<'requesting' | 'responding' | 'thinking' | 'tool-input' | 'tool-use', { label: Key; color: string }> = {
  requesting: { label: 'chat.spin.requesting', color: THEME.textSecondary },
  thinking: { label: 'chat.spin.thinking', color: THEME.info },
  responding: { label: 'chat.spin.responding', color: THEME.success },
  'tool-input': { label: 'chat.spin.tool', color: THEME.warning },
  'tool-use': { label: 'chat.spin.tool', color: THEME.warning },
}

// Platzhalter der Engine ohne eigene Aussage: dann bleibt nur die Phase stehen.
export const spinnerText = (word: string, message: string | null): string => {
  const text = (message ?? word).trim()
  return text === '' || text === 'Working' ? '' : text
}

// Lesbarer Modellname: `claude-opus-5-5` → „Opus 5.5“, `claude-sonnet-5-5[1m]` → „Sonnet 5.5“; Unbekanntes bleibt die ID.
export function modelLabel(id: string): string {
  const m = /^claude-([a-z]+)-(\d+)(?:-(\d+))?/i.exec(id)
  if (!m || !m[1] || !m[2]) return id
  return `${m[1].charAt(0).toUpperCase()}${m[1].slice(1)} ${m[2]}${m[3] && m[3].length <= 2 ? `.${m[3]}` : ''}`
}


// Kurze Legende des Bands als ein Bild: Farbmarke und Wort je Eintrag, `tip` als <title> (die ganze Legende mit Erklärung).
// Schriftbreite geschätzt (ca. 6,6 px je Zeichen bei 12 px); ein Bild statt Text, damit genau ein Tooltip-Frame entsteht.
export function legendSvg(items: readonly { color: string; label: string }[], tip: string, textColor: string, height = 16): string {
  const esc = (x: string) => x.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
  let x = 0
  const parts = items.map(i => {
    const w = 10 + 4 + Math.ceil(i.label.length * 6.6)
    const g = `<rect x="${x}" y="${height / 2 - 5}" width="10" height="10" rx="2" fill="${i.color}"/><text x="${x + 14}" y="${height / 2 + 4}" font-family="sans-serif" font-size="12" fill="${textColor}">${esc(i.label)}</text>`
    x += w + 12
    return g
  })
  const width = Math.max(1, x - 12)
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}"><title>${esc(tip)}</title>${parts.join('')}</svg>`
}

// Claudes eigener Vorschlag (prompt.suggest) steht vorn; Haiku-Vorschläge mit gleichem Prompt fallen weg, höchstens 5.
export function withClaude(list: readonly Step[], hint: Step | null, max = 3): Step[] {
  if (hint === null) return list.slice(0, max)
  const same = (s: Step) => s.prompt.trim().toLowerCase() === hint.prompt.trim().toLowerCase()
  return [hint, ...list.filter(s => !same(s))].slice(0, max)
}

// Task-Benachrichtigung der Engine (`<task-notification>` im Prompt): Kennung, Status und Kurztext; null, wenn es keine ist.
export function parseTaskNotification(text: string): { id: string; status: string; summary: string } | null {
  const tag = (name: string) => new RegExp(`<${name}>([\\s\\S]*?)</${name}>`).exec(text)?.[1]?.trim() ?? ''
  const id = tag('task-id')
  return id === '' ? null : { id, status: tag('status') || 'completed', summary: tag('summary') }
}

// Tool-Namen lesbar: `mcp__dev-workbench__plan_session` → Server „dev-workbench“, Tool „plan_session“.
// Plugin-Server (`plugin_<plugin>_<server>`) werden auf den Servernamen gekürzt, bei gleichem Namen einmal.
export function splitTool(name: string): { server: string | null; tool: string } {
  const m = /^mcp__(.+?)__(.+)$/.exec(name)
  if (!m || !m[1] || !m[2]) return { server: null, tool: name }
  let server = m[1].replace(/^plugin_/, '')
  const half = server.length % 2 === 1 ? (server.length - 1) / 2 : -1
  if (half > 0 && server[half] === '_' && server.slice(0, half) === server.slice(half + 1)) server = server.slice(0, half)
  return { server, tool: m[2] }
}

export const toolText = (name: string): string => {
  const p = splitTool(name)
  return p.server === null ? p.tool : `${p.server} › ${p.tool}`
}

// ---- Arbeitshilfen: Compact-Fokus, Schleifen, Kontext-Diät, Turn-Bilanz, Agent-Bremse ----

// Was /compact behalten soll: Ziel (letzter Prompt), geänderte Dateien, offene Folge-Schritte. Leer, wenn nichts bekannt ist.
export function compactFocus(lastPrompt: string, touched: readonly string[], steps: readonly { label: string }[], lang: UiLang = 'de'): string {
  const parts = [
    lastPrompt.trim() !== '' && t(lang, 'compact.goal', { text: oneLine(lastPrompt, 120) }),
    touched.length > 0 && t(lang, 'compact.files', { list: touched.slice(-8).map(p => p.replace(/\\/g, '/').split('/').slice(-2).join('/')).join(', ') }),
    steps.length > 0 && t(lang, 'compact.steps', { list: steps.slice(0, 3).map(s => s.label).join('; ') }),
  ].filter((x): x is string => typeof x === 'string')
  return parts.length === 0 ? '' : `${t(lang, 'compact.keep')} ${parts.join(' · ')}`
}

export const LOOP_ERRORS = 3
export const LOOP_EDITS = 5
const LOOP_WINDOW = 8

// Derselbe Aufruf scheitert mindestens LOOP_ERRORS-mal in den letzten LOOP_WINDOW Aufrufen, oder dieselbe Datei wird im Turn
// mindestens LOOP_EDITS-mal geändert. Fehler wiegen schwerer und gewinnen.
export function detectLoop(marks: readonly CallMark[]): LoopAlert | null {
  const count = (list: readonly CallMark[], pick: (m: CallMark) => boolean) => {
    const n = new Map<string, number>()
    for (const m of list) if (pick(m)) n.set(m.key, (n.get(m.key) ?? 0) + 1)
    return [...n].sort((a, z) => z[1] - a[1])[0]
  }
  const err = count(marks.slice(-LOOP_WINDOW), m => m.isError)
  if (err !== undefined && err[1] >= LOOP_ERRORS) return { kind: 'error', target: err[0], n: err[1] }
  const edit = count(marks, m => m.isEdit && !m.isError)
  return edit !== undefined && edit[1] >= LOOP_EDITS ? { kind: 'edit', target: edit[0], n: edit[1] } : null
}

// Serverteil eines MCP-Tool-Namens so, wie er im Namen steht (`plugin_serena_serena`); null für eingebaute Tools.
export const mcpKey = (tool: string): string | null => /^mcp__(.+?)__/.exec(tool)?.[1] ?? null
export const DIET_MIN_TOKENS = 500

// Kontext-Diät: geladene MCP-Schemas je Server, die in dieser Session nie aufgerufen wurden, die teuersten zuerst.
export function mcpDiet(tools: readonly McpSchema[], used: readonly string[]): { server: string; tokens: number; tools: number }[] {
  const by = new Map<string, { server: string; tokens: number; tools: number }>()
  for (const x of tools) {
    const key = mcpKey(x.name)
    if (!x.isLoaded || key === null || used.includes(key)) continue
    const row = by.get(key) ?? { server: x.serverName, tokens: 0, tools: 0 }
    by.set(key, { ...row, tokens: row.tokens + x.tokens, tools: row.tools + 1 })
  }
  return [...by.values()].filter(r => r.tokens >= DIET_MIN_TOKENS).sort((a, z) => z.tokens - a.tokens)
}

// Turn-Bilanz: teuer ohne Ergebnis heißt ab WASTE_USD ohne einen einzigen Edit.
export const WASTE_USD = 0.25
export const isWasteful = (r: TurnRecord): boolean => (r.usd ?? 0) >= WASTE_USD && r.edits === 0

// Ein Balken je Turn, Höhe nach Kosten (ohne Kostenangabe nach Tool-Calls); verschwenderische Turns in Warnfarbe.
export function turnBarsSvg(list: readonly TurnRecord[], height = 24): string {
  const w = 6
  const val = (r: TurnRecord) => r.usd ?? r.tools / 100
  const max = Math.max(...list.map(val), 1e-9)
  const bars = list.map((r, i) => {
    const h = Math.max(1, Math.round((val(r) / max) * height))
    return `<rect x="${i * w}" y="${height - h}" width="${w - 2}" height="${h}" rx="1" fill="${isWasteful(r) ? THEME.warning : THEME.accentFill}"/>`
  })
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${Math.max(w, list.length * w)}" height="${height}" viewBox="0 0 ${Math.max(w, list.length * w)} ${height}">${bars.join('')}</svg>`
}

// Agent-Bremse: ab so vielen Tool-Calls oder so langer Laufzeit gilt ein laufender Subagent als teuer.
export const AGENT_STEPS_WARN = 40
export const AGENT_MS_WARN = 10 * 60_000
export const isAgentHeavy = (a: Pick<Agent, 'steps' | 'since'>, now: number): boolean => (a.steps ?? 0) >= AGENT_STEPS_WARN || now - a.since >= AGENT_MS_WARN

// Cache-Wächter: so kurz vor Ablauf schlägt das Cockpit einen Prompt vor, der den Cache warm hält.
export const CACHE_WARN_MS = 90_000


// ---- Auto-Effort, Limit-Alarm, Handoff (Ideen aus effortless, eigener Code) ----

export type EffortLevel = 'low' | 'medium' | 'high'
export const EFFORT_LEVELS: readonly EffortLevel[] = ['low', 'medium', 'high']

// „ok“, „ja, weiter“, Slash-Befehle: kein Judge-Call, die letzte Stufe bleibt.
export const isTrivialPrompt = (text: string): boolean => {
  const x = text.trim()
  return x === '' || x.startsWith('/') || (x.length <= 24 && x.split(/\s+/).length <= 3)
}

// Eingabe für $.model.classify: kurze Skala, dann der Prompt als Daten.
export const effortJudgeText = (text: string): string =>
  `Reasoning effort a coding assistant needs for this request. low = quick question, lookup, trivial edit. medium = normal feature or fix. high = hard multi-step work, architecture, tricky debugging.

Request:
${text.trim().slice(0, 2000)}`

export const LIMIT_ALERT_AT = 80

// Das vollste Limit-Fenster ab LIMIT_ALERT_AT, sonst null.
export function limitAlert(limits: readonly Limit[]): Limit | null {
  const top = [...limits].sort((a, z) => z.percentUsed - a.percentUsed)[0]
  return top !== undefined && top.percentUsed >= LIMIT_ALERT_AT ? top : null
}

// Stufe, die gesendet wird: im Sparmodus mit vollem Limit immer low.
export const effectiveEffort = (pick: EffortLevel, saveMode: boolean, limits: readonly Limit[]): EffortLevel =>
  saveMode && limitAlert(limits) !== null ? 'low' : pick

export const HANDOFF_AT = 60
// Ein frischer Chat lohnt, wenn ein großer Kontext kalt neu gelesen werden müsste.
// ponytail: feste Schwelle statt Haiku-Urteil; Model-Call nachrüsten, wenn die Schwelle oft daneben liegt.
export const handoffPays = (contextPercent: number | undefined, cacheLeft: number): boolean => (contextPercent ?? 0) >= HANDOFF_AT && cacheLeft === 0

// HANDOFF.md aus dem, was das Cockpit ohnehin kennt; ohne Zusammenfassung liefert der Aufrufer den Text eines Forks.
export function handoffMarkdown(p: { summary: string; lastPrompt: string; touched: readonly string[]; steps: readonly { label: string }[]; git: string }, lang: UiLang = 'de'): string {
  const sec = (key: 'handoff.summary' | 'handoff.goal' | 'handoff.files' | 'handoff.steps', body: string) => (body.trim() === '' ? [] : [`## ${t(lang, key)}`, '', body.trim(), ''])
  const fence = '```'
  return [
    t(lang, 'handoff.title'),
    '',
    ...sec('handoff.summary', p.summary),
    ...sec('handoff.goal', p.lastPrompt),
    ...sec('handoff.files', p.touched.map(f => `- ${f}`).join('\n')),
    ...sec('handoff.steps', p.steps.map(s => `- ${s.label}`).join('\n')),
    '## Git',
    '',
    fence,
    p.git.trim(),
    fence,
    '',
  ].join('\n')
}

// Farbe des Effort-Blocks: low gelb, medium grün, high (und höher) blau; ohne Stufe Standardfarbe.
export const effortColor = (level: string | number | null | undefined): string | undefined =>
  level === 'low' ? THEME.warning : level === 'medium' ? THEME.success : level === undefined || level === null ? undefined : THEME.accent
