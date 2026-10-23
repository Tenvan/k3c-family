import { describe, expect, it } from 'vitest';
import { areaPath, deltas, H, linePath, nearestIndex, niceMax, peakOf, timeTicks, W } from './chart';

describe('niceMax', () => {
  it('rundet auf 1, 2, 5, 10 mal Zehnerpotenz auf', () => {
    expect(niceMax(0)).toBe(1);
    expect(niceMax(0.7)).toBe(1);
    expect(niceMax(1.1)).toBe(2);
    expect(niceMax(3)).toBe(5);
    expect(niceMax(7)).toBe(10);
    expect(niceMax(130)).toBe(200);
  });
});

describe('linePath', () => {
  it('beginnt links, endet rechts und klemmt Werte über dem Maximum', () => {
    const d = linePath([0, 5, 99], 10);
    expect(d.startsWith('M 0.0 ')).toBe(true);
    expect(d).toContain(`L ${W}.0 `);
    const ys = [...d.matchAll(/[ML] [\d.]+ ([\d.]+)/g)].map((m) => Number(m[1]));
    expect(ys[0]).toBe(H);
    expect(ys[2]).toBeCloseTo(H - H * 0.94);
  });
  it('liefert für leere Werte nichts und für eine Fläche den Abschluss', () => {
    expect(linePath([], 1)).toBe('');
    expect(areaPath([], 1)).toBe('');
    expect(areaPath([1, 2], 2).endsWith('Z')).toBe(true);
  });
});

describe('Hilfen', () => {
  it('peakOf nimmt den größten Wert aller Linien', () => {
    expect(peakOf([{ label: 'a', values: [1, 4], cls: '' }, { label: 'b', values: [9, 2], cls: '' }])).toBe(9);
    expect(peakOf([])).toBe(0);
  });
  it('nearestIndex bleibt im Bereich', () => {
    expect(nearestIndex(-1, 10)).toBe(0);
    expect(nearestIndex(0.5, 11)).toBe(5);
    expect(nearestIndex(2, 10)).toBe(9);
    expect(nearestIndex(0.4, 1)).toBe(0);
  });
  it('deltas macht aus einem Zähler Zuwächse', () => {
    expect(deltas([5, 7, 7, 12])).toEqual([0, 2, 0, 5]);
  });
  it('timeTicks gibt die gewünschte Zahl Marken von 0 bis 1', () => {
    const t = timeTicks([0, 3_600_000], 3);
    expect(t.map((x) => x.at)).toEqual([0, 0.5, 1]);
    expect(timeTicks([1], 3)).toEqual([]);
  });
});
