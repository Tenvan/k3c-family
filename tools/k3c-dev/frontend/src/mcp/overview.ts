import type { ToolStats } from '../api';
import { formatDuration, formatNumber, formatUptime } from '../lib/format';

// Reine Funktionen der Übersicht (B-065, Band 1 und 2 links).

/** Kacheln nach Aufrufen, dann Name. */
export function sortTools(tools: ToolStats[]): ToolStats[] {
  return [...tools].sort((a, b) => b.calls - a.calls || a.name.localeCompare(b.name));
}

/** Anteil in Prozent; ohne Aufrufe 0. */
export function share(part: number, total: number): number {
  return total > 0 ? (100 * part) / total : 0;
}

/** Ø Dauer gewichtet über alle Tools; ohne Aufrufe null. */
export function weightedAvg(tools: ToolStats[]): number | null {
  const calls = tools.reduce((n, t) => n + t.calls, 0);
  return calls > 0 ? tools.reduce((sum, t) => sum + t.avgMs * t.calls, 0) / calls : null;
}

/** Laufzeit seit dem Start; ein kaputter Zeitstempel ergibt `–`. */
export function uptimeText(startedAt: string, now: number): string {
  const t = Date.parse(startedAt);
  return Number.isNaN(t) ? '–' : formatUptime(now - t);
}

/** Dauer oder `–`. */
export function durationText(ms: number | null): string {
  return ms === null ? '–' : formatDuration(ms);
}

/** Zeile einer Kachel: „N Aufrufe · N Fehler · Ø x“. */
export function toolLine(t: ToolStats): string {
  return `${formatNumber(t.calls)} Aufrufe · ${formatNumber(t.errors)} Fehler · Ø ${durationText(t.calls > 0 ? t.avgMs : null)}`;
}

/** Tooltip und aria-label einer Kachel: Beschreibung und letzter Aufruf. */
export function toolHint(t: ToolStats): string {
  return `${t.name}: ${t.description || 'ohne Beschreibung'} · ${t.lastCall ? `zuletzt ${t.lastCall}` : 'noch nicht aufgerufen'}`;
}
