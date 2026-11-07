// RD1.2: Catmull-Rom-Kurven wie im Prototyp (crPath, spline).
import { describe, expect, it } from 'vitest';
import { crPath, spline } from './kurve';

describe('Kurven', () => {
  it('crPath durch 3 Punkte: M plus 2 C-Segmente, Koordinaten über num()', () => {
    const d = crPath([[0, 0], [10, 0.333333], [20, 0]]);
    expect(d.startsWith('M0 0C')).toBe(true);
    expect(d.match(/C/g)).toHaveLength(2);
    expect(d).not.toMatch(/\d\.\d{3}/);
    expect(d.endsWith('20 0')).toBe(true);
  });

  it('crPath mit cont = true beginnt mit L statt M', () => {
    expect(crPath([[1, 2], [3, 4]], true).startsWith('L1 2C')).toBe(true);
  });

  it('spline: Endpunkte und Mitte der Stützpunkte', () => {
    const P: [number, number][] = [[-1, 0], [0, 1], [1, 0]];
    expect(spline(P, 0)).toEqual([-1, 0]);
    const m = spline(P, 0.5);
    expect(m[0]).toBeCloseTo(0, 9);
    expect(m[1]).toBeCloseTo(1, 9);
    const e = spline(P, 1);
    expect(e[0]).toBeCloseTo(1, 2);
  });
});
