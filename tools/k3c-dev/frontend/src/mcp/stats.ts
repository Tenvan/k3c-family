import type { SlowCall, ToolUsage, UsageRules, UsageScope } from '../api';
import { formatDuration, formatNumber, formatPercent, formatTime } from '../lib/format';

// Reine Funktionen der Statistik (B-065): Kennzahlen, Sortierung, Erklärzeile, Zeitangaben. Perzentile und
// Ausreißer rechnet Go; hier entstehen nur Darstellungswerte.

export const SCOPES = ['session', 'allTime'] as const;
export type ScopeKey = (typeof SCOPES)[number];

export interface Kpi {
  label: string;
  value: string;
  bad?: boolean;
}

const HOUR = 3_600_000;

/** Fehlerquote in Prozent; ohne Aufrufe oder ohne Fehlerzahl (Zeitraum, NaN) `–`. */
export function rateText(errors: number, calls: number): string {
  return calls > 0 && Number.isFinite(errors) ? formatPercent((100 * errors) / calls) : '–';
}

/** Dauer; ohne Aufrufe oder ohne Wert (Zeitraum, NaN) `–`. */
export const dur = (ms: number, calls: number) => (calls > 0 && Number.isFinite(ms) ? formatDuration(ms) : '–');

/** Die zehn Kennzahl-Kacheln eines Bereichs. */
export function kpis(s: UsageScope, now: number): Kpi[] {
  const hours = Math.max((now - Date.parse(s.since)) / HOUR, 1 / 60);
  const perHour = s.calls > 0 && Number.isFinite(hours) ? formatNumber(s.calls / hours, 1) : '–';
  return [
    { label: 'AUFRUFE', value: formatNumber(s.calls) },
    { label: 'FEHLERQUOTE', value: rateText(s.errors, s.calls), bad: s.errors > 0 },
    { label: 'AUFRUFE/H', value: perHour },
    { label: 'AKTIVE TOOLS', value: formatNumber(s.tools.filter((t) => t.calls > 0).length) },
    { label: 'Ø DAUER', value: dur(s.avgMs, s.calls) },
    { label: 'P50', value: dur(s.p50Ms, s.calls) },
    { label: 'P95', value: dur(s.p95Ms, s.calls) },
    { label: 'MAX', value: dur(s.maxMs, s.calls) },
    { label: 'Σ LAUFZEIT', value: dur(s.sumMs, s.calls) },
    { label: 'AUSREISSER', value: formatNumber(s.outliers), bad: s.outliers > 0 },
  ];
}

/** Zeitpunkt: heute nur Uhrzeit, sonst Datum und Uhrzeit. */
export function whenText(iso: string, now: number): string {
  const t = new Date(iso);
  if (Number.isNaN(t.getTime())) return '–';
  if (t.toDateString() === new Date(now).toDateString()) return formatTime(t);
  return `${t.toLocaleDateString('de-DE', { day: '2-digit', month: '2-digit', year: 'numeric' })} ${formatTime(t)}`;
}

/** Erklärzeile unter den Kacheln; die Werte kommen aus Go (UsageRules), nicht doppelt gepflegt. */
export function ruleLine(r: UsageRules, since: string, now: number): string {
  const factor = formatNumber(r.outlierFactor, Number.isInteger(r.outlierFactor) ? 0 : 1);
  return `seit ${whenText(since, now)} · Perzentile aus Histogramm, ±${r.percentileErrorPct} % · Ausreißer: über ` +
    `${factor}× p95 der Vergleichsgruppe (gleiches Tool und gleiche Argumente, sonst das Tool) und mindestens ` +
    `${formatDuration(r.outlierFloorMs)}; Vergleichsgruppe ab ${r.baselineCalls} Aufrufen`;
}

export const SORT_KEYS = ['name', 'calls', 'share', 'errors', 'rate', 'avgMs', 'p50Ms', 'p95Ms', 'maxMs', 'sumMs', 'outliers'] as const;
export type SortKey = (typeof SORT_KEYS)[number];

export interface Sort {
  key: SortKey;
  desc: boolean;
}

export const DEFAULT_SORT: Sort = { key: 'calls', desc: true };

/** Klick auf einen Spaltenkopf: gleiche Spalte dreht, neue Spalte beginnt absteigend (Name aufsteigend). */
export function nextSort(cur: Sort, key: SortKey): Sort {
  if (cur.key === key) return { key, desc: !cur.desc };
  return { key, desc: key !== 'name' };
}

function value(t: ToolUsage, key: SortKey): number | string {
  if (key === 'name') return t.name;
  if (key === 'share') return t.calls;
  const v = key === 'rate' ? (t.calls > 0 ? t.errors / t.calls : 0) : t[key];
  return Number.isNaN(v) ? -Infinity : v; // unbekannte Werte (Zeitraum) unten
}

/** Tool-Tabelle sortiert; bei Gleichstand nach Name. */
export function sortTools(tools: ToolUsage[], s: Sort): ToolUsage[] {
  return [...tools].sort((a, b) => {
    const va = value(a, s.key);
    const vb = value(b, s.key);
    const cmp = typeof va === 'string' ? va.localeCompare(vb as string) : va - (vb as number);
    return (s.desc ? -cmp : cmp) || a.name.localeCompare(b.name);
  });
}

/** „x× p95“ eines langsamen Aufrufs; ohne Vergleichswert leer. */
export function timesP95(c: SlowCall): string {
  return c.baselineMs > 0 ? `${formatNumber(c.durationMs / c.baselineMs, 1)}× p95` : '';
}
