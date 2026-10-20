// Kontext-Inspektor (Idee: context-inspector aus YeonwooSung/my-claude-code-mods, Apache-2.0, kein Code übernommen):
// was das Kontextfenster füllt, was doppelt hereinkommt und was eine Kompaktierung behält oder verliert. Rein
// beobachtend: die Funktionen lesen Events, greifen nie ein (kein deny, kein Umschreiben). Reine Funktionen ohne `$`,
// ohne Netzwerk und ohne Dateizugriffe.
//
// Zustand: `InspectState` aus `newInspect()` ist ein einfaches Objekt (Arrays und Records, keine Klassen). Die
// `ingest*`-Funktionen und `sampleInput` ändern ihn an Ort und Stelle (mutierend, Rückgabe `void`); `summarize` liest
// nur. Gedacht als Modul-Variable in register.tsx: ein Reload startet leer, ein Atom wäre bei 20.000 Items zu groß.
// Rohinhalte werden nicht gespeichert, nur Länge, Hash und SimHash je Item.
//
// Verdrahtung (Feldnamen am claude-code.d.ts 2.1.299 geprüft):
// - `ingestAppend(st, e)` ← `on('session.append', …)`: `e.message` ({ type, name?, isMeta?, content: ApiContentBlock[] }),
//   `e.door` ('prompt' | 'response' | 'tool-result' | 'attachment' | 'compaction' …), `e.uuid`, `e.agentId?`.
//   tool_result-Blöcke (`tool_use_id`, `content`, `is_error`) werden je Aufruf ein Item, der Rest der Zeile ein Item.
// - `ingestToolCall(st, id, tool, agentId, input)` ← `on('tool.call', …)` vor `await next(e)`: `e.tool_use_id`,
//   `e.tool`, `e.agentId`, `e` selbst als `input` (die Argumente liegen im Event, z. B. `file_path`, `command`).
// - `ingestToolResult(st, id, answer)` ← dasselbe `tool.call` nach `const answer = await next(e)`: Ergebnisform
//   `{ deny }` oder `{ result, text?, isError?, context?, ref? }`; Read meldet `result.type === 'file_unchanged'`, wenn
//   die Engine die Datei als unverändert ersetzt (engine-deduped), `result.type === 'image'` zählt als Bild.
// - `sampleInput(st, agentId, inputTokens)` ← `on('turn.step', async function* …)`: je Chunk mit `kind === 'stop'`
//   `chunk.usage` (TurnUsage = ModelUsage, kann `null` sein) und `e.agentId`. Input-Tokens =
//   `usage.input_tokens + usage.cache_read_input_tokens + usage.cache_creation_input_tokens` (uncached + cache read +
//   cache write), siehe `inputOf`.
// - `ingestCompaction(st, e, result)` ← `on('session.compact', …)` mit `const result = await next(e)`: Eingang
//   `e.trigger` ('manual' | 'auto' | 'plugin' | 'precompute'), `e.agentId?`, `e.messages` (SessionMessage: role, text,
//   toolUses[{ tool_use_id, tool, input, text?, isError? }], toolResults?[{ tool_use_id, text, isError }], handle?);
//   Ergebnis `{ messages, tokensBefore?, tokensAfter? }` oder `{ skip }`. 'precompute' und `skip` zählen nicht.
//   Vermutet (ungeprüft): `handle` einer SessionMessage entspricht der `uuid` aus session.append. Tool-Ergebnisse
//   bleiben zusätzlich über `tool_use_id` zuordenbar.
import type { UiLang } from '../types'
import { oneLine } from './core'

// ── Grenzen ──────────────────────────────────────────────────────────────────────────────────────────────────────────
export const MAX_ITEMS = 20_000
export const TARGET_MAX = 200
export const IMAGE_TOKENS = 1500
export const K_MIN = 0.5
export const K_MAX = 2
// Unter dieser geschätzten Summe bleibt k = 1: zu wenig Messung für eine Aussage.
const K_MIN_EST = 200
// Ähnlichkeit erst ab dieser Länge; sehr große Ausgaben nur exakt (eine Stichprobe rechtfertigt keine Verschwendung).
const SIM_MIN_CHARS = 200
const NEAR_MAX_CHARS = 262_144
const NEAR_MAX_WORDS = 20_000
export const HAMMING_MAX = 3
const BUCKET_REPS = 256
const MAX_CALLS = 2000
const MAX_EDITS = 5000
const MAX_TURNS = 500
// Schwellen der Hinweise (geschätzte Tokens).
const LARGE_READ = 8000
const LARGE_READ_KEEP = 2000
const LARGE_SHELL = 4000
const SHELL_KEEP = 1000
const AGENT_VERBOSE = 3000
const AGENT_KEEP = 1000
const REFLOW_MIN = 2000
const REPEAT_MIN = 1000

const MAIN = 'main'
const EDIT_TOOLS = new Set(['Edit', 'MultiEdit', 'Write', 'NotebookEdit'])
const SHELL_TOOLS = new Set(['Bash', 'PowerShell'])
const AGENT_TOOLS = new Set(['Agent', 'Task'])

// ── Typen ────────────────────────────────────────────────────────────────────────────────────────────────────────────
export type TargetKind = 'file' | 'command' | 'pattern' | 'url' | 'agent'
export type Target = { kind: TargetKind; value: string }

export type InspectItem = {
  seq: number
  ctx: string
  turn: number
  door: string
  category: string
  row?: string
  id?: string
  target?: Target
  range?: string
  whole?: true
  chars: number
  images: number
  // geschätzte Tokens vor Kalibrierung: chars/4 + Bilder × 1500
  raw: number
  hash: string
  sim?: [number, number]
  // Kompaktierungs-Epoche des Kontexts beim Eintritt bzw. beim Hinauswurf
  born: number
  died?: number
  live: boolean
  deduped?: true
  attached?: true
  isError?: true
}

type Call = { tool: string; ctx: string; target?: Target; range?: string; whole?: true }
type Edit = { seq: number; id: string; ctx: string; turn: number; path: string; lines: number }
type TurnStat = { turn: number; first?: number; last?: number; lines: number }
type EntitySet = { all: string[]; lost: string[] }

export type CompactionRecord = {
  ctx: string
  epoch: number
  trigger: string
  before?: number
  after?: number
  droppedItems: number
  droppedRaw: number
  edited: EntitySet
  read: EntitySet
  errors: EntitySet
}

export type InspectState = {
  seq: number
  turn: number
  items: InspectItem[]
  byId: Record<string, InspectItem>
  calls: Record<string, Call>
  edits: Edit[]
  turns: TurnStat[]
  epoch: Record<string, number>
  // letzte gemessene Input-Größe, geschätztes Wachstum seit dieser Messung, Kompaktierung dazwischen – je Kontext
  last: Record<string, number>
  pending: Record<string, number>
  broken: Record<string, boolean>
  calMeasured: number
  calEst: number
  samples: number
  measured?: number
  compactions: CompactionRecord[]
  dropped: number
}

export type ReReadKind = 'unchanged-live' | 'after-edit' | 'after-compaction' | 'restored' | 'different-range' | 'changed-externally' | 'engine-deduped'
export const REREAD_KINDS: readonly ReReadKind[] = ['unchanged-live', 'after-edit', 'after-compaction', 'restored', 'different-range', 'changed-externally', 'engine-deduped']

export type ReRead = { ctx: string; path: string; kind: ReReadKind; est: number; wasteNow: boolean; epoch: number }
export type SimilarGroup = { ctx: string; kind: 'exact' | 'near'; label: string; count: number; est: number; extra: number; wasteNow: number }
export type Share = { name: string; est: number; count: number }
export type TurnEfficiency = { turn: number; lines: number; growth: number; exploring: boolean }
export type CompactionView = Omit<CompactionRecord, 'droppedRaw'> & { droppedEst: number; reflow: number }
export type AdviceKey = 'reread' | 'largeRead' | 'largeShell' | 'repeated' | 'lostEdits' | 'reflow' | 'agentVerbose'
export type Advice = { key: AdviceKey; vars: Record<string, string | number>; saving: number }

export type InspectSummary = {
  scope: 'main' | 'all'
  items: number
  liveItems: number
  dropped: number
  liveEst: number
  measured?: number
  k: number
  kClamped: boolean
  calibrated: boolean
  waste: number
  occupancy: { byCategory: Share[]; byTarget: Share[] }
  rereads: { rows: ReRead[]; counts: Record<ReReadKind, number>; est: Record<ReReadKind, number>; wasteNow: number; wasteSession: number }
  similar: { groups: SimilarGroup[]; wasteNow: number }
  compactions: CompactionView[]
  efficiency: { turns: TurnEfficiency[]; lines: number; growth: number; exploring: number }
  advice: Advice[]
}

// ── Kleine Helfer ────────────────────────────────────────────────────────────────────────────────────────────────────
const str = (v: unknown): string | undefined => (typeof v === 'string' && v.trim() !== '' ? v : undefined)
const rec = (v: unknown): Record<string, unknown> => (v !== null && typeof v === 'object' ? (v as Record<string, unknown>) : {})
const ctxOf = (agentId: string | undefined): string => agentId ?? MAIN
const lineCount = (v: unknown): number => (typeof v === 'string' && v !== '' ? v.split('\n').length : 0)
export const normPath = (p: string): string => p.replace(/\\/g, '/')
const baseName = (p: string): string => normPath(p).split('/').pop() ?? p
const shortPath = (p: string): string => normPath(p).split('/').slice(-3).join('/')

/** Secrets in Befehlen und URLs maskieren (best effort, kein vollständiger Detektor). */
export function maskSecrets(s: string): string {
  return s
    .replace(/\b(authorization\s*[:=]\s*)(?:(bearer|basic|token)\s+)?(["']?)[^\s"']+/gi, (_m, head: string, scheme: string | undefined, q: string) => `${head}${scheme ? `${scheme} ` : ''}${q}***`)
    .replace(/\b(bearer\s+)[A-Za-z0-9._~+/=-]+/gi, '$1***')
    .replace(/(--(?:password|passwd|token|access-token|api-key|apikey|secret)(?:=|\s+))("[^"]*"|'[^']*'|\S+)/gi, '$1***')
    .replace(/\b([A-Za-z0-9_]*(?:KEY|TOKEN|SECRET|PASSWORD|PASSWD)[A-Za-z0-9_]*)=("[^"]*"|'[^']*'|[^\s;&|]+)/gi, '$1=***')
    .replace(/(:\/\/[^\s:/@]+:)[^\s@/]+@/g, '$1***@')
    .replace(/((?:^|\s)(?:-u\s+|--user[=\s]+))([^\s:]+):\S+/g, '$1$2:***')
}

/** Ziel für Anzeige und Export: erst maskieren, dann einzeilig auf 200 Zeichen. */
export const clipTarget = (s: string): string => oneLine(maskSecrets(s), TARGET_MAX)

/** Ziel eines Tool-Aufrufs: Datei, Befehl, Muster, URL oder Agent. Pfade bleiben ungekürzt und unmaskiert. */
export function targetOf(tool: string, input: unknown): Target | undefined {
  const i = rec(input)
  if (AGENT_TOOLS.has(tool)) {
    const d = str(i.description) ?? str(i.subagent_type) ?? str(i.prompt)
    return d === undefined ? undefined : { kind: 'agent', value: clipTarget(d) }
  }
  // Grep/Glob: `path` ist nur der Suchort, das Ziel ist das Muster.
  const path = str(i.file_path) ?? str(i.notebook_path) ?? (tool === 'Grep' || tool === 'Glob' ? undefined : str(i.path))
  if (path !== undefined) return { kind: 'file', value: normPath(path) }
  const cmd = str(i.command)
  if (cmd !== undefined) return { kind: 'command', value: clipTarget(cmd) }
  const url = str(i.url) ?? str(i.uri)
  if (url !== undefined) return { kind: 'url', value: clipTarget(url) }
  const pattern = str(i.pattern) ?? str(i.query)
  return pattern === undefined ? undefined : { kind: 'pattern', value: clipTarget(pattern) }
}

/** Geänderte Zeilen eines Schreib-Tools (Edit: die längere Seite, Write: der ganze Inhalt). */
export function editLines(tool: string, input: unknown): number {
  const i = rec(input)
  const pair = (x: Record<string, unknown>) => Math.max(lineCount(x.old_string), lineCount(x.new_string))
  if (tool === 'Edit') return pair(i)
  if (tool === 'MultiEdit') return Array.isArray(i.edits) ? i.edits.reduce((n: number, x) => n + pair(rec(x)), 0) : 0
  if (tool === 'Write') return lineCount(i.content)
  if (tool === 'NotebookEdit') return Math.max(1, lineCount(i.new_source))
  return 0
}

// ── Hashes ───────────────────────────────────────────────────────────────────────────────────────────────────────────
function fnv(s: string, seed: number): number {
  let h = seed >>> 0
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i)
    h = Math.imul(h, 0x01000193)
  }
  return h >>> 0
}
// Murmur3-Finalizer: verteilt die Bits, damit die zwei Hälften des SimHash nicht korrelieren.
function mix(h: number): number {
  h ^= h >>> 16
  h = Math.imul(h, 0x85ebca6b)
  h ^= h >>> 13
  h = Math.imul(h, 0xc2b2ae35)
  return (h ^ (h >>> 16)) >>> 0
}
const hex = (n: number): string => n.toString(16).padStart(8, '0')

/** Exakter Inhalts-Hash (64 Bit aus zwei 32-Bit-FNV). */
export const contentHash = (s: string): string => hex(mix(fnv(s, 0x811c9dc5))) + hex(mix(fnv(s, 0x2b992ddf)))

/** Wörter für den SimHash: klein, Zahlen zu `#`, Satzzeichen weg. */
export const simWords = (text: string): string[] =>
  text.toLowerCase().replace(/\d+(?:[.,:]\d+)*/g, '#').split(/[^\p{L}\p{N}#_]+/u).filter(w => w !== '')

/** 64-Bit-SimHash über Wort-3-Shingles als [hi, lo]; `undefined` bei leerem oder zu großem Text. */
export function simHash(text: string): [number, number] | undefined {
  if (text.length > NEAR_MAX_CHARS) return undefined
  const words = simWords(text)
  if (words.length === 0 || words.length > NEAR_MAX_WORDS) return undefined
  const v = new Int32Array(64)
  const n = Math.max(1, words.length - 2)
  for (let i = 0; i < n; i++) {
    const sh = words.slice(i, i + 3).join(' ')
    const a = mix(fnv(sh, 0x811c9dc5))
    const b = mix(fnv(sh, 0x2b992ddf))
    for (let bit = 0; bit < 32; bit++) {
      v[bit] = (v[bit] ?? 0) + (((a >>> bit) & 1) === 1 ? 1 : -1)
      v[bit + 32] = (v[bit + 32] ?? 0) + (((b >>> bit) & 1) === 1 ? 1 : -1)
    }
  }
  let lo = 0
  let hi = 0
  for (let bit = 0; bit < 32; bit++) {
    if ((v[bit] ?? 0) > 0) lo |= 1 << bit
    if ((v[bit + 32] ?? 0) > 0) hi |= 1 << bit
  }
  return [hi >>> 0, lo >>> 0]
}

const pop = (x: number): number => {
  x -= (x >>> 1) & 0x55555555
  x = (x & 0x33333333) + ((x >>> 2) & 0x33333333)
  return Math.imul((x + (x >>> 4)) & 0x0f0f0f0f, 0x01010101) >>> 24
}
export const hamming = (a: [number, number], b: [number, number]): number => pop((a[0] ^ b[0]) >>> 0) + pop((a[1] ^ b[1]) >>> 0)

// ── Inhalt der Blöcke ────────────────────────────────────────────────────────────────────────────────────────────────
type Measured = { text: string; images: number }

function blockText(b: unknown, acc: Measured): void {
  if (typeof b === 'string') {
    acc.text += b
    return
  }
  const x = rec(b)
  if (x.type === 'text' && typeof x.text === 'string') acc.text += x.text
  else if (x.type === 'image') acc.images += 1
  else if (x.type === 'thinking' && typeof x.thinking === 'string') acc.text += x.thinking
  else if (x.type === 'tool_use') acc.text += `${String(x.name ?? '')} ${JSON.stringify(x.input ?? {})}`
  else if (x.type === 'tool_result') contentText(x.content, acc)
  else acc.text += JSON.stringify(b) ?? ''
}
function contentText(content: unknown, acc: Measured): void {
  if (Array.isArray(content)) for (const b of content) blockText(b, acc)
  else if (content !== undefined && content !== null) blockText(content, acc)
}

// ── Ledger ───────────────────────────────────────────────────────────────────────────────────────────────────────────
export function newInspect(): InspectState {
  return {
    seq: 0, turn: 0, items: [], byId: {}, calls: {}, edits: [], turns: [], epoch: {}, last: {}, pending: {}, broken: {},
    calMeasured: 0, calEst: 0, samples: 0, compactions: [], dropped: 0,
  }
}

const rawOf = (chars: number, images: number): number => Math.ceil(chars / 4) + images * IMAGE_TOKENS

// Neues Item oder (gleiche tool_use_id) Messung ersetzen; das geschätzte Wachstum für die Kalibrierung mitführen.
function upsert(st: InspectState, ctx: string, base: Omit<InspectItem, 'seq' | 'ctx' | 'turn' | 'raw' | 'hash' | 'born' | 'live' | 'chars' | 'images' | 'sim'>, m: Measured): void {
  const chars = m.text.length
  const raw = rawOf(chars, m.images)
  const hash = contentHash(m.text)
  const sim = base.id !== undefined && chars >= SIM_MIN_CHARS ? simHash(m.text) : undefined
  const old = base.id === undefined ? undefined : st.byId[base.id]
  if (old !== undefined) {
    st.pending[old.ctx] = (st.pending[old.ctx] ?? 0) + raw - old.raw
    Object.assign(old, base, { chars, images: m.images, raw, hash })
    if (sim === undefined) delete old.sim
    else old.sim = sim
    return
  }
  const item: InspectItem = { ...base, seq: ++st.seq, ctx, turn: st.turn, chars, images: m.images, raw, hash, born: st.epoch[ctx] ?? 0, live: true }
  if (sim !== undefined) item.sim = sim
  st.items.push(item)
  if (item.id !== undefined) st.byId[item.id] = item
  st.pending[ctx] = (st.pending[ctx] ?? 0) + raw
  if (st.items.length > MAX_ITEMS) dropOne(st)
}

// Deckel: zuerst das älteste Item außerhalb des Fensters, sonst das älteste überhaupt.
function dropOne(st: InspectState): void {
  const dead = st.items.findIndex(i => !i.live)
  const [gone] = st.items.splice(dead >= 0 ? dead : 0, 1)
  if (gone?.id !== undefined && st.byId[gone.id] === gone) delete st.byId[gone.id]
  st.dropped += 1
}

function turnStat(st: InspectState, turn: number): TurnStat {
  let t = st.turns[st.turns.length - 1]
  if (t?.turn !== turn) {
    t = { turn, lines: 0 }
    st.turns.push(t)
    if (st.turns.length > MAX_TURNS) st.turns.shift()
  }
  return t
}

const FILE_ATTACHMENT = /"file_path"\s*:\s*"((?:[^"\\]|\\.)*)"/

export type AppendIn = {
  message: { type: string; name?: string; isMeta?: boolean; content: unknown }
  door: string
  uuid?: string
  agentId?: string
}

/** Eine Zeile, die in eine Unterhaltung kommt: ein Item je tool_result-Block, eines für den Rest. */
export function ingestAppend(st: InspectState, e: AppendIn): void {
  const ctx = ctxOf(e.agentId)
  if (ctx === MAIN && e.door === 'prompt' && e.message.isMeta !== true) st.turn += 1
  const content = typeof e.message.content === 'string' ? [{ type: 'text', text: e.message.content }] : e.message.content
  const rest: Measured = { text: '', images: 0 }
  for (const b of Array.isArray(content) ? content : []) {
    const x = rec(b)
    const id = x.type === 'tool_result' ? str(x.tool_use_id) : undefined
    if (id === undefined) {
      blockText(b, rest)
      continue
    }
    const m: Measured = { text: '', images: 0 }
    contentText(x.content, m)
    const call = st.calls[id]
    const old = st.byId[id]
    upsert(st, call?.ctx ?? old?.ctx ?? ctx, {
      ...(old === undefined ? {} : stripMeasure(old)),
      id, door: e.door, category: call?.tool ?? old?.category ?? 'tool',
      ...(e.uuid === undefined ? {} : { row: e.uuid }),
      ...(call?.target === undefined ? {} : { target: call.target }),
      ...(call?.range === undefined ? {} : { range: call.range }),
      ...(call?.whole === undefined ? {} : { whole: call.whole }),
      ...(x.is_error === true ? { isError: true as const } : {}),
    }, m)
    delete st.calls[id]
  }
  if (rest.text === '' && rest.images === 0) return
  const attachment = e.message.type === 'attachment'
  const name = e.message.name ?? '?'
  const file = attachment && name === 'file' ? FILE_ATTACHMENT.exec(rest.text)?.[1] : undefined
  upsert(st, ctx, {
    door: e.door,
    category: attachment ? `attachment:${name}` : e.door,
    ...(e.uuid === undefined ? {} : { row: e.uuid }),
    ...(file === undefined ? {} : { target: { kind: 'file' as const, value: normPath(file.replace(/\\\\/g, '\\')) }, attached: true as const }),
  }, rest)
}

// Beschreibende Felder eines vorhandenen Items (ohne Messwerte und Verwaltungsfelder) für das Zusammenführen.
function stripMeasure(i: InspectItem): Partial<InspectItem> {
  const { seq: _s, ctx: _c, turn: _t, raw: _r, hash: _h, born: _b, live: _l, chars: _ch, images: _im, sim: _si, died: _d, ...rest } = i
  return rest
}

/** Tool-Aufruf merken (Ziel, Leseumfang) und Schreib-Tools als geänderte Zeilen buchen. */
export function ingestToolCall(st: InspectState, id: string | undefined, tool: string, agentId: string | undefined, input: unknown): void {
  if (id === undefined) return
  const ctx = ctxOf(agentId)
  const i = rec(input)
  const target = targetOf(tool, i)
  const call: Call = { tool, ctx }
  if (target !== undefined) call.target = target
  if (tool === 'Read') {
    call.range = [i.offset, i.limit, i.pages].map(v => (v === undefined ? '' : String(v))).join(':')
    if (call.range === '::') call.whole = true
  }
  st.calls[id] = call
  const keys = Object.keys(st.calls)
  if (keys.length > MAX_CALLS) delete st.calls[keys[0] as string]
  if (EDIT_TOOLS.has(tool) && target?.kind === 'file') {
    const lines = editLines(tool, i)
    st.edits.push({ seq: ++st.seq, id, ctx, turn: st.turn, path: target.value, lines })
    if (st.edits.length > MAX_EDITS) st.edits.shift()
    if (ctx === MAIN) turnStat(st, st.turn).lines += lines
  }
}

/** Antwort eines Tool-Aufrufs: Fehler/Ablehnung nimmt die Zeilen zurück, sonst ein Item (vom Zeilen-Append ersetzt). */
export function ingestToolResult(st: InspectState, id: string | undefined, answer: unknown): void {
  if (id === undefined) return
  const a = rec(answer)
  const call = st.calls[id]
  if (typeof a.deny === 'string' || a.isError === true) {
    const at = st.edits.findIndex(x => x.id === id)
    const edit = st.edits[at]
    if (edit !== undefined) {
      st.edits.splice(at, 1)
      const t = st.turns.find(x => x.turn === edit.turn)
      if (t !== undefined && edit.ctx === MAIN) t.lines -= edit.lines
    }
  }
  if (typeof a.deny === 'string' || call === undefined) return
  const result = rec(a.result)
  const m: Measured = {
    text: typeof a.text === 'string' ? a.text : typeof a.result === 'string' ? a.result : (JSON.stringify(a.result ?? '') ?? ''),
    images: result.type === 'image' ? 1 : 0,
  }
  if (result.type === 'image' && typeof a.text !== 'string') m.text = ''
  upsert(st, call.ctx, {
    id, door: 'tool-result', category: call.tool,
    ...(call.target === undefined ? {} : { target: call.target }),
    ...(call.range === undefined ? {} : { range: call.range }),
    ...(call.whole === undefined ? {} : { whole: call.whole }),
    ...(result.type === 'file_unchanged' ? { deduped: true as const } : {}),
    ...(a.isError === true ? { isError: true as const } : {}),
  }, m)
}

/** Usage eines `stop`-Chunks → Input-Tokens (uncached + cache read + cache write). */
export function inputOf(u: { input_tokens: number; cache_read_input_tokens: number; cache_creation_input_tokens: number } | null | undefined): number | undefined {
  return u === null || u === undefined ? undefined : u.input_tokens + u.cache_read_input_tokens + u.cache_creation_input_tokens
}

/** Gemessene Input-Größe einer Anfrage: kalibriert k und liefert das Wachstum je Turn. */
export function sampleInput(st: InspectState, agentId: string | undefined, inputTokens: number | undefined): void {
  if (inputTokens === undefined || !Number.isFinite(inputTokens) || inputTokens <= 0) return
  const ctx = ctxOf(agentId)
  const last = st.last[ctx]
  const pending = st.pending[ctx] ?? 0
  // Intervalle über eine Kompaktierung hinweg sagen nichts über k.
  if (last !== undefined && st.broken[ctx] !== true && pending > 0 && inputTokens > last) {
    st.calMeasured += inputTokens - last
    st.calEst += pending
  }
  st.last[ctx] = inputTokens
  st.pending[ctx] = 0
  st.broken[ctx] = false
  st.samples += 1
  if (ctx === MAIN) {
    st.measured = inputTokens
    const t = turnStat(st, st.turn)
    t.first ??= inputTokens
    t.last = inputTokens
  }
}

type CompactMsg = {
  text: string
  handle?: string
  toolUses?: readonly { tool_use_id: string; tool: string; input: Record<string, unknown>; text?: string; isError?: boolean }[]
  toolResults?: readonly { tool_use_id: string; text: string; isError: boolean }[]
}
export type CompactIn = { trigger: string; agentId?: string; messages: readonly CompactMsg[] }
export type CompactOut = { messages?: readonly CompactMsg[]; tokensBefore?: number; tokensAfter?: number; skip?: string }

const errorKey = (text: string): string => oneLine(text.split('\n').find(l => l.trim() !== '') ?? '', 80)

/** Kompaktierung: was nicht behalten wurde, verlässt das Fenster; Entitäten prüfen, ob sie überlebt haben. */
export function ingestCompaction(st: InspectState, e: CompactIn, result: CompactOut): void {
  if (e.trigger === 'precompute' || typeof result.skip === 'string' || result.messages === undefined) return
  const ctx = ctxOf(e.agentId)
  const before = new Set(e.messages.flatMap(m => (m.handle === undefined ? [] : [m.handle])))
  const keptRows = new Set<string>()
  const keptTools = new Set<string>()
  for (const m of result.messages) {
    // Die Zusammenfassung trägt auch ein handle, aber keines der ersetzten Nachrichten.
    if (m.handle !== undefined && before.has(m.handle)) keptRows.add(m.handle)
    for (const u of m.toolUses ?? []) keptTools.add(u.tool_use_id)
    for (const r of m.toolResults ?? []) keptTools.add(r.tool_use_id)
  }
  const epoch = (st.epoch[ctx] ?? 0) + 1
  st.epoch[ctx] = epoch
  st.broken[ctx] = true
  let droppedItems = 0
  let droppedRaw = 0
  for (const i of st.items) {
    if (i.ctx !== ctx || !i.live) continue
    if ((i.row !== undefined && keptRows.has(i.row)) || (i.id !== undefined && keptTools.has(i.id))) continue
    i.live = false
    i.died = epoch
    droppedItems += 1
    droppedRaw += i.raw
  }
  // Entitäten aus der Unterhaltung vor der Kompaktierung
  const edited = new Set<string>()
  const read = new Set<string>()
  const errors = new Set<string>()
  for (const m of e.messages) {
    for (const u of m.toolUses ?? []) {
      const p = str(u.input.file_path) ?? str(u.input.notebook_path)
      if (p !== undefined && EDIT_TOOLS.has(u.tool)) edited.add(normPath(p))
      else if (p !== undefined && u.tool === 'Read') read.add(normPath(p))
      if (u.isError === true && u.text !== undefined && errorKey(u.text) !== '') errors.add(errorKey(u.text))
    }
    for (const r of m.toolResults ?? []) if (r.isError && errorKey(r.text) !== '') errors.add(errorKey(r.text))
  }
  const corpus = normPath(
    result.messages.map(m => [m.text, ...(m.toolUses ?? []).map(u => JSON.stringify(u.input)), ...(m.toolResults ?? []).map(r => r.text)].join('\n')).join('\n'),
  ).replace(/\/\//g, '/')
  const survives = (s: string) => corpus.includes(s)
  const files = (set: Set<string>): EntitySet => {
    const all = [...set]
    return { all, lost: all.filter(p => !survives(p) && !survives(baseName(p))) }
  }
  const errs = [...errors]
  const rec0: CompactionRecord = {
    ctx, epoch, trigger: e.trigger, droppedItems, droppedRaw,
    edited: files(edited), read: files(read), errors: { all: errs, lost: errs.filter(x => !survives(normPath(x))) },
  }
  if (result.tokensBefore !== undefined) rec0.before = result.tokensBefore
  if (result.tokensAfter !== undefined) rec0.after = result.tokensAfter
  st.compactions.push(rec0)
}

// ── Analysen ─────────────────────────────────────────────────────────────────────────────────────────────────────────
const calibration = (st: InspectState): { k: number; clamped: boolean; calibrated: boolean } => {
  if (st.calEst < K_MIN_EST) return { k: 1, clamped: false, calibrated: false }
  const k = st.calMeasured / st.calEst
  const c = Math.min(K_MAX, Math.max(K_MIN, k))
  return { k: c, clamped: c !== k, calibrated: true }
}

function share<T>(items: readonly T[], key: (i: T) => string | undefined, est: (i: T) => number): Share[] {
  const m = new Map<string, Share>()
  for (const i of items) {
    const name = key(i)
    if (name === undefined) continue
    const s = m.get(name) ?? { name, est: 0, count: 0 }
    s.est += est(i)
    s.count += 1
    m.set(name, s)
  }
  return [...m.values()].sort((a, b) => b.est - a.est)
}

const aliveAt = (p: InspectItem, epoch: number): boolean => p.died === undefined || p.died > epoch

function rereads(items: readonly InspectItem[], edits: readonly Edit[], est: (i: InspectItem) => number): { rows: ReRead[]; pairs: [InspectItem, ReReadKind][] } {
  const reads = items.filter(i => i.target?.kind === 'file' && (i.category === 'Read' || i.attached === true))
  const prev = new Map<string, InspectItem>()
  const rows: ReRead[] = []
  const pairs: [InspectItem, ReReadKind][] = []
  for (const r of reads) {
    const path = r.target?.value as string
    const key = `${r.ctx}\u0000${path}`
    const p = prev.get(key)
    prev.set(key, r)
    if (p === undefined) continue
    let kind: ReReadKind
    if (r.deduped === true) kind = 'engine-deduped'
    else if (!aliveAt(p, r.born)) kind = r.attached === true ? 'restored' : 'after-compaction'
    else if ((p.range ?? '::') !== (r.range ?? '::')) kind = 'different-range'
    // Bearbeitungen aus jedem Kontext zählen: die Datei hat sich geändert, egal wer schrieb.
    else if (edits.some(x => x.path === path && x.seq > p.seq && x.seq < r.seq)) kind = 'after-edit'
    else if (p.hash !== r.hash) kind = 'changed-externally'
    else kind = 'unchanged-live'
    rows.push({ ctx: r.ctx, path, kind, est: est(r), wasteNow: kind === 'unchanged-live' && r.live && p.live, epoch: r.born })
    pairs.push([r, kind])
  }
  return { rows, pairs }
}

// Ähnliche Ausgaben je Kontext: exakter Hash, dann SimHash über 4 Bänder à 16 Bit (Hamming ≤ 3 ⇒ ein Band gleich).
function similar(items: readonly InspectItem[]): InspectItem[][] {
  // Datei-Reads deckt die Re-Read-Analyse ab, sonst zählte dieselbe Kopie doppelt.
  const cand = items.filter(i => i.id !== undefined && i.category !== 'Read' && i.chars >= SIM_MIN_CHARS)
  const parent = cand.map((_, n) => n)
  const find = (n: number): number => {
    while (parent[n] !== n) {
      parent[n] = parent[parent[n] as number] as number
      n = parent[n] as number
    }
    return n
  }
  const union = (a: number, b: number) => {
    const ra = find(a)
    const rb = find(b)
    if (ra !== rb) parent[Math.max(ra, rb)] = Math.min(ra, rb)
  }
  const exact = new Map<string, number>()
  const buckets = new Map<string, number[]>()
  cand.forEach((it, n) => {
    const hk = `${it.ctx}\u0000${it.hash}`
    const first = exact.get(hk)
    if (first !== undefined) return union(first, n)
    exact.set(hk, n)
    const s = it.sim
    if (s === undefined) return
    const bands = [s[0] >>> 16, s[0] & 0xffff, s[1] >>> 16, s[1] & 0xffff]
    bands.forEach((v, b) => {
      const key = `${it.ctx}\u0000${b}\u0000${v}`
      const reps = buckets.get(key) ?? []
      for (const o of reps) {
        const os = cand[o]?.sim
        if (os !== undefined && hamming(s, os) <= HAMMING_MAX) union(o, n)
      }
      if (reps.length < BUCKET_REPS) reps.push(n)
      buckets.set(key, reps)
    })
  })
  const groups = new Map<number, InspectItem[]>()
  cand.forEach((it, n) => {
    const r = find(n)
    groups.set(r, [...(groups.get(r) ?? []), it])
  })
  return [...groups.values()].filter(g => g.length > 1)
}

const targetLabel = (t: Target): string => (t.kind === 'file' ? shortPath(t.value) : oneLine(t.value, 60))

export function summarize(st: InspectState, all = false): InspectSummary {
  const { k, clamped, calibrated } = calibration(st)
  const est = (i: InspectItem) => Math.round(i.raw * k)
  const items = all ? st.items : st.items.filter(i => i.ctx === MAIN)
  const live = items.filter(i => i.live)

  const rr = rereads(items, st.edits, est)
  const counts = Object.fromEntries(REREAD_KINDS.map(x => [x, 0])) as Record<ReReadKind, number>
  const rrEst = Object.fromEntries(REREAD_KINDS.map(x => [x, 0])) as Record<ReReadKind, number>
  const wasteSeq = new Set<number>()
  for (const [r, kind] of rr.pairs) {
    counts[kind] += 1
    rrEst[kind] += est(r)
  }
  rr.pairs.forEach(([r], n) => {
    if (rr.rows[n]?.wasteNow === true) wasteSeq.add(r.seq)
  })
  const rereadNow = rr.rows.reduce((s, r) => s + (r.wasteNow ? r.est : 0), 0)

  const groups: SimilarGroup[] = []
  let similarNow = 0
  for (const g of similar(items)) {
    const first = g[0] as InspectItem
    const liveCopies = g.filter(i => i.live)
    let now = 0
    for (const i of liveCopies.slice(1)) {
      if (wasteSeq.has(i.seq)) continue
      wasteSeq.add(i.seq)
      now += est(i)
    }
    similarNow += now
    groups.push({
      ctx: first.ctx,
      kind: g.every(i => i.hash === first.hash) ? 'exact' : 'near',
      label: first.target === undefined ? first.category : `${first.category} · ${targetLabel(first.target)}`,
      count: g.length,
      est: g.reduce((s, i) => s + est(i), 0),
      extra: g.slice(1).reduce((s, i) => s + est(i), 0),
      wasteNow: now,
    })
  }
  groups.sort((a, b) => b.wasteNow - a.wasteNow || b.extra - a.extra)

  const compactions: CompactionView[] = st.compactions
    .filter(c => all || c.ctx === MAIN)
    .map(({ droppedRaw, ...c }) => ({
      ...c,
      droppedEst: Math.round(droppedRaw * k),
      reflow: rr.rows.filter(r => r.ctx === c.ctx && r.epoch === c.epoch && (r.kind === 'after-compaction' || r.kind === 'restored')).reduce((s, r) => s + r.est, 0),
    }))

  const turns: TurnEfficiency[] = []
  let prevLast: number | undefined
  for (const t of st.turns) {
    if (t.turn === 0) continue
    const base = prevLast ?? t.first
    const growth = t.last === undefined || base === undefined ? 0 : Math.max(0, t.last - base)
    if (t.last !== undefined) prevLast = t.last
    turns.push({ turn: t.turn, lines: t.lines, growth, exploring: t.lines === 0 && growth > 0 })
  }

  const sum: InspectSummary = {
    scope: all ? 'all' : 'main',
    items: items.length,
    liveItems: live.length,
    dropped: st.dropped,
    liveEst: live.reduce((s, i) => s + est(i), 0),
    k, kClamped: clamped, calibrated,
    waste: rereadNow + similarNow,
    occupancy: {
      byCategory: share(live, i => i.category, est),
      byTarget: share(live, i => (i.target === undefined ? undefined : `${i.target.kind}\u0000${targetLabel(i.target)}`), est),
    },
    rereads: { rows: rr.rows, counts, est: rrEst, wasteNow: rereadNow, wasteSession: rrEst['unchanged-live'] },
    similar: { groups, wasteNow: similarNow },
    compactions,
    efficiency: {
      turns,
      lines: turns.reduce((s, t) => s + t.lines, 0),
      growth: turns.reduce((s, t) => s + t.growth, 0),
      exploring: turns.filter(t => t.exploring).length,
    },
    advice: [],
  }
  if (st.measured !== undefined) sum.measured = st.measured
  sum.advice = advise(sum, items, est)
  return sum
}

// Hinweise mit Beleg und ≈Ersparnis. Gewohnheits-Hinweise zählen die ganze Sitzung, auch was eine Kompaktierung entfernte.
function advise(sum: InspectSummary, items: readonly InspectItem[], est: (i: InspectItem) => number): Advice[] {
  const out: Advice[] = []
  const lost = [...new Set(sum.compactions.flatMap(c => c.edited.lost))]
  if (lost.length > 0) out.push({ key: 'lostEdits', vars: { n: lost.length, files: lost.slice(0, 3).map(shortPath).join(', ') }, saving: 0 })
  const unchanged = sum.rereads.rows.filter(r => r.kind === 'unchanged-live')
  if (unchanged.length > 0) {
    const top = share(unchanged, r => r.path, r => r.est)[0]
    out.push({ key: 'reread', vars: { n: unchanged.length, file: shortPath(top?.name ?? ''), count: top?.count ?? 0 }, saving: sum.rereads.wasteSession })
  }
  const big = items.filter(i => i.category === 'Read' && i.whole === true && i.target !== undefined && est(i) >= LARGE_READ).sort((a, b) => est(b) - est(a))
  if (big.length > 0) {
    out.push({ key: 'largeRead', vars: { n: big.length, file: targetLabel((big[0] as InspectItem).target as Target), est: est(big[0] as InspectItem) }, saving: big.reduce((s, i) => s + est(i) - LARGE_READ_KEEP, 0) })
  }
  const shell = items.filter(i => SHELL_TOOLS.has(i.category) && est(i) >= LARGE_SHELL).sort((a, b) => est(b) - est(a))
  if (shell.length > 0) {
    out.push({ key: 'largeShell', vars: { n: shell.length, cmd: (shell[0] as InspectItem).target?.value.slice(0, 60) ?? '?', est: est(shell[0] as InspectItem) }, saving: shell.reduce((s, i) => s + est(i) - SHELL_KEEP, 0) })
  }
  const repeated = sum.similar.groups.reduce((s, g) => s + g.extra, 0)
  if (repeated >= REPEAT_MIN) out.push({ key: 'repeated', vars: { n: sum.similar.groups.length, label: sum.similar.groups[0]?.label ?? '' }, saving: repeated })
  const reflow = sum.compactions.reduce((s, c) => s + c.reflow, 0)
  if (reflow >= REFLOW_MIN) out.push({ key: 'reflow', vars: { n: sum.compactions.length }, saving: reflow })
  const agents = items.filter(i => AGENT_TOOLS.has(i.category) && est(i) >= AGENT_VERBOSE)
  if (agents.length > 0) out.push({ key: 'agentVerbose', vars: { n: agents.length }, saving: agents.reduce((s, i) => s + est(i) - AGENT_KEEP, 0) })
  // Verlorene Bearbeitungen zuerst (kein Token-Wert, aber das größte Risiko), dann nach Ersparnis.
  return out.sort((a, b) => (a.key === 'lostEdits' ? -1 : b.key === 'lostEdits' ? 1 : b.saving - a.saving))
}

// ── Texte ────────────────────────────────────────────────────────────────────────────────────────────────────────────
const DE = {
  'inspect.band.ctx': 'Kontext {pct} %',
  'inspect.band.waste': 'Verschwendung ≈{n}',
  'inspect.report.title': 'Kontext-Inspektor',
  'inspect.report.scopeMain': 'Hauptkontext',
  'inspect.report.scopeAll': 'Hauptkontext und Subagents',
  'inspect.empty': 'noch nichts erfasst',
  'inspect.sec.overview': 'Überblick',
  'inspect.sec.occupancy': 'Belegung',
  'inspect.sec.rereads': 'Re-Reads',
  'inspect.sec.similar': 'Ähnliche Ausgaben',
  'inspect.sec.compactions': 'Kompaktierungen',
  'inspect.sec.efficiency': 'Effizienz',
  'inspect.sec.advice': 'Hinweise',
  'inspect.ov.window': 'Im Fenster',
  'inspect.ov.windowValue': '≈{n} · {items} Items',
  'inspect.ov.measured': 'Gemessen (letzte Anfrage)',
  'inspect.ov.waste': 'Verschwendung jetzt',
  'inspect.ov.session': 'Re-Read-Kosten der Sitzung',
  'inspect.ov.k': 'Kalibrierung k',
  'inspect.ov.clamped': '{k} (begrenzt)',
  'inspect.ov.uncalibrated': '1 (noch ohne Messung)',
  'inspect.ov.ledger': 'Ledger',
  'inspect.ov.ledgerValue': '{items} Items · {dropped} verworfen',
  'inspect.ov.note': 'Gemessen = Input laut API; ≈ = Schätzung (Zeichen/4, Bilder 1.500, × k).',
  'inspect.occ.value': '≈{n} · {count}×',
  'inspect.occ.note': 'Nur Items, die noch im Fenster liegen: Kategorie, darunter Ziele.',
  'inspect.kind.file': 'Datei',
  'inspect.kind.command': 'Befehl',
  'inspect.kind.pattern': 'Muster',
  'inspect.kind.url': 'URL',
  'inspect.kind.agent': 'Agent',
  'inspect.rr.unchanged-live': 'unverändert, noch im Fenster',
  'inspect.rr.after-edit': 'nach Bearbeitung',
  'inspect.rr.after-compaction': 'nach Kompaktierung',
  'inspect.rr.restored': 'von der Engine wiederhergestellt',
  'inspect.rr.different-range': 'anderer Bereich',
  'inspect.rr.changed-externally': 'extern geändert',
  'inspect.rr.engine-deduped': 'von der Engine dedupliziert',
  'inspect.rr.value': '{count}× · ≈{n}',
  'inspect.rr.waste': 'Verschwendung: {file}',
  'inspect.rr.note': 'Verschwendung jetzt ≈{now} · Sitzung gesamt ≈{session}',
  'inspect.sim.exact': 'exakt',
  'inspect.sim.near': 'ähnlich',
  'inspect.sim.value': '{count}× {kind} · ≈{n} im Fenster',
  'inspect.sim.note': 'Lexikalisch (Wort-3-Shingles, Zahlen → #, Hamming ≤ 3), nicht semantisch; je Kontext getrennt.',
  'inspect.cmp.row': '#{n} {trigger}',
  'inspect.cmp.value': '{before} → {after} · {items} Items raus (≈{est})',
  'inspect.cmp.entities': 'behalten: bearbeitet {ek}/{et} · gelesen {rk}/{rt} · Fehler {xk}/{xt}',
  'inspect.cmp.lost': 'verloren: {files}',
  'inspect.cmp.reflow': 'danach erneut gelesen ≈{n}',
  'inspect.cmp.note': 'Überlebt = Teilstring in Zusammenfassung oder behaltenen Nachrichten; Umschriebenes zählt als verloren.',
  'inspect.eff.lines': 'Geänderte Zeilen',
  'inspect.eff.growth': 'Input-Wachstum',
  'inspect.eff.ratio': 'Zeilen je 1k Input',
  'inspect.eff.exploring': 'Erkundungs-Turns',
  'inspect.eff.turn': 'Turn {n}',
  'inspect.eff.turnValue': '{lines} Zeilen · +{growth}',
  'inspect.eff.explore': ' · Erkundung',
  'inspect.eff.note': 'Erkundung (Wachstum ohne Bearbeitung) ist kein Fehler.',
  'inspect.adv.saving': '−≈{n}',
  'inspect.adv.none': 'keine Auffälligkeiten',
  'inspect.adv.lostEdits': 'Kompaktierung hat {n} bearbeitete Datei(en) verloren',
  'inspect.adv.lostEdits.ev': 'fehlt in der Zusammenfassung: {files}',
  'inspect.adv.reread': 'Unveränderte Datei erneut gelesen ({n}×)',
  'inspect.adv.reread.ev': 'z. B. {file} ({count}×) – die frühere Kopie lag noch im Fenster',
  'inspect.adv.largeRead': 'Große Dateien ganz gelesen ({n}×)',
  'inspect.adv.largeRead.ev': 'z. B. {file} ≈{est} – offset/limit oder Grep nutzen',
  'inspect.adv.largeShell': 'Lange Befehlsausgaben ({n}×)',
  'inspect.adv.largeShell.ev': 'z. B. {cmd} ≈{est} – Ausgabe filtern oder kürzen',
  'inspect.adv.repeated': 'Wiederholte Ausgaben ({n} Gruppen)',
  'inspect.adv.repeated.ev': 'größte: {label}',
  'inspect.adv.reflow': 'Nach Kompaktierung wieder eingelesen',
  'inspect.adv.reflow.ev': 'über {n} Kompaktierung(en)',
  'inspect.adv.agentVerbose': 'Lange Subagent-Ergebnisse ({n}×)',
  'inspect.adv.agentVerbose.ev': 'kürzere Antwort im Agent-Prompt verlangen',
} as const

export type InspectKey = keyof typeof DE

const EN: { readonly [K in InspectKey]: string } = {
  'inspect.band.ctx': 'ctx {pct}%',
  'inspect.band.waste': 'waste≈{n}',
  'inspect.report.title': 'Context inspector',
  'inspect.report.scopeMain': 'main context',
  'inspect.report.scopeAll': 'main context and subagents',
  'inspect.empty': 'nothing recorded yet',
  'inspect.sec.overview': 'Overview',
  'inspect.sec.occupancy': 'Occupants',
  'inspect.sec.rereads': 'Re-reads',
  'inspect.sec.similar': 'Similar outputs',
  'inspect.sec.compactions': 'Compactions',
  'inspect.sec.efficiency': 'Efficiency',
  'inspect.sec.advice': 'Advice',
  'inspect.ov.window': 'In the window',
  'inspect.ov.windowValue': '≈{n} · {items} items',
  'inspect.ov.measured': 'Measured (last request)',
  'inspect.ov.waste': 'Waste now',
  'inspect.ov.session': 'Session re-read cost',
  'inspect.ov.k': 'Calibration k',
  'inspect.ov.clamped': '{k} (clamped)',
  'inspect.ov.uncalibrated': '1 (no measurement yet)',
  'inspect.ov.ledger': 'Ledger',
  'inspect.ov.ledgerValue': '{items} items · {dropped} dropped',
  'inspect.ov.note': 'Measured = input reported by the API; ≈ = estimate (chars/4, images 1,500, × k).',
  'inspect.occ.value': '≈{n} · {count}×',
  'inspect.occ.note': 'Items still in the window only: category, then targets.',
  'inspect.kind.file': 'File',
  'inspect.kind.command': 'Command',
  'inspect.kind.pattern': 'Pattern',
  'inspect.kind.url': 'URL',
  'inspect.kind.agent': 'Agent',
  'inspect.rr.unchanged-live': 'unchanged, still in window',
  'inspect.rr.after-edit': 'after edit',
  'inspect.rr.after-compaction': 'after compaction',
  'inspect.rr.restored': 'restored by the engine',
  'inspect.rr.different-range': 'different range',
  'inspect.rr.changed-externally': 'changed externally',
  'inspect.rr.engine-deduped': 'deduped by the engine',
  'inspect.rr.value': '{count}× · ≈{n}',
  'inspect.rr.waste': 'waste: {file}',
  'inspect.rr.note': 'Waste now ≈{now} · whole session ≈{session}',
  'inspect.sim.exact': 'exact',
  'inspect.sim.near': 'near',
  'inspect.sim.value': '{count}× {kind} · ≈{n} in window',
  'inspect.sim.note': 'Lexical (word 3-shingles, numbers → #, Hamming ≤ 3), not semantic; per context.',
  'inspect.cmp.row': '#{n} {trigger}',
  'inspect.cmp.value': '{before} → {after} · {items} items out (≈{est})',
  'inspect.cmp.entities': 'kept: edited {ek}/{et} · read {rk}/{rt} · errors {xk}/{xt}',
  'inspect.cmp.lost': 'lost: {files}',
  'inspect.cmp.reflow': 're-read afterwards ≈{n}',
  'inspect.cmp.note': 'Survived = substring of the summary or kept messages; paraphrases count as lost.',
  'inspect.eff.lines': 'Lines changed',
  'inspect.eff.growth': 'Input growth',
  'inspect.eff.ratio': 'Lines per 1k input',
  'inspect.eff.exploring': 'Exploration turns',
  'inspect.eff.turn': 'Turn {n}',
  'inspect.eff.turnValue': '{lines} lines · +{growth}',
  'inspect.eff.explore': ' · exploration',
  'inspect.eff.note': 'Exploration (growth without edits) is not a defect.',
  'inspect.adv.saving': '−≈{n}',
  'inspect.adv.none': 'nothing stands out',
  'inspect.adv.lostEdits': 'Compaction lost {n} edited file(s)',
  'inspect.adv.lostEdits.ev': 'missing from the summary: {files}',
  'inspect.adv.reread': 'Unchanged file read again ({n}×)',
  'inspect.adv.reread.ev': 'e.g. {file} ({count}×) – the earlier copy was still in the window',
  'inspect.adv.largeRead': 'Large files read whole ({n}×)',
  'inspect.adv.largeRead.ev': 'e.g. {file} ≈{est} – use offset/limit or Grep',
  'inspect.adv.largeShell': 'Long command output ({n}×)',
  'inspect.adv.largeShell.ev': 'e.g. {cmd} ≈{est} – filter or trim the output',
  'inspect.adv.repeated': 'Repeated output ({n} groups)',
  'inspect.adv.repeated.ev': 'largest: {label}',
  'inspect.adv.reflow': 'Re-read after compaction',
  'inspect.adv.reflow.ev': 'across {n} compaction(s)',
  'inspect.adv.agentVerbose': 'Long subagent results ({n}×)',
  'inspect.adv.agentVerbose.ev': 'ask for a shorter answer in the agent prompt',
}

export const INSPECT_TEXT = { de: DE, en: EN } as const

export function tx(L: UiLang, key: InspectKey, vars?: Record<string, string | number>): string {
  const text: string = INSPECT_TEXT[L][key]
  return vars === undefined ? text : text.replace(/\{(\w+)\}/g, (m, name: string) => (name in vars ? String(vars[name]) : m))
}

/** Tokenzahl kurz: 812, 8,1k / 8.1k, 123k, 1,2M. */
export function num(n: number, L: UiLang): string {
  const a = Math.abs(n)
  const s = a < 1000 ? String(Math.round(n)) : a < 100_000 ? `${(n / 1000).toFixed(1)}k` : a < 1_000_000 ? `${Math.round(n / 1000)}k` : `${(n / 1_000_000).toFixed(1)}M`
  return L === 'de' ? s.replace('.', ',') : s
}

// ── Anzeige ──────────────────────────────────────────────────────────────────────────────────────────────────────────
export type InspectRow = { label: string; value: string; tone?: 'ok' | 'warn' | 'bad' | 'dim' }
export type InspectSection = { title: string; rows: InspectRow[]; note?: string }

/** Kurzinfo fürs Band: `Kontext 62 % · Verschwendung ≈8,1k` / `ctx 62% · waste≈8.1k`. */
export function bandText(sum: InspectSummary, percent: number | undefined, L: UiLang): string {
  const waste = tx(L, 'inspect.band.waste', { n: num(sum.waste, L) })
  return percent === undefined || !Number.isFinite(percent) ? waste : `${tx(L, 'inspect.band.ctx', { pct: Math.round(percent) })} · ${waste}`
}

function sections(sum: InspectSummary, L: UiLang, max: number): InspectSection[] {
  const n = (v: number) => num(v, L)
  const empty: InspectRow = { label: tx(L, 'inspect.empty'), value: '', tone: 'dim' }
  const orEmpty = (rows: InspectRow[]) => (rows.length === 0 ? [empty] : rows.slice(0, max))

  const k = L === 'de' ? sum.k.toFixed(2).replace('.', ',') : sum.k.toFixed(2)
  const overview: InspectSection = {
    title: tx(L, 'inspect.sec.overview'),
    rows: [
      { label: tx(L, 'inspect.ov.window'), value: tx(L, 'inspect.ov.windowValue', { n: n(sum.liveEst), items: sum.liveItems }) },
      ...(sum.measured === undefined ? [] : [{ label: tx(L, 'inspect.ov.measured'), value: n(sum.measured) }]),
      { label: tx(L, 'inspect.ov.waste'), value: `≈${n(sum.waste)}`, tone: sum.waste > 0 ? 'warn' : 'ok' },
      { label: tx(L, 'inspect.ov.session'), value: `≈${n(sum.rereads.wasteSession)}`, tone: 'dim' },
      {
        label: tx(L, 'inspect.ov.k'),
        value: !sum.calibrated ? tx(L, 'inspect.ov.uncalibrated') : sum.kClamped ? tx(L, 'inspect.ov.clamped', { k }) : k,
        ...(sum.kClamped ? { tone: 'warn' as const } : {}),
      },
      { label: tx(L, 'inspect.ov.ledger'), value: tx(L, 'inspect.ov.ledgerValue', { items: sum.items, dropped: sum.dropped }), tone: 'dim' },
    ],
    note: tx(L, 'inspect.ov.note'),
  }

  const half = Math.max(1, Math.floor(max / 2))
  const cats = sum.occupancy.byCategory.slice(0, half).map(s => ({ label: s.name, value: tx(L, 'inspect.occ.value', { n: n(s.est), count: s.count }) }))
  const targets = sum.occupancy.byTarget.slice(0, max - cats.length).map(s => {
    const [kind, name] = s.name.split('\u0000') as [TargetKind, string]
    return { label: `${tx(L, `inspect.kind.${kind}`)} · ${name}`, value: tx(L, 'inspect.occ.value', { n: n(s.est), count: s.count }), tone: 'dim' as const }
  })
  const occupancy: InspectSection = { title: tx(L, 'inspect.sec.occupancy'), rows: orEmpty([...cats, ...targets]), note: tx(L, 'inspect.occ.note') }

  const kinds = REREAD_KINDS.filter(x => sum.rereads.counts[x] > 0).map(x => ({
    label: tx(L, `inspect.rr.${x}`),
    value: tx(L, 'inspect.rr.value', { count: sum.rereads.counts[x], n: n(sum.rereads.est[x]) }),
    ...(x === 'unchanged-live' ? { tone: 'bad' as const } : x === 'engine-deduped' ? { tone: 'dim' as const } : {}),
  }))
  const wasteFiles = share(sum.rereads.rows.filter(r => r.wasteNow), r => r.path, r => r.est).map(s => ({ label: tx(L, 'inspect.rr.waste', { file: shortPath(s.name) }), value: `${s.count}× · ≈${n(s.est)}`, tone: 'warn' as const }))
  const rereadSec: InspectSection = {
    title: tx(L, 'inspect.sec.rereads'),
    rows: orEmpty([...kinds, ...wasteFiles]),
    note: tx(L, 'inspect.rr.note', { now: n(sum.rereads.wasteNow), session: n(sum.rereads.wasteSession) }),
  }

  const similarSec: InspectSection = {
    title: tx(L, 'inspect.sec.similar'),
    rows: orEmpty(sum.similar.groups.map(g => ({
      label: g.label,
      value: tx(L, 'inspect.sim.value', { count: g.count, kind: tx(L, g.kind === 'exact' ? 'inspect.sim.exact' : 'inspect.sim.near'), n: n(g.wasteNow) }),
      ...(g.wasteNow > 0 ? { tone: 'warn' as const } : { tone: 'dim' as const }),
    }))),
    note: tx(L, 'inspect.sim.note'),
  }

  const cmpRows: InspectRow[] = []
  sum.compactions.slice(-Math.max(1, Math.floor(max / 3))).forEach(c => {
    const idx = sum.compactions.indexOf(c) + 1
    const kept = (s: EntitySet) => s.all.length - s.lost.length
    cmpRows.push({
      label: tx(L, 'inspect.cmp.row', { n: idx, trigger: c.ctx === MAIN ? c.trigger : `${c.trigger} · ${c.ctx}` }),
      value: tx(L, 'inspect.cmp.value', { before: c.before === undefined ? '?' : n(c.before), after: c.after === undefined ? '?' : n(c.after), items: c.droppedItems, est: n(c.droppedEst) }),
    })
    cmpRows.push({
      label: '',
      value: tx(L, 'inspect.cmp.entities', { ek: kept(c.edited), et: c.edited.all.length, rk: kept(c.read), rt: c.read.all.length, xk: kept(c.errors), xt: c.errors.all.length }),
      tone: 'dim',
    })
    if (c.edited.lost.length > 0) cmpRows.push({ label: '', value: tx(L, 'inspect.cmp.lost', { files: c.edited.lost.slice(0, 3).map(shortPath).join(', ') }), tone: 'bad' })
    else if (c.reflow > 0) cmpRows.push({ label: '', value: tx(L, 'inspect.cmp.reflow', { n: n(c.reflow) }), tone: 'warn' })
  })
  const compSec: InspectSection = { title: tx(L, 'inspect.sec.compactions'), rows: orEmpty(cmpRows.slice(-max)), note: tx(L, 'inspect.cmp.note') }

  const eff = sum.efficiency
  const effRows: InspectRow[] = eff.turns.length === 0 ? [] : [
    { label: tx(L, 'inspect.eff.lines'), value: String(eff.lines) },
    { label: tx(L, 'inspect.eff.growth'), value: `+${n(eff.growth)}` },
    { label: tx(L, 'inspect.eff.ratio'), value: eff.growth > 0 ? (L === 'de' ? (eff.lines / (eff.growth / 1000)).toFixed(1).replace('.', ',') : (eff.lines / (eff.growth / 1000)).toFixed(1)) : '–' },
    { label: tx(L, 'inspect.eff.exploring'), value: String(eff.exploring), tone: 'dim' },
    ...eff.turns.slice(-(max - 4)).map(t => ({
      label: tx(L, 'inspect.eff.turn', { n: t.turn }),
      value: tx(L, 'inspect.eff.turnValue', { lines: t.lines, growth: n(t.growth) }) + (t.exploring ? tx(L, 'inspect.eff.explore') : ''),
      ...(t.exploring ? { tone: 'dim' as const } : {}),
    })),
  ]
  const effSec: InspectSection = { title: tx(L, 'inspect.sec.efficiency'), rows: orEmpty(effRows), note: tx(L, 'inspect.eff.note') }

  const advRows: InspectRow[] = sum.advice.slice(0, Math.max(1, Math.floor(max / 2))).flatMap(a => {
    const vars = Object.fromEntries(Object.entries(a.vars).map(([key, v]) => [key, key === 'est' && typeof v === 'number' ? n(v) : v]))
    return [
      { label: tx(L, `inspect.adv.${a.key}`, vars), value: a.saving > 0 ? tx(L, 'inspect.adv.saving', { n: n(a.saving) }) : '', tone: a.key === 'lostEdits' ? 'bad' as const : 'warn' as const },
      { label: '', value: tx(L, `inspect.adv.${a.key}.ev`, vars), tone: 'dim' as const },
    ]
  })
  const advSec: InspectSection = { title: tx(L, 'inspect.sec.advice'), rows: advRows.length === 0 ? [{ label: tx(L, 'inspect.adv.none'), value: '', tone: 'ok' }] : advRows }

  return [overview, occupancy, rereadSec, similarSec, compSec, effSec, advSec]
}

/** Abschnitte fürs Panel (Desktop und Terminal, nur Text), je höchstens 8 Zeilen. */
export const paneView = (sum: InspectSummary, L: UiLang): InspectSection[] => sections(sum, L, 8)

/** Klartext für `/cockpit-kontext`: alle Abschnitte, je bis zu 20 Zeilen, mit Hinweiszeile. */
export function reportText(sum: InspectSummary, L: UiLang): string {
  const out = [`${tx(L, 'inspect.report.title')} (${tx(L, sum.scope === 'all' ? 'inspect.report.scopeAll' : 'inspect.report.scopeMain')})`]
  for (const s of sections(sum, L, 20)) {
    out.push('', s.title, '─'.repeat(s.title.length))
    const width = Math.min(40, Math.max(0, ...s.rows.map(r => r.label.length)))
    for (const r of s.rows) out.push(r.label === '' ? `  ${' '.repeat(width)}  ${r.value}` : `  ${r.label.padEnd(width)}  ${r.value}`.trimEnd())
    if (s.note !== undefined) out.push(`  (${s.note})`)
  }
  return out.join('\n')
}
