// RD1.1–RD1.3: Golden der Fixture-Specs (fixtures.golden.json) und die Golden-Regel aus paper-spec.md › 6.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { rastere } from '../render/raster';
import { FIXTURES, goldenEintrag, rendereFixture } from './fixtures';
import { GOLDEN_REGEL, gleich, goldenAbweichungen } from './regel';

const golden = JSON.parse(readFileSync(join(import.meta.dirname, 'fixtures.golden.json'), 'utf8'));
const bild = (extra: string) => rastere(`<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">` +
  `<rect width="100" height="100" fill="#808080"/>${extra}</svg>`).png;

describe('Golden-Regel', { timeout: 60_000 }, () => {
  it('GOLDEN_REGEL folgt paper-spec.md › 6 (exakt)', () => {
    expect(GOLDEN_REGEL).toBe('exakt');
  });

  it('gleich: exakt vergleicht SHA-256, toleranz erlaubt ≤ 0,1 % Pixel mit Kanal-Differenz > 2', () => {
    const a = bild(''), ein = bild('<rect x="5" y="5" width="1" height="1" fill="#ff0000"/>');
    const viele = bild('<rect x="5" y="5" width="20" height="1" fill="#ff0000"/>');
    expect(gleich(a, bild(''), 'exakt')).toBe(true);
    expect(gleich(a, ein, 'exakt')).toBe(false);
    expect(gleich(a, ein, 'toleranz')).toBe(true);
    expect(gleich(a, viele, 'toleranz')).toBe(false);
  });
});

describe('goldenAbweichungen (Sprint-Review RD1.5)', () => {
  const a = bild(''), b = bild('<rect x="5" y="5" width="1" height="1" fill="#ff0000"/>');
  const soll = { x: { renderKey: 'k', dateien: { 'd.png': 'h1' } } }, ist = { x: { renderKey: 'k', dateien: { 'd.png': 'h2' } } };
  const png = (seite: 'soll' | 'ist') => (seite === 'soll' ? a : b);

  it('anderer Hash: exakt meldet die Datei, toleranz vergleicht die Pixel', () => {
    expect(goldenAbweichungen(soll, ist, png, 'exakt')).toEqual(['x/d.png']);
    expect(goldenAbweichungen(soll, ist, png, 'toleranz')).toEqual([]);
  });

  it('anderer renderKey oder andere Dateien: immer Abweichung', () => {
    expect(goldenAbweichungen(soll, { x: { renderKey: 'z', dateien: soll.x.dateien } }, png, 'toleranz')).toEqual(['x: renderKey']);
    expect(goldenAbweichungen(soll, { x: { renderKey: 'k', dateien: {} } }, png, 'toleranz')).toEqual(['x: Dateien']);
    expect(goldenAbweichungen(soll, {}, png, 'toleranz')).toEqual(['x: fehlt']);
  });
});

describe('Golden der Fixtures', { timeout: 60_000 }, () => {
  it.each(FIXTURES)('%s: zweimal byte-gleich und gleich fixtures.golden.json', (id) => {
    const a = rendereFixture(id), b = rendereFixture(id);
    expect(a.pngs.map(([d]) => d)).toEqual(Object.keys(golden[id].dateien));
    for (let i = 0; i < a.pngs.length; i++) {
      expect(a.pngs[i][1].equals(b.pngs[i][1]), a.pngs[i][0]).toBe(true);
    }
    // Für Fixtures liegen keine Referenz-PNGs im Repo: unter `exakt` gelten die Hashes, unter `toleranz` nur renderKey und Dateien.
    const ist = goldenEintrag(a);
    if (GOLDEN_REGEL === 'exakt') expect(ist).toEqual(golden[id]);
    else expect([ist.renderKey, Object.keys(ist.dateien)]).toEqual([golden[id].renderKey, Object.keys(golden[id].dateien)]);
  }, 60_000);
});
