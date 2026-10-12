import { describe, expect, it } from 'vitest';
import sprites from '../../data/sprites.json';

/**
 * Sattelpunkte des Standard-Reittiers (B-320, GR7.2): Beim Wechsel Stehen → Laufen darf der Reiter nicht auf die Kruppe
 * rutschen. Die Punkte stammen aus dem Bild (Abstand zum Mähnenansatz wie im `idle`-Frame), Einheit Sheet-Pixel.
 */
const MAX_DIST = 4;
/** Frames, in denen der Rücken im Galopp höher liegt als im Stand (Sammelphase in der Luft); der Reiter folgt dem Rücken */
const BOBBING: Record<number, string> = { 2: 'Sammelphase: Rücken liegt 5 px höher als im Stand' };

describe.each(['horse', 'blackHorse'] as const)('Sattelpunkte %s', (key) => {
  const { idle, run } = sprites.mounts[key].saddle;
  const [ix, iy] = idle[0] as [number, number];

  it('jeder run-Frame liegt höchstens 4 px (x) neben dem idle-Punkt', () => {
    run.forEach(([x], f) => expect(Math.abs(x! - ix), `run[${f}]`).toBeLessThanOrEqual(MAX_DIST));
  });

  it('jeder run-Frame liegt höchstens 4 px (y) vom idle-Punkt, Ausnahmen sind begründet', () => {
    run.forEach(([, y], f) => {
      const dy = Math.abs(y! - iy);
      if (BOBBING[f]) expect(dy, `run[${f}] ${BOBBING[f]}`).toBeLessThanOrEqual(MAX_DIST + 2);
      else expect(dy, `run[${f}]`).toBeLessThanOrEqual(MAX_DIST);
    });
  });
});
