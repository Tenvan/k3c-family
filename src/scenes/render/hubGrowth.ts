import type { World } from '../../model/types';

/**
 * Wachstumsreihe der Burg (GEB11, docs/rules/materialien-gebaeude.md § 2.1): Looks vom Lagerfeuer bis zur Kristallburg.
 * Nur Darstellung, Hub-Stufen und Freischaltungen bleiben. Reine Logik ohne Phaser und ohne Texte; das Asset-Studio zeigt dieselbe
 * Reihe mit Namen und Bedingung (`tools/k3c-studio/frontend/src/graphics/scenesLogic.ts` › `hubWachstum`).
 */

/** Grafik-Id und Stufe unter `public/grafik/gebaeude/` (`<id>-<stufe>-<dichte>.png`) */
export interface HubGrowthLook {
  id: string;
  stufe: number;
}

/** Spielreihenfolge: Lagerfeuer, Zelt, Holzhütte (`lagerfeuer` 1–3, Hub-Stufe 1 vor vollem Gold), dann die Burgen der Hub-Stufen 1–5 */
export const HUB_GROWTH: readonly HubGrowthLook[] = [
  ...[1, 2, 3].map((stufe) => ({ id: 'lagerfeuer', stufe })),
  ...[1, 2, 3, 4, 5].map((stufe) => ({ id: 'burg', stufe })),
];

/** Bild vor dem Look, solange ein Bauer den Hub-Ausbau baut (`hubUpgrade.state` = `waitingWorker` ohne `danger`, GEB12 D8) */
export const HUB_SCAFFOLD = 'hub-geruest';

/** Look der Burg im Spiel: Grafik-Id und Stufe (`burg` = Burg der Hub-Stufe), `scaffold` = Baugerüst davor */
export interface HubLook {
  id: string;
  stufe: number;
  scaffold: boolean;
}

/**
 * Look aus Hub-Stufe und Ausbau-Stand (§ 2.1): Auf Hub-Stufe 1, solange Gold offen ist, Lagerfeuer (nichts bezahlt), Zelt (unter der
 * Hälfte) oder Holzhütte (ab der Hälfte); sonst die Burg der Hub-Stufe. Ohne `hubUpgrade` (letzte Stufe, älterer Server) die Burg.
 * Das Gerüst steht nur, wenn gebaut werden kann: Bei Gefahr (`danger`, Nacht oder Gegner) wartet der Bau, dann kein Gerüst.
 */
export function hubLookOf(world: Pick<World, 'hubLevel' | 'hubUpgrade' | 'danger'>): HubLook {
  const level = Math.max(1, Math.trunc(world.hubLevel ?? 1) || 1);
  const up = world.hubUpgrade;
  const scaffold = up?.state === 'waitingWorker' && !world.danger;
  if (level !== 1 || !up || up.state || up.gold <= 0) return { id: 'burg', stufe: level, scaffold };
  return { id: 'lagerfeuer', stufe: up.paid <= 0 ? 1 : up.paid * 2 < up.gold ? 2 : 3, scaffold };
}
