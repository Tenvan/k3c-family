import type { ConsoleLine, Source } from '../api';
import { badgeFor } from '../services/tables';
import type { Tone } from '../ui/parts';
import { stripAnsi } from './ansi';

// Reine Funktionen der Logs-Seite (B-064): Zeilen einsortieren, Randmarken, Quellen und ihre Punkte.

/** Höchstens so viele Zeilen zeigt die Konsole (wie der Puffer in Go, console.DefaultCapacity). */
export const MAX_LINES = 2000;

export interface Merged {
  lines: ConsoleLine[];
  /** Zwischen der bisher letzten und der ersten neuen Zeile fehlen Nummern: Go hat Ereignisse verworfen. */
  gap: boolean;
}

/**
 * Sortiert Zeilen über ihre Nummer ein (auch solche, die während des Ladens kamen), verwirft doppelte und behält
 * die neuesten max.
 */
export function mergeLines(current: ConsoleLine[], incoming: ConsoleLine[], max = MAX_LINES): Merged {
  const last = current.length > 0 ? current[current.length - 1].seq : 0;
  const fresh = incoming.filter((l) => l.seq > last).sort((a, b) => a.seq - b.seq);
  const gap = current.length > 0 && fresh.length > 0 && fresh[0].seq > last + 1;
  let lines: ConsoleLine[];
  if (fresh.length === incoming.length) {
    lines = current.concat(fresh); // Normalfall: nur neue Zeilen hinten dran
  } else {
    const bySeq = new Map<number, ConsoleLine>();
    for (const l of current.concat(incoming)) bySeq.set(l.seq, l);
    lines = [...bySeq.values()].sort((a, b) => a.seq - b.seq);
  }
  return { lines: lines.length > max ? lines.slice(lines.length - max) : lines, gap };
}

/** Zeilen nach `Leeren`: nur die mit größerer Nummer als die letzte beim Leeren. */
export function visibleLines(lines: ConsoleLine[], clearedAt: number): ConsoleLine[] {
  return clearedAt > 0 ? lines.filter((l) => l.seq > clearedAt) : lines;
}

/** Randmarke einer Zeile: ERROR vor WARN, sonst keine. */
export function markOf(text: string): 'error' | 'warn' | null {
  const plain = stripAnsi(text);
  if (/\bERROR\b/.test(plain)) return 'error';
  if (/\bWARN(ING)?\b/.test(plain)) return 'warn';
  return null;
}

const RUN_TONES: Record<string, Tone> = { running: 'info', ok: 'ok', failed: 'error', timeout: 'error' };
const LOG_TONES: Record<string, Tone> = { entries: 'ok', empty: 'neutral' };

/** Ton des Zustands-Punkts; ein unbekannter Zustand oder eine unbekannte Art ist neutral. */
export function dotTone(src: Source): Tone {
  if (src.kind === 'service') return badgeFor(src.state).tone;
  if (src.kind === 'run') return RUN_TONES[src.state] ?? 'neutral';
  if (src.kind === 'log') return LOG_TONES[src.state] ?? 'neutral';
  return 'neutral';
}

/** Übernimmt eine gemeldete Quelle: ersetzt sie oder hängt sie an (ein Lauf erscheint beim ersten Mal). */
export function upsertSource(list: Source[], src: Source): Source[] {
  const i = list.findIndex((s) => s.name === src.name);
  if (i < 0) return [...list, src];
  const copy = list.slice();
  copy[i] = src;
  return copy;
}
