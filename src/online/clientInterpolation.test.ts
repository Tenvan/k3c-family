import { describe, expect, it } from 'vitest';
import { blendAlpha, interpolate } from './clientInterpolation';
import type { WorldState } from './clientProtocol';

function state(players: { id: number; x: number }[], extra: Partial<Record<string, unknown>> = {}): WorldState {
  return { players, troops: [], enemies: [], projectiles: [], coins: [], time: 0, events: [], depth: 0, ...extra } as unknown as WorldState;
}

describe('interpolate', () => {
  const a = state([{ id: 1, x: 10 }], { time: 1, coins: [{ id: 9, x: 5 }] });
  const b = state([{ id: 1, x: 12 }, { id: 2, x: 50 }], { time: 2, coins: [{ id: 9, x: 6 }] });

  it('alpha 0, 0,5 und 1 liefern Anfang, Mitte und Ende', () => {
    expect(interpolate(a, b, 0).players[0]?.x).toBe(10);
    expect(interpolate(a, b, 0.5).players[0]?.x).toBe(11);
    expect(interpolate(a, b, 1)).toBe(b);
  });

  it('neue Einträge erscheinen am Ziel, alles andere springt zum neueren Zustand', () => {
    const mid = interpolate(a, b, 0.5);
    expect(mid.players[1]?.x).toBe(50);
    expect(mid.time).toBe(2);
    expect((mid.coins[0] as { x: number }).x).toBe(6);
  });

  it('entfernte Einträge sind weg, große Sprünge werden nicht überblendet', () => {
    const c = state([{ id: 1, x: 200 }]);
    expect(interpolate(a, c, 0.5).players[0]?.x).toBe(200);
    expect(interpolate(b, state([]), 0.5).players).toEqual([]);
  });

  it('überblendet y, wenn beide Zustände eines haben, und verändert die Eingaben nicht', () => {
    const p = state([], { projectiles: [{ id: 3, x: 0, y: 10 }] });
    const q = state([], { projectiles: [{ id: 3, x: 4, y: 20 }] });
    expect(interpolate(p, q, 0.5).projectiles[0]).toMatchObject({ x: 2, y: 15 });
    expect((p.projectiles[0] as { x: number }).x).toBe(0);
  });
});

describe('blendAlpha', () => {
  it('beginnt beim Eintreffen, dauert einen Tick und bleibt bei Ausfall am Ziel stehen', () => {
    expect(blendAlpha(100, 100, 33)).toBe(0);
    expect(blendAlpha(116.5, 100, 33)).toBeCloseTo(0.5);
    expect(blendAlpha(500, 100, 33)).toBe(1);
    expect(blendAlpha(90, 100, 33)).toBe(0);
  });
});
