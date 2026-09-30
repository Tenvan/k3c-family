import type { LevelCounts, ServiceStatus } from '../api';
import { formatBytes, formatPercent, formatUptime } from '../lib/format';
import type { Tone } from '../ui/parts';

// Reine Daten und Funktionen der Dienste-Seite (B-068): Zustand → Badge, Zustand → Knöpfe, Ereignisse einsortieren,
// Log-Balken, Metriken.

export interface Badge {
  label: string;
  tone: Tone;
}

const BADGES: Record<string, Badge> = {
  läuft: { label: 'Läuft', tone: 'ok' },
  startet: { label: 'Startet', tone: 'info' },
  übernommen: { label: 'Übernommen', tone: 'warn' },
  stoppt: { label: 'Stoppt', tone: 'neutral' },
  gestoppt: { label: 'Gestoppt', tone: 'neutral' },
  fehlgeschlagen: { label: 'Fehlgeschlagen', tone: 'error' },
};

/** Badge eines Zustands; ein unbekannter bleibt neutral mit seinem Namen. */
export function badgeFor(state: string): Badge {
  return BADGES[state] ?? { label: state || 'unbekannt', tone: 'neutral' };
}

export type Command = 'start' | 'stop' | 'restart';

const BUTTONS: Record<string, readonly Command[]> = {
  gestoppt: ['start'],
  fehlgeschlagen: ['start'],
  startet: ['stop'],
  läuft: ['stop', 'restart'],
  übernommen: ['stop'], // nur nach Bestätigung; Neustart lehnt Go für übernommene ab
  stoppt: [],
};

/** Freie Knöpfe eines Zustands; bei einem unbekannten keiner. */
export function buttonsFor(state: string): readonly Command[] {
  return BUTTONS[state] ?? [];
}

/**
 * Übernimmt ein Ereignis in die Liste: unbekannte Dienste (Konfiguration geändert) und veraltete Stände (kleinere
 * Seq, Go meldet ungeordnet) bleiben draußen. Gibt bei keiner Änderung dieselbe Liste zurück.
 */
export function applyStatus(list: ServiceStatus[], next: ServiceStatus): ServiceStatus[] {
  const i = list.findIndex((s) => s.name === next.name);
  if (i < 0 || next.seq <= list[i].seq) return list;
  const copy = list.slice();
  copy[i] = next;
  return copy;
}

/** Zeile unter dem Kopf: Stopp-Reihenfolge rückwärts zur Konfiguration. */
export function orderLine(names: string[]): string {
  return `„Alle starten“ startet parallel · „Alle stoppen“ rückwärts: ${[...names].reverse().join(' → ')}`;
}

export const LEVELS = ['DEBUG', 'INFO', 'WARN', 'ERROR'] as const;

export interface Share {
  level: string;
  count: number;
  percent: number;
}

/** Anteile je Level für den Balken und die Zahlen darunter: WARN und ERROR immer, die anderen nur mit Einträgen. */
export function levelShares(c: LevelCounts): Share[] {
  const total = LEVELS.reduce((sum, l) => sum + (c.counts[l] ?? 0), 0);
  return LEVELS.map((level) => {
    const count = c.counts[level] ?? 0;
    return { level, count, percent: total > 0 ? (100 * count) / total : 0 };
  }).filter((s) => s.count > 0 || s.level === 'WARN' || s.level === 'ERROR');
}

export interface Metrics {
  pid: string;
  cpu: string;
  memory: string;
  uptime: string;
}

const NONE: Metrics = { pid: '–', cpu: '–', memory: '–', uptime: '–' };

/** Metriken der Karte; `–`, solange nichts läuft. */
export function metricsOf(s: ServiceStatus, now: number): Metrics {
  if (s.pid <= 0 || (s.state !== 'läuft' && s.state !== 'übernommen' && s.state !== 'startet')) return NONE;
  const started = Date.parse(s.startedAt);
  return {
    pid: String(s.pid),
    cpu: formatPercent(s.cpu),
    memory: s.memory > 0 ? formatBytes(s.memory) : '–',
    uptime: Number.isNaN(started) || started <= 0 ? '–' : formatUptime(now - started),
  };
}
