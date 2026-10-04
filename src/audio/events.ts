/**
 * Ereignis → Ton (B-011, SO1.3): reine Zuordnung und Positions-Dämpfung. Audio rechnet keine Spielregel,
 * es reagiert nur auf Ereignisse und Kamera-Ausschnitte. Welche Ereignisse klingen, klärt SO2 (Q15, B-167).
 */
import type { GameEvent } from '../model/types';

/** Sichtbarer Ausschnitt eines lokalen Spielers in Units (Mitte und Breite der Kamera). */
export interface Listener {
  center: number;
  span: number;
}

export interface SoundCue {
  sprite: string;
  /** Warnung: überall gleich laut, keine Dämpfung */
  global: boolean;
}

/** Faktor direkt am Rand des sichtbaren Bereichs; darüber hinaus fällt er bis auf 0 (Abstand = eine Bildschirmbreite). */
export const EDGE_GAIN = 0.5;

export function cueFor(e: GameEvent): SoundCue | null {
  switch (e.type) {
    case 'built': return { sprite: 'coin', global: false }; // Demo-Ton (Atlas SO1.2)
    case 'wave':
    case 'castleFallen': return { sprite: 'warn', global: true };
    default: return null;
  }
}

/** Lautstärke 0–1 einer Quelle bei `x` (Units): im Bereich eines Listeners 1, sonst leiser; der lauteste Listener gilt. */
export function attenuation(x: number | undefined, listeners: readonly Listener[], global: boolean): number {
  if (global || listeners.length === 0 || x === undefined) return 1;
  const best = Math.min(...listeners.map((l) => Math.max(0, Math.abs(x - l.center) - l.span / 2) / Math.max(l.span, 1)));
  return best === 0 ? 1 : Math.max(0, EDGE_GAIN * (1 - best));
}
