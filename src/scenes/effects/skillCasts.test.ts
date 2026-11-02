import { describe, expect, it } from 'vitest';
import { UNIT_PX } from '../../core/constants';
import { MONARCH } from '../../model/data';
import type { GameEvent, Player } from '../../model/types';
import { LINE_COLORS } from './effects';
import { SKILL_LOOKS } from './skillLooks';
import { SkillCasts } from './skillCasts';

const player = (index: number, over: Partial<Player> = {}): Player =>
  ({ id: 100 + index, index, x: 10 + index, vx: 0, facing: 1, gold: 0, hp: 100, maxHp: 100, respawnIn: 0, payCooldown: 0, paying: false, payKey: null, payAmount: 0, points: 0, actions: [], ...over }) as Player;

const cast = (from: number, skill: string, x: number, radius: number): GameEvent => ({ type: 'skillCast', from, skill, x, radius });

describe('SkillCasts (SK2.3, B-503, B-674)', () => {
  it('B-674/AC-03: Flächenzauber am gemeldeten Wirkort mit dem Radius des Skills, nicht am Monarchen', () => {
    const casts = new SkillCasts();
    const players = [player(0, { x: 0 })];
    const fx = casts.diff(players, [cast(100, 'fireball', 8, 3)]);
    expect(fx).toEqual([expect.objectContaining({ kind: 'skill', x: 8 * UNIT_PX, look: { ...SKILL_LOOKS.fireball, radius: 3 * UNIT_PX } })]);
  });

  it('Skill ohne Fläche (radius 0) behält Größe seines Looks am gemeldeten Ort; je Spieler getrennt', () => {
    const casts = new SkillCasts();
    const players = [player(0), player(1)];
    const fx = casts.diff(players, [cast(100, 'taunt', 10, 0), cast(101, 'meteor', 25, 15)]);
    expect(fx).toEqual([
      expect.objectContaining({ x: 10 * UNIT_PX, look: SKILL_LOOKS.taunt }),
      expect.objectContaining({ x: 25 * UNIT_PX, look: { ...SKILL_LOOKS.meteor, radius: 15 * UNIT_PX } }),
    ]);
  });

  it('Abklingzeit allein zeigt nichts mehr, unbekannter Skill im Ereignis wird ignoriert', () => {
    const heard: unknown[] = [];
    const casts = new SkillCasts((c) => heard.push(c));
    const mage = { skills: ['fireball'], slots: ['fireball'] };
    casts.diff([player(0, { ...mage, cooldowns: [0] })]);
    expect(casts.diff([player(0, { ...mage, cooldowns: [8] })])).toEqual([]);
    expect(casts.diff([player(0)], [cast(100, 'gibtEsNicht', 5, 2)])).toEqual([]);
    expect(heard).toEqual([]);
  });

  it('ein neu gelernter passiver Skill zeigt den Ring, ein aktiver nicht; reset vergisst den Stand', () => {
    const casts = new SkillCasts();
    casts.diff([player(0)]);
    expect(casts.diff([player(0, { skills: ['heal'] })])).toEqual([]);
    expect(casts.diff([player(0, { skills: ['heal', 'healingAura'] })])).toEqual([expect.objectContaining({ look: SKILL_LOOKS.healingAura })]);
    casts.reset();
    expect(casts.diff([player(0, { skills: ['heal', 'healingAura', 'thickSkin'] })])).toEqual([]);
  });

  it('meldet Wirken (Ort aus dem Ereignis) und Lernen je Spieler mit Skill-Id (SO8.2)', () => {
    const heard: unknown[] = [];
    const casts = new SkillCasts((c) => heard.push(c));
    casts.diff([player(0), player(1)]);
    casts.diff([player(0), player(1, { skills: ['regeneration'] })], [cast(100, 'heal', 10, 0)]);
    expect(heard).toEqual([
      { index: 0, skill: 'heal', x: 10 },
      { index: 1, skill: 'regeneration', x: 11 },
    ]);
  });

  it('jede Linie aus data/monarch.json mit Skills hat eine Farbe (Rückfall je Klasse für alle Skills)', () => {
    const lines = new Set(Object.values(MONARCH.skills as Record<string, { line: string }>).map((s) => s.line));
    for (const line of lines) expect(LINE_COLORS[line], line).toBeTypeOf('number');
  });
});
