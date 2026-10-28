import { describe, expect, it } from 'vitest';
import type { ResourceNode, Site } from '../../model/types';
import { isPlantationTree, plantationPlots, plotX } from './plantation';

const P = { slots: 6, stripUnits: 3 };
const nursery = (state: Site['state'] = 'built', x = 100, id = 7): Pick<Site, 'id' | 'kind' | 'x' | 'state'> => ({ id, kind: 'nursery', x, state });
const tree = (id: number, x: number, kind = 'tree'): ResourceNode => ({ id, kind, x, paidGold: 0, marked: false, workerId: null, progress: 0 });

describe('Plantage-Plätze (GEB5 AC-02, AC-05)', () => {
  it('sechs Plätze gleichmäßig im Streifen ±3 um die Baumschule, gleiche Formel wie plotX in Go', () => {
    expect([0, 1, 2, 3, 4, 5].map((i) => plotX(100, i, P))).toEqual([97, 98.2, 99.4, 100.6, 101.8, 103]);
  });

  it('ohne Bäume sind alle sechs Plätze leer (Setzling)', () => {
    const plots = plantationPlots([nursery()], [], P);
    expect(plots).toHaveLength(6);
    expect(plots.every((p) => p.tree === null)).toBe(true);
  });

  it('ein Baum auf einem Platz füllt genau diesen Platz', () => {
    const t = tree(1, plotX(100, 2, P));
    const plots = plantationPlots([nursery()], [t], P);
    expect(plots.map((p) => p.tree)).toEqual([null, null, t, null, null, null]);
    expect(isPlantationTree(t, [nursery()], P)).toBe(true);
  });

  it('Baumschule im Bau oder zerstört: keine Plätze, kein Plantage-Baum', () => {
    const t = tree(1, plotX(100, 0, P));
    for (const state of ['unpaid', 'waitingMaterial', 'waitingWorker'] as const) {
      expect(plantationPlots([nursery(state)], [t], P)).toEqual([]);
      expect(isPlantationTree(t, [nursery(state)], P)).toBe(false);
    }
  });

  it('Waldbaum im Streifen, aber nicht auf einem Platz, bleibt ein Waldbaum', () => {
    const wald = tree(2, 100.2);
    expect(plantationPlots([nursery()], [wald], P).every((p) => p.tree === null)).toBe(true);
    expect(isPlantationTree(wald, [nursery()], P)).toBe(false);
  });

  it('nur Bäume zählen, nicht Fels oder Erz auf dem Platz', () => {
    const fels = tree(3, plotX(100, 1, P), 'rock');
    expect(plantationPlots([nursery()], [fels], P).every((p) => p.tree === null)).toBe(true);
    expect(isPlantationTree(fels, [nursery()], P)).toBe(false);
  });

  it('zwei Baumschulen haben je eigene Plätze mit verschiedenen Kennungen', () => {
    const plots = plantationPlots([nursery('built', 100, 7), nursery('built', -100, 8)], [], P);
    expect(plots).toHaveLength(12);
    expect(new Set(plots.map((p) => p.id)).size).toBe(12);
  });

  it('echte Daten: 6 Plätze', () => {
    expect(plantationPlots([nursery()], [])).toHaveLength(6);
  });
});
