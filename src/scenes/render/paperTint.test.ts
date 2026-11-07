import { describe, expect, it, vi } from 'vitest';
vi.mock('phaser', () => ({ default: { Textures: { FilterMode: { NEAREST: 0 } }, Display: {} } })); // nur Konstanten, keine Laufzeit

import { ENEMY_SPRITES, PLAYER_SPRITES, TROOP_SPRITES } from './sprites';

describe('Einfärbung der Papier-Assets (Review US1, W4)', () => {
  it('Truppen und Gegner ohne alte Einfärbung, Spieler 3/4 behalten sie', () => {
    for (const s of [...Object.values(TROOP_SPRITES), ...Object.values(ENEMY_SPRITES)]) expect(s.tint, s.sheet).toBeUndefined();
    expect(PLAYER_SPRITES[2]!.tint).toBeDefined();
  });
});
