// RD1.3: Beinpfad des Vierbeiners wie im Prototyp (drawLegs): Stützpunkte, Breiten, Farben.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ladeStil } from '../render/stil';
import { beinpfad, beinPunkte } from './beinpfad';

const pferd = JSON.parse(readFileSync(join(import.meta.dirname, '../fixtures/specs/pferd.paper.json'), 'utf8')) as Spec;
const stil = ladeStil();
const fell = stil.materialien['fell-fuchs'];

describe('Beinpfad', () => {
  it('Stützpunkte: [x, −72], Hüfte, Hüfte + d1·12, Knie, Knie + d2·11, Fessel', () => {
    expect(beinPunkte(pferd, 'hn', {})).toEqual([[-30, -72], [-30, -56], [-30, -44], [-30, -31], [-30, -20], [-30, -9]]);
    const p = beinPunkte(pferd, 'vn', { 'oberbein-vn': 90 });
    expect(p[3][0]).toBeCloseTo(26 - 25, 9);
    expect(p[3][1]).toBeCloseTo(-56, 9);
  });

  it('Breite je Stützpunkt aus bein.breiten_vorn|hinten, vorn Wulst 1.6 am Punkt 2', () => {
    const hinten = beinpfad(pferd, 'hn', {}, stil), vorn = beinpfad(pferd, 'vn', {}, stil);
    expect(hinten).toContain('M-39.5 -72');
    expect(vorn).toContain('M18 -72');
    expect(vorn).toContain('33.35 -44');
    expect(hinten).toContain('-36.25 -44');
  });

  it('Material: erste flaeche von rumpf-vorn (vorn) bzw. rumpf-hinten (hinten), Schattenkante Ton schatten, Lichtkante falz', () => {
    const nah = beinpfad(pferd, 'vn', {}, stil), fern = beinpfad(pferd, 'hf', {}, stil);
    expect(nah).toContain(`fill="${fell.basis}" filter="url(#ds)"`);
    expect(nah).toContain(`fill="${fell.schatten}" filter="url(#ds2)"`);
    expect(nah).toContain(`stroke="${fell.licht}" stroke-width="1.3"`);
    expect(fern).toContain(`fill="${fell.schatten}" filter="url(#ds)"`);
    expect(fern).toContain(`stroke="${fell.basis}" stroke-width="1.3"`);
  });
});
