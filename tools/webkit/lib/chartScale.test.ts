import { describe, expect, it } from 'vitest';
import { extent, formatTick, linePath, niceExtent, niceTicks, scaleLinear } from './chartScale';

describe('chartScale', () => {
  it('extent: Rand, leer und flach', () => {
    expect(extent([0, 10], 0.1)).toEqual({ min: -1, max: 11 });
    expect(extent([])).toEqual({ min: 0, max: 1 });
    expect(extent([NaN, 5])).toEqual({ min: 4, max: 6 });
  });
  it('niceTicks: runde Schritte', () => {
    expect(niceTicks(0, 10, 5)).toEqual([0, 2, 4, 6, 8, 10]);
    expect(niceTicks(0, 1, 5)).toEqual([0, 0.2, 0.4, 0.6, 0.8, 1]);
    expect(niceTicks(3, 3)).toEqual([3]);
  });
  it('niceExtent rundet nach außen', () => {
    expect(niceExtent({ min: 0.3, max: 9.1 })).toEqual({ min: 0, max: 10 });
  });
  it('scaleLinear bildet ab, auch umgekehrt und flach', () => {
    const y = scaleLinear({ min: 0, max: 10 }, [100, 0]);
    expect(y(0)).toBe(100);
    expect(y(10)).toBe(0);
    expect(y(5)).toBe(50);
    expect(scaleLinear({ min: 2, max: 2 }, [0, 10])(2)).toBe(5);
  });
  it('formatTick und linePath', () => {
    expect(formatTick(4)).toBe('4');
    expect(formatTick(0.30000000004)).toBe('0.3');
    expect(linePath([])).toBe('');
    expect(linePath([{ x: 1, y: 2 }, { x: 3, y: 4 }])).toBe('M1.0 2.0 L3.0 4.0');
  });
});
