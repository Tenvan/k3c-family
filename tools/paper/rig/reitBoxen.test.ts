// B-845: Bein-Boxen des Reittiers umfassen den Beinpfad samt Wulst der Vorderbeine; --ohne blendet Teile aus.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { points } from '../schema/form';
import type { Lage, Punkt, Spec } from '../schema/types';
import { beinKanten } from './beinpfad';
import { BEINE, type Bein, gangPose } from './gangarten';
import { reittierGeometrie } from './reitBoxen';

const pferd = JSON.parse(readFileSync(join(import.meta.dirname, '../fixtures/specs/pferd.paper.json'), 'utf8')) as Spec;

/** Box [x0, y0, x1, y1] aus der Ersatz-Lage eines Beinteils (lokale Koordinaten des Rumpfs) */
const box = (lage: Lage): number[] => {
  const pts = lage.art === 'punkt' ? [lage.mitte] : points(lage.form);
  const xs = pts.map((p) => p[0]), ys = pts.map((p) => p[1]);
  return [Math.min(...xs), Math.min(...ys), Math.max(...xs), Math.max(...ys)];
};
const drin = ([x, y]: Punkt, [x0, y0, x1, y1]: number[]) => x >= x0 - 1e-9 && x <= x1 + 1e-9 && y >= y0 - 1e-9 && y <= y1 + 1e-9;

describe('Bein-Boxen des Reittiers (B-845/AC-02)', () => {
  it.each([0, 0.25, 0.5, 0.75])('Oberbein-Box umfasst beide Kanten des Beinpfads samt Wulst (galopp, Phase %s)', (phase) => {
    const p = gangPose(pferd, 'galopp', phase), geo = reittierGeometrie(pferd, p);
    for (const b of Object.keys(BEINE) as Bein[]) {
      const ober = geo.find((g) => g.teil.id === `reittier:oberbein-${b}`)!, k = beinKanten(pferd, b, p.winkel);
      const B = box(ober.teil.lagen[0]);
      for (const i of [1, 2, 3]) {
        expect(drin(k.links[i], B), `${b} links ${i}`).toBe(true);
        expect(drin(k.rechts[i], B), `${b} rechts ${i}`).toBe(true);
      }
    }
  });

  it('ohne: genannte Teile fehlen, Ober- oder Unterbein nimmt beide Boxen des Beins', () => {
    const p = gangPose(pferd, 'galopp', 0.25);
    const ids = reittierGeometrie(pferd, p, new Set(['hals', 'unterbein-vn'])).map((g) => g.teil.id);
    expect(ids).not.toContain('reittier:hals');
    expect(ids).not.toContain('reittier:unterbein-vn');
    expect(ids).not.toContain('reittier:oberbein-vn');
    expect(ids).toContain('reittier:oberbein-vf');
    expect(ids).toContain('reittier:rumpf-vorn');
  });
});
