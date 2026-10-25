import type { GameEvent } from '../../model/types';

/** Wie lange die Leiste nach einem Speichern „gesichert“ zeigt, in Sekunden (B-186) */
export const SAVED_SECONDS = 2;

/**
 * Restzeit der Anzeige „gesichert“: Ein Ereignis `saved` des Servers setzt sie auf `SAVED_SECONDS`, sonst läuft sie um
 * `dtSec` ab. Der Client entscheidet nichts selbst, er zeichnet nur, was der Server meldet.
 */
export function stepSaved(left: number, events: readonly GameEvent[], dtSec: number): number {
  if (events.some((e) => e.type === 'saved')) return SAVED_SECONDS;
  return Math.max(0, left - dtSec);
}
