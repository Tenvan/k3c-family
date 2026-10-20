// Profiler (Funktionsumfang nach dem Community-Mod „agent-profiler“, eigene Umsetzung, kein Code übernommen): wohin die
// Zeit einer Session geht – Modell, Tools, Subagents, Engine-Overhead –, dazu Testläufe, wiederholte Datei-Reads und Funde
// in Klartext. Rein beobachtend: kein deny, kein Prompt-Einschub, keine eigenen Tools, nie in Events eingreifen. Reine
// Funktionen ohne `$`, kein Netzwerk, keine Dateizugriffe; Zeit kommt immer als Parameter (`at`/`now`, ms wie
// `$.clock.now()`). Der Zustand ist ein einfaches Objekt, das die Eingabefunktionen an Ort und Stelle ändern: der Adapter
// hält es in einer Modul-Variablen, nicht in einem Atom (bis zu 20.000 Datensätze; ein Reload beginnt neu).
//
// Zeitzerlegung je Kontext (Hauptschleife oder ein Subagent, Schlüssel `agentId`):
//   Turn          turn.start → turn.complete                                     beobachtet
//   Model-Request turn.step: Senden → erster Chunk (TTFT) → Thinking → Output    beobachtet, Tokens laut API
//   Tool          tool.call Eintritt → Rückkehr (wall)                           beobachtet
//     exec        classic.PostToolUse.duration_ms                                 gemessen
//     wait        wall − exec; Freigabewartezeit bei tool.check 'ask', sonst Overhead   abgeleitet
//   Subagent      classic.SubagentStart → classic.SubagentStop                     beobachtet
//   Overhead      Turn − (Requests ∪ blockierende Subagents ∪ Tools)               abgeleitet
//   Leerlauf zwischen turn.complete und dem nächsten turn.start zählt nicht mit.
// TTFT und Stream-Zeiten sind Wartezeiten am Client (Netz, Warteschlange, Prefill nicht trennbar), keine Inferenzzeit.
//
// Verdrahtung (Adapter in register.tsx; Feldnamen geprüft an claude-code.d.ts, Engine 2.1.299):
// - turnStart(st, turnId, agentId, at)       ← `turn.start` beim Eintritt: `e.turnId`, `e.agentId` (fehlt = Hauptschleife).
// - turnComplete(st, turnId, at, reason)     ← `turn.complete` beim Eintritt: `e.turnId`, `e.reason`
//                                              ('answer' | 'aborted' | 'refusal' | 'error'). Schließt offene Requests und
//                                              Tools des Turns.
// - stepBegin(st, turnId, index, model, agentId, at)
//                                            ← `turn.step` (StreamingEventName, Hook ist `async function* ($, e, next)`), vor
//                                              dem ersten Lesen von `next(e)`: `e.turnId`, `e.index`, `e.model`, `e.agentId`.
// - stepChunk(st, turnId, index, chunk, at)  ← jeder Chunk `c` des Streams `next(e)`, unverändert weitergereicht (`yield c`).
//                                              `c.kind`: 'text' { index, text } | 'thinking' { index, text } |
//                                              'tool' { index, id, name } (Beginn eines Tool-Aufrufs) | 'input' { index, json }
//                                              | 'stop' { stopReason, usage: TurnUsage | null } (letzter Chunk) |
//                                              'engine' { ref } (Hülle, Blockgrenzen, Retry-Marker). Übergeben wird
//                                              `{ kind: c.kind, usage: c.kind === 'stop' ? c.usage : undefined }`; TurnUsage
//                                              hat input_tokens, output_tokens, cache_read_input_tokens,
//                                              cache_creation_input_tokens, model.
// - stepEnd(st, turnId, index, at, usage, ok)← derselbe Hook, wenn der Stream erschöpft ist: von Hand iterieren
//                                              (`const it = next(e); for (;;) { const r = await it.next(); if (r.done) … }`),
//                                              `r.value` ist TurnStepResult mit `usage` und `stopReason`. Wirft der Stream
//                                              oder bricht der Generator ab (`finally`), dann `ok = false`.
// - toolBegin(st, id, tool, agentId, input, at)
//                                            ← `tool.call` beim Eintritt, vor `await next(e)`: `e.tool_use_id` (optional,
//                                              sonst eigene Kennung), `e.tool`, `e.agentId`, `e` selbst als Eingabe (flach
//                                              `{ tool, tool_use_id, ...args }`).
// - toolEnd(st, id, at, ok)                  ← derselbe Hook nach `await next(e)`: `ok = !r.deny && r.isError !== true`;
//                                              wirft `next(e)`, dann `ok = false`.
// - toolApproval(st, id, decision)           ← `tool.check` nach `await next(e)`: `e.tool_use_id`, `r.decision`
//                                              ('allow' | 'ask' | 'deny'). Darf vor oder nach toolBegin kommen.
// - toolExec(st, id, durationMs)             ← `classic.PostToolUse` (und `classic.PostToolUseFailure`): `e.tool_use_id`,
//                                              `e.duration_ms` (optional; ohne Freigabe- und Hook-Zeit).
// - subagentStart(st, agentId, type, at, toolUseId)
//                                            ← `classic.SubagentStart`: `e.agent_id`, `e.agent_type`. `toolUseId` verknüpft
//                                              mit dem Agent-Aufruf: `agent.spawn` liefert `e.tool_use_id` und im Ergebnis
//                                              `r.agentId`; der Adapter merkt sich agentId → tool_use_id.
// - subagentStop(st, agentId, at)            ← `classic.SubagentStop`: `e.agent_id`.
// - stopFailure(st, agentId, at, error)      ← `classic.StopFailure`: `e.agent_id` (fehlt = Hauptschleife), `e.error`
//                                              (SDKAssistantMessageError, z. B. 'rate_limit'). Schließt nur diesen Kontext;
//                                              das Warten auf ein Limit-Reset zählt danach als Leerlauf.
import type { UiLang } from '../types'
import { duration, oneLine } from './core'

// Speicherdeckel: Requests + Tools zusammen. Darüber fallen die ältesten fertigen Turns samt Datensätzen weg.
export const MAX_RECORDS = 20_000
export const MAIN = 'main'
const LABEL_MAX = 48
const CMD_MAX = 40
const TOP = 5
// Unbestätigte Freigaben (tool.check ohne tool.call) höchstens so viele merken.
const PENDING_MAX = 500

export type Grade = 'measured' | 'observed' | 'reported' | 'inferred'
// Beobachtungsgrad je Wert; nie „Inferenzzeit“ behaupten.
export const GRADE = {
  turn: 'observed',
  request: 'observed',
  ttft: 'observed',
  stream: 'observed',
  tokens: 'reported',
  toolWall: 'observed',
  exec: 'measured',
  wait: 'inferred',
  subagent: 'observed',
  overhead: 'inferred',
} as const satisfies Record<string, Grade>

export type CmdCategory = 'test' | 'build' | 'git' | 'install' | 'search' | 'other'
export type Decision = 'allow' | 'ask' | 'deny'
export type StepChunkKind = 'text' | 'thinking' | 'tool' | 'input' | 'stop' | 'engine'
export type TokenUsage = { input_tokens: number; output_tokens: number; cache_read_input_tokens: number; cache_creation_input_tokens: number }
export type StepChunk = { kind: StepChunkKind; usage?: TokenUsage | null | undefined }

type Usage = { in: number; out: number; cacheRead: number; cacheWrite: number }

export type TurnRec = { id: string; ctx: string; start: number; end?: number; reason?: string; failed?: boolean }
export type RequestRec = {
  turnId: string; index: number; ctx: string; model: string; start: number
  first?: number; thinkFrom?: number; thinkTo?: number; outFrom?: number; end?: number
  usage?: Usage; failed?: boolean; cut?: boolean
}
export type ToolRec = {
  id: string; tool: string; ctx: string; turnId?: string; label: string; start: number; end?: number; ok?: boolean
  execMs?: number; decision?: Decision; cat?: CmdCategory; file?: string; range?: string; writes?: string; background?: boolean; cut?: boolean
}
export type SubagentRec = { id: string; type: string; ctx: string; start: number; end?: number; blocking: boolean; failed?: boolean }
export type ProfilerState = {
  turns: TurnRec[]
  requests: RequestRec[]
  tools: ToolRec[]
  subagents: SubagentRec[]
  // offener Turn je Kontext
  open: Record<string, string>
  // Freigaben, die vor dem tool.call eintrafen
  pending: Record<string, Decision>
  droppedTurns: number
}

export function newProfiler(): ProfilerState {
  return { turns: [], requests: [], tools: [], subagents: [], open: {}, pending: {}, droppedTurns: 0 }
}

const ctxOf = (agentId: string | undefined): string => agentId ?? MAIN
const str = (v: unknown): string => (typeof v === 'string' ? v : '')
const num = (v: unknown): number | undefined => (typeof v === 'number' && Number.isFinite(v) ? v : undefined)

// ---- Maskierung ----

// Gängige Geheimnis-Formen maskieren (best effort): Authorization-Header, Bearer-Token, --password/--token, NAME=wert mit
// KEY/TOKEN/SECRET/PASSWORD im Namen, mysql -p<pw>, Passwort in URLs. Immer vor dem Kürzen.
export function maskSecrets(text: string): string {
  return text
    .replace(/(authorization\s*[:=]\s*)(?:(bearer|basic|token|digest)\s+)?[^\s"']+/gi, (_m, head: string, scheme?: string) => `${head}${scheme ? `${scheme} ` : ''}***`)
    .replace(/\bbearer\s+[A-Za-z0-9._~+/=-]+/gi, m => `${m.slice(0, 6)} ***`)
    .replace(/(--?(?:password|passwd|pwd|token|api[-_]?key|secret|access[-_]?token)(?:=|\s+))("[^"]*"|'[^']*'|\S+)/gi, '$1***')
    .replace(/\b([A-Za-z_][A-Za-z0-9_]*(?:key|token|secret|password|passwd)[A-Za-z0-9_]*)=("[^"]*"|'[^']*'|\S+)/gi, '$1=***')
    .replace(/(\bmysql\b[^\n]*?\s-p)(?!\s)(\S+)/gi, '$1***')
    .replace(/(\/\/[^/\s:@]+:)[^@\s/]+@/g, '$1***@')
}

// Maskiert, einzeilig, gekürzt: so landen Befehle und Suchmuster im Zustand.
export function shortText(text: string, max = CMD_MAX): string {
  return oneLine(maskSecrets(text), max)
}

// Grobe Kategorie eines Shell-Befehls.
export function cmdCategory(cmd: string): CmdCategory {
  const c = cmd.trim().toLowerCase()
  if (/^git\b/.test(c)) return 'git'
  if (/\b(test|tests|jest|vitest|pytest|mocha|phpunit|rspec|ctest)\b/.test(c)) return 'test'
  if (/\b(npm|pnpm|yarn|bun)\s+(i|install|ci|add)\b|\bpip3?\s+install\b|\b(cargo|go)\s+(add|get)\b|\bdotnet\s+(restore|add)\b|\b(brew|apt|apt-get|choco|winget)\s+install\b|^(yarn|pnpm i)$/.test(c)) return 'install'
  if (/\b(build|tsc|make|cmake|msbuild|gradle|mvn|webpack|esbuild|rollup|compile)\b/.test(c)) return 'build'
  if (/^(grep|rg|ag|find|fd|ls|dir|tree|select-string|get-childitem)\b|\|\s*(grep|rg|select-string)\b/.test(c)) return 'search'
  if (/\bgit\s/.test(c)) return 'git'
  return 'other'
}

const baseName = (p: string): string => p.split(/[\\/]/).filter(Boolean).pop() ?? p

type ToolInfo = Pick<ToolRec, 'label' | 'cat' | 'file' | 'range' | 'writes' | 'background'>

// Was vom Tool-Aufruf gespeichert wird: Anzeige-Label (maskiert, gekürzt), Kategorie, Datei und Bereich.
function describeTool(tool: string, input: unknown): ToolInfo {
  const a = (input !== null && typeof input === 'object' ? input : {}) as Record<string, unknown>
  const label = (rest: string): string => oneLine(rest === '' ? tool : `${tool} ${rest}`, LABEL_MAX)
  if (tool === 'Bash' || tool === 'PowerShell') {
    const cmd = str(a.command)
    return { label: label(shortText(cmd)), cat: cmdCategory(cmd), ...(a.run_in_background === true ? { background: true } : {}) }
  }
  if (tool === 'Read') {
    const file = str(a.file_path)
    const offset = num(a.offset)
    const limit = num(a.limit)
    const range = offset === undefined && limit === undefined ? '' : `${offset ?? 1}+${limit ?? ''}`
    return { label: label(baseName(file)), ...(file ? { file } : {}), ...(range ? { range } : {}) }
  }
  if (tool === 'Edit' || tool === 'Write' || tool === 'MultiEdit' || tool === 'NotebookEdit') {
    const file = str(a.file_path) || str(a.notebook_path)
    return { label: label(baseName(file)), ...(file ? { writes: file } : {}) }
  }
  if (tool === 'Grep' || tool === 'Glob') return { label: label(shortText(str(a.pattern))) }
  if (tool === 'Agent' || tool === 'Task') {
    return { label: label(shortText(str(a.subagent_type) || str(a.description))), ...(a.run_in_background === true ? { background: true } : {}) }
  }
  return { label: label('') }
}

// ---- Eingaben ----

// Offene Datensätze schließen, die `match` trifft: Ende `at`, als abgeschnitten markiert.
function settle(st: ProfilerState, match: (r: { ctx: string; turnId?: string }) => boolean, at: number, failed: boolean): void {
  for (const r of st.requests) {
    if (r.end === undefined && match(r)) {
      r.end = Math.max(at, r.start)
      r.cut = true
      if (failed) r.failed = true
    }
  }
  for (const t of st.tools) {
    if (t.end === undefined && match(t)) {
      t.end = Math.max(at, t.start)
      t.cut = true
      t.ok = false
    }
  }
}

export function turnStart(st: ProfilerState, turnId: string, agentId: string | undefined, at: number): void {
  const ctx = ctxOf(agentId)
  const prev = st.open[ctx]
  if (prev !== undefined) {
    const old = st.turns.findLast(t => t.id === prev)
    if (old && old.end === undefined) old.end = at
  }
  st.turns.push({ id: turnId, ctx, start: at })
  st.open[ctx] = turnId
}

export function turnComplete(st: ProfilerState, turnId: string, at: number, reason = 'answer'): void {
  const turn = st.turns.findLast(t => t.id === turnId)
  if (!turn || turn.end !== undefined) return
  turn.end = Math.max(at, turn.start)
  turn.reason = reason
  if (reason === 'error') turn.failed = true
  if (st.open[turn.ctx] === turnId) delete st.open[turn.ctx]
  settle(st, r => r.turnId === turnId, at, reason === 'error')
  trim(st)
}

export function stepBegin(st: ProfilerState, turnId: string, index: number, model: string, agentId: string | undefined, at: number): void {
  st.requests.push({ turnId, index, ctx: ctxOf(agentId), model, start: at })
  trim(st)
}

const openRequest = (st: ProfilerState, turnId: string, index: number): RequestRec | undefined =>
  st.requests.findLast(r => r.turnId === turnId && r.index === index && r.end === undefined)

const toUsage = (u: TokenUsage): Usage => ({
  in: u.input_tokens, out: u.output_tokens, cacheRead: u.cache_read_input_tokens, cacheWrite: u.cache_creation_input_tokens,
})

export function stepChunk(st: ProfilerState, turnId: string, index: number, chunk: StepChunk, at: number): void {
  const r = openRequest(st, turnId, index)
  if (!r) return
  r.first ??= at
  if (chunk.kind === 'thinking') {
    r.thinkFrom ??= at
    r.thinkTo = at
  } else if (chunk.kind === 'text' || chunk.kind === 'tool' || chunk.kind === 'input') {
    r.outFrom ??= at
  } else if (chunk.kind === 'stop' && chunk.usage) {
    r.usage = toUsage(chunk.usage)
  }
}

export function stepEnd(st: ProfilerState, turnId: string, index: number, at: number, usage?: TokenUsage | null, ok = true): void {
  const r = openRequest(st, turnId, index)
  if (!r) return
  r.end = Math.max(at, r.start)
  if (usage && !r.usage) r.usage = toUsage(usage)
  if (!ok) r.failed = true
}

export function toolBegin(st: ProfilerState, id: string, tool: string, agentId: string | undefined, input: unknown, at: number): void {
  const ctx = ctxOf(agentId)
  const turnId = st.open[ctx]
  const decision = st.pending[id]
  if (decision !== undefined) delete st.pending[id]
  st.tools.push({
    id, tool, ctx, start: at, ...describeTool(tool, input),
    ...(turnId === undefined ? {} : { turnId }),
    ...(decision === undefined ? {} : { decision }),
  })
  trim(st)
}

export function toolEnd(st: ProfilerState, id: string, at: number, ok: boolean): void {
  const t = st.tools.findLast(x => x.id === id && x.end === undefined)
  if (!t) return
  t.end = Math.max(at, t.start)
  t.ok = ok
}

export function toolApproval(st: ProfilerState, id: string, decision: Decision): void {
  const t = st.tools.findLast(x => x.id === id)
  if (t) {
    t.decision = decision
    return
  }
  // ponytail: Freigabe vor dem tool.call wird kurz gemerkt, bei Überlauf verworfen; Ausbaupfad: Alter je Eintrag.
  if (Object.keys(st.pending).length >= PENDING_MAX) st.pending = {}
  st.pending[id] = decision
}

export function toolExec(st: ProfilerState, id: string, durationMs: number): void {
  const t = st.tools.findLast(x => x.id === id)
  if (t && Number.isFinite(durationMs)) t.execMs = Math.max(0, durationMs)
}

export function subagentStart(st: ProfilerState, agentId: string, type: string, at: number, toolUseId?: string): void {
  const parent = toolUseId === undefined ? undefined : st.tools.findLast(t => t.id === toolUseId)
  st.subagents.push({ id: agentId, type, ctx: parent?.ctx ?? MAIN, start: at, blocking: parent?.background !== true })
}

export function subagentStop(st: ProfilerState, agentId: string, at: number): void {
  const s = st.subagents.findLast(x => x.id === agentId && x.end === undefined)
  if (s) s.end = Math.max(at, s.start)
}

export function stopFailure(st: ProfilerState, agentId: string | undefined, at: number, error: string): void {
  const ctx = ctxOf(agentId)
  const turnId = st.open[ctx]
  if (turnId !== undefined) {
    const turn = st.turns.findLast(t => t.id === turnId)
    if (turn && turn.end === undefined) {
      turn.end = Math.max(at, turn.start)
      turn.reason = `error:${error}`
      turn.failed = true
    }
    delete st.open[ctx]
  }
  settle(st, r => r.ctx === ctx, at, true)
  if (agentId !== undefined) {
    const s = st.subagents.findLast(x => x.id === agentId && x.end === undefined)
    if (s) {
      s.end = Math.max(at, s.start)
      s.failed = true
    }
  }
  trim(st)
}

// Deckel: älteste fertige Turns samt Requests, Tools und vorher beendeten Subagents fallen weg. Gibt es keinen fertigen
// Turn mehr, fallen die ältesten fertigen Einzeldatensätze.
export function trim(st: ProfilerState, max = MAX_RECORDS): void {
  while (st.requests.length + st.tools.length > max) {
    const at = st.turns.findIndex(t => t.end !== undefined)
    const turn = st.turns[at]
    if (turn === undefined || turn.end === undefined) break
    const end = turn.end
    st.turns.splice(at, 1)
    st.requests = st.requests.filter(r => r.turnId !== turn.id)
    st.tools = st.tools.filter(t => t.turnId !== turn.id)
    st.subagents = st.subagents.filter(s => s.end === undefined || s.end > end)
    st.droppedTurns++
  }
  let excess = st.requests.length + st.tools.length - max
  if (excess <= 0) return
  const keep = (r: { end?: number }): boolean => (r.end !== undefined && excess > 0 ? (excess--, false) : true)
  st.tools = st.tools.filter(keep)
  st.requests = st.requests.filter(keep)
}

// ---- Auswertung ----

type Span = readonly [number, number]

function unionMs(spans: readonly Span[]): number {
  const sorted = spans.filter(s => s[1] > s[0]).sort((a, b) => a[0] - b[0])
  let total = 0
  let from = -Infinity
  let to = -Infinity
  for (const [s, e] of sorted) {
    if (s > to) {
      if (to > from) total += to - from
      from = s
      to = e
    } else if (e > to) to = e
  }
  if (to > from) total += to - from
  return total
}

const clip = (spans: readonly Span[], from: number, to: number): Span[] =>
  spans.map(([s, e]): Span => [Math.max(s, from), Math.min(e, to)]).filter(([s, e]) => e > s)

export type Share = { total: number; model: number; tools: number; subagents: number; overhead: number }
export type SlowTool = { label: string; tool: string; wallMs: number; execMs?: number; waitMs?: number; wait: 'approval' | 'overhead'; ok?: boolean; open: boolean }
export type TestRun = { label: string; wallMs: number; execMs?: number; ok?: boolean; open: boolean }
export type RequestStats = {
  count: number; ttftAvg: number; ttftMax: number; thinkMs: number; outMs: number
  tokensIn: number; tokensOut: number; cacheRead: number; cacheWrite: number; cacheHit: number | null
}
export type SubagentView = { id: string; type: string; ms: number; open: boolean; blocking: boolean; failed: boolean; requests: number; tools: number }
export type RepeatedRead = { file: string; range: string; count: number }
export type FindingKey = 'overhead' | 'approval' | 'reread' | 'tests' | 'ttft' | 'cache' | 'failed' | 'slowTool'
export type Finding = { key: FindingKey; vars: Record<string, string | number> }
export type ScopeStats = {
  share: Share; turns: number; failedTurns: number; approvalMs: number
  slowTools: SlowTool[]; tests: TestRun[]; requests: RequestStats; subagents: SubagentView[]; repeatedReads: RepeatedRead[]; findings: Finding[]
}
export type InFlight =
  | { kind: 'tool'; label: string; since: number; ctx: string }
  | { kind: 'request'; phase: 'wait' | 'think' | 'write'; model: string; since: number; ctx: string }
export type ProfileSummary = { at: number; session: ScopeStats; turn: ScopeStats | null; inflight: InFlight[]; droppedTurns: number }

const endOf = (r: { end?: number }, now: number): number => r.end ?? now

function shareOf(st: ProfilerState, turns: readonly TurnRec[], now: number): Share {
  const byTurn = new Map<string, { req: Span[]; tool: Span[] }>()
  const bucket = (id: string) => {
    let b = byTurn.get(id)
    if (!b) byTurn.set(id, (b = { req: [], tool: [] }))
    return b
  }
  for (const r of st.requests) if (r.ctx === MAIN) bucket(r.turnId).req.push([r.start, endOf(r, now)])
  for (const t of st.tools) if (t.ctx === MAIN && t.turnId !== undefined) bucket(t.turnId).tool.push([t.start, endOf(t, now)])
  const subs: Span[] = st.subagents.filter(s => s.ctx === MAIN && s.blocking).map(s => [s.start, endOf(s, now)])
  const sum: Share = { total: 0, model: 0, tools: 0, subagents: 0, overhead: 0 }
  for (const turn of turns) {
    const from = turn.start
    const to = Math.max(endOf(turn, now), from)
    const b = byTurn.get(turn.id) ?? { req: [], tool: [] }
    const m = clip(b.req, from, to)
    const ms = m.concat(clip(subs, from, to))
    const mst = ms.concat(clip(b.tool, from, to))
    const um = unionMs(m)
    const ums = unionMs(ms)
    const umst = unionMs(mst)
    sum.total += to - from
    sum.model += um
    sum.subagents += ums - um
    sum.tools += umst - ums
    sum.overhead += to - from - umst
  }
  return sum
}

function requestStats(reqs: readonly RequestRec[], now: number): RequestStats {
  const ttfts = reqs.flatMap(r => (r.first === undefined ? [] : [r.first - r.start]))
  const s = { tokensIn: 0, tokensOut: 0, cacheRead: 0, cacheWrite: 0, thinkMs: 0, outMs: 0 }
  for (const r of reqs) {
    if (r.thinkFrom !== undefined && r.thinkTo !== undefined) s.thinkMs += r.thinkTo - r.thinkFrom
    if (r.outFrom !== undefined) s.outMs += endOf(r, now) - r.outFrom
    if (r.usage) {
      s.tokensIn += r.usage.in
      s.tokensOut += r.usage.out
      s.cacheRead += r.usage.cacheRead
      s.cacheWrite += r.usage.cacheWrite
    }
  }
  const input = s.tokensIn + s.cacheRead + s.cacheWrite
  return {
    count: reqs.length,
    ttftAvg: ttfts.length === 0 ? 0 : Math.round(ttfts.reduce((a, b) => a + b, 0) / ttfts.length),
    ttftMax: ttfts.length === 0 ? 0 : Math.max(...ttfts),
    ...s,
    cacheHit: input === 0 ? null : s.cacheRead / input,
  }
}

// Read derselben Datei mit gleichem Bereich, ohne Schreibzugriff dazwischen (Edit/Write); Bash-Schreibzugriffe sieht das nicht.
function repeatedReads(tools: readonly ToolRec[]): RepeatedRead[] {
  // Reads je Datei/Bereich seit dem letzten Schreibzugriff; gemeldet wird je Datei/Bereich der längste Lauf.
  const run = new Map<string, number>()
  const best = new Map<string, RepeatedRead>()
  for (const t of tools) {
    if (t.writes !== undefined) {
      for (const k of [...run.keys()]) if (k.startsWith(`${t.writes}|`)) run.delete(k)
      continue
    }
    if (t.tool !== 'Read' || t.file === undefined) continue
    const range = t.range ?? ''
    const key = `${t.file}|${range}`
    const n = (run.get(key) ?? 0) + 1
    run.set(key, n)
    if (n >= 2 && (best.get(key)?.count ?? 0) < n) best.set(key, { file: t.file, range, count: n })
  }
  return [...best.values()].sort((a, b) => b.count - a.count).slice(0, TOP)
}

function findingsOf(s: Omit<ScopeStats, 'findings'>): Finding[] {
  const out: Finding[] = []
  const { share } = s
  const pct = (part: number) => Math.round((part / share.total) * 100)
  if (share.total >= 30_000 && share.overhead / share.total >= 0.25) out.push({ key: 'overhead', vars: { pct: pct(share.overhead) } })
  if (s.approvalMs >= 30_000) out.push({ key: 'approval', vars: { time: duration(s.approvalMs) } })
  for (const r of s.repeatedReads) if (r.count >= 3) out.push({ key: 'reread', vars: { file: baseName(r.file), n: r.count } })
  if (s.tests.length >= 3) out.push({ key: 'tests', vars: { n: s.tests.length, time: duration(s.tests.reduce((a, t) => a + t.wallMs, 0)) } })
  if (s.requests.count >= 3 && s.requests.ttftAvg >= 15_000) out.push({ key: 'ttft', vars: { time: duration(s.requests.ttftAvg) } })
  const input = s.requests.tokensIn + s.requests.cacheRead + s.requests.cacheWrite
  if (s.requests.cacheHit !== null && input >= 100_000 && s.requests.cacheHit < 0.5) out.push({ key: 'cache', vars: { pct: Math.round(s.requests.cacheHit * 100) } })
  if (s.failedTurns > 0) out.push({ key: 'failed', vars: { n: s.failedTurns } })
  const slow = s.slowTools[0]
  if (slow && slow.wallMs >= 120_000) out.push({ key: 'slowTool', vars: { label: slow.label, time: duration(slow.wallMs) } })
  return out
}

function scopeStats(st: ProfilerState, turns: readonly TurnRec[], from: number, to: number, now: number): ScopeStats {
  const inScope = (r: { start: number }) => r.start >= from && r.start <= to
  const tools = st.tools.filter(inScope)
  const reqs = st.requests.filter(inScope)
  const wallOf = (t: ToolRec) => endOf(t, now) - t.start
  const slowTools: SlowTool[] = [...tools]
    .sort((a, b) => wallOf(b) - wallOf(a))
    .slice(0, TOP)
    .map(t => {
      const wallMs = wallOf(t)
      return {
        label: t.label, tool: t.tool, wallMs, wait: t.decision === 'ask' ? 'approval' : 'overhead', open: t.end === undefined,
        ...(t.execMs === undefined ? {} : { execMs: t.execMs, waitMs: Math.max(0, wallMs - t.execMs) }),
        ...(t.ok === undefined ? {} : { ok: t.ok }),
      }
    })
  const approvalMs = tools.reduce((a, t) => (t.decision === 'ask' && t.execMs !== undefined ? a + Math.max(0, wallOf(t) - t.execMs) : a), 0)
  const tests: TestRun[] = tools
    .filter(t => t.cat === 'test')
    .map(t => ({ label: t.label, wallMs: wallOf(t), open: t.end === undefined, ...(t.execMs === undefined ? {} : { execMs: t.execMs }), ...(t.ok === undefined ? {} : { ok: t.ok }) }))
  const subagents: SubagentView[] = st.subagents.filter(inScope).map(s => ({
    id: s.id, type: s.type, ms: endOf(s, now) - s.start, open: s.end === undefined, blocking: s.blocking, failed: s.failed === true,
    requests: st.requests.filter(r => r.ctx === s.id).length,
    tools: st.tools.filter(t => t.ctx === s.id).length,
  }))
  const base = {
    share: shareOf(st, turns, now),
    turns: turns.length,
    failedTurns: st.turns.filter(t => t.failed && inScope(t)).length,
    approvalMs, slowTools, tests,
    requests: requestStats(reqs, now),
    subagents,
    repeatedReads: repeatedReads(tools),
  }
  return { ...base, findings: findingsOf(base) }
}

export function summarize(st: ProfilerState, now: number): ProfileSummary {
  const main = st.turns.filter(t => t.ctx === MAIN)
  const last = main[main.length - 1]
  const inflight: InFlight[] = [
    ...st.tools.filter(t => t.end === undefined).map((t): InFlight => ({ kind: 'tool', label: t.label, since: t.start, ctx: t.ctx })),
    ...st.requests.filter(r => r.end === undefined).map((r): InFlight => ({
      kind: 'request', model: r.model, since: r.start, ctx: r.ctx,
      phase: r.outFrom !== undefined ? 'write' : r.thinkFrom !== undefined ? 'think' : 'wait',
    })),
  ].sort((a, b) => a.since - b.since)
  return {
    at: now,
    session: scopeStats(st, main, -Infinity, Infinity, now),
    turn: last === undefined ? null : scopeStats(st, [last], last.start, endOf(last, now), now),
    inflight,
    droppedTurns: st.droppedTurns,
  }
}

// ---- Texte ----

const DE = {
  'profile.pct': '{n} %',
  'profile.cat.model': 'Modell',
  'profile.cat.tools': 'Tools',
  'profile.cat.subagents': 'Subagents',
  'profile.cat.overhead': 'Engine-Overhead',
  'profile.band.running': 'läuft: {what} {time}',
  'profile.req.wait': 'Modell wartet',
  'profile.req.think': 'Modell denkt',
  'profile.req.write': 'Modell schreibt',
  'profile.sub': '↳ {what}',
  'profile.grade.measured': 'gemessen',
  'profile.grade.observed': 'beobachtet',
  'profile.grade.reported': 'API-Angabe',
  'profile.grade.inferred': 'abgeleitet',
  'profile.sec.split': 'Zeitverteilung',
  'profile.sec.running': 'Läuft gerade',
  'profile.sec.slow': 'Langsamste Tools',
  'profile.sec.tests': 'Testläufe',
  'profile.sec.requests': 'Modell-Requests',
  'profile.sec.subagents': 'Subagents',
  'profile.sec.reads': 'Wiederholte Reads',
  'profile.sec.findings': 'Funde',
  'profile.split.note': '{turns} Turns · Agentenzeit {time} · Leerlauf ausgeschlossen',
  'profile.dropped': 'deckt die letzten {n} Turns ab, {m} ältere verworfen',
  'profile.tool.exec': 'Ausführung {time}',
  'profile.tool.wait.approval': 'Freigabe {time}',
  'profile.tool.wait.overhead': 'Overhead {time}',
  'profile.tool.failed': 'fehlgeschlagen',
  'profile.tool.open': 'läuft',
  'profile.slow.note': 'Ausführung: {exec} · Freigabe/Overhead: {wait}',
  'profile.req.count': 'Requests',
  'profile.req.ttft': 'TTFT Ø / max',
  'profile.req.streams': 'Thinking / Output',
  'profile.req.tokens': 'Tokens ein / aus',
  'profile.req.cache': 'Cache gelesen / geschrieben',
  'profile.req.hit': 'Trefferquote {pct}',
  'profile.req.note': 'TTFT = Senden bis erster Chunk (Netz, Warteschlange, Prefill), Wartezeit am Client, keine Inferenzzeit.',
  'profile.sub.async': 'im Hintergrund',
  'profile.sub.failed': 'abgebrochen',
  'profile.sub.counts': '{r} Requests · {t} Tools',
  'profile.reads.value': '{n}× gelesen',
  'profile.find.overhead': 'Engine-Overhead bei {pct} – Hooks, Kontextaufbau oder Kompaktierung prüfen.',
  'profile.find.approval': 'Freigaben warteten insgesamt {time} – Erlaubnisregeln für häufige Tools erweitern.',
  'profile.find.reread': '{file} wurde {n}× unverändert gelesen.',
  'profile.find.tests': 'Tests liefen {n}× ({time}) – gezielter testen?',
  'profile.find.ttft': 'Erster Chunk kam im Schnitt erst nach {time} – großer Kontext?',
  'profile.find.cache': 'Cache-Trefferquote nur {pct} – Kontext wird oft neu geschrieben.',
  'profile.find.failed': '{n} Turn(s) mit API-Fehler abgebrochen.',
  'profile.find.slowTool': '{label} dauerte {time}.',
  'profile.find.none': 'Keine Auffälligkeiten.',
  'profile.report.title.session': 'Profil · Session',
  'profile.report.title.turn': 'Profil · letzter Turn',
  'profile.report.empty': 'Noch keine Profildaten.',
} as const

export type ProfileKey = keyof typeof DE

const EN: Record<ProfileKey, string> = {
  'profile.pct': '{n}%',
  'profile.cat.model': 'Model',
  'profile.cat.tools': 'Tools',
  'profile.cat.subagents': 'Subagents',
  'profile.cat.overhead': 'Engine overhead',
  'profile.band.running': 'running: {what} {time}',
  'profile.req.wait': 'Model waiting',
  'profile.req.think': 'Model thinking',
  'profile.req.write': 'Model writing',
  'profile.sub': '↳ {what}',
  'profile.grade.measured': 'measured',
  'profile.grade.observed': 'observed',
  'profile.grade.reported': 'reported',
  'profile.grade.inferred': 'inferred',
  'profile.sec.split': 'Time split',
  'profile.sec.running': 'Running now',
  'profile.sec.slow': 'Slowest tools',
  'profile.sec.tests': 'Test runs',
  'profile.sec.requests': 'Model requests',
  'profile.sec.subagents': 'Subagents',
  'profile.sec.reads': 'Repeated reads',
  'profile.sec.findings': 'Findings',
  'profile.split.note': '{turns} turns · agent time {time} · idle excluded',
  'profile.dropped': 'covers the last {n} turns, {m} older dropped',
  'profile.tool.exec': 'exec {time}',
  'profile.tool.wait.approval': 'approval {time}',
  'profile.tool.wait.overhead': 'overhead {time}',
  'profile.tool.failed': 'failed',
  'profile.tool.open': 'running',
  'profile.slow.note': 'exec: {exec} · approval/overhead: {wait}',
  'profile.req.count': 'Requests',
  'profile.req.ttft': 'TTFT avg / max',
  'profile.req.streams': 'Thinking / output',
  'profile.req.tokens': 'Tokens in / out',
  'profile.req.cache': 'Cache read / written',
  'profile.req.hit': 'hit rate {pct}',
  'profile.req.note': 'TTFT = send to first chunk (network, queueing, prefill), a wait at the client, not inference time.',
  'profile.sub.async': 'background',
  'profile.sub.failed': 'aborted',
  'profile.sub.counts': '{r} requests · {t} tools',
  'profile.reads.value': 'read {n}×',
  'profile.find.overhead': 'Engine overhead at {pct} – check hooks, context building or compaction.',
  'profile.find.approval': 'Approvals waited {time} in total – widen permission rules for frequent tools.',
  'profile.find.reread': '{file} was read {n}× unchanged.',
  'profile.find.tests': 'Tests ran {n}× ({time}) – test more selectively?',
  'profile.find.ttft': 'First chunk took {time} on average – large context?',
  'profile.find.cache': 'Cache hit rate only {pct} – context is rewritten often.',
  'profile.find.failed': '{n} turn(s) ended by an API error.',
  'profile.find.slowTool': '{label} took {time}.',
  'profile.find.none': 'Nothing stands out.',
  'profile.report.title.session': 'Profile · session',
  'profile.report.title.turn': 'Profile · last turn',
  'profile.report.empty': 'No profile data yet.',
}

export const PROFILE_TEXT = { de: DE, en: EN } as const

export function tx(L: UiLang, key: ProfileKey, vars?: Record<string, string | number>): string {
  const text: string = PROFILE_TEXT[L][key]
  return vars === undefined ? text : text.replace(/\{(\w+)\}/g, (m, name: string) => (name in vars ? String(vars[name]) : m))
}

const pctOf = (part: number, total: number): number => (total <= 0 ? 0 : Math.round((part / total) * 100))
const pctText = (L: UiLang, n: number): string => tx(L, 'profile.pct', { n })

function tokens(n: number, L: UiLang): string {
  const s = n < 1000 ? String(n) : n < 1_000_000 ? `${(n / 1000).toFixed(1)}k` : `${(n / 1_000_000).toFixed(1)}M`
  return L === 'de' ? s.replace('.', ',') : s
}

function inflightLabel(f: InFlight, L: UiLang): string {
  const what = f.kind === 'tool' ? f.label : tx(L, `profile.req.${f.phase}`)
  return f.ctx === MAIN ? what : tx(L, 'profile.sub', { what })
}

// Was das Band als „läuft“ zeigt: das älteste offene Tool, sonst der älteste offene Request.
const runningOf = (sum: ProfileSummary): InFlight | undefined => sum.inflight.find(f => f.kind === 'tool') ?? sum.inflight[0]

export function bandText(sum: ProfileSummary, now: number, L: UiLang): string {
  const s = sum.session.share
  const parts: string[] = []
  if (s.total > 0) {
    parts.push(`${tx(L, 'profile.cat.model')} ${pctText(L, pctOf(s.model, s.total))}`, `${tx(L, 'profile.cat.tools')} ${pctText(L, pctOf(s.tools, s.total))}`)
    if (pctOf(s.subagents, s.total) >= 1) parts.push(`${tx(L, 'profile.cat.subagents')} ${pctText(L, pctOf(s.subagents, s.total))}`)
  }
  const run = runningOf(sum)
  if (run) parts.push(tx(L, 'profile.band.running', { what: inflightLabel(run, L), time: duration(Math.max(0, now - run.since)) }))
  return parts.join(' · ')
}

export type ProfileRow = { label: string; value: string; pct?: number; tone?: 'ok' | 'warn' | 'bad' | 'dim' }
export type ProfileSection = { title: string; rows: ProfileRow[]; note?: string }

const gradeOf = (L: UiLang, g: Grade): string => tx(L, `profile.grade.${g}`)

function splitRows(s: Share, L: UiLang): ProfileRow[] {
  const row = (key: ProfileKey, ms: number, tone?: ProfileRow['tone']): ProfileRow => ({
    label: tx(L, key), value: `${duration(ms)} · ${pctText(L, pctOf(ms, s.total))}`, pct: pctOf(ms, s.total), ...(tone ? { tone } : {}),
  })
  return [
    row('profile.cat.model', s.model),
    row('profile.cat.tools', s.tools),
    row('profile.cat.subagents', s.subagents),
    row('profile.cat.overhead', s.overhead, s.total > 0 && s.overhead / s.total >= 0.25 ? 'warn' : 'dim'),
  ]
}

function toolValue(t: SlowTool, L: UiLang): string {
  const parts = [duration(t.wallMs)]
  if (t.execMs !== undefined) parts.push(tx(L, 'profile.tool.exec', { time: duration(t.execMs) }))
  if (t.waitMs !== undefined && t.waitMs > 0) parts.push(tx(L, `profile.tool.wait.${t.wait}`, { time: duration(t.waitMs) }))
  if (t.open) parts.push(tx(L, 'profile.tool.open'))
  else if (t.ok === false) parts.push(tx(L, 'profile.tool.failed'))
  return parts.join(' · ')
}

function requestRows(r: RequestStats, L: UiLang): ProfileRow[] {
  return [
    { label: tx(L, 'profile.req.count'), value: String(r.count) },
    { label: tx(L, 'profile.req.ttft'), value: `${duration(r.ttftAvg)} / ${duration(r.ttftMax)}` },
    { label: tx(L, 'profile.req.streams'), value: `${duration(r.thinkMs)} / ${duration(r.outMs)}` },
    { label: tx(L, 'profile.req.tokens'), value: `${tokens(r.tokensIn, L)} / ${tokens(r.tokensOut, L)}` },
    {
      label: tx(L, 'profile.req.cache'),
      value: `${tokens(r.cacheRead, L)} / ${tokens(r.cacheWrite, L)}${r.cacheHit === null ? '' : ` · ${tx(L, 'profile.req.hit', { pct: pctText(L, Math.round(r.cacheHit * 100)) })}`}`,
      ...(r.cacheHit !== null && r.cacheHit < 0.5 ? { tone: 'warn' as const } : {}),
    },
  ]
}

function subagentRow(s: SubagentView, L: UiLang): ProfileRow {
  const parts = [duration(s.ms), tx(L, 'profile.sub.counts', { r: s.requests, t: s.tools })]
  if (!s.blocking) parts.push(tx(L, 'profile.sub.async'))
  if (s.open) parts.push(tx(L, 'profile.tool.open'))
  if (s.failed) parts.push(tx(L, 'profile.sub.failed'))
  return { label: s.type, value: parts.join(' · '), ...(s.failed ? { tone: 'bad' as const } : !s.blocking ? { tone: 'dim' as const } : {}) }
}

const findingText = (f: Finding, L: UiLang): string => {
  const vars = f.key === 'overhead' || f.key === 'cache' ? { ...f.vars, pct: pctText(L, Number(f.vars.pct)) } : f.vars
  return tx(L, `profile.find.${f.key}`, vars)
}

function droppedNote(sum: ProfileSummary, L: UiLang): string {
  return sum.droppedTurns > 0 ? tx(L, 'profile.dropped', { n: sum.session.turns, m: sum.droppedTurns }) : ''
}

// Abschnitte fürs Panel (Session-Sicht). Zeitverteilung immer; leere Abschnitte fallen weg.
export function paneView(sum: ProfileSummary, now: number, L: UiLang): ProfileSection[] {
  const s = sum.session
  const dropped = droppedNote(sum, L)
  const out: ProfileSection[] = [{
    title: tx(L, 'profile.sec.split'),
    rows: splitRows(s.share, L),
    note: [tx(L, 'profile.split.note', { turns: s.turns, time: duration(s.share.total) }), dropped].filter(Boolean).join(' · '),
  }]
  if (sum.inflight.length > 0) {
    out.push({
      title: tx(L, 'profile.sec.running'),
      rows: sum.inflight.map(f => {
        const ms = Math.max(0, now - f.since)
        return { label: inflightLabel(f, L), value: duration(ms), ...(ms >= 60_000 ? { tone: 'warn' as const } : {}) }
      }),
    })
  }
  if (s.slowTools.length > 0) {
    out.push({
      title: tx(L, 'profile.sec.slow'),
      rows: s.slowTools.map(t => ({ label: t.label, value: toolValue(t, L), ...(t.ok === false ? { tone: 'bad' as const } : {}) })),
      note: tx(L, 'profile.slow.note', { exec: gradeOf(L, GRADE.exec), wait: gradeOf(L, GRADE.wait) }),
    })
  }
  if (s.requests.count > 0) out.push({ title: tx(L, 'profile.sec.requests'), rows: requestRows(s.requests, L), note: tx(L, 'profile.req.note') })
  if (s.subagents.length > 0) out.push({ title: tx(L, 'profile.sec.subagents'), rows: s.subagents.map(x => subagentRow(x, L)) })
  if (s.findings.length > 0) out.push({ title: tx(L, 'profile.sec.findings'), rows: s.findings.map(f => ({ label: findingText(f, L), value: '', tone: 'warn' as const })) })
  return out
}

// Bericht für `/cockpit-profil` (Markdown-Text, terminal-tauglich): ganze Session oder letzter Turn.
export function reportText(sum: ProfileSummary, now: number, L: UiLang, scope: 'session' | 'turn'): string {
  const s = scope === 'turn' ? sum.turn : sum.session
  if (!s || (s.share.total === 0 && s.requests.count === 0 && s.slowTools.length === 0)) return tx(L, 'profile.report.empty')
  const g = (grade: Grade) => ` [${gradeOf(L, grade)}]`
  const lines: string[] = [`## ${tx(L, scope === 'turn' ? 'profile.report.title.turn' : 'profile.report.title.session')}`, '']
  lines.push(tx(L, 'profile.split.note', { turns: s.turns, time: duration(s.share.total) }) + g(GRADE.turn))
  const dropped = droppedNote(sum, L)
  if (dropped && scope === 'session') lines.push(dropped)
  lines.push('')
  const split: [ProfileKey, number, Grade][] = [
    ['profile.cat.model', s.share.model, GRADE.request],
    ['profile.cat.tools', s.share.tools, GRADE.toolWall],
    ['profile.cat.subagents', s.share.subagents, GRADE.subagent],
    ['profile.cat.overhead', s.share.overhead, GRADE.overhead],
  ]
  for (const [key, ms, grade] of split) lines.push(`- ${tx(L, key)}: ${pctText(L, pctOf(ms, s.share.total))} (${duration(ms)})${g(grade)}`)
  const section = (key: ProfileKey, rows: string[]) => {
    if (rows.length > 0) lines.push('', `### ${tx(L, key)}`, ...rows.map(r => `- ${r}`))
  }
  if (scope === 'session') section('profile.sec.running', sum.inflight.map(f => `${inflightLabel(f, L)}: ${duration(Math.max(0, now - f.since))}`))
  section('profile.sec.slow', s.slowTools.map(t => `${t.label}: ${toolValue(t, L)}`))
  section('profile.sec.tests', s.tests.map(t => `${t.label}: ${toolValue({ ...t, tool: 'Bash', wait: 'overhead' }, L)}`))
  if (s.requests.count > 0) {
    const r = s.requests
    section('profile.sec.requests', [
      `${tx(L, 'profile.req.count')}: ${r.count}`,
      `${tx(L, 'profile.req.ttft')}: ${duration(r.ttftAvg)} / ${duration(r.ttftMax)}${g(GRADE.ttft)}`,
      `${tx(L, 'profile.req.streams')}: ${duration(r.thinkMs)} / ${duration(r.outMs)}${g(GRADE.stream)}`,
      `${tx(L, 'profile.req.tokens')}: ${tokens(r.tokensIn, L)} / ${tokens(r.tokensOut, L)}${g(GRADE.tokens)}`,
      ...requestRows(r, L).slice(4).map(row => `${row.label}: ${row.value}${g(GRADE.tokens)}`),
    ])
  }
  section('profile.sec.subagents', s.subagents.map(x => {
    const row = subagentRow(x, L)
    return `${row.label}: ${row.value}`
  }))
  section('profile.sec.reads', s.repeatedReads.map(r => `${r.file}${r.range ? ` (${r.range})` : ''}: ${tx(L, 'profile.reads.value', { n: r.count })}`))
  section('profile.sec.findings', s.findings.length > 0 ? s.findings.map(f => findingText(f, L)) : [tx(L, 'profile.find.none')])
  if (s.requests.count > 0) lines.push('', tx(L, 'profile.req.note'))
  return lines.join('\n')
}
