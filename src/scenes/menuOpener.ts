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

/**
 * Slots, die „Spieler abmelden“ treffen kann: der des Öffners; öffnet Tastatur 1 und ist Tastatur 2 auch belegt, beide
 * (Tastatur 2 hat keine Menü-Taste, der Spieler wählt). Leer = ausgegraut (Öffner ohne Slot oder ohne Platz am Server).
 */
export function removeTargets<I>(opener: I | null, bound: readonly (I | null)[], keyboards: readonly [I, I], seated: readonly number[]): number[] {
  const slotOf = (input: I | null) => (input === null ? -1 : bound.indexOf(input));
  const slot = slotOf(opener);
  if (slot < 0 || !seated.includes(slot)) return [];
  if (opener === keyboards[0]) {
    const second = slotOf(keyboards[1]);
    if (second >= 0 && seated.includes(second)) return [slot, second];
  }
  return [slot];
}
