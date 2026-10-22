import { describe, expect, it } from 'vitest';
import sprites from '../../../data/sprites.json';
import { effectFor } from '../effects/effects';
import { DEAD_ALPHA, type MonarchFrame, pickMonarchAnim } from './monarchAnim';
import type { AnimName } from './sprites';

const alive: MonarchFrame = { respawnIn: 0, hp: 10, moving: false };
const dead: MonarchFrame = { respawnIn: 3, hp: 0, moving: false };
const all = () => true;
const legacy = (a: AnimName) => a === 'idle' || a === 'run' || a === 'attack';

describe('pickMonarchAnim', () => {
  it('steht und läuft in Schleife', () => {
    expect(pickMonarchAnim(alive, alive, null, all)).toEqual({ anim: 'idle', restart: false, alpha: 1 });
    expect(pickMonarchAnim(alive, { ...alive, moving: true }, null, all).anim).toBe('run');
  });

  it('Tod: death hält, ohne death-Animation idle mit Deckkraft 0,25', () => {
    expect(pickMonarchAnim(alive, dead, null, all)).toEqual({ anim: 'death', restart: false, alpha: 1 });
    expect(pickMonarchAnim(dead, dead, null, all)).toEqual({ anim: 'death', restart: false, alpha: 1 });
    expect(pickMonarchAnim(alive, dead, null, legacy)).toEqual({ anim: 'idle', restart: false, alpha: DEAD_ALPHA });
  });

  it('Respawn beim Wechsel respawnIn > 0 → 0, Rückfall Stehen', () => {
    expect(pickMonarchAnim(dead, alive, null, all)).toEqual({ anim: 'respawn', restart: true, alpha: 1 });
    expect(pickMonarchAnim(dead, alive, null, legacy)).toEqual({ anim: 'idle', restart: false, alpha: 1 });
  });

  it('Treffer, wenn hp sinkt; Heilen und der erste Frame zählen nicht', () => {
    expect(pickMonarchAnim(alive, { ...alive, hp: 7 }, null, all)).toEqual({ anim: 'hit', restart: true, alpha: 1 });
    expect(pickMonarchAnim({ ...alive, hp: 7 }, alive, null, all).anim).toBe('idle');
    expect(pickMonarchAnim(undefined, alive, null, all).anim).toBe('idle');
    expect(pickMonarchAnim(alive, { ...alive, hp: 7 }, null, legacy).anim).toBe('idle');
  });

  it('Münze geben: coinGive trägt den Spieler, der Hinweis spielt give; Tod geht vor', () => {
    const fx = effectFor({ type: 'coinGive', player: 1, x: 2, to: 'site' }, () => undefined, new Map());
    expect(fx?.giver).toBe(1);
    expect(pickMonarchAnim(alive, alive, 'give', all)).toEqual({ anim: 'give', restart: true, alpha: 1 });
    expect(pickMonarchAnim(alive, alive, 'give', legacy).anim).toBe('idle');
    expect(pickMonarchAnim(alive, dead, 'give', all).anim).toBe('death');
  });

  it('beide Spieler-Sheets haben give, hit, death und respawn', () => {
    const sheets = sprites.sheets as Record<string, { anims: Record<string, unknown> }>;
    for (const { sheet } of sprites.players) {
      expect(Object.keys(sheets[sheet]!.anims), sheet).toEqual(expect.arrayContaining(['give', 'hit', 'death', 'respawn']));
    }
  });
});
