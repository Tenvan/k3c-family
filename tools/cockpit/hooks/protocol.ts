// Protokoll im Reiter Diagnose: Log, Fehler-Eingang, Tool-Freigaben und Zeitreise waren vier Listen über dieselben
// Tool-Aufrufe. Hier werden sie zu einer Liste, neueste zuerst, mit Filter. Rein ohne `$`, damit es direkt testbar ist.
import type { LogEntry, LogFilter, LogSrc } from '../types'
import type { ErrEntry } from './errors'
import type { Step as TlStep } from './timeline'

export const PROTO_MAX = 60
// Rückfall ohne `tool_use_id` (ältere Einträge, Mockdaten): Log, Zeitreise und Fehler-Eingang stempeln denselben Aufruf mit
// eigener Uhr (Start bzw. Ende); innerhalb dieses Abstands gilt er als derselbe.
export const MATCH_MS = 3_000

export type ProtoRow = {
  at: number
  src: LogSrc
  text: string
  tool?: string
  target?: string
  action?: string
  // tool_use_id des Aufrufs: verbindet Log, Fehler-Eingang und Zeitreise exakt
  id?: string
  isError: boolean
  // Zeitreise: Nummer des Schritts mit Snapshot (0 = Stand beim Start der Session)
  snap?: number
  // Fehler-Eingang: erste Fehlerzeile, Wiederholungen, abgelehnt statt gescheitert, aus einem Subagent
  errText?: string
  times?: number
  denied?: boolean
  agent?: boolean
}

const keep = (r: ProtoRow, f: LogFilter): boolean =>
  f === 'all' ? true : f === 'errors' ? r.isError && r.denied !== true : f === 'gate' ? r.src === 'gate' || r.denied === true : r.snap !== undefined

// `base`: die Zeitreise hat einen Start-Snapshot (#0); er steht als ältester Eintrag ganz unten.
export function protocolRows(log: readonly LogEntry[], errors: readonly ErrEntry[], steps: readonly TlStep[], base: boolean, filter: LogFilter, max = PROTO_MAX): ProtoRow[] {
  const rows: ProtoRow[] = log.map(l => ({
    at: l.at,
    src: l.src ?? 'main',
    text: l.text,
    ...(l.tool !== undefined ? { tool: l.tool } : {}),
    ...(l.target !== undefined ? { target: l.target } : {}),
    ...(l.action !== undefined ? { action: l.action } : {}),
    ...(l.id !== undefined ? { id: l.id } : {}),
    isError: l.isError === true,
  }))
  // Derselbe Aufruf über die tool_use_id; ohne sie der zeitlich nächste noch freie Tool-Eintrag mit demselben Tool und ohne
  // eigene id, der `fits` erfüllt.
  const closest = (tool: string, at: number, used: Set<ProtoRow>, fits: (r: ProtoRow) => boolean, id?: string): ProtoRow | undefined => {
    if (id !== undefined) {
      const hit = rows.find(r => r.id === id && !used.has(r))
      if (hit !== undefined) return hit
    }
    let best: ProtoRow | undefined
    for (const r of rows) {
      if (id !== undefined && r.id !== undefined) continue
      if (r.tool !== tool || used.has(r) || Math.abs(r.at - at) > MATCH_MS || !fits(r)) continue
      if (best === undefined || Math.abs(r.at - at) < Math.abs(best.at - at)) best = r
    }
    return best
  }
  const usedSnap = new Set<ProtoRow>()
  for (const st of steps) {
    if (st.snap === undefined) continue
    const hit = closest(st.tool, st.at, usedSnap, () => true, st.id)
    if (hit !== undefined) {
      hit.snap = st.n
      usedSnap.add(hit)
    } else rows.push({ at: st.at, src: 'main', text: '', tool: st.tool, target: st.target, isError: !st.ok, snap: st.n, ...(st.id !== undefined ? { id: st.id } : {}) })
  }
  const usedErr = new Set<ProtoRow>()
  for (const x of errors) {
    const denied = x.kind === 'denied'
    // Aufrufe eines Subagents stehen im Log mit Quelle `agent`.
    const hit = closest(x.tool, x.at, usedErr, r => r.src === (x.agent ? 'agent' : 'main') && (r.isError || denied), x.id)
    if (hit !== undefined) {
      Object.assign(hit, { errText: x.text, times: x.times, denied, isError: true })
      usedErr.add(hit)
    } else rows.push({ at: x.at, src: x.agent ? 'agent' : 'main', text: '', tool: x.tool, target: x.target, isError: true, errText: x.text, times: x.times, denied, agent: x.agent })
  }
  const withBase = base && (filter === 'all' || filter === 'snaps')
  const shown = rows.filter(r => keep(r, filter)).sort((a, b) => b.at - a.at).slice(0, Math.max(0, withBase ? max - 1 : max))
  if (withBase) shown.push({ at: 0, src: 'main', text: '', isError: false, snap: 0 })
  return shown
}

// Zähler für die Filter-Links („Fehler 3“), ohne Kappung; Fehler zählen mit ihren Wiederholungen wie die Bilanz darüber.
export function protocolCounts(log: readonly LogEntry[], errors: readonly ErrEntry[], steps: readonly TlStep[], base: boolean): Record<LogFilter, number> {
  const all = protocolRows(log, errors, steps, base, 'all', Number.MAX_SAFE_INTEGER)
  return {
    all: all.length,
    errors: all.filter(r => keep(r, 'errors')).reduce((n, r) => n + (r.times ?? 1), 0),
    gate: all.filter(r => keep(r, 'gate')).length,
    snaps: all.filter(r => keep(r, 'snaps')).length,
  }
}
