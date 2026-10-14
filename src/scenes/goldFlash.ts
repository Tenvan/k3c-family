/** So lange hebt das HUD die Goldzahl nach einer Änderung hervor (B-398, 🧑 2026-10-14: kurz hervorheben). */
export const GOLD_FLASH_MS = 800;

export interface GoldFlash {
  gold: number;
  /** Zeitpunkt der letzten Änderung (−∞ solange keine war) */
  changedAt: number;
}

/** Hervorhebung der Goldzahl eines Spielers: der erste gesehene Wert zählt nicht als Änderung. */
export function stepGoldFlash(prev: GoldFlash | undefined, gold: number, now: number): { state: GoldFlash; flash: boolean } {
  const changedAt = !prev ? Number.NEGATIVE_INFINITY : prev.gold !== gold ? now : prev.changedAt;
  return { state: { gold, changedAt }, flash: now - changedAt < GOLD_FLASH_MS };
}
