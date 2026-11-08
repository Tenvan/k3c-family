/**
 * Wer hat das Spielmenü geöffnet (B-411), ohne Phaser: Öffner und die Slots, die „Spieler abmelden“ treffen darf.
 */

export interface OpenSignal<I> {
  input: I;
  /** Diese Eingabe hat im Frame das Menü ausgelöst (Esc, Touch-Schaltfläche, Menu kurz) */
  opens: boolean;
}

/** Erster auslösender Eingang in der Reihenfolge der Liste (zwei Pads im selben Frame: das erste gilt), sonst `null`. */
export function findOpener<I>(signals: readonly OpenSignal<I>[]): { input: I } | null {
  const hit = signals.find((s) => s.opens);
  return hit ? { input: hit.input } : null;
}

/** Slots, die „Spieler abmelden“ treffen kann: der des Öffners. Leer = ausgegraut (Öffner ohne Slot oder ohne Platz am Server). */
export function removeTargets<I>(opener: I | null, bound: readonly (I | null)[], seated: readonly number[]): number[] {
  const slot = opener === null ? -1 : bound.indexOf(opener);
  return slot >= 0 && seated.includes(slot) ? [slot] : [];
}
