import { BUILDINGS } from '../model/data';
import type { ResourceNode, Site } from '../model/types';

/**
 * Plätze der Plantage (GEB5): Die Sim legt je gebauter Farm `slots` Plätze im Streifen ±`stripUnits` an und lässt dort
 * normale Bäume nachwachsen (`engine/sim/plantation.go` › `plotX`). Der Snapshot kennt keine leeren Plätze; der Client
 * rechnet sie mit derselben Formel nach. Rein, ohne Phaser.
 */

export interface PlantationData {
  slots: number;
  stripUnits: number;
}

export interface Plot {
  /** stabile Kennung je Farm und Platz */
  id: number;
  x: number;
  /** Baum auf dem Platz, `null` = leer (Setzling) */
  tree: ResourceNode | null;
}

/** Abstand in Units, bis zu dem ein Baum als auf dem Platz stehend gilt (Waldbäume liegen nie so genau) */
export const PLOT_TOLERANCE = 0.01;

type FarmSite = Pick<Site, 'id' | 'kind' | 'x' | 'state'>;

const FARM = (BUILDINGS.farm as { plantation?: PlantationData }).plantation ?? { slots: 0, stripUnits: 0 };

/** Position von Platz `i` im Streifen um die Farm bei `farmX` (gleiche Formel wie `plotX` in Go) */
export function plotX(farmX: number, i: number, p: PlantationData = FARM): number {
  if (p.slots < 2) return farmX;
  return farmX - p.stripUnits + (i * 2 * p.stripUnits) / (p.slots - 1);
}

/** Alle Plätze gebauter Farmen mit ihrem Baum (falls einer dort steht); Farm im Bau oder zerstört: keine Plätze */
export function plantationPlots(sites: readonly FarmSite[], nodes: readonly ResourceNode[], p: PlantationData = FARM): Plot[] {
  const trees = nodes.filter((n) => n.kind === 'tree');
  return sites
    .filter((s) => s.kind === 'farm' && s.state === 'built')
    .flatMap((s) =>
      Array.from({ length: p.slots }, (_, i) => {
        const x = plotX(s.x, i, p);
        return { id: s.id * 100 + i, x, tree: trees.find((n) => Math.abs(n.x - x) < PLOT_TOLERANCE) ?? null };
      }),
    );
}

/** Ist `n` ein Plantage-Baum (steht auf einem Platz)? Dann zeigt der Renderer die Pappel statt des Waldbaums */
export function isPlantationTree(n: ResourceNode, sites: readonly FarmSite[], p: PlantationData = FARM): boolean {
  return n.kind === 'tree' && plantationPlots(sites, [n], p).some((plot) => plot.tree === n);
}
