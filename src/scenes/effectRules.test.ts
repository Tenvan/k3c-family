import { describe, expect, it, vi } from 'vitest';
import type { GameEvent } from '../model/types';
import { FLASH_MIN_GAP_MS, MAX_FLASHES_PER_SECOND } from './effects';
import { flashAllowed, hurtSeat, rumblePad, runEffect, shakeCell } from './effectRules';
import type { Cell } from './layout';

const hit = (target: 'player' | 'enemy', id: number): GameEvent => ({ type: 'hit', x: 3, target, id, damage: 1 });
const cell = (kind: Cell['kind'], seat: number): Cell => ({ x: 0, y: 0, w: 1, h: 1, kind, seat });

describe('Blitzgrenze (GR5.2 AC-05)', () => {
  it('Mindestabstand erlaubt höchstens 3 Blitze pro Sekunde', () => {
    expect(MAX_FLASHES_PER_SECOND).toBe(3);
    expect(FLASH_MIN_GAP_MS * MAX_FLASHES_PER_SECOND).toBeGreaterThanOrEqual(1000);
    let last: number | null = null;
    let ran = 0;
    for (let t = 0; t < 1000; t += 16) {
      if (flashAllowed(t, last)) {
        last = t;
        ran++;
      }
    }
    expect(ran).toBeLessThanOrEqual(3);
    expect(flashAllowed(FLASH_MIN_GAP_MS - 1, 0)).toBe(false);
    expect(flashAllowed(FLASH_MIN_GAP_MS, 0)).toBe(true);
  });
});

describe('runEffect (GR5.2 AC-02)', () => {
  it('Blitz „aus“ → entfällt, „an“ → läuft; Nicht-Blitz-Effekte immer', () => {
    expect(runEffect(true, { flash: false }, 1000, null)).toBe(false);
    expect(runEffect(true, { flash: true }, 1000, null)).toBe(true);
    expect(runEffect(true, { flash: true }, 100, 0)).toBe(false);
    expect(runEffect(false, { flash: false }, 100, 0)).toBe(true);
  });
});

describe('shakeCell (GR5.2 AC-04)', () => {
  const seats2 = [{ slot: 1, monarch: 5, depth: 0 }, { slot: 0, monarch: 2, depth: 0 }];
  const cells2 = [cell('player', 0), cell('player', 1)];
  it('2 lokale Spieler: nur die Zelle des Getroffenen', () => {
    expect(shakeCell(hit('player', 2), seats2, cells2)).toBe(0);
    expect(shakeCell(hit('player', 5), seats2, cells2)).toBe(1);
  });
  it('1 Spieler mit Partner-Zelle: Partner-Zelle schüttelt nicht', () => {
    const cells = [cell('partner', -1), cell('player', 0)];
    expect(shakeCell(hit('player', 2), [{ slot: 0, monarch: 2, depth: 0 }], cells)).toBe(1);
  });
  it('anderes Gerät, Gegner, andere Events → keine Zelle', () => {
    expect(shakeCell(hit('player', 9), seats2, cells2)).toBeNull();
    expect(shakeCell(hit('enemy', 2), seats2, cells2)).toBeNull();
    expect(hurtSeat({ type: 'kill', kind: 'gnoll', x: 1, gold: 0 }, seats2)).toBeNull();
  });
});

describe('rumblePad (GR5.2 AC-06)', () => {
  it('ohne vibrationActuator: keine Ausnahme, onFail einmal', () => {
    const fail = vi.fn();
    expect(() => rumblePad({ mapping: 'standard' }, fail)).not.toThrow();
    expect(() => rumblePad(null)).not.toThrow();
    expect(fail).toHaveBeenCalledTimes(1);
  });
  it('abgelehntes Promise und werfender Actuator: keine Ausnahme', async () => {
    const fail = vi.fn();
    rumblePad({ vibrationActuator: { playEffect: () => Promise.reject(new Error('x')) } }, fail);
    rumblePad({ vibrationActuator: { playEffect: () => { throw new Error('y'); } } }, fail);
    await Promise.resolve();
    await Promise.resolve();
    expect(fail).toHaveBeenCalledTimes(2);
  });
  it('mit Actuator: dual-rumble wird gespielt', () => {
    const playEffect = vi.fn(() => Promise.resolve('complete'));
    rumblePad({ vibrationActuator: { playEffect } });
    expect(playEffect).toHaveBeenCalledWith('dual-rumble', expect.objectContaining({ duration: expect.any(Number) }));
  });
});
