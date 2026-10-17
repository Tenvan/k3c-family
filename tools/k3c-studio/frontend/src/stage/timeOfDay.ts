import { daylight } from '@game/scenes/viewRules';
import type { CycleInfo } from '@game/model/types';

// Tageszeit der Live-Vorschau (B-460/AC-05, WZ7.4): vier Einträge in der Oberfläche, drei Sim-Phasen in Go. Das Morgengrauen
// ist in der Sim nur ein Ereignis beim Tagesbeginn und keine Phase; der Client zeichnet es als die letzten 10 % der Nacht
// (`daylight()` in `src/scenes/viewRules.ts`). Das Studio bildet es deshalb auf die Phase `night` mit Fortschritt
// 0,9 + 0,1·p ab. Eine echte Phase baut B-470; dann entfällt die Abbildung.

export type SimPhase = 'day' | 'dusk' | 'night';
export type TimeOfDayId = SimPhase | 'dawn';

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
  return id === 'dawn' ? { phase: 'night', progress: 0.9 + 0.1 * clamped } : { phase: id, progress: clamped };
}

/** Helligkeitsfaktor der Kamera: wie in `GameScene` (Nacht-Untergrenze 0,45, darüber nach `daylight()`). B-472 macht die
 *  Nacht dunkler; wer zuletzt landet, zieht diese Konstante nach. */
export const NIGHT_BRIGHTNESS = 0.45;

export function brightnessOf(cycle: Pick<CycleInfo, 'phase' | 'progress'>): number {
  return NIGHT_BRIGHTNESS + (1 - NIGHT_BRIGHTNESS) * daylight(cycle as CycleInfo);
}
