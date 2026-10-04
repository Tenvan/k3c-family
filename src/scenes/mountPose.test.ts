import { describe, expect, it } from 'vitest';
import monarch from '../../data/monarch.json';
import sprites from '../../data/sprites.json';
import { type MountContext, mountPose } from './mountPose';
import type { MountData, SheetData, SpriteSpec } from './sprites';

// Mini-Daten: Reittier-Sheet 10×10, Origin 0.5/1, Zoom 2; Reiter-Sheet 20×20, Origin 0.5/0.8, Höhe 10, Zoom 3 × Spec 2 = 6.
// Hüfte = 0.8·20 − 10·(1 − 0.5) = 11, top = sy − 10·0.5·6 = sy − 30.
const sheets = {
  m: { frameWidth: 10, frameHeight: 10, originX: 0.5, originY: 1, height: 8, scale: 1, anims: {} },
  r: { frameWidth: 20, frameHeight: 20, originX: 0.5, originY: 0.8, height: 10, scale: 3, anims: {} },
} as unknown as Record<string, SheetData>;
const mounts: Record<string, MountData> = {
  horse: { label: 'Pferd', sheet: 'm', scale: 2, tint: '#abcdef', saddle: { idle: [[5, 4]], run: [[5, 4], [6, 3], [4, 5]] } },
  stuck: { label: 'Nur idle', sheet: 'm', scale: 2, saddle: { idle: [[7, 8]] } },
  empty: { label: 'Leer', sheet: 'm', scale: 2, saddle: {} },
};
const rider: SpriteSpec = { sheet: 'r', scale: 2 };
const ctx: MountContext = { key: 'horse', rider, refSpeed: 5, mounts, sheets, riderWaist: 0.5 };
const right = { vx: 5, facing: 1 as const };

describe('mountPose', () => {
  it('(a) Stehen: idle, timeScale 1', () => {
    const pose = mountPose({ vx: 0, facing: 1 }, 0, ctx)!;
    expect(pose.anim).toBe('idle');
    expect(pose.timeScale).toBe(1);
    expect(pose.sheet).toBe('m');
    expect(pose.tint).toBe('#abcdef');
  });

  it('(b) Laufen und Sprint: timeScale folgt |vx| / refSpeed, geklemmt', () => {
    expect(mountPose(right, 0, ctx)).toMatchObject({ anim: 'run', timeScale: 1 });
    expect(mountPose({ vx: 9, facing: 1 }, 0, ctx)!.timeScale).toBeCloseTo(1.8, 12);
    expect(mountPose({ vx: 500, facing: 1 }, 0, ctx)!.timeScale).toBe(2.5);
    expect(mountPose({ vx: 0.1, facing: 1 }, 0, ctx)!.timeScale).toBe(0.5);
    expect(mountPose({ vx: -5, facing: -1 }, 0, ctx)!.timeScale).toBe(1);
  });

  it('(c) Reiterposition je Frame von Hand gerechnet', () => {
    const want = [
      { x: 0, y: -12 },
      { x: 2, y: -14 },
      { x: -2, y: -10 },
    ];
    want.forEach((w, f) => {
      const pose = mountPose(right, f, ctx)!;
      expect(pose.rider.x).toBeCloseTo(w.x, 9);
      expect(pose.rider.y).toBeCloseTo(w.y, 9);
      expect(pose.top).toBeCloseTo(w.y - 30, 9);
      expect(pose.rider.cropHeight).toBeCloseTo(11, 9);
      expect(pose.rider.originY).toBeCloseTo(0.55, 9);
      expect(pose.rider.scale).toBe(6);
      expect(pose.saddleFrame).toBe(f);
    });
  });

  it('(d) Blick nach links: gespiegelt um den Fußpunkt', () => {
    const l = mountPose({ vx: 5, facing: -1 }, 1, ctx)!;
    const r = mountPose(right, 1, ctx)!;
    expect(l.flip).toBe(true);
    expect(r.flip).toBe(false);
    expect(l.rider.x).toBe(-r.rider.x);
    expect(l.rider.y).toBe(r.rider.y);
    expect(l.rider.originX).toBe(1 - r.rider.originX);
  });

  it('(e) Rückfall: unbekannt → null, Frame außerhalb und fehlende Animation ohne Wurf', () => {
    expect(mountPose(right, 0, { ...ctx, key: 'nope' })).toBeNull();
    expect(mountPose(right, 0, { ...ctx, key: undefined })).toBeNull();
    expect(mountPose(right, 0, { ...ctx, rider: { sheet: 'x' } })).toBeNull();
    expect(mountPose(right, 0, { ...ctx, key: 'empty' })).toBeNull();
    expect(mountPose(right, 7, ctx)).toEqual(mountPose(right, 1, ctx));
    const stuck = mountPose(right, 2, { ...ctx, key: 'stuck' })!;
    expect(stuck.anim).toBe('run');
    expect(stuck.rider.x).toBeCloseTo(4, 9);
    expect(stuck.rider.y).toBeCloseTo(-4, 9);
  });

  it('(f) rein: Reihenfolge egal, Eingaben unverändert', () => {
    const frozen: MountContext = Object.freeze({ ...ctx, mounts: Object.freeze({ horse: Object.freeze(mounts.horse!) }) });
    const a = { vx: 0, facing: 1 as const };
    const b = { vx: -9, facing: -1 as const };
    const both = [mountPose(a, 0, frozen), mountPose(b, 2, frozen)];
    expect([mountPose(b, 2, frozen), mountPose(a, 0, frozen)]).toEqual([both[1], both[0]]);
  });

  it('(g) echte Daten: Reittier aus monarch.json, jeder Reiter, jedes run-Frame', () => {
    const key = monarch.mount.sprite;
    const real = sprites as unknown as { sheets: Record<string, SheetData>; mounts: Record<string, MountData>; riderWaist: number; players: SpriteSpec[] };
    expect(real.mounts[key]).toBeDefined();
    const frames = real.sheets[real.mounts[key]!.sheet]!.anims.run!.frames;
    for (const spec of real.players) {
      for (let f = 0; f < frames; f++) {
        const c: MountContext = { key, rider: spec, refSpeed: monarch.base.speed * monarch.mount.speedFactor, mounts: real.mounts, sheets: real.sheets, riderWaist: real.riderWaist };
        expect(mountPose({ vx: 5, facing: 1 }, f, c), `${spec.sheet} Frame ${f}`).not.toBeNull();
      }
    }
  });
});
