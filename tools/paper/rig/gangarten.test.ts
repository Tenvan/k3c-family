// RD1.3: Gangarten wie im Prototyp (GAITS, gait): Standphase am Boden, Schwungbahn, Rückengelenk, Reiter-Pose.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { anwenden } from '../render/matrix';
import { BEINE, GANGARTEN, gangPose } from './gangarten';
import { vierbeinerMatrizen } from './vierbeiner';

const pferd = JSON.parse(readFileSync(join(import.meta.dirname, '../fixtures/specs/pferd.paper.json'), 'utf8')) as Spec;
const BEINTEILE = ['oberbein', 'unterbein', 'huf'].flatMap((t) => ['hf', 'hn', 'vf', 'vn'].map((b) => `${t}-${b}`)).sort();

describe('Gangarten', () => {
  it('GANGARTEN: schritt freq 0.9, trab 1.5, galopp 2.1; stance 0.65 / 0.45 / 0.25', () => {
    expect([GANGARTEN.schritt.freq, GANGARTEN.trab.freq, GANGARTEN.galopp.freq]).toEqual([0.9, 1.5, 2.1]);
    expect([GANGARTEN.schritt.stance, GANGARTEN.trab.stance, GANGARTEN.galopp.stance]).toEqual([0.65, 0.45, 0.25]);
  });

  it('Standphase: Huf am Boden (ty = 0), bewegt sich von +r nach −r, r = reach · massstab', () => {
    for (const g of ['schritt', 'trab', 'galopp'] as const) {
      const G = GANGARTEN[g], bein = 'hn', offs = G.offs[bein];
      const xs = [0.05, 0.5, 0.95].map((k) => {
        const p = gangPose(pferd, g, (offs + k * G.stance) % 1);
        const m = vierbeinerMatrizen(pferd, p).get(`unterbein-${bein}`)!;
        const [x, y] = anwenden(m, [BEINE[bein].x, 0]);
        // Bei voller Streckung kappt die IK (HL1 + HL2 − 0.01) wie im Prototyp; im Trab (dy bis −1.5) schwebt der Huf dann bis ca. 3.5 über dem Boden.
        expect(Math.abs(y), `${g}: Huf am Boden`).toBeLessThan(4);
        return x;
      });
      expect(xs[0]).toBeGreaterThan(xs[1]);
      expect(xs[1]).toBeGreaterThan(xs[2]);
      const r = G.reach * 1.3;
      expect(Math.abs(xs[0] - xs[2] - 2 * r * 0.9)).toBeLessThan(3);
    }
  });

  it('Galopp-Schwungbahn folgt spline(path.front|back): Huf hebt in der Schwungphase ab', () => {
    const G = GANGARTEN.galopp, k = 0.5, phase = (G.offs.vn + G.stance + k * (1 - G.stance)) % 1;
    const m = vierbeinerMatrizen(pferd, gangPose(pferd, 'galopp', phase)).get('unterbein-vn')!;
    expect(anwenden(m, [BEINE.vn.x, 0])[1]).toBeLessThan(-5);
  });

  it('gangPose(galopp, 0): Winkel für alle 12 Beinteile, hals = nod, schweif = tail, rumpf-hinten = spine', () => {
    const p = gangPose(pferd, 'galopp', 0), TAU = Math.PI * 2;
    expect(Object.keys(p.winkel).filter((k) => /bein|huf/.test(k)).sort()).toEqual(BEINTEILE);
    expect(p.winkel.hals).toBeCloseTo(10 * Math.cos(TAU * -0.45), 9);
    expect(p.winkel.schweif).toBeCloseTo(28 + 10 * Math.sin(TAU * -0.2), 9);
    expect(p.winkel['rumpf-hinten']).toBeCloseTo(7 * Math.cos(TAU * -0.2), 9);
    expect(p.ride.pitch).toBeCloseTo(6 * Math.sin(TAU * -0.2), 9);
  });

  it('wind = 2r / (stance / freq)', () => {
    const G = GANGARTEN.trab;
    expect(gangPose(pferd, 'trab', 0.3).wind).toBeCloseTo((2 * G.reach * 1.3) / (G.stance / G.freq), 9);
  });

  it('Reiter: Versatz [−2, −54 + riderY], Drehung lean, Winkel der Figur', () => {
    const p = gangPose(pferd, 'schritt', 0);
    expect(p.reiter.versatz[0]).toBe(-2);
    expect(p.reiter.drehung).toBe(3);
    expect(p.reiter.winkel['oberschenkel-v']).toBe(-43);
    expect(p.reiter.winkel['unterschenkel-v']).toBe(50);
  });
});
