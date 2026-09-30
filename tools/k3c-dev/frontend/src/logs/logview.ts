import type { ErrorsView, LogEntry, LogView } from '../api';
import { formatBytes, formatNumber, formatTime } from '../lib/format';
import type { Tone } from '../ui/parts';

// Reine Funktionen der Reiter Log und Fehler (verdichtet) (B-064).

export const TABS = ['konsole', 'log', 'fehler'] as const;
export type Tab = (typeof TABS)[number];

export const LIMITS = [100, 200, 500] as const;
export const LEVEL_FILTERS = ['', 'DEBUG', 'INFO', 'WARN', 'ERROR'] as const; // '' = Alle

/** `Log` und `Fehler` gibt es nur für Log-Dateien; bei Läufen und Diensten nur die Konsole. */
export function tabEnabled(tab: Tab, kind: string): boolean {
  return tab === 'konsole' || kind === 'log';
}

/** Gemerkter Reiter, solange er für die Quelle geht; sonst `Konsole`. */
export function pickTab(wanted: Tab, kind: string): Tab {
  return tabEnabled(wanted, kind) ? wanted : 'konsole';
}

const LEVEL_TONES: Record<string, Tone> = { DEBUG: 'neutral', INFO: 'info', WARN: 'warn', ERROR: 'error' };

export function levelTone(level: string): Tone {
  return LEVEL_TONES[level] ?? 'neutral';
}

/** Go liefert neueste zuerst, die Tabelle zeigt älteste oben. */
export function oldestFirst(entries: LogEntry[]): LogEntry[] {
  return [...entries].reverse();
}

/** Fußzeile: „N Einträge · X KB gelesen · läuft mit“. */
export function footer(v: LogView | ErrorsView, count: number, following: boolean): string {
  const parts = [`${formatNumber(count)} Einträge`, `${formatBytes(v.bytesRead)} gelesen`];
  if (following) parts.push('läuft mit');
  return parts.join(' · ');
}

/** Warnhinweis, wenn Go das Lese-Budget ausgeschöpft hat; sonst leer. */
export function budgetNote(v: LogView | ErrorsView): string {
  return v.budgetHit ? 'Lese-Budget erreicht: ältere Treffer möglich.' : '';
}

/** Zeitfenster einer Gruppe: „08:03–13:51“, ein Zeitpunkt allein „08:03“, über Tage mit Datum. */
export function timeWindow(first: string, last: string): string {
  const a = new Date(first);
  const b = new Date(last);
  if (a.getTime() === b.getTime()) return formatTime(a);
  if (a.toDateString() === b.toDateString()) return `${formatTime(a)}–${formatTime(b)}`;
  const day = (d: Date) => d.toLocaleDateString('de-DE', { day: '2-digit', month: '2-digit' });
  return `${day(a)} ${formatTime(a)}–${day(b)} ${formatTime(b)}`;
}

/** Beispiel einer Gruppe in einer Zeile, gekürzt. */
export function oneLine(text: string, max = 160): string {
  const line = text.split('\n', 1)[0].trim();
  return line.length > max ? `${line.slice(0, max - 1)}…` : line;
}

/** Daten einer aufklappbaren Zeile, eingerückt. */
export function prettyData(data: Record<string, unknown> | undefined): string {
  return data && Object.keys(data).length > 0 ? JSON.stringify(data, null, 2) : '';
}

/**
 * Schlüssel der Zeilen: stabil über Nachladen hinweg (Aufklappen bleibt erhalten) und eindeutig, auch wenn zwei
 * Einträge in Zeit, Level, ns und msg gleich sind (grobe Zeitstempel fremder Logs).
 */
export function entryKeys(entries: LogEntry[]): string[] {
  const seen = new Map<string, number>();
  return entries.map((e) => {
    const base = `${e.time}|${e.level}|${e.ns}|${e.msg}`;
    const n = seen.get(base) ?? 0;
    seen.set(base, n + 1);
    return n === 0 ? base : `${base}#${n}`;
  });
}
