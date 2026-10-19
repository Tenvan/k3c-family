/**
 * Gesten für Debug-Anzeige und Cheat-Dialog (B-231), reine Logik ohne DOM:
 * Controller: LB + RB 3 s halten = Cheat-Dialog, nur RB 3 s halten = Diagnose. Touch: Doppeltap mit zwei Fingern =
 * Cheat-Dialog, mit einem Finger = Diagnose. B, View + Menu bleiben frei.
 */

export type DebugGesture = 'cheats' | 'diag';

/** Haltezeit der Schultertasten bis zur Auslösung */
export const HOLD_MS = 3000;
/** Höchstdauer eines Taps und größter Abstand zwischen den beiden Taps eines Doppeltaps */
export const TAP_MS = 300;
export const DOUBLE_TAP_MS = 400;

export interface HoldState {
  mode: DebugGesture | null;
  since: number;
  fired: boolean;
}

export const HOLD_IDLE: HoldState = { mode: null, since: 0, fired: false };

/** Nächster Halte-Zustand; `fire` genau einmal je Halten, sobald `holdMs` erreicht ist. Wechsel der Tasten beginnt neu. */
export function holdStep(s: HoldState, lb: boolean, rb: boolean, now: number, holdMs = HOLD_MS): { state: HoldState; fire: DebugGesture | null } {
  const mode: DebugGesture | null = lb && rb ? 'cheats' : rb ? 'diag' : null;
  const state = mode === s.mode ? s : { mode, since: now, fired: false };
  if (mode === null || state.fired || now - state.since < holdMs) return { state, fire: null };
  return { state: { ...state, fired: true }, fire: mode };
}

/** Ein abgeschlossener Touch-Vorgang: höchste Fingerzahl, Beginn und Ende (ms). */
export interface Tap {
  fingers: number;
  start: number;
  end: number;
}

/** Doppeltap: zwei kurze Taps mit gleicher Fingerzahl (1 oder 2) kurz nacheinander. `prev` ist der vorige kurze Tap. */
export function tapStep(prev: Tap | null, tap: Tap): { prev: Tap | null; fire: DebugGesture | null } {
  if (tap.end - tap.start > TAP_MS || tap.fingers > 2) return { prev: null, fire: null };
  if (prev && prev.fingers === tap.fingers && tap.start - prev.end <= DOUBLE_TAP_MS) {
    return { prev: null, fire: tap.fingers === 2 ? 'cheats' : 'diag' };
  }
  return { prev: tap, fire: null };
}

/** Lauscht auf Touch am Fenster (passiv, stört das Touch-Overlay nicht) und meldet Doppeltaps. Rückgabe: abmelden. */
export function listenTaps(onGesture: (g: DebugGesture) => void): () => void {
  let fingers = 0;
  let start = 0;
  let prev: Tap | null = null;
  const down = (e: TouchEvent): void => {
    if (fingers === 0) start = performance.now();
    fingers = Math.max(fingers, e.touches.length);
  };
  const up = (e: TouchEvent): void => {
    if (e.touches.length > 0 || fingers === 0) return;
    const r = tapStep(prev, { fingers, start, end: performance.now() });
    prev = r.prev;
    fingers = 0;
    if (r.fire) onGesture(r.fire);
  };
  const opts = { capture: true, passive: true };
  window.addEventListener('touchstart', down, opts);
  window.addEventListener('touchend', up, opts);
  window.addEventListener('touchcancel', up, opts);
  return () => {
    window.removeEventListener('touchstart', down, opts);
    window.removeEventListener('touchend', up, opts);
    window.removeEventListener('touchcancel', up, opts);
  };
}
