import type { McpCall } from '../api';
import type { Window } from './series';

// Auswertung der check_run-Aufrufe (B-876) aus dem Aufruf-Log: Läufe je Ziel mit Ergebnis und Laufzeit. Die Zusammen-
// fassung eines Laufs ist die Kopfzeile von check_run: „<ziel> · exit <n> · <dauer>“, „<ziel> · Zeitlimit … · <dauer>“
// oder „<ziel> läuft noch seit …“ (Hintergrund). Fehlschlag = exit ≠ 0, Zeitlimit oder ein abgelehnter Aufruf.

export type RunState = 'ok' | 'failed' | 'pending';

export interface CheckRun {
  call: McpCall;
  target: string;
  /** Ende des Aufrufs in ms seit 1970. */
  at: number;
  /** Laufzeit der Prüfung; aus der Kopfzeile, sonst die Dauer des Aufrufs. */
  ms: number;
  state: RunState;
  exit: number | null;
}

export interface TargetStats {
  target: string;
  count: number;
  failed: number;
  avgMs: number;
  maxMs: number;
  /** Fehlerquote in Prozent der beendeten Läufe. */
  failRate: number;
}

const UNKNOWN = '(ohne Ziel)';

/** Dauer im Format von formatMs (Go): „850 ms“, „4,2 s“, „3 min 5 s“, „1 h 2 min“; unlesbar null. */
export function parseDuration(text: string): number | null {
  const t = text.trim();
  let m = /^(\d+) ms$/.exec(t);
  if (m) return Number(m[1]);
  m = /^(\d+(?:,\d+)?) s$/.exec(t);
  if (m) return Number(m[1].replace(',', '.')) * 1000;
  m = /^(\d+) min (\d+) s$/.exec(t);
  if (m) return (Number(m[1]) * 60 + Number(m[2])) * 1000;
  m = /^(\d+) h (\d+) min$/.exec(t);
  if (m) return (Number(m[1]) * 60 + Number(m[2])) * 60_000;
  return null;
}

/** Ziel aus Feld `target` (Go, B-876), sonst aus den Argumenten oder der Kopfzeile. */
function targetOf(c: McpCall): string {
  if (c.target) return c.target;
  const arg = /"target"\s*:\s*"([^"]+)"/.exec(c.args);
  if (arg) return arg[1];
  const head = /^(\S+) (?:·|läuft)/.exec(c.summary);
  return head ? head[1] : UNKNOWN;
}

/** Ein beendeter check_run-Aufruf als Lauf; laufende Aufrufe und andere Tools fallen weg. */
export function runOf(c: McpCall): CheckRun | null {
  if (c.tool !== 'check_run' || c.running || !c.ts) return null;
  const at = Date.parse(c.ts);
  if (!Number.isFinite(at)) return null;
  const run: CheckRun = { call: c, target: targetOf(c), at, ms: c.durationMs, state: 'ok', exit: null };
  if (!c.ok) return { ...run, state: 'failed' };
  const parts = c.summary.split(' · ');
  const exit = /^exit (-?\d+)$/.exec(parts[1] ?? '');
  const dur = parts.length >= 3 ? parseDuration(parts[parts.length - 1]) : null;
  if (dur !== null) run.ms = dur;
  if (exit) return { ...run, exit: Number(exit[1]), state: Number(exit[1]) === 0 ? 'ok' : 'failed' };
  if (/^Zeitlimit /.test(parts[1] ?? '')) return { ...run, state: 'failed' };
  if (/ läuft noch seit /.test(c.summary)) return { ...run, state: 'pending' };
  return run;
}

/** Läufe im Zeitfenster, älteste zuerst. */
export function runsIn(calls: McpCall[], w: Pick<Window, 'from' | 'to'>): CheckRun[] {
  return calls.flatMap((c) => {
    const r = runOf(c);
    return r && r.at >= w.from && r.at < w.to ? [r] : [];
  }).sort((a, b) => a.at - b.at);
}

/** Kennzahlen je Ziel, häufigstes zuerst; Läufe im Hintergrund zählen mit, gehen aber nicht in Laufzeit und Quote ein. */
export function byTarget(runs: CheckRun[]): TargetStats[] {
  const map = new Map<string, { count: number; done: number; failed: number; sum: number; max: number }>();
  for (const r of runs) {
    const s = map.get(r.target) ?? { count: 0, done: 0, failed: 0, sum: 0, max: 0 };
    s.count++;
    if (r.state !== 'pending') {
      s.done++;
      s.sum += r.ms;
      s.max = Math.max(s.max, r.ms);
      if (r.state === 'failed') s.failed++;
    }
    map.set(r.target, s);
  }
  return [...map].map(([target, s]) => ({
    target, count: s.count, failed: s.failed, maxMs: s.max,
    avgMs: s.done > 0 ? s.sum / s.done : 0,
    failRate: s.done > 0 ? (s.failed / s.done) * 100 : 0,
  })).sort((a, b) => b.count - a.count || a.target.localeCompare(b.target));
}

/** Quelle der Konsole eines Ziels (logs: `check:<ziel>`); sie hält nur den letzten Lauf dieses Ziels. */
export function consoleSource(target: string): string {
  return `check:${target}`;
}
