import { describe, expect, it } from 'vitest';
import { hudPx, stagePx, worldPx } from './hudBar';

describe('Aufteilung der Bühne wie im Spiel', () => {
  it('Leiste verhält sich zur Welt wie BAR_HEIGHT zur Zellenhöhe (134 : 946)', () => {
    expect(hudPx(48) / worldPx(48)).toBeCloseTo(134 / 946, 1);
  });

  it('Gesamthöhe ist Leiste plus Welt und wächst mit dem Zoom', () => {
    expect(stagePx(12)).toBe(hudPx(12) + worldPx(12));
    expect(stagePx(24)).toBeGreaterThan(stagePx(12));
  });
});
