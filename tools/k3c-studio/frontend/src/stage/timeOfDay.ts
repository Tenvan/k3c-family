
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

// Helligkeit der Kamera und Licht wie im Spiel (W13.2, B-472/AC-03): `brightnessOf` und `NIGHT_BRIGHTNESS` aus
// `src/scenes/render/nightLight.ts` (Untergrenze aus `data/light.json`), das Lagerfeuer-Licht zeichnet der `WorldRenderer` selbst.
export { NIGHT_BRIGHTNESS, brightnessOf } from '@game/scenes/render/nightLight';
