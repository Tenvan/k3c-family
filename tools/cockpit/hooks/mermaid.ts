// Mermaid-Karten: reine Funktionen ohne `$` und ohne UI – Parser, einfaches eigenes Layout und Svg-Quelle.
// Unterstützt flowchart/graph (TD, TB, LR, RL, BT) und sequenceDiagram; alles andere oder ein Parse-Fehler ergibt null,
// dann bleibt der Codeblock natives Markdown. Idee aus claude-skins (MIT), eigener Code.
import { THEME } from './core'

export type FlowShape = 'rect' | 'round' | 'circle' | 'diamond'
export type FlowNode = { id: string; label: string; shape: FlowShape }
export type FlowEdge = { from: string; to: string; label: string; dashed: boolean; arrow: boolean }
export type Flowchart = { type: 'flowchart'; dir: 'TD' | 'LR' | 'BT' | 'RL'; nodes: FlowNode[]; edges: FlowEdge[] }
export type SeqStep =
  | { kind: 'msg'; from: string; to: string; text: string; dashed: boolean; arrow: boolean }
  | { kind: 'note'; over: string[]; text: string }
  | { kind: 'block'; text: string }
export type Sequence = { type: 'sequence'; actors: { id: string; label: string }[]; steps: SeqStep[] }
export type Diagram = Flowchart | Sequence

const LABEL_MAX = 36
const clean = (s: string): string => {
  const x = s.trim().replace(/^"(.*)"$/s, '$1').replace(/<br\s*\/?>/gi, ' ').replace(/\s+/g, ' ').trim()
  return x.length > LABEL_MAX ? `${x.slice(0, LABEL_MAX - 1)}…` : x
}

// Zeilen ohne Kommentare und Leerzeilen; Semikolons trennen Anweisungen wie Zeilenumbrüche.
const statements = (src: string): string[] =>
  src
    .replace(/\r\n?/g, '\n')
    .split('\n')
    .flatMap(l => l.replace(/%%.*$/, '').split(';'))
    .map(l => l.trim())
    .filter(l => l !== '')

// Typ aus der ersten Zeile, für den Kartenkopf („Mermaid · flowchart“); '' ohne erkennbaren Kopf.
export function diagramType(src: string): string {
  const first = statements(src)[0] ?? ''
  const m = /^([A-Za-z][\w-]*)/.exec(first)
  return m ? (m[1] === 'graph' ? 'flowchart' : (m[1] ?? '')) : ''
}

export function parseMermaid(src: string): Diagram | null {
  const lines = statements(src)
  const head = lines[0] ?? ''
  if (/^(graph|flowchart)\b/i.test(head)) return parseFlow(head, lines.slice(1))
  if (/^sequenceDiagram\s*$/.test(head)) return parseSequence(lines.slice(1))
  return null
}

// ── Flowchart ─────────────────────────────────────────────────────────────────

const SHAPES: readonly [string, string, FlowShape][] = [
  ['((', '))', 'circle'],
  ['([', '])', 'round'],
  ['[[', ']]', 'rect'],
  ['[(', ')]', 'rect'],
  ['{{', '}}', 'diamond'],
  ['[/', '/]', 'rect'],
  ['[\\', '\\]', 'rect'],
  ['[', ']', 'rect'],
  ['(', ')', 'round'],
  ['{', '}', 'diamond'],
  ['>', ']', 'rect'],
]
const IGNORED = /^(subgraph|end|classDef|class|style|linkStyle|click|direction|accTitle|accDescr)\b/
const ID = /^[\p{L}\p{N}_]+/u
// Text-Form „-- Text -->“, sonst Operator mit optionalem „|Text|“.
const LINK_TEXT = /^(--|==|-\.)\s+(.+?)\s+(-{2,}>|={2,}>|\.-+>|-{3,}|={3,}|\.-+)/
const LINK_OP = /^<?(-{2,}>|-{3,}|={2,}>|={3,}|-\.+->|-\.+-|--[xo]|==[xo])(?:\|([^|]*)\|)?/

function parseFlow(head: string, lines: readonly string[]): Flowchart | null {
  const d = /^(?:graph|flowchart)(?:\s+(TD|TB|LR|RL|BT))?\s*$/i.exec(head)
  if (!d) return null
  const dirRaw = (d[1] ?? 'TD').toUpperCase()
  const dir = (dirRaw === 'TB' ? 'TD' : dirRaw) as Flowchart['dir']
  const nodes = new Map<string, FlowNode>()
  const edges: FlowEdge[] = []

  // Ein Knoten an Position 0 von `s`: Länge des gelesenen Stücks, sonst 0.
  const node = (s: string): { id: string; len: number } | null => {
    const m = ID.exec(s)
    if (!m) return null
    const id = m[0]
    let len = id.length
    let rest = s.slice(len)
    for (const [open, close, shape] of SHAPES) {
      if (!rest.startsWith(open)) continue
      const end = rest.indexOf(close, open.length)
      if (end < 0) return null
      nodes.set(id, { id, label: clean(rest.slice(open.length, end)) || id, shape })
      len += end + close.length
      rest = s.slice(len)
      break
    }
    const cls = /^:::[\w-]+/.exec(rest)
    if (cls) len += cls[0].length
    if (!nodes.has(id)) nodes.set(id, { id, label: id, shape: 'rect' })
    return { id, len }
  }
  // Gruppe „A & B“.
  const group = (s: string): { ids: string[]; len: number } | null => {
    const ids: string[] = []
    let at = 0
    for (;;) {
      const n = node(s.slice(at))
      if (!n) return null
      ids.push(n.id)
      at += n.len
      const amp = /^\s*&\s*/.exec(s.slice(at))
      if (!amp) return { ids, len: at }
      at += amp[0].length
    }
  }

  for (const line of lines) {
    if (IGNORED.test(line)) continue
    const first = group(line)
    if (!first) return null
    let at = first.len
    let prev = first.ids
    while (at < line.length) {
      const s = line.slice(at).replace(/^\s+/, '')
      at = line.length - s.length
      if (s === '') break
      const tm = LINK_TEXT.exec(s)
      const om = tm ? null : LINK_OP.exec(s)
      const m = tm ?? om
      if (!m) return null
      const op = tm ? `${tm[1]}${tm[3]}` : (om?.[1] ?? '')
      const label = clean((tm ? tm[2] : om?.[2]) ?? '')
      at += m[0].length
      const restS = line.slice(at).replace(/^\s+/, '')
      at = line.length - restS.length
      const next = group(restS)
      if (!next) return null
      at += next.len
      for (const from of prev) for (const to of next.ids) edges.push({ from, to, label, dashed: op.includes('.'), arrow: /[>xo]$/.test(op) })
      prev = next.ids
    }
  }
  if (nodes.size === 0) return null
  return { type: 'flowchart', dir, nodes: [...nodes.values()], edges }
}

// ── Sequenz ───────────────────────────────────────────────────────────────────

const SEQ_ARROW = /^(.+?)\s*(-->>|->>|--x|-x|--\)|-\)|-->|->)\s*[+-]?\s*(.+?)\s*:\s*(.*)$/
const SEQ_BLOCK = /^(loop|alt|else|opt|par|and|critical|break|rect)\b\s*(.*)$/
const SEQ_SKIP = /^(end|activate|deactivate|autonumber|title|box)\b/

function parseSequence(lines: readonly string[]): Sequence | null {
  const actors = new Map<string, { id: string; label: string }>()
  const actor = (raw: string, label?: string): string => {
    const id = raw.trim()
    const known = actors.get(id)
    if (!known) actors.set(id, { id, label: clean(label ?? id) })
    else if (label !== undefined) known.label = clean(label)
    return id
  }
  const steps: SeqStep[] = []
  for (const line of lines) {
    const p = /^(participant|actor)\s+(.+?)(?:\s+as\s+(.+))?$/.exec(line)
    if (p) {
      actor(p[2] ?? '', p[3])
      continue
    }
    const note = /^note\s+(?:left of|right of|over)\s+([^:]+?)\s*:\s*(.*)$/i.exec(line)
    if (note) {
      steps.push({ kind: 'note', over: (note[1] ?? '').split(',').map(a => actor(a)), text: clean(note[2] ?? '') })
      continue
    }
    const block = SEQ_BLOCK.exec(line)
    if (block) {
      steps.push({ kind: 'block', text: clean(`${block[1]} ${block[2] ?? ''}`) })
      continue
    }
    if (SEQ_SKIP.test(line)) continue
    const m = SEQ_ARROW.exec(line)
    if (!m) return null
    const op = m[2] ?? '->'
    steps.push({ kind: 'msg', from: actor(m[1] ?? ''), to: actor(m[3] ?? ''), text: clean(m[4] ?? ''), dashed: op.startsWith('--'), arrow: op !== '->' && op !== '-->' })
  }
  if (actors.size === 0) return null
  return { type: 'sequence', actors: [...actors.values()], steps }
}

// ── Layout und Svg ────────────────────────────────────────────────────────────

const FONT = 12
// ponytail: Textbreite geschätzt (7 px je Zeichen bei 12 px), keine Messung; lange Labels sind auf LABEL_MAX gekürzt.
const textW = (s: string): number => Math.ceil(s.length * 7)
const esc = (s: string): string => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')
const r1 = (n: number): number => Math.round(n * 10) / 10
const MAX_W = 720

// Breite und Höhe begrenzt, viewBox behält die Zeichnung; gleiche Eingabe ergibt dieselbe Quelle (keine Zeit, kein Zufall).
function frame(w: number, h: number, body: string): string {
  const scale = Math.min(1, MAX_W / w)
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${Math.round(w * scale)}" height="${Math.round(h * scale)}" viewBox="0 0 ${w} ${h}" font-family="system-ui, Segoe UI, sans-serif" font-size="${FONT}">${body}</svg>`
}

const arrowHead = (x: number, y: number, dx: number, dy: number, color: string): string => {
  const len = Math.hypot(dx, dy) || 1
  const ux = dx / len
  const uy = dy / len
  const a = `${r1(x - ux * 8 - uy * 4)},${r1(y - uy * 8 + ux * 4)}`
  const b = `${r1(x - ux * 8 + uy * 4)},${r1(y - uy * 8 - ux * 4)}`
  return `<polygon points="${r1(x)},${r1(y)} ${a} ${b}" fill="${color}"/>`
}

type Box = { x: number; y: number; w: number; h: number; shape: FlowShape }

// Punkt auf dem Rand eines Knotens in Richtung (dx, dy) vom Mittelpunkt aus.
function borderPoint(b: Box, dx: number, dy: number): [number, number] {
  const hw = b.w / 2
  const hh = b.h / 2
  const ax = Math.abs(dx) || 1e-9
  const ay = Math.abs(dy) || 1e-9
  const t = b.shape === 'diamond' ? 1 / (ax / hw + ay / hh) : b.shape === 'circle' ? hw / Math.hypot(dx, dy) : Math.min(hw / ax, hh / ay)
  return [b.x + dx * t, b.y + dy * t]
}

// Rang je Knoten: längster Pfad über Vorwärtskanten; Rückkanten (Zyklen) zählen nicht.
export function flowRanks(f: Flowchart): Map<string, number> {
  const ids = f.nodes.map(n => n.id)
  const out = new Map<string, string[]>(ids.map(id => [id, []]))
  for (const e of f.edges) if (e.from !== e.to) out.get(e.from)?.push(e.to)
  const state = new Map<string, 1 | 2>()
  const forward: [string, string][] = []
  const visit = (v: string) => {
    state.set(v, 1)
    for (const w of out.get(v) ?? []) {
      if (state.get(w) === 1) continue
      forward.push([v, w])
      if (!state.has(w)) visit(w)
    }
    state.set(v, 2)
  }
  for (const id of ids) if (!state.has(id)) visit(id)
  const rank = new Map<string, number>(ids.map(id => [id, 0]))
  for (let i = 0; i < ids.length; i++) {
    let changed = false
    for (const [a, b] of forward) {
      const r = (rank.get(a) ?? 0) + 1
      if (r > (rank.get(b) ?? 0)) {
        rank.set(b, r)
        changed = true
      }
    }
    if (!changed) break
  }
  return rank
}

function flowSvg(f: Flowchart): string {
  const rank = flowRanks(f)
  const layers: string[][] = []
  for (const n of f.nodes) (layers[rank.get(n.id) ?? 0] ??= []).push(n.id)
  // Eine Sortierung nach dem Mittel der Vorgänger-Positionen mindert Kreuzungen.
  const pos = new Map<string, number>()
  layers.forEach(l => l.forEach((id, i) => pos.set(id, i)))
  for (let li = 1; li < layers.length; li++) {
    const l = layers[li] ?? []
    const bary = (id: string) => {
      const ps = f.edges.filter(e => e.to === id && (rank.get(e.from) ?? 0) < li).map(e => pos.get(e.from) ?? 0)
      return ps.length === 0 ? (pos.get(id) ?? 0) : ps.reduce((a, b) => a + b, 0) / ps.length
    }
    l.sort((a, b) => bary(a) - bary(b))
    l.forEach((id, i) => pos.set(id, i))
  }
  const byId = new Map(f.nodes.map(n => [n.id, n]))
  const size = (n: FlowNode): [number, number] => {
    const tw = textW(n.label)
    if (n.shape === 'circle') return [Math.max(44, tw + 20), Math.max(44, tw + 20)]
    if (n.shape === 'diamond') return [Math.max(70, tw + 44), 48]
    return [Math.max(60, tw + 24), 34]
  }
  const vertical = f.dir === 'TD' || f.dir === 'BT'
  const PAD = 16
  const GAP_IN = vertical ? 28 : 22
  const GAP_OUT = vertical ? 54 : 64
  // Je Ebene: Ausdehnung quer (Summe der Knoten) und längs (größter Knoten).
  const across = layers.map(l => l.reduce((s, id) => s + (size(byId.get(id) as FlowNode)[vertical ? 0 : 1] ?? 0), 0) + GAP_IN * Math.max(0, l.length - 1))
  const along = layers.map(l => Math.max(...l.map(id => size(byId.get(id) as FlowNode)[vertical ? 1 : 0] ?? 0)))
  const crossMax = Math.max(...across)
  const mainTotal = along.reduce((a, b) => a + b, 0) + GAP_OUT * Math.max(0, layers.length - 1)
  const W = (vertical ? crossMax : mainTotal) + PAD * 2
  const H = (vertical ? mainTotal : crossMax) + PAD * 2
  const boxes = new Map<string, Box>()
  let main = PAD
  layers.forEach((l, li) => {
    let cross = PAD + (crossMax - (across[li] ?? 0)) / 2
    const a = along[li] ?? 0
    for (const id of l) {
      const n = byId.get(id) as FlowNode
      const [w, h] = size(n)
      const c = vertical ? w : h
      let x = vertical ? cross + w / 2 : main + a / 2
      let y = vertical ? main + a / 2 : cross + h / 2
      if (f.dir === 'BT') y = H - y
      if (f.dir === 'RL') x = W - x
      boxes.set(id, { x, y, w, h, shape: n.shape })
      cross += c + GAP_IN
    }
    main += a + GAP_OUT
  })
  const line = THEME.textSecondary
  const edges = f.edges
    .map(e => {
      const a = boxes.get(e.from)
      const b = boxes.get(e.to)
      if (!a || !b) return ''
      const dash = e.dashed ? ' stroke-dasharray="4 3"' : ''
      if (a === b) {
        const x = a.x + a.w / 2
        const path = `<path d="M${r1(x)} ${r1(a.y - 6)}c22 -14 22 26 0 12" fill="none" stroke="${line}"${dash}/>`
        return `${path}${e.arrow ? arrowHead(x, a.y + 6, -6, 1, line) : ''}${e.label ? `<text x="${r1(x + 22)}" y="${r1(a.y + 4)}" fill="${THEME.textMuted}">${esc(e.label)}</text>` : ''}`
      }
      const [x1, y1] = borderPoint(a, b.x - a.x, b.y - a.y)
      const [x2, y2] = borderPoint(b, a.x - b.x, a.y - b.y)
      const mx = (x1 + x2) / 2
      const my = (y1 + y2) / 2
      const lw = textW(e.label) + 8
      const label = e.label
        ? `<rect x="${r1(mx - lw / 2)}" y="${r1(my - 9)}" width="${lw}" height="18" rx="3" fill="${THEME.surface}"/><text x="${r1(mx)}" y="${r1(my + 4)}" text-anchor="middle" fill="${THEME.textMuted}">${esc(e.label)}</text>`
        : ''
      return `<line x1="${r1(x1)}" y1="${r1(y1)}" x2="${r1(x2)}" y2="${r1(y2)}" stroke="${line}"${dash}/>${e.arrow ? arrowHead(x2, y2, x2 - x1, y2 - y1, line) : ''}${label}`
    })
    .join('')
  const nodes = f.nodes
    .map(n => {
      const b = boxes.get(n.id) as Box
      const st = `fill="${THEME.chip}" stroke="${THEME.accent}"`
      const shape =
        n.shape === 'circle'
          ? `<circle cx="${r1(b.x)}" cy="${r1(b.y)}" r="${r1(b.w / 2)}" ${st}/>`
          : n.shape === 'diamond'
            ? `<polygon points="${r1(b.x)},${r1(b.y - b.h / 2)} ${r1(b.x + b.w / 2)},${r1(b.y)} ${r1(b.x)},${r1(b.y + b.h / 2)} ${r1(b.x - b.w / 2)},${r1(b.y)}" ${st}/>`
            : `<rect x="${r1(b.x - b.w / 2)}" y="${r1(b.y - b.h / 2)}" width="${b.w}" height="${b.h}" rx="${n.shape === 'round' ? 17 : 4}" ${st}/>`
      return `${shape}<text x="${r1(b.x)}" y="${r1(b.y + 4)}" text-anchor="middle" fill="${THEME.orga}">${esc(n.label)}</text>`
    })
    .join('')
  return frame(W, H, `${edges}${nodes}`)
}

function sequenceSvg(s: Sequence): string {
  const PAD = 16
  const colW = Math.max(110, ...s.actors.map(a => textW(a.label) + 28), ...s.steps.map(st => (st.kind === 'msg' && st.from !== st.to ? textW(st.text) + 24 : 0)))
  const headH = 30
  const rowH = 30
  const x = new Map(s.actors.map((a, i) => [a.id, PAD + colW / 2 + i * colW]))
  const W = PAD * 2 + colW * s.actors.length + 40
  const top = PAD + headH + 14
  const H = top + s.steps.length * rowH + headH + PAD
  const heads = (y: number) =>
    s.actors
      .map(a => {
        const cx = x.get(a.id) ?? 0
        const w = Math.min(colW - 16, textW(a.label) + 20)
        return `<rect x="${r1(cx - w / 2)}" y="${y}" width="${w}" height="${headH}" rx="4" fill="${THEME.chip}" stroke="${THEME.accent}"/><text x="${r1(cx)}" y="${y + 19}" text-anchor="middle" fill="${THEME.orga}">${esc(a.label)}</text>`
      })
      .join('')
  const lifelines = s.actors.map(a => `<line x1="${x.get(a.id)}" y1="${PAD + headH}" x2="${x.get(a.id)}" y2="${H - PAD - headH}" stroke="${THEME.border}" stroke-dasharray="3 3"/>`).join('')
  const line = THEME.textSecondary
  const rows = s.steps
    .map((st, i) => {
      const y = top + i * rowH + rowH / 2
      if (st.kind === 'block')
        return `<text x="${PAD}" y="${y + 4}" fill="${THEME.violet}" font-style="italic">${esc(st.text)}</text><line x1="${PAD + textW(st.text) + 8}" y1="${y}" x2="${W - PAD}" y2="${y}" stroke="${THEME.violet}" stroke-opacity="0.4" stroke-dasharray="2 4"/>`
      if (st.kind === 'note') {
        const xs = st.over.map(a => x.get(a) ?? 0)
        const cx = (Math.min(...xs) + Math.max(...xs)) / 2
        const w = Math.max(textW(st.text) + 16, Math.max(...xs) - Math.min(...xs) + 40)
        return `<rect x="${r1(cx - w / 2)}" y="${y - 11}" width="${w}" height="22" rx="3" fill="${THEME.surface}" stroke="${THEME.warning}"/><text x="${r1(cx)}" y="${y + 4}" text-anchor="middle" fill="${THEME.warning}">${esc(st.text)}</text>`
      }
      const x1 = x.get(st.from) ?? 0
      const x2 = x.get(st.to) ?? 0
      const dash = st.dashed ? ' stroke-dasharray="4 3"' : ''
      if (x1 === x2) {
        const path = `<path d="M${x1} ${y - 4}h26v12h-26" fill="none" stroke="${line}"${dash}/>`
        return `${path}${st.arrow ? arrowHead(x1, y + 8, -1, 0, line) : ''}<text x="${x1 + 32}" y="${y + 4}" fill="${THEME.textSecondary}">${esc(st.text)}</text>`
      }
      return `<line x1="${x1}" y1="${y + 4}" x2="${x2}" y2="${y + 4}" stroke="${line}"${dash}/>${st.arrow ? arrowHead(x2, y + 4, x2 - x1, 0, line) : ''}<text x="${r1((x1 + x2) / 2)}" y="${y - 2}" text-anchor="middle" fill="${THEME.textSecondary}">${esc(st.text)}</text>`
    })
    .join('')
  return frame(W, H, `${lifelines}${heads(PAD)}${rows}${heads(H - PAD - headH)}`)
}

// Einfaches Bild (nie interaktiv); Quelle hängt nur vom Diagramm ab, damit ein Neuzeichnen kein neues Bild lädt.
export const mermaidSvg = (d: Diagram): string => (d.type === 'flowchart' ? flowSvg(d) : sequenceSvg(d))
