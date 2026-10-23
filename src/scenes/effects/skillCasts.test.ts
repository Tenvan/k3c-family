import { describe, expect, it } from 'vitest';
import { UNIT_PX } from '../../core/constants';
import { MONARCH } from '../../model/data';
import type { Player } from '../../model/types';
import { LINE_COLORS } from './effects';
import { SkillCasts } from './skillCasts';

const player = (index: number, over: Partial<Player> = {}): Player =>
  ({ id: index, index, x: 10 + index, vx: 0, facing: 1, gold: 0, hp: 100, maxHp: 100, respawnIn: 0, payCooldown: 0, paying: false, payKey: null, payAmount: 0, points: 0, actions: [], ...over }) as Player;

describe('SkillCasts (SK2.3, B-503/AC-01)', () => {
  it('Anstieg der Abklingzeit ergibt einen Ring in der Farbe der Linie, je Spieler getrennt', () => {
    const casts = new SkillCasts();
    const tank = { skills: ['taunt'], slots: ['taunt'] };
    const mage = { skills: ['fireball'], slots: ['fireball'] };
    expect(casts.diff([player(0, { ...tank, cooldowns: [0] }), player(1, { ...mage, cooldowns: [0] })])).toEqual([]);
    const fx = casts.diff([player(0, { ...tank, cooldowns: [0] }), player(1, { ...mage, cooldowns: [8] })]);
    expect(fx).toEqual([expect.objectContaining({ kind: 'skill', x: 11 * UNIT_PX, color: LINE_COLORS.mage })]);
    expect(casts.diff([player(0, { ...tank, cooldowns: [15] }), player(1, { ...mage, cooldowns: [7.9] })])).toEqual([
      expect.objectContaining({ x: 10 * UNIT_PX, color: LINE_COLORS.tank }),
    ]);
  });

  it('ein neu gelernter passiver Skill zeigt den Ring, ein aktiver erst beim Wirken; reset vergisst den Stand', () => {
    const casts = new SkillCasts();
    casts.diff([player(0)]);
    expect(casts.diff([player(0, { skills: ['heal'] })])).toEqual([]);
    expect(casts.diff([player(0, { skills: ['heal', 'healingAura'] })])).toEqual([expect.objectContaining({ color: LINE_COLORS.healer })]);
    casts.reset();
    expect(casts.diff([player(0, { skills: ['heal', 'healingAura', 'thickSkin'], slots: ['heal'], cooldowns: [9] })])).toEqual([]);
  });

  it('jede Linie aus data/monarch.json mit Skills hat eine Farbe (Rückfall je Klasse für alle Skills)', () => {
    const lines = new Set(Object.values(MONARCH.skills as Record<string, { line: string }>).map((s) => s.line));
    for (const line of lines) expect(LINE_COLORS[line], line).toBeTypeOf('number');
  });
});
