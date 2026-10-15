/** So lange steht der Steuerhinweis nach dem Beitritt, einem Gerätewechsel oder dem Einschalten von „Hinweise“ (B-399, 🧑 2026-10-14). */
export const CONTROLS_HINT_MS = 20_000;

export interface ControlsTrack {
  /** Zeitpunkt, seit dem der Hinweis zählt */
  since: number;
  seats: number;
  device: string;
  guide: boolean;
}

/**
 * Sichtbarkeit des Steuerhinweises. Die Zeit startet beim ersten Aufruf, bei mehr Plätzen (Beitritt), bei neuem
 * Gerät (nur mit einem Platz: bei zwei Spielern wechselt das zuletzt benutzte Gerät ständig) und wenn „Hinweise“
 * (`guideHints`) wieder eingeschaltet wird; weniger Plätze (Abmelden) starten sie nicht.
 */
export function stepControlsHint(
  prev: ControlsTrack | null,
  now: number,
  cur: { seats: number; device: string; guide: boolean },
): { track: ControlsTrack; visible: boolean } {
  const restart = !prev || cur.seats > prev.seats || (cur.seats <= 1 && cur.device !== prev.device) || (cur.guide && !prev.guide);
  const track: ControlsTrack = { since: prev && !restart ? prev.since : now, ...cur };
  return { track, visible: now - track.since < CONTROLS_HINT_MS };
}
