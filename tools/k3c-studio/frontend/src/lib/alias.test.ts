import { describe, expect, it } from 'vitest';
import { UNIT_PX } from '@game/core/constants';
import sprites from '@data/sprites.json';

describe('Aliase des Studios (B-459)', () => {
  it('liest Spiel-Code über @game und Daten über @data', () => {
    expect(UNIT_PX).toBe(32);
    expect(Object.keys(sprites)).toContain('sheets');
  });
});
