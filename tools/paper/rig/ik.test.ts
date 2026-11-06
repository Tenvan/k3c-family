// RD1.3: Zwei-Knochen-IK wie im Prototyp (ik, HL1 = 25, HL2 = 31), Winkel 0 = Bein hängt senkrecht.
import { describe, expect, it } from 'vitest';
import { HL1, HL2, ik } from './ik';

describe('IK', () => {
  it('Bein fast gestreckt: ik(0, 56, true) liefert beide Winkel betragsmäßig < 3°', () => {
    expect(HL1 + HL2).toBe(56);
    const [a, b] = ik(0, 56, true);
    expect(Math.abs(a)).toBeLessThan(3);
    expect(Math.abs(b)).toBeLessThan(3);
  });

  it('Knie vorn/hinten spiegelt die Beugung', () => {
    expect(ik(10, 40, true)[1]).toBeCloseTo(-ik(10, 40, false)[1], 9);
    expect(ik(10, 40, true)[1]).toBeGreaterThan(0);
  });

  it('Ziel außerhalb der Reichweite wird auf |HL1 − HL2| + 0.01 bzw. HL1 + HL2 − 0.01 gekappt', () => {
    expect(ik(0, 500, true)).toEqual(ik(0, 55.99, true));
    expect(ik(0, 1, false)).toEqual(ik(0, 6.01, false));
  });

  it('Endpunkt der Kette liegt auf dem Ziel', () => {
    const [u1, u2] = ik(12, 40, true), r = (g: number) => (g * Math.PI) / 180;
    const knie = [-Math.sin(r(u1)) * HL1, Math.cos(r(u1)) * HL1];
    const fuss = [knie[0] - Math.sin(r(u1 + u2)) * HL2, knie[1] + Math.cos(r(u1 + u2)) * HL2];
    expect(fuss[0]).toBeCloseTo(12, 6);
    expect(fuss[1]).toBeCloseTo(40, 6);
  });
});
