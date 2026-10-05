import { beforeEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import { KEY_ACTIONS, type Action, type Device } from '../input/slotBindings';
import { GLYPH_COLORS, glyphOf } from './glyphs';

/** Alle Aktionen aus `src/input/playerInput.ts` (Typ `Action`), in der Reihenfolge der Belegung */
const ACTIONS = Object.keys(KEY_ACTIONS) as Action[];

beforeEach(() => setLanguage('de'));

describe('glyphOf: Aktion + Gerät → Glyph-Schlüssel (S6.2 AC-04, B-149 AC-01)', () => {
  it('kennt alle neun Aktionen', () => {
    expect(ACTIONS).toEqual(['confirm', 'pause', 'fullscreen', 'attack', 'skill1', 'skill2', 'skill3', 'skill4', 'skillMenu']);
  });

  it.each<[Device, (string | null)[]]>([
    ['pad', ['pad:A', 'pad:MENU', null, 'pad:X', 'pad:LB', 'pad:RB', 'pad:LT', 'pad:UP', 'pad:DOWN']],
    ['keyboard', ['key:SPACE', 'key:ESC', 'key:F', 'key:E', 'key:Q', 'key:R', 'key:T', 'key:Z', 'key:K']],
    ['touch', ['touch:confirm', null, 'touch:fullscreen', 'touch:attack', 'touch:skill1', 'touch:skill2', 'touch:skill3', 'touch:skill4', 'touch:skillMenu']],
  ])('%s', (device, keys) => {
    const glyphs = ACTIONS.map((a) => glyphOf(a, device));
    expect(glyphs.map((g) => g.key)).toEqual(keys);
    expect(glyphs.every((g) => g.label.length > 0)).toBe(true);
  });

  it('Beschriftung wie im Overlay', () => {
    expect(glyphOf('confirm', 'keyboard').label).toBe('Leertaste');
    expect(glyphOf('skillMenu', 'pad').label).toBe('D-Pad ↓');
    expect(glyphOf('attack', 'touch').label).toBe('⚔');
    expect(glyphOf('confirm', 'touch').label).toBe('🪙');
  });

  it('Text-Rückfall: rechter Stick, Pausieren auf Touch, unbekannte Aktion, unbekanntes Gerät', () => {
    expect(glyphOf('fullscreen', 'pad')).toEqual({ key: null, label: 'RS' });
    expect(glyphOf('pause', 'touch')).toEqual({ key: null, label: 'pause' });
    expect(glyphOf('dance', 'pad')).toEqual({ key: null, label: 'dance' });
    expect(glyphOf('toString', 'keyboard')).toEqual({ key: null, label: 'toString' });
    expect(glyphOf('attack', 'wheel')).toEqual({ key: null, label: 'attack' });
  });

  it('B und View erzeugen nie einen Glyph', () => {
    const keys = (['pad', 'keyboard', 'touch'] as Device[]).flatMap((d) => ACTIONS.map((a) => glyphOf(a, d).key));
    expect(keys).not.toContain('pad:B');
    expect(keys).not.toContain('pad:VIEW');
  });
});

/** WCAG-Kontrast zweier Farben `0xRRGGBB` */
function contrast(a: number, b: number): number {
  const lum = (c: number) => {
    const [r, g, bl] = [16, 8, 0].map((s) => {
      const v = ((c >> s) & 0xff) / 255;
      return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
    });
    return 0.2126 * r! + 0.7152 * g! + 0.0722 * bl!;
  };
  const [hi, lo] = [lum(a), lum(b)].sort((x, y) => y - x);
  return (hi! + 0.05) / (lo! + 0.05);
}

describe('Glyph-Farben (Kontrast ≥ 4,5:1)', () => {
  it.each(Object.entries(GLYPH_COLORS))('%s', (_name, { fill, ink }) => {
    expect(contrast(fill, ink)).toBeGreaterThanOrEqual(4.5);
  });
});
