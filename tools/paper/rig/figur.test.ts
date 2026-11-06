// RD1.2: Posen der Figur wie im Prototyp (POSES.idle, POSES.walk), Keyframes obendrauf.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { Abgelehnt } from '../render/fehler';
import { figurPose, FIGUR_POSEN } from './figur';

const monarch = JSON.parse(readFileSync(join(import.meta.dirname, '../fixtures/specs/monarch.paper.json'), 'utf8')) as Spec;
/** Zeit zu einer Phase der Pose (Phase = t · freq). */
const zeit = (name: 'idle' | 'lauf', phase: number) => phase / FIGUR_POSEN[name].freq;

describe('Figur-Posen', () => {
  it('idle bei Phase 0: oberarm-v 5, unterarm-v -6, oberarm-h -3, unterarm-h -4, kopf 0', () => {
    const p = figurPose(monarch, 'idle', 0);
    expect(p.winkel).toMatchObject({ 'oberarm-v': 5, 'unterarm-v': -6, 'oberarm-h': -3, 'unterarm-h': -4, kopf: 0 });
    expect(p.versatz).toEqual([0, 0]);
    expect(p.wind).toBe(0);
  });

  it('lauf bei Phase 0.25 (sin = 1): oberschenkel-v -30, oberschenkel-h 30, unterschenkel-v 0', () => {
    const p = figurPose(monarch, 'lauf', zeit('lauf', 0.25));
    expect(p.winkel['oberschenkel-v']).toBeCloseTo(-30, 9);
    expect(p.winkel['oberschenkel-h']).toBeCloseTo(30, 9);
    expect(p.winkel['unterschenkel-v']).toBeCloseTo(0, 9);
    expect(p.wind).toBe(30);
  });

  it('lauf verschiebt die Figur um mon = [0, 2.5·|sin| − 2]', () => {
    expect(figurPose(monarch, 'lauf', zeit('lauf', 0.25)).versatz[1]).toBeCloseTo(0.5, 9);
    expect(figurPose(monarch, 'lauf', 0).versatz).toEqual([0, -2]);
  });

  it('Keyframes der Spec addieren sich zum Rig-Winkel', () => {
    const s = structuredClone(monarch);
    s.animationen!.idle.keyframes = [{ t: 0, winkel: { kopf: 10, krone: 3 } }];
    const p = figurPose(s, 'idle', 0);
    expect(p.winkel.kopf).toBe(10);
    expect(p.winkel['oberarm-v']).toBe(5);
  });

  it('unbekannte Animation einer figur ohne Keyframes: Exit 1, Regel animation-unbekannt', () => {
    const s = structuredClone(monarch);
    s.animationen!.winken = { frames: 4 };
    expect(() => figurPose(s, 'winken', 0)).toThrow(Abgelehnt);
    expect(() => figurPose(monarch, 'gibts-nicht', 0)).toThrow(Abgelehnt);
    s.animationen!.winken.keyframes = [{ t: 0, winkel: { 'oberarm-v': -90 } }];
    expect(figurPose(s, 'winken', 0).winkel['oberarm-v']).toBe(-90);
  });
});
