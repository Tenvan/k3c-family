import { describe, expect, it } from 'vitest';
import type { ResourceNode, Site } from '../model/types';
import { isPlantationTree, plantationPlots, plotX } from './plantation';

const P = { slots: 6, stripUnits: 3 };
const farm = (state: Site['state'] = 'built', x = 100, id = 7): Pick<Site, 'id' | 'kind' | 'x' | 'state'> => ({ id, kind: 'farm', x, state });
const tree = (id: number, x: number, kind = 'tree'): ResourceNode => ({ id, kind, x, paidGold: 0, marked: false, workerId: null, progress: 0 });

describe('Plantage-Plätze (GEB5 AC-02, AC-05)', () => {
  it('sechs Plätze gleichmäßig im Streifen ±3 um die Farm, gleiche Formel wie plotX in Go', () => {
    expect([0, 1, 2, 3, 4, 5].map((i) => plotX(100, i, P))).toEqual([97, 98.2, 99.4, 100.6, 101.8, 103]);
  });

  it('ohne Bäume sind alle sechs Plätze leer (Setzling)', () => {
    const plots = plantationPlots([farm()], [], P);
    expect(plots).toHaveLength(6);
    expect(plots.every((p) => p.tree === null)).toBe(true);
  });

  it('ein Baum auf einem Platz füllt genau diesen Platz', () => {
    const t = tree(1, plotX(100, 2, P));
    const plots = plantationPlots([farm()], [t], P);
    expect(plots.map((p) => p.tree)).toEqual([null, null, t, null, null, null]);
    expect(isPlantationTree(t, [farm()], P)).toBe(true);
  });

  it('Farm im Bau oder zerstört: keine Plätze, kein Plantage-Baum', () => {
    const t = tree(1, plotX(100, 0, P));
    for (const state of ['unpaid', 'waitingMaterial', 'waitingWorker'] as const) {
      expect(plantationPlots([farm(state)], [t], P)).toEqual([]);
      expect(isPlantationTree(t, [farm(state)], P)).toBe(false);
    }
  });

  it('Waldbaum im Streifen, aber nicht auf einem Platz, bleibt ein Waldbaum', () => {
    const wald = tree(2, 100.2);
    expect(plantationPlots([farm()], [wald], P).every((p) => p.tree === null)).toBe(true);
    expect(isPlantationTree(wald, [farm()], P)).toBe(false);
  });

  it('nur Bäume zählen, nicht Fels oder Erz auf dem Platz', () => {
    const fels = tree(3, plotX(100, 1, P), 'rock');
    expect(plantationPlots([farm()], [fels], P).every((p) => p.tree === null)).toBe(true);
    expect(isPlantationTree(fels, [farm()], P)).toBe(false);
  });

  it('zwei Farmen haben je eigene Plätze mit verschiedenen Kennungen', () => {
    const plots = plantationPlots([farm('built', 100, 7), farm('built', -100, 8)], [], P);
    expect(plots).toHaveLength(12);
    expect(new Set(plots.map((p) => p.id)).size).toBe(12);
  });

  it('echte Daten: 6 Plätze', () => {
    expect(plantationPlots([farm()], [])).toHaveLength(6);
  });
});
