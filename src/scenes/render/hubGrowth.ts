/**
 * Wachstumsreihe der Burg (GEB11, docs/rules/materialien-gebaeude.md § 2.1): Looks vom Lagerfeuer bis zur Kristallburg.
 * Nur Darstellung, Hub-Stufen und Freischaltungen bleiben. Reine Daten ohne Phaser; das Asset-Studio zeigt dieselbe Reihe
 * (`tools/k3c-studio/frontend/src/graphics/HubGrowthStrip.tsx`).
 */

export interface HubGrowthLook {
  /** Grafik-Id und Stufe unter `public/grafik/gebaeude/` (`<id>-<stufe>-<dichte>.png`) */
  id: string;
  stufe: number;
  name: string;
  /** Wann der Look im Spiel steht (Hub-Stufe und Ausbau auf die nächste) */
  wann: string;
}

export const HUB_GROWTH: readonly HubGrowthLook[] = [
  { id: 'lagerfeuer', stufe: 1, name: 'Lagerfeuer mit Grill', wann: 'Hub-Stufe 1, noch kein Gold bezahlt' },
  { id: 'lagerfeuer', stufe: 2, name: 'Zelt am Feuer', wann: 'Hub-Stufe 1, weniger als die Hälfte des Golds' },
  { id: 'lagerfeuer', stufe: 3, name: 'Holzhütte am Feuer', wann: 'Hub-Stufe 1, ab der Hälfte des Golds' },
  { id: 'burg', stufe: 1, name: 'Holzburg', wann: 'Hub-Stufe 1, Gold voll' },
  { id: 'burg', stufe: 2, name: 'Steinburg', wann: 'Hub-Stufe 2' },
  { id: 'burg', stufe: 3, name: 'Kupferburg', wann: 'Hub-Stufe 3' },
  { id: 'burg', stufe: 4, name: 'Eisenburg', wann: 'Hub-Stufe 4' },
  { id: 'burg', stufe: 5, name: 'Kristallburg', wann: 'Hub-Stufe 5' },
];

/** Bild vor dem Look, solange ein Bauer den Hub-Ausbau baut (`hubUpgrade.state` = `waitingWorker`) */
export const HUB_SCAFFOLD = 'hub-geruest';
