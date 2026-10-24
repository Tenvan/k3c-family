import { describe, expect, it } from 'vitest';
import { MONARCH } from '../../model/data';
import { EFFECT_CONFIG, LINE_COLORS, MAX_LIVE_EFFECTS } from './effects';
import { SKILL_LOOKS, particleOffset, skillLook } from './skillLooks';

const SKILLS = MONARCH.skills as Record<string, { line: string }>;

describe('Effekt je Skill (B-578/AC-01)', () => {
  it('jeder Skill aus data/monarch.json hat einen eigenen Effekt, jede Linie den Ring als Rückfall', () => {
    for (const [id, s] of Object.entries(SKILLS)) {
      expect(SKILL_LOOKS[id], id).toBeDefined();
      expect(LINE_COLORS[s.line], s.line).toBeTypeOf('number');
    }
    expect(SKILL_LOOKS.backstab).toBeDefined();
    expect(SKILL_LOOKS.smokeBomb).toBeDefined();
  });

  it('kein Look für einen Skill, den es nicht gibt; jeder Look passt unter die Höchstzahl lebender Effekte', () => {
    for (const [id, l] of Object.entries(SKILL_LOOKS)) {
      expect(SKILLS[id], id).toBeDefined();
      expect(1 + l.particles, id).toBeLessThanOrEqual(MAX_LIVE_EFFECTS / 4); // vier Monarchen wirken zugleich
    }
  });

  it('ohne eigenen Look: Ring der Linie mit den Werten von EFFECT_CONFIG.skill; unbekannt ergibt undefined', () => {
    const saved = SKILL_LOOKS.taunt;
    delete SKILL_LOOKS.taunt;
    try {
      const { radius, durationMs, particles } = EFFECT_CONFIG.skill;
      expect(skillLook('taunt')).toEqual({ color: LINE_COLORS.tank, radius, durationMs, particles, motion: 'burst' });
    } finally {
      SKILL_LOOKS.taunt = saved;
    }
    expect(skillLook('gibtEsNicht')).toBeUndefined();
    expect(skillLook('fireball')).toBe(SKILL_LOOKS.fireball);
  });

  it('Bewegungen: rise steigt, fall sinkt, wave verteilt zur Seite', () => {
    expect(particleOffset('rise', 0, 4, 30)[1]).toBeLessThan(0);
    expect(particleOffset('fall', 0, 4, 30)[1]).toBeGreaterThan(0);
    const wave = [0, 1, 2, 3].map((i) => particleOffset('wave', i, 4, 30)[0]);
    expect(wave.filter((dx) => dx < 0)).toHaveLength(2);
  });
});
