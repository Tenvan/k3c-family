import { describe, expect, it } from 'vitest';
import { computeLayout } from './layout';
import { cellStages } from './cellStages';

const seat = (slot: number, depth: number) => ({ slot, depth });
const all = new Set([1, 2, 3, 4]);

describe('cellStages', () => {
  it('ein Spieler: eine Zelle mit seiner Stufe', () => {
    expect(cellStages(computeLayout(1, false), [seat(0, 2)], null, all)).toEqual([{ depth: 2, ready: true }]);
  });

  it('ein Spieler mit Partner: oben Partner-Stufe, unten eigene', () => {
    const r = cellStages(computeLayout(1, true), [seat(0, 1)], 3, all);
    expect(r.map((c) => c.depth)).toEqual([3, 1]);
  });

  it.each([2, 3, 4])('%i Spieler: je Zelle die Stufe des eigenen Platzes', (n) => {
    const seats = Array.from({ length: n }, (_, i) => seat(i, i + 1));
    const r = cellStages(computeLayout(n, false), seats, null, all);
    expect(r.map((c) => c.depth)).toEqual(seats.map((s) => s.depth));
  });

  it('Spieler in verschiedenen Stufen ergeben verschiedene Tiefen', () => {
    const [a, b] = cellStages(computeLayout(2, false), [seat(0, 1), seat(1, 2)], null, all);
    expect(a!.depth).not.toBe(b!.depth);
  });

  it('ungeladene Stufe: ready false, geladene bleibt ready', () => {
    const r = cellStages(computeLayout(2, false), [seat(0, 1), seat(1, 2)], null, new Set([1]));
    expect(r).toEqual([
      { depth: 1, ready: true },
      { depth: 2, ready: false },
    ]);
  });

  it('unsortierte Plätze ergeben dieselbe Zuordnung wie sortierte', () => {
    const sorted = [seat(0, 1), seat(1, 2), seat(2, 3)];
    const shuffled = [sorted[2]!, sorted[0]!, sorted[1]!];
    const cells = computeLayout(3, false);
    expect(cellStages(cells, shuffled, null, all)).toEqual(cellStages(cells, sorted, null, all));
  });

  it('0 Plätze und Partner ohne Stufe: kein Absturz, nichts bereit', () => {
    expect(cellStages(computeLayout(1, false), [], null, all)).toEqual([{ depth: null, ready: false }]);
    expect(cellStages(computeLayout(1, true), [seat(0, 1)], null, all)[0]).toEqual({ depth: null, ready: false });
  });
});
