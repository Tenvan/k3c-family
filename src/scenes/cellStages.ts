import type { Cell } from './layout';

/** Welche Stufe eine Zelle zeigt und ob deren Welt schon geladen ist (sonst Platzhalter) */
export interface CellStage {
  depth: number | null;
  ready: boolean;
}

/**
 * Ordnet jeder Zelle die Stufe ihres Spielers zu (B-106): Spieler-Zelle `seat` = Position in der nach `slot` sortierten
 * Platzliste (wie die Kameras), Partner-Zelle = `partnerDepth`. Reine Auswahl aus Server-Daten, keine Simulation.
 */
export function cellStages(
  cells: readonly Pick<Cell, 'kind' | 'seat'>[],
  seats: readonly { slot: number; depth: number }[],
  partnerDepth: number | null,
  loaded: ReadonlySet<number>,
): CellStage[] {
  const sorted = [...seats].sort((a, b) => a.slot - b.slot);
  return cells.map((cell) => {
    const depth = cell.kind === 'partner' ? partnerDepth : (sorted[cell.seat]?.depth ?? null);
    return { depth, ready: depth !== null && loaded.has(depth) };
  });
}
