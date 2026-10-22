import type { UsageBucket } from '../api';
import { formatDuration, formatNumber, formatTime } from '../lib/format';

// Reine Funktionen der Live-Monitore (B-065, Band 2 rechts): Zeitfenster, Punkte und Ranglisten aus der
// Minuten-Zeitreihe. Gerechnet wird nur lokal aus den Minuten; Perzentile und Ausreißer kommen aus Go.

export const METRICS = ['calls', 'duration'] as const;
export type Metric = (typeof METRICS)[number];

export const RANGES = ['15m', '1h', '24h', '7d'] as const;
export type Range = (typeof RANGES)[number];

const MIN = 60_000;
const HOUR = 60 * MIN;

/** Spanne und Punkt-Abstand je Zeitraum: 1 min, 2 min, 30 min, 6 h. */
export const RANGE_SPEC: Record<Range, { label: string; span: number; step: number }> = {
  '15m': { label: '15 min', span: 15 * MIN, step: MIN },
  '1h': { label: '1 h', span: HOUR, step: 2 * MIN },
  '24h': { label: '24 h', span: 24 * HOUR, step: 30 * MIN },
  '7d': { label: '7 T', span: 7 * 24 * HOUR, step: 6 * HOUR },
};

export interface Window {
  from: number; // Beginn des ersten Punkts
  to: number; // Ende des letzten Punkts (der laufende Punkt)
  step: number;
  count: number;
}

/** Fenster, das mit der Uhr wandert: der letzte Punkt enthält jetzt. */
export function windowOf(range: Range, now: number): Window {
  const { span, step } = RANGE_SPEC[range];
  const to = Math.floor(now / step) * step + step;
  return { from: to - span, to, step, count: span / step };
}

export interface Point {
  ts: number;
  calls: number;
  errors: number;
  maxMs: number;
  sumMs: number;
  outliers: number;
}

const sum = (m: Record<string, number>) => Object.values(m).reduce((a, b) => a + b, 0);
const max = (m: Record<string, number>) => Object.values(m).reduce((a, b) => Math.max(a, b), 0);

/** Punkte des Fensters; Minuten außerhalb fallen weg, leere Punkte sind 0. */
export function pointsOf(minutes: UsageBucket[], w: Window): Point[] {
  const pts: Point[] = Array.from({ length: w.count }, (_, i) =>
    ({ ts: w.from + i * w.step, calls: 0, errors: 0, maxMs: 0, sumMs: 0, outliers: 0 }));
  for (const m of minutes) {
    const i = Math.floor((m.ts - w.from) / w.step);
    if (i < 0 || i >= w.count) continue;
    const p = pts[i];
    p.calls += sum(m.calls);
    p.errors += m.errors;
    p.maxMs = Math.max(p.maxMs, max(m.maxMs));
    p.sumMs += sum(m.ms);
    p.outliers += sum(m.outliers);
  }
  return pts;
}

/** Kopfzeile des Liniendiagramms. */
export function header(pts: Point[], metric: Metric): string {
  const calls = pts.reduce((a, p) => a + p.calls, 0);
  if (metric === 'calls') {
    const errors = pts.reduce((a, p) => a + p.errors, 0);
    const peak = pts.reduce((a, p) => Math.max(a, p.calls), 0);
    return `${formatNumber(calls)} Aufrufe · ${formatNumber(errors)} Fehler · max ${formatNumber(peak)}/Punkt`;
  }
  if (calls === 0) return 'max – · 0 Ausreißer · Ø –';
  const peak = pts.reduce((a, p) => Math.max(a, p.maxMs), 0);
  const outliers = pts.reduce((a, p) => a + p.outliers, 0);
  const avg = pts.reduce((a, p) => a + p.sumMs, 0) / calls;
  return `max ${formatDuration(peak)} · ${formatNumber(outliers)} Ausreißer · Ø ${formatDuration(avg)}`;
}

/** Ø je Punkt; ohne Aufrufe null (Lücke in der Linie). */
export function avgOf(p: Point): number | null {
  return p.calls > 0 ? p.sumMs / p.calls : null;
}

export interface Tick {
  at: number; // 0..1 über die Breite
  label: string;
}

/** Achse: vier Marken mit Uhrzeit (bei 7 T Datum), rechts `jetzt`. */
export function ticks(range: Range, w: Window): Tick[] {
  const label = (t: number) => (range === '7d'
    ? new Date(t).toLocaleDateString('de-DE', { day: '2-digit', month: '2-digit' })
    : formatTime(new Date(t)));
  const out: Tick[] = [0, 0.25, 0.5, 0.75].map((at) => ({ at, label: label(w.from + at * (w.to - w.from)) }));
  return [...out, { at: 1, label: 'jetzt' }];
}
