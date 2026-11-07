// MG9.2 (B-717/AC-01): Gangart `stand` für das Reit-Idle: alle Hufe am Boden, nur Atmen, nie Pflicht, nie fremd.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ladeStil } from '../render/stil';
import './register';
import { gangPose } from './gangarten';
import { pruefeRenderRegeln } from './pruefen';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const pferd = JSON.parse(readFileSync(join(SPECS, 'pferd.paper.json'), 'utf8')) as Spec, stil = ladeStil();

describe('Gangart stand', () => {
  it('Pose ist endlich, die Beine hängen gleich in allen vier Hufen und die Phase verändert nur Atmen und Nicken', () => {
    const a = gangPose(pferd, 'stand', 0.25), b = gangPose(pferd, 'stand', 0.75);
    for (const p of [a, b]) for (const w of Object.values(p.winkel)) expect(Number.isFinite(w)).toBe(true);
    expect(a.winkel['oberbein-hf']).toBeCloseTo(a.winkel['oberbein-hn'], 0);
    expect(a.ride.dy).not.toBe(b.ride.dy);
    expect(a.wind).toBe(0);
  });

  it('Animation mit gangart stand ist ohne Eintrag in proportionen.gangarten gültig, fehlt sie, ist das kein Fehler', () => {
    const mit: Spec = { ...structuredClone(pferd), animationen: { ...pferd.animationen, 'reiten-idle': { frames: 8, gangart: 'stand' } } };
    expect(pruefeRenderRegeln(mit, join(SPECS, 'pferd.paper.json'), stil).map((f) => f.regel)).toEqual([]);
    expect(pruefeRenderRegeln(pferd, join(SPECS, 'pferd.paper.json'), stil).map((f) => f.regel)).toEqual([]);
  });
});
