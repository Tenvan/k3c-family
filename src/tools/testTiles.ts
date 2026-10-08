/**
 * Kacheln der Testseite (`testing.html`) für Werkzeug-Seiten und die Fokus-Weiterschaltung per Controller.
 * Szenarien stehen in `testScenarios.ts`; hier liegt, was auf eine andere Seite führt.
 */
import { type DevTile, tile } from './devTiles';

/** Abschnitt „Level“ (B-092): Der Level-Betrachter ist von der Testseite aus erreichbar, mit demselben Text wie auf der Entwicklerseite. */
export const LEVEL_TILES: readonly DevTile[] = [tile('level', '🗺️', 'leveltest.html')];

export type FocusKey = 'prev' | 'next';

/** Index der Kachel, die nach `key` den Fokus bekommt; an den Rändern bleibt der Fokus stehen. */
export function nextFocus(count: number, at: number, key: FocusKey): number {
  if (count <= 0) return 0;
  const from = Math.min(count - 1, Math.max(0, at));
  return key === 'next' ? Math.min(count - 1, from + 1) : Math.max(0, from - 1);
}
