// Chat-Skins: reine Funktionen ohne `$` und ohne UI – Klassifizierung, Parsen, Formatieren und Svg-Quellen.
// Die Zeichnung je Oberfläche liegt in register.tsx; eine spätere Terminal-Version nutzt dieselben Funktionen.
// Ideen aus claude-skins und prismantis (beide MIT), eigener Code.
import type { UiLang } from '../types'
import { THEME, oneLine, toolText } from './core'
import { t } from './i18n'
import { parseMermaid } from './mermaid'
import type { Diagram } from './mermaid'

// ── 1 · Tool-Symbole ──────────────────────────────────────────────────────────

export type IconKind = 'read' | 'edit' | 'write' | 'shell' | 'search' | 'web' | 'agent' | 'other'

const KIND_OF: Record<string, IconKind> = {
  Read: 'read', NotebookRead: 'read',
  Edit: 'edit', MultiEdit: 'edit', NotebookEdit: 'edit',
  Write: 'write',
  Bash: 'shell', PowerShell: 'shell', BashOutput: 'shell', KillShell: 'shell',
  Grep: 'search', Glob: 'search', ToolSearch: 'search', LS: 'search',
  WebFetch: 'web', WebSearch: 'web',
  Agent: 'agent', Task: 'agent', SendMessage: 'agent',
}

export const iconKind = (tool: string): IconKind => KIND_OF[tool] ?? 'other'

export const ICON_COLOR: Record<IconKind, string> = {
  read: THEME.info,
  edit: THEME.warning,
  write: THEME.success,
  shell: THEME.textSecondary,
  search: THEME.max,
  web: THEME.accentFill,
  agent: THEME.violet,
  other: THEME.textMuted,
}

// Glyphen auf 16 × 16, nur Striche: Dokument, Stift, Dokument mit Plus, Prompt, Lupe, Globus, Knoten, Punkt im Kreis.
const GLYPH: Record<IconKind, string> = {
  read: '<path d="M4 1.5h5.5L12.5 4.5V14.5H4z"/><path d="M6 7h4.5M6 9.5h4.5M6 12h3"/>',
  edit: '<path d="M10.5 2.5l3 3L6 13H3v-3z"/><path d="M9 4l3 3"/>',
  write: '<path d="M4 1.5h5.5L12.5 4.5V14.5H4z"/><path d="M8.25 7v5M5.75 9.5h5"/>',
  shell: '<rect x="1.5" y="2.5" width="13" height="11" rx="1.5"/><path d="M4 6l2.5 2L4 10M8 10.5h4"/>',
  search: '<circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5l4 4"/>',
  web: '<circle cx="8" cy="8" r="6.5"/><ellipse cx="8" cy="8" rx="2.75" ry="6.5"/><path d="M1.5 8h13"/>',
  agent: '<circle cx="8" cy="3.5" r="2"/><circle cx="3.5" cy="12.5" r="2"/><circle cx="12.5" cy="12.5" r="2"/><path d="M7 5.25L4.5 10.75M9 5.25l2.5 5.5M5.5 12.5h5"/>',
  other: '<circle cx="8" cy="8" r="6"/><circle cx="8" cy="8" r="1.5"/>',
}

// Statisches Symbol: Quelle hängt nur von Art und Farbe ab, damit das Bild beim Neuzeichnen nicht neu geladen wird.
export function toolIconSvg(kind: IconKind, color: string): string {
  return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="${color}" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">${GLYPH[kind]}</svg>`
}

// ── 2 · Ruhige Ergebnisse und echte Fehlerzeile ───────────────────────────────

export const QUIET_TOOLS = new Set(['Read', 'Grep', 'Glob'])
const SHELLS = new Set(['Bash', 'PowerShell'])
export const isShell = (tool: string): boolean => SHELLS.has(tool)

// Befehle, die nur lesen. ponytail: Positivliste nach erstem Wort (bei git das zweite), keine echte Shell-Analyse; Unbekanntes bleibt laut.
const READ_CMDS = new Set([
  'ls', 'dir', 'cat', 'head', 'tail', 'less', 'more', 'grep', 'rg', 'find', 'fd', 'pwd', 'echo', 'wc', 'which', 'where', 'type', 'file', 'stat',
  'tree', 'du', 'df', 'sort', 'uniq', 'cut', 'diff', 'jq', 'whoami', 'date', 'env', 'printenv', 'realpath', 'dirname', 'basename',
  'get-childitem', 'get-content', 'select-string', 'get-item', 'get-location', 'test-path', 'resolve-path', 'measure-object', 'select-object', 'where-object', 'format-table', 'format-list', 'get-command',
])
const READ_GIT = new Set(['status', 'log', 'diff', 'show', 'branch', 'rev-parse', 'ls-files', 'blame', 'remote', 'describe', 'shortlog', 'tag'])

export function isReadOnlyShell(command: string): boolean {
  // Umleitung in eine Datei schreibt; 2>&1 und Umleitung ins Nichts sind harmlos.
  const cleaned = command.replace(/\d?>\s*&\d|\d?>\s*(\/dev\/null|\$null|nul)\b/gi, '')
  if (/>|`|\$\(/.test(cleaned)) return false
  const parts = cleaned.split(/&&|\|\||[|;\n]/).map(p => p.trim()).filter(p => p !== '')
  return (
    parts.length > 0 &&
    parts.every(p => {
      const [first = '', second = ''] = p.split(/\s+/)
      const cmd = first.toLowerCase()
      return cmd === 'git' ? READ_GIT.has(second.toLowerCase()) : cmd === 'cd' || READ_CMDS.has(cmd)
    })
  )
}

// Befehl eines Shell-Aufrufs aus dem tool.call-Event (Argumente im Event selbst oder unter `input`).
export function shellCommand(e: unknown): string {
  const o = (e ?? {}) as Record<string, unknown>
  const i = (o.input ?? o) as Record<string, unknown>
  return typeof i.command === 'string' ? i.command : ''
}

const linesOf = (s: string): number => (s === '' ? 0 : s.replace(/\n$/, '').split('\n').length)
const shortPath = (p: string): string => p.replace(/\\/g, '/').split('/').slice(-2).join('/')

// Eine Zeile für ein erfolgreiches, ruhiges Ergebnis; null, wenn das Ergebnis nicht passt (dann zeichnet die Engine).
export function quietLine(tool: string, output: unknown, lang: UiLang): string | null {
  const o = (output ?? {}) as Record<string, unknown>
  if (tool === 'Read') {
    const f = o.file as { filePath?: unknown; numLines?: unknown } | undefined
    if (o.type !== 'text' || typeof f?.numLines !== 'number') return null
    return t(lang, 'skin.quiet.read', { tool: toolText(tool), n: f.numLines, file: typeof f.filePath === 'string' ? shortPath(f.filePath) : '' })
  }
  if (tool === 'Grep' || tool === 'Glob') {
    if (tool === 'Grep' && o.mode === 'content' && typeof o.numLines === 'number') return t(lang, 'skin.quiet.lines', { tool: toolText(tool), n: o.numLines })
    if (tool === 'Grep' && o.mode === 'count' && typeof o.numMatches === 'number') return t(lang, 'skin.quiet.matches', { tool: toolText(tool), n: o.numMatches })
    return typeof o.numFiles === 'number' ? t(lang, 'skin.quiet.files', { tool: toolText(tool), n: o.numFiles }) : null
  }
  if (isShell(tool)) {
    if (typeof o.stdout !== 'string') return null
    const n = linesOf(o.stdout)
    return n === 0 ? t(lang, 'skin.quiet.empty', { tool }) : t(lang, 'skin.quiet.lines', { tool: toolText(tool), n })
  }
  return null
}

// Fehlertext eines Ergebnisses: der Text, den das Modell las, sonst stderr, sonst stdout.
export function errorText(output: unknown): string {
  if (typeof output === 'string') return output
  const o = (output ?? {}) as Record<string, unknown>
  for (const k of ['stderr', 'error', 'message', 'stdout'] as const) if (typeof o[k] === 'string' && o[k] !== '') return o[k] as string
  return ''
}

const ERROR_HINT = /error|fehler|exception|failed|fatal|denied|not found|cannot|can't|unable|invalid|no such|refused|traceback/i

// Die erste aussagekräftige Fehlerzeile statt „Exit code 1“: Rahmen-Tags und Exit-Code-Zeilen fallen weg.
export function firstErrorLine(text: string): string {
  const lines = text
    .replace(/<\/?[a-z_]+_error>/gi, '')
    .split('\n')
    .map(l => l.trim())
    .filter(l => l !== '' && !/^exit code:? -?\d+$/i.test(l))
  return oneLine(lines.find(l => ERROR_HINT.test(l)) ?? lines[0] ?? text.trim(), 160)
}

// ── 3 · Diff-Karte ────────────────────────────────────────────────────────────

export type DiffRow = { sign: '+' | '-' | ' ' | '…'; no: number | null; text: string }
export type Diff = { file: string; rows: DiffRow[]; added: number; removed: number }

type Hunk = { oldStart: number; newStart: number; lines: readonly string[] }

// Zeilen aus `structuredPatch` von Edit/Write/MultiEdit; eine neue Datei ohne Patch wird ganz als „+“ gezeigt.
export function diffOf(output: unknown): Diff | null {
  const o = (output ?? {}) as Record<string, unknown>
  const file = typeof o.filePath === 'string' ? o.filePath : ''
  const hunks = Array.isArray(o.structuredPatch) ? (o.structuredPatch as Hunk[]) : null
  if (file === '' || hunks === null) return null
  const rows: DiffRow[] = []
  if (hunks.length === 0 && typeof o.content === 'string') {
    o.content.replace(/\n$/, '').split('\n').forEach((text, i) => rows.push({ sign: '+', no: i + 1, text }))
  }
  hunks.forEach((h, hi) => {
    if (hi > 0) rows.push({ sign: '…', no: null, text: '' })
    let oldNo = h.oldStart
    let newNo = h.newStart
    for (const l of h.lines) {
      const c = l.charAt(0)
      if (c === '+') rows.push({ sign: '+', no: newNo++, text: l.slice(1) })
      else if (c === '-') rows.push({ sign: '-', no: oldNo++, text: l.slice(1) })
      else if (c === ' ') {
        rows.push({ sign: ' ', no: newNo++, text: l.slice(1) })
        oldNo++
      }
    }
  })
  if (rows.length === 0) return null
  return { file, rows, added: rows.filter(r => r.sign === '+').length, removed: rows.filter(r => r.sign === '-').length }
}

export const DIFF_MAX = 40
const esc = (s: string): string => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')

// Gerahmte Karte als Svg (einfaches Bild, nie interaktiv): Kopf mit Datei und +/−, darunter je Zeile Nummer, Zeichen, Text.
// Ab `max` Zeilen steht `more` (i18n, mit {n}) in der letzten Zeile. Gleiche Eingabe ergibt dieselbe Quelle.
export function diffCardSvg(d: Diff, lang: UiLang, max = DIFF_MAX, width = 720): string {
  const shown = d.rows.slice(0, max)
  const hidden = d.rows.length - shown.length
  const rowH = 17
  const head = 26
  const height = head + (shown.length + (hidden > 0 ? 1 : 0)) * rowH + 8
  const chars = Math.floor((width - 76) / 7.2)
  const clipLine = (s: string) => {
    const x = s.replace(/\t/g, '  ')
    return x.length > chars ? `${x.slice(0, chars - 1)}…` : x
  }
  const color = { '+': THEME.success, '-': THEME.error, ' ': THEME.textSecondary, '…': THEME.textMuted } as const
  const body = shown
    .map((r, i) => {
      const y = head + i * rowH
      const tint = r.sign === '+' || r.sign === '-' ? `<rect x="1" y="${y}" width="${width - 2}" height="${rowH}" fill="${color[r.sign]}" fill-opacity="0.12"/>` : ''
      return `${tint}<text x="40" y="${y + 12.5}" text-anchor="end" fill="${THEME.textMuted}">${r.no ?? ''}</text><text x="50" y="${y + 12.5}" fill="${color[r.sign]}">${r.sign === ' ' ? '' : esc(r.sign)}</text><text x="64" y="${y + 12.5}" fill="${color[r.sign]}" xml:space="preserve">${esc(clipLine(r.text))}</text>`
    })
    .join('')
  const more = hidden > 0 ? `<text x="64" y="${head + shown.length * rowH + 12.5}" fill="${THEME.textMuted}">${esc(t(lang, 'skin.more', { n: hidden }))}</text>` : ''
  const title = `<text x="12" y="17" fill="${THEME.accent}" font-weight="bold">${esc(shortPath(d.file))}</text><text x="${width - 12}" y="17" text-anchor="end"><tspan fill="${THEME.success}">+${d.added}</tspan><tspan fill="${THEME.textMuted}"> </tspan><tspan fill="${THEME.error}">−${d.removed}</tspan></text>`
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" font-family="ui-monospace, Consolas, monospace" font-size="12"><rect x="0.5" y="0.5" width="${width - 1}" height="${height - 1}" rx="6" fill="${THEME.surface}" stroke="${THEME.border}"/><path d="M1 ${head - 2}H${width - 1}" stroke="${THEME.border}"/>${title}${body}${more}</svg>`
}

// ── 4 · Details aufklappen ────────────────────────────────────────────────────

export const DETAIL_MAX = 20_000
const capText = (s: string): string => (s.length > DETAIL_MAX ? `${s.slice(0, DETAIL_MAX)}\n…` : s)

// Eingabe eines Tools für das native Code-Element: Shell-Befehle als Shell, sonst JSON.
export function detailInput(tool: string, input: unknown): { source: string; language: string } {
  const cmd = shellCommand({ input })
  if (isShell(tool) && cmd !== '') return { source: capText(cmd), language: tool === 'PowerShell' ? 'powershell' : 'bash' }
  return { source: capText(JSON.stringify(input, null, 2) ?? ''), language: 'json' }
}

// Ergebnis für das Code-Element: Text bleibt Text, Read zeigt den Dateiinhalt (Sprache über den Pfad), sonst JSON.
export function detailOutput(output: unknown): { source: string; language?: string; path?: string } | null {
  if (output === undefined || output === null) return null
  if (typeof output === 'string') return { source: capText(output) }
  const o = output as Record<string, unknown>
  const f = o.file as { filePath?: unknown; content?: unknown } | undefined
  if (typeof f?.content === 'string') return { source: capText(f.content), ...(typeof f.filePath === 'string' ? { path: f.filePath } : {}) }
  if (typeof o.stdout === 'string') return { source: capText([o.stdout, typeof o.stderr === 'string' ? o.stderr : ''].filter(x => x !== '').join('\n')) }
  return { source: capText(JSON.stringify(output, null, 2) ?? ''), language: 'json' }
}

// ── 5 · Prompt-Rahmen ─────────────────────────────────────────────────────────

// Text wörtlich als Markdown: Satzzeichen maskiert, führende Leerzeichen erhalten, jede Zeile ein harter Umbruch.
export const mdLiteral = (text: string): string =>
  text
    .replace(/\r\n?/g, '\n')
    .split('\n')
    .map(l => l.replace(/[!-/:-@[-`{-~]/g, '\\$&').replace(/^ +/, s => '\u00a0'.repeat(s.length)))
    .join('  \n')

// ── 6 + 7 · Antworten: Karten, Hinweise, Aufgaben, Hervorhebungen ─────────────

export type AlertKind = 'note' | 'tip' | 'important' | 'warning' | 'caution'
export type Segment =
  | { kind: 'md'; text: string }
  | { kind: 'plain'; text: string }
  | { kind: 'code'; lang: string; source: string }
  | { kind: 'table'; source: string; rows: number; cols: number }
  | { kind: 'alert'; alert: AlertKind; text: string }
  | { kind: 'tasks'; text: string; done: number; total: number }
  | { kind: 'mermaid'; diagram: Diagram; source: string }
  | { kind: 'math'; tex: string; text: string | null }

// `mermaid`: unterstützte Diagramme als Karte; `math`: $$…$$ und math/latex-Fences als Formelkarte.
export type SplitOptions = { cards: boolean; inline: boolean; mermaid?: boolean; math?: boolean }

export const ALERT_COLOR: Record<AlertKind, string> = { note: THEME.info, tip: THEME.success, important: THEME.violet, warning: THEME.warning, caution: THEME.error }

// Fences, die die Oberfläche selbst besonders zeichnet (Diagramme, Formeln), bleiben im Markdown.
const NATIVE_FENCE = new Set(['mermaid', 'math', 'latex', 'katex'])
const MATH_FENCE = new Set(['math', 'latex', 'katex'])
const TABLE_SEP = /^\s*\|?\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)+\|?\s*$/
const TASK = /^\s*[-*+]\s+\[([ xX])\]\s/
// Ein Absatz ohne jedes Markdown-Zeichen kann ohne Verlust als farbiger Text gezeichnet werden.
const MD_SYNTAX = /[*_`#>|[\]<\\~]|:\/\/|^\s*([-+]|\d+[.)])\s/m

// Zerlegt Markdown in Abschnitte. `cards`: Codeblöcke und Tabellen; `inline`: Hinweise, Aufgabenlisten, reine Absätze.
// Ein offener Fence (Antwort streamt noch) bleibt Markdown, bis er geschlossen ist.
export function splitMessage(text: string, o: SplitOptions): Segment[] {
  const lines = text.split('\n')
  const out: Segment[] = []
  let buf: string[] = []
  const flush = () => {
    const md = buf.join('\n')
    buf = []
    if (md.trim() === '') return
    if (!o.inline) return void out.push({ kind: 'md', text: md })
    for (const para of md.split(/\n\s*\n/)) {
      if (para.trim() === '') continue
      const kind = MD_SYNTAX.test(para) ? 'md' : 'plain'
      const last = out[out.length - 1]
      if (kind === 'md' && last?.kind === 'md') last.text += `\n\n${para}`
      else out.push({ kind, text: para })
    }
  }
  let i = 0
  while (i < lines.length) {
    const line = lines[i] ?? ''
    const fence = /^ {0,3}(`{3,}|~{3,})\s*([\w+#.-]*)/.exec(line)
    if (fence) {
      const mark = fence[1] ?? '```'
      const lang = (fence[2] ?? '').toLowerCase()
      let j = i + 1
      while (j < lines.length && !((lines[j] ?? '').trim().startsWith(mark.charAt(0).repeat(mark.length)) && (lines[j] ?? '').trim().replace(/[`~]/g, '') === '')) j++
      if (j >= lines.length) {
        buf.push(...lines.slice(i))
        break
      }
      const source = lines.slice(i + 1, j).join('\n')
      const diagram = o.mermaid && lang === 'mermaid' ? parseMermaid(source) : null
      if (diagram !== null) {
        flush()
        out.push({ kind: 'mermaid', diagram, source })
      } else if (o.math && MATH_FENCE.has(lang)) {
        flush()
        out.push({ kind: 'math', tex: source, text: texToText(source) })
      } else if (o.cards && !NATIVE_FENCE.has(lang)) {
        flush()
        out.push({ kind: 'code', lang, source })
      } else buf.push(...lines.slice(i, j + 1))
      i = j + 1
      continue
    }
    // $$…$$ als Block (eine oder mehrere Zeilen); ohne schließendes $$ (Streaming) bleibt es Markdown.
    const dd = o.math ? /^\s*\$\$(.*)$/.exec(line) : null
    if (dd) {
      const rest = dd[1] ?? ''
      let j = i
      let tex: string | null = null
      if (/\$\$\s*$/.test(rest)) tex = rest.replace(/\$\$\s*$/, '')
      else {
        j = i + 1
        while (j < lines.length && !/\$\$\s*$/.test(lines[j] ?? '')) j++
        if (j < lines.length) tex = [rest, ...lines.slice(i + 1, j), (lines[j] ?? '').replace(/\$\$\s*$/, '')].join('\n')
      }
      if (tex !== null && tex.trim() !== '') {
        flush()
        out.push({ kind: 'math', tex: tex.trim(), text: texToText(tex.trim()) })
        i = j + 1
        continue
      }
    }
    if (o.cards && line.includes('|') && TABLE_SEP.test(lines[i + 1] ?? '')) {
      let j = i + 2
      while (j < lines.length && (lines[j] ?? '').includes('|') && (lines[j] ?? '').trim() !== '') j++
      flush()
      const cols = line.replace(/^\s*\||\|\s*$/g, '').split('|').length
      out.push({ kind: 'table', source: lines.slice(i, j).join('\n'), rows: j - i - 2, cols })
      i = j
      continue
    }
    const alert = o.inline ? /^\s*>\s*\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*$/i.exec(line) : null
    if (alert) {
      let j = i + 1
      while (j < lines.length && /^\s*>/.test(lines[j] ?? '')) j++
      flush()
      out.push({ kind: 'alert', alert: (alert[1] ?? 'note').toLowerCase() as AlertKind, text: lines.slice(i + 1, j).map(l => l.replace(/^\s*>\s?/, '')).join('\n') })
      i = j
      continue
    }
    if (o.inline && TASK.test(line)) {
      let j = i
      while (j < lines.length && TASK.test(lines[j] ?? '')) j++
      flush()
      const items = lines.slice(i, j)
      out.push({ kind: 'tasks', text: items.join('\n'), done: items.filter(l => /\[[xX]\]/.test(l)).length, total: items.length })
      i = j
      continue
    }
    buf.push(line)
    i++
  }
  flush()
  return out
}

// Lohnt eine eigene Zeichnung? Nur mit Karte, Hinweis, Aufgaben oder einem reinen Absatz, in dem etwas gefärbt wird.
export const isStyled = (segs: readonly Segment[]): boolean =>
  segs.some(s => (s.kind === 'plain' ? inlineTokens(s.text).some(p => p.tone !== undefined) : s.kind !== 'md'))

// Dateipfad eines Tool-Aufrufs aus dem tool.call-Event, für „in diesem Turn geändert“.
export function inputPath(e: unknown): string {
  const o = (e ?? {}) as Record<string, unknown>
  const i = (o.input ?? o) as Record<string, unknown>
  const p = [i.file_path, i.notebook_path].find(x => typeof x === 'string' && x !== '')
  return typeof p === 'string' ? p : ''
}

export type Tone = 'path' | 'touched' | 'version' | 'number'
export const TONE_COLOR: Record<Tone, string> = { path: THEME.accent, touched: THEME.success, version: THEME.max, number: THEME.warning }

const TOKEN = /((?:[A-Za-z]:)?(?:[\w.-]+[\\/])+[\w.-]+|\b[\w-]+\.(?:tsx?|jsx?|json|md|py|cs|sql|ya?ml|toml|css|html|sh|ps1|txt)\b)|(\bv?\d+\.\d+\.\d+(?:-[\w.]+)?\b)|(\b\d+(?:[.,]\d+)?\s?%?)/g

const baseName = (p: string): string => p.replace(/\\/g, '/').split('/').pop()?.toLowerCase() ?? ''

// Teilt einen reinen Absatz in Stücke mit Farbrolle: Pfade (in diesem Turn geändert: eigene Rolle), Versionen, Zahlen.
export function inlineTokens(text: string, touched: readonly string[] = []): { text: string; tone?: Tone }[] {
  const names = new Set(touched.map(baseName))
  const out: { text: string; tone?: Tone }[] = []
  let at = 0
  for (const m of text.matchAll(TOKEN)) {
    const idx = m.index ?? 0
    if (idx > at) out.push({ text: text.slice(at, idx) })
    const tone: Tone = m[1] ? (names.has(baseName(m[1])) ? 'touched' : 'path') : m[2] ? 'version' : 'number'
    out.push({ text: m[0], tone })
    at = idx + m[0].length
  }
  if (at < text.length) out.push({ text: text.slice(at) })
  return out
}

export const lineCountOf = (source: string): number => (source === '' ? 0 : source.split('\n').length)

// ── 9 · Rückfrage-Kopf ───────────────────────────────────────────────────────

// Kopfzeile über dem Rückfrage-Dialog: Anzahl und die Kurztitel (`header`) der Fragen; der Dialog selbst bleibt der Engine.
export function askSummary(questions: readonly unknown[], lang: UiLang): string {
  const heads = questions.map(q => (q as { header?: unknown } | null)?.header).filter((h): h is string => typeof h === 'string' && h.trim() !== '')
  const base = t(lang, questions.length === 1 ? 'skin.askOne' : 'skin.askMany', { n: questions.length })
  return heads.length === 0 ? base : `${base} · ${oneLine(heads.join(', '), 80)}`
}

// ── 8 · Formeln ($$…$$) ───────────────────────────────────────────────────────

const TEX_SYMBOL: Record<string, string> = {
  alpha: 'α', beta: 'β', gamma: 'γ', delta: 'δ', epsilon: 'ε', varepsilon: 'ε', zeta: 'ζ', eta: 'η', theta: 'θ', iota: 'ι', kappa: 'κ', lambda: 'λ', mu: 'μ',
  nu: 'ν', xi: 'ξ', pi: 'π', rho: 'ρ', sigma: 'σ', tau: 'τ', upsilon: 'υ', phi: 'φ', varphi: 'φ', chi: 'χ', psi: 'ψ', omega: 'ω',
  Gamma: 'Γ', Delta: 'Δ', Theta: 'Θ', Lambda: 'Λ', Xi: 'Ξ', Pi: 'Π', Sigma: 'Σ', Phi: 'Φ', Psi: 'Ψ', Omega: 'Ω',
  sum: '∑', prod: '∏', int: '∫', oint: '∮', partial: '∂', nabla: '∇', infty: '∞', pm: '±', mp: '∓', times: '×', cdot: '·', div: '÷', ast: '∗',
  leq: '≤', le: '≤', geq: '≥', ge: '≥', neq: '≠', ne: '≠', approx: '≈', equiv: '≡', sim: '∼', propto: '∝', to: '→', rightarrow: '→', leftarrow: '←',
  Rightarrow: '⇒', Leftarrow: '⇐', Leftrightarrow: '⇔', implies: '⇒', iff: '⇔', mapsto: '↦', in: '∈', notin: '∉', subset: '⊂', subseteq: '⊆', supset: '⊃',
  cup: '∪', cap: '∩', emptyset: '∅', forall: '∀', exists: '∃', neg: '¬', land: '∧', lor: '∨', ldots: '…', cdots: '⋯', dots: '…', circ: '∘', degree: '°',
  quad: ' ', qquad: '  ', ',': ' ', ';': ' ', '!': '', ' ': ' ', '{': '{', '}': '}', '%': '%', '$': '$', '&': '&', '#': '#', '_': '_',
  lim: 'lim', log: 'log', ln: 'ln', exp: 'exp', sin: 'sin', cos: 'cos', tan: 'tan', max: 'max', min: 'min', det: 'det',
}
const SUP: Record<string, string> = { '0': '⁰', '1': '¹', '2': '²', '3': '³', '4': '⁴', '5': '⁵', '6': '⁶', '7': '⁷', '8': '⁸', '9': '⁹', '+': '⁺', '-': '⁻', '=': '⁼', '(': '⁽', ')': '⁾', n: 'ⁿ', i: 'ⁱ', T: 'ᵀ' }
const SUB: Record<string, string> = { '0': '₀', '1': '₁', '2': '₂', '3': '₃', '4': '₄', '5': '₅', '6': '₆', '7': '₇', '8': '₈', '9': '₉', '+': '₊', '-': '₋', '=': '₌', '(': '₍', ')': '₎', a: 'ₐ', e: 'ₑ', i: 'ᵢ', j: 'ⱼ', k: 'ₖ', n: 'ₙ', o: 'ₒ', x: 'ₓ', t: 'ₜ' }

// Hoch-/Tiefstellung: als Unicode, wenn jedes Zeichen eins hat, sonst „^(…)“ bzw. „_(…)“.
const script = (s: string, map: Record<string, string>, mark: string): string =>
  [...s].every(c => map[c] !== undefined) ? [...s].map(c => map[c]).join('') : s.length === 1 ? `${mark}${s}` : `${mark}(${s})`

// LaTeX → lesbarer Unicode-Text; null, wenn ein Befehl übrig bleibt, den wir nicht kennen (dann zeigt die Karte den Quelltext).
// ponytail: nur Symbole, \frac, \sqrt, \text, Hoch-/Tiefstellung und Umgebungszeilen; kein Layout in 2D, keine Matrizen.
export function texToText(tex: string): string | null {
  const ARG = '\\{([^{}]*)\\}'
  let s = tex
    .replace(/\\\\/g, '\n')
    .replace(/\\(left|right|big|Big|bigg|Bigg|displaystyle|limits)\b\.?/g, '')
    .replace(/\\begin\{[a-z*]+\}|\\end\{[a-z*]+\}/g, '')
  for (let i = 0; i < 8; i++) {
    const before = s
    s = s
      .replace(/\\(?:text|mathrm|mathbf|mathit|mathsf|mathbb|mathcal|operatorname)\{([^{}]*)\}/g, '$1')
      .replace(new RegExp(`\\\\[dt]?frac${ARG}${ARG}`, 'g'), (_, a: string, b: string) => `${a.length > 1 ? `(${a})` : a}/${b.length > 1 ? `(${b})` : b}`)
      .replace(new RegExp(`\\\\sqrt${ARG}`, 'g'), (_, a: string) => `√${a.length > 1 ? `(${a})` : a}`)
      .replace(/\^\{([^{}\\]*)\}/g, (_, a: string) => script(a, SUP, '^'))
      .replace(/_\{([^{}\\]*)\}/g, (_, a: string) => script(a, SUB, '_'))
      .replace(/\\([A-Za-z]+|[,;! {}%$&#_])/g, (m, name: string) => TEX_SYMBOL[name] ?? m)
    if (s === before) break
  }
  // Einzelzeichen erst zum Schluss, damit „^(…)“ aus der Klammerform nicht noch einmal gelesen wird.
  s = s.replace(/\^([^\s{}\\(])/g, (_, a: string) => script(a, SUP, '^')).replace(/_([^\s{}\\(])/g, (_, a: string) => script(a, SUB, '_'))
  s = s.replace(/&/g, ' ').replace(/[{}]/g, '').replace(/[ \t]+/g, ' ')
  if (/\\[A-Za-z]/.test(s)) return null
  return s
    .split('\n')
    .map(l => l.trim())
    .filter(l => l !== '')
    .join('\n')
}

// Formel als einfaches Bild: je Zeile zentriert, Serifenschrift; gleiche Eingabe ergibt dieselbe Quelle.
export function mathSvg(text: string, width = 560): string {
  const lines = text.split('\n')
  const lineH = 26
  const w = Math.max(width, ...lines.map(l => l.length * 10 + 32))
  const h = lines.length * lineH + 16
  const body = lines.map((l, i) => `<text x="${w / 2}" y="${8 + (i + 1) * lineH - 7}" text-anchor="middle" xml:space="preserve">${esc(l)}</text>`).join('')
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}" font-family="Cambria Math, STIX Two Math, Times New Roman, serif" font-size="18" font-style="italic" fill="${THEME.orga}">${body}</svg>`
}
