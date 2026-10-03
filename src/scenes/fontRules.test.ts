import { describe, expect, it } from 'vitest';
import { computeLayout } from './layout';
import { contrastRatio, effectiveFontPx, FONTS, minFontPx, OUTLINE_COLOR, type FontName } from './fontRules';

const LAYOUTS = [
  { name: 'Layout 1', seats: 1, partner: false },
  { name: 'Layout 1 mit Partner', seats: 1, partner: true },
  { name: 'Layout 2', seats: 2, partner: false },
  { name: 'Layout 3', seats: 3, partner: false },
  { name: 'Layout 4', seats: 4, partner: false },
];
const NAMES = Object.keys(FONTS) as FontName[];

describe('Mindest-Schriftgröße je Layout (S4.2, AC-03, bedienung.md § 2)', () => {
  for (const { name, seats, partner } of LAYOUTS) {
    it(`${name}: jede Rolle in jeder Zelle erreicht ihre Mindestgröße`, () => {
      const faults: string[] = [];
      computeLayout(seats, partner).forEach((cell, i) => {
        for (const role of NAMES) {
          const font = FONTS[role];
          const min = minFontPx(seats, cell, font.art);
          if (min === null) continue;
          const px = effectiveFontPx(font, cell);
          if (px < min) faults.push(`${name}, Zelle ${i} (${cell.w}x${cell.h}), Rolle ${role}: ${px} px < ${min} px`);
        }
      });
      expect(faults).toEqual([]);
    });
  }

  it('Mindestwerte laut Tabelle', () => {
    const [full] = computeLayout(1, false);
    const strips = computeLayout(3, false);
    expect([minFontPx(1, full!, 'info'), minFontPx(1, full!, 'side')]).toEqual([28, 24]);
    expect([minFontPx(3, strips[0]!, 'info'), minFontPx(3, strips[0]!, 'side')]).toEqual([24, 20]);
    expect([minFontPx(3, strips[2]!, 'info'), minFontPx(3, strips[2]!, 'side')]).toEqual([28, 20]);
    expect(minFontPx(2, computeLayout(2, false)[0]!, 'side')).toBe(24);
    expect(minFontPx(1, computeLayout(1, true)[0]!, 'info')).toBeNull(); // Mitspieler-Zelle: Regel offen
  });

  it('Welt-Schrift schrumpft mit der Zellenhöhe, HUD-Schrift nicht', () => {
    expect(effectiveFontPx({ px: 56, space: 'world' }, { h: 540 })).toBe(28);
    expect(effectiveFontPx({ px: 56, space: 'hud' }, { h: 540 })).toBe(56);
  });

  it('Kontrast jeder Textfarbe gegen die Kontur ≥ 4,5 : 1', () => {
    for (const role of NAMES) expect(contrastRatio(FONTS[role].color, OUTLINE_COLOR), role).toBeGreaterThanOrEqual(4.5);
    expect(contrastRatio('#ffffff', '#000000')).toBeCloseTo(21, 5);
  });
});
