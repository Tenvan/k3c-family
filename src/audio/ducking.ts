/**
 * Ducking (SO4.3, B-168/AC-03, Beschluss Q16): Eine Warnung (`dusk`, `wave`, `castleFallen`) senkt den Bus „Musik“ kurz ab und
 * stellt ihn danach wieder her. Rein über Zeitfenster; der Audio-Kern plant die Rampen auf einem eigenen Gain vor dem Bus,
 * der sich mit dem Crossfade der Stücke multipliziert (kein Sprung). Werte nur hier.
 */

export const DUCK = {
  /** Pegel der Musik während der Warnung (Faktor) */
  level: 0.35,
  /** Absenken in Sekunden */
  attack: 0.15,
  /** Gesenkt halten nach der letzten Warnung in Sekunden (Länge des Warn-Sounds plus Rand) */
  hold: 1.2,
  /** Zurück auf vollen Pegel in Sekunden */
  release: 0.8,
} as const;

export type DuckConfig = typeof DUCK;

/** Fenster einer Absenkung: Rampe ab `start` hinab, gesenkt bis `end`, danach Rampe hinauf. */
export interface DuckWindow {
  start: number;
  end: number;
}

/** Rein: Pegelfaktor der Musik zum Zeitpunkt `t` (Sekunden); ohne Fenster 1. */
export function duckGain(t: number, w: DuckWindow | undefined, c: DuckConfig = DUCK): number {
  if (!w || t <= w.start) return 1;
  if (t < w.start + c.attack) return 1 - ((1 - c.level) * (t - w.start)) / c.attack;
  if (t <= w.end) return c.level;
  if (t < w.end + c.release) return c.level + ((1 - c.level) * (t - w.end)) / c.release;
  return 1;
}

/**
 * Rein: neue Warnung zum Zeitpunkt `t`. Läuft schon eine Absenkung, wird sie verlängert statt gestapelt (der Pegel fällt nie
 * unter `level`); in der Rampe hinauf beginnt die neue Rampe hinab beim aktuellen Pegel, ohne Sprung.
 */
export function extendDuck(w: DuckWindow | undefined, t: number, c: DuckConfig = DUCK): DuckWindow {
  const g = duckGain(t, w, c);
  const end = Math.max(w && t <= w.end ? w.end : 0, t + c.hold);
  if (w && t > w.start && t <= w.end) return { start: w.start, end };
  // ab Pegel g so tun, als hätte die Rampe hinab früher begonnen: duckGain(t) = g
  const start = t - (c.attack * (1 - g)) / (1 - c.level);
  return { start, end };
}
