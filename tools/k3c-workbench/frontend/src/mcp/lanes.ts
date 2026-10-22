import type { McpCall } from '../api';
import { formatNumber } from '../lib/format';

// Reine Funktionen des Aufruf-Logs (B-065, Band 3): Zeilen für Start und Ende je Aufruf und Spuren wie bei Git.
// Jede Spur führt vom Start- zum Endknoten eines Aufrufs; parallele Aufrufe liegen auf eigenen Spuren, eine freie Spur
// wird wiederverwendet, ein laufender Aufruf ist nach oben offen.

export interface Through {
  lane: number;
  color: number;
  running: boolean;
}

export interface Row {
  kind: 'start' | 'end';
  seq: number;
  call: McpCall;
  lane: number; // Spur des eigenen Aufrufs
  color: number; // Farbe je Aufruf (Index in die Palette)
  through: Through[]; // andere Aufrufe, deren Spur durch diese Zeile läuft
}

export interface Graph {
  rows: Row[]; // neueste oben
  lanes: number; // Breite des Graphen in Spuren
}

export const PALETTE_SIZE = 8;

const colorOf = (c: McpCall) => c.id % PALETTE_SIZE;

interface Event {
  kind: 'start' | 'end';
  seq: number;
  call: McpCall;
}

/** Start- und Endereignisse aller Aufrufe, älteste zuerst; laufende haben nur den Start. */
function events(calls: McpCall[]): Event[] {
  const list: Event[] = [];
  for (const c of calls) {
    list.push({ kind: 'start', seq: c.startSeq, call: c });
    if (!c.running) list.push({ kind: 'end', seq: c.endSeq, call: c });
  }
  return list.sort((a, b) => a.seq - b.seq);
}

/** Legt die Spuren an: beim Start die kleinste freie Spur, beim Ende wird sie frei. */
export function buildGraph(calls: McpCall[]): Graph {
  const active = new Map<number, McpCall>(); // Spur → Aufruf
  const laneOf = new Map<number, number>(); // Aufruf-ID → Spur
  const rows: Row[] = [];
  let lanes = 0;
  for (const e of events(calls)) {
    const through = [...active].filter(([, c]) => c.id !== e.call.id)
      .map(([lane, c]) => ({ lane, color: colorOf(c), running: c.running }));
    let lane = laneOf.get(e.call.id);
    if (e.kind === 'start' || lane === undefined) {
      lane = 0;
      while (active.has(lane)) lane++;
      active.set(lane, e.call);
      laneOf.set(e.call.id, lane);
      lanes = Math.max(lanes, lane + 1);
    }
    if (e.kind === 'end') active.delete(lane);
    rows.push({ kind: e.kind, seq: e.seq, call: e.call, lane, color: colorOf(e.call), through });
  }
  return { rows: rows.reverse(), lanes };
}

export interface CallFilter {
  tool: string; // '' = alle
  errorsOnly: boolean;
}

/** Filter nach Tool und nur Fehlern; ein Tool, das die Zähler nicht kennen (nach Neustart), bleibt unter „Alle“. */
export function filterCalls(calls: McpCall[], f: CallFilter): McpCall[] {
  return calls.filter((c) => (!f.tool || c.tool === f.tool) && (!f.errorsOnly || (!c.running && !c.ok)));
}

/** Fußzeile: „N Aufrufe · N laufend · N Fehler“. */
export function callFooter(calls: McpCall[]): string {
  const running = calls.filter((c) => c.running).length;
  const errors = calls.filter((c) => !c.running && !c.ok).length;
  return `${formatNumber(calls.length)} Aufrufe · ${formatNumber(running)} laufend · ${formatNumber(errors)} Fehler`;
}

/** Argumente eingerückt, falls sie JSON sind; sonst wie gesendet. */
export function prettyArgs(args: string): string {
  try {
    return JSON.stringify(JSON.parse(args), null, 2);
  } catch {
    return args;
  }
}
