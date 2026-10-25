import { daylight } from '@game/scenes/render/viewRules';
import type { CycleInfo } from '@game/model/types';

// Tageszeit der Live-Vorschau (B-460/AC-05, WZ7.4): vier Einträge, vier Sim-Phasen in Go. Das Morgengrauen ist seit B-470
// (W12) eine echte Phase `dawn`; Helligkeit nach `daylight()` in `src/scenes/render/viewRules.ts`.

export type SimPhase = 'day' | 'dusk' | 'night' | 'dawn';
export type TimeOfDayId = SimPhase;

export const TIMES_OF_DAY: readonly { id: TimeOfDayId; label: string }[] = [
  { id: 'day', label: 'Tag' },
  { id: 'dusk', label: 'Dämmerung' },
  { id: 'night', label: 'Nacht' },
  { id: 'dawn', label: 'Morgengrauen' },
];

/** Größter wählbarer Fortschritt: p = 1 wäre der Beginn der nächsten Phase (Go lehnt es ab). */
export const MAX_PROGRESS = 0.99;

/** Sim-Phase und Fortschritt in [0, 1) zu einem Eintrag und seinem Fortschritt p (p wird auf [0, 0,99] begrenzt). */
export function simTime(id: TimeOfDayId, p: number): { phase: SimPhase; progress: number } {
  const clamped = Math.min(MAX_PROGRESS, Math.max(0, Number.isFinite(p) ? p : 0));
  return { phase: id, progress: clamped };
}

/** Helligkeitsfaktor der Kamera: wie in `GameScene` (Nacht-Untergrenze 0,45, darüber nach `daylight()`). B-472 macht die
 *  Nacht dunkler; wer zuletzt landet, zieht diese Konstante nach. */
export const NIGHT_BRIGHTNESS = 0.45;

export function brightnessOf(cycle: Pick<CycleInfo, 'phase' | 'progress'>): number {
  return NIGHT_BRIGHTNESS + (1 - NIGHT_BRIGHTNESS) * daylight(cycle as CycleInfo);
}
