import type { ToolStats } from '../api';
import { formatDuration, formatNumber, formatUptime } from '../lib/format';

// Reine Funktionen der Übersicht (B-065, Kopfband).

/** Anteil in Prozent; ohne Aufrufe 0. */
export function share(part: number, total: number): number {
  return total > 0 ? (100 * part) / total : 0;
}

/** Tools nach Aufrufen absteigend, bei Gleichstand nach Name (B-350). */
export function sortByCalls(tools: ToolStats[]): ToolStats[] {
  return [...tools].sort((a, b) => b.calls - a.calls || a.name.localeCompare(b.name));
}

/** Kleine Statistik einer Tool-Kachel: „12× · Ø 34 ms · 1 Fehler“, ohne Aufruf „noch kein Aufruf“. */
export function toolLine(t: ToolStats): string {
  if (t.calls === 0) return 'noch kein Aufruf';
  const parts = [`${formatNumber(t.calls)}×`, `Ø ${formatDuration(t.avgMs)}`];
  if (t.errors > 0) parts.push(`${formatNumber(t.errors)} Fehler`);
  return parts.join(' · ');
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
