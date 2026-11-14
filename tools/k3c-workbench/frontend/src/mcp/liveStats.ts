import type { UsageBucket, UsageRules } from '../api';
import { niceExtent, niceTicks } from '@webkit/lib/chartScale';
import { formatDuration, formatNumber, formatTime } from '../lib/format';
import { avgOf, RANGE_SPEC, type Metric, type Point, type Range, type Window } from './series';

// Lesbarkeit des Kastens „Live“ (B-874): Summen je Tool im Fenster, Werteachse mit Einheit, Tooltips je Punkt und die
// Erklärung der Ausreißer. Rein, ohne DOM.

export interface ToolTotal {
  name: string;
  calls: number;
  sumMs: number;
  maxMs: number;
  outliers: number;
}

/** Summen je Tool aus den Minuten des Fensters, meiste Aufrufe zuerst. */
export function toolTotals(minutes: UsageBucket[], w: Pick<Window, 'from' | 'to'>): ToolTotal[] {
  const map = new Map<string, ToolTotal>();
  for (const m of minutes) {
    if (m.ts < w.from || m.ts >= w.to) continue;
    for (const [name, calls] of Object.entries(m.calls)) {
      const t = map.get(name) ?? { name, calls: 0, sumMs: 0, maxMs: 0, outliers: 0 };
      t.calls += calls;
      t.sumMs += m.ms[name] ?? 0;
      t.maxMs = Math.max(t.maxMs, m.maxMs[name] ?? 0);
      t.outliers += m.outliers[name] ?? 0;
      map.set(name, t);
    }
  }
  return [...map.values()].filter((t) => t.calls > 0).sort((a, b) => b.calls - a.calls || a.name.localeCompare(b.name));
}

/** Balkenwert je Tool: Aufrufe oder Ø-Laufzeit in ms. */
export function barValue(t: ToolTotal, metric: Metric): number {
  return metric === 'calls' ? t.calls : t.sumMs / t.calls;
}

/** Beschriftung des Balkens mit Einheit. */
export function barLabel(t: ToolTotal, metric: Metric): string {
  return metric === 'calls' ? `${formatNumber(t.calls)}×` : `Ø ${formatDuration(t.sumMs / t.calls)}`;
}

/** Tooltip eines Balkens mit allen Zahlen des Tools. */
export function barTip(t: ToolTotal): string {
  const out = t.outliers > 0 ? ` · ${formatNumber(t.outliers)} Ausreißer` : '';
  return `${t.name}: ${formatNumber(t.calls)} Aufrufe · Ø ${formatDuration(t.sumMs / t.calls)} · max ${formatDuration(t.maxMs)}${out}`;
}

export interface Axis {
  max: number;
  ticks: { v: number; label: string }[];
}

/** Werteachse ab 0 mit runden Marken; Aufrufe ganzzahlig, Laufzeit als Dauer beschriftet. */
export function axisOf(peak: number, metric: Metric): Axis {
  const top = niceExtent({ min: 0, max: Math.max(metric === 'calls' ? 4 : 10, peak) }, 3).max;
  const ticks = niceTicks(0, top, 3).filter((v) => metric === 'duration' || Number.isInteger(v));
  return { max: top, ticks: ticks.map((v) => ({ v, label: metric === 'calls' ? formatNumber(v) : formatDuration(v) })) };
}

/** Punkt-Abstand als Text: „1 min“, „2 min“, „30 min“, „6 h“. */
export function stepLabel(range: Range): string {
  const min = RANGE_SPEC[range].step / 60_000;
  return min >= 60 ? `${min / 60} h` : `${min} min`;
}

/** Einheit der Werteachse. */
export function unitLabel(range: Range, metric: Metric): string {
  return metric === 'calls' ? `Aufrufe je ${stepLabel(range)}` : `Laufzeit je Aufruf (max und Ø je ${stepLabel(range)})`;
}

/** Tooltip eines Punkts: Zeitraum des Punkts und alle Werte. */
export function pointTip(p: Point, range: Range): string {
  const { step } = RANGE_SPEC[range];
  const when = range === '7d'
    ? `${new Date(p.ts).toLocaleString('de-DE', { day: '2-digit', month: '2-digit', hour: '2-digit', minute: '2-digit' })}`
    : formatTime(new Date(p.ts));
  const head = `${when} – ${formatTime(new Date(p.ts + step))}`;
  if (p.calls === 0) return `${head}: keine Aufrufe`;
  const avg = avgOf(p) ?? 0;
  const out = p.outliers > 0 ? ` · ${formatNumber(p.outliers)} Ausreißer` : '';
  return `${head}: ${formatNumber(p.calls)} Aufrufe · ${formatNumber(p.errors)} Fehler · max ${formatDuration(p.maxMs)} · Ø ${formatDuration(avg)}${out}`;
}

/** Erklärung der Ausreißer aus den Regeln in Go; ohne Regeln eine allgemeine Fassung. */
export function outlierNote(rules?: UsageRules): string {
  if (!rules) return 'Ausreißer: Aufruf deutlich langsamer als üblich für dieses Tool und diese Argumente.';
  return `Ausreißer: Aufruf länger als ${formatNumber(rules.outlierFactor, 1)}× das p95 desselben Tools (mit denselben `
    + `Argumenten, sobald ${formatNumber(rules.baselineCalls)} Aufrufe vorliegen) und mindestens ${formatDuration(rules.outlierFloorMs)}.`;
}
