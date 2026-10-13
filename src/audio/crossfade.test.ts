import { describe, expect, it } from 'vitest';
import { crossfadeCurves, fadeOutFrom } from './crossfade';

describe('Crossfade-Kurven', () => {
  it('Enden stimmen, gleiche Leistung, kein Sprung', () => {
    const { out, in: inn } = crossfadeCurves(64);
    expect([out[0], out[63], inn[0], inn[63]].map((v) => Math.round(v * 1000) / 1000)).toEqual([1, 0, 0, 1]);
    for (let i = 0; i < 64; i++) expect(out[i] ** 2 + inn[i] ** 2).toBeCloseTo(1, 5);
    for (let i = 1; i < 64; i++) {
      expect(Math.abs(out[i] - out[i - 1])).toBeLessThan(0.05);
      expect(Math.abs(inn[i] - inn[i - 1])).toBeLessThan(0.05);
      expect(out[i]).toBeLessThanOrEqual(out[i - 1]);
    }
  });

  it('fadeOutFrom beginnt beim Pegel, endet bei 0 und fällt monoton; Müll zählt als 1', () => {
    const c = fadeOutFrom(0.4);
    expect(c[0]).toBeCloseTo(0.4, 5);
    expect(c.at(-1)).toBeCloseTo(0, 5);
    for (let i = 1; i < c.length; i++) expect(c[i]).toBeLessThanOrEqual(c[i - 1]);
    expect(fadeOutFrom(Number.NaN)[0]).toBe(1);
    expect(fadeOutFrom(7)[0]).toBe(1);
    expect(fadeOutFrom(-1)[0]).toBe(0);
  });
});
