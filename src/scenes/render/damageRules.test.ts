import { describe, expect, it } from 'vitest';
import { damageStage, showRubble } from './damageRules';

const site = (hp: number, maxHp = 90, state: 'built' | 'unpaid' = 'built', ruin?: boolean) => ({ state, hp, maxHp, ruin });

describe('damageStage (WZ14.6)', () => {
  it('Stufen nach HP-Anteil', () => {
    expect(damageStage(site(90))).toBe(0);
    expect(damageStage(site(89))).toBe(1);
    expect(damageStage(site(60))).toBe(1);
    expect(damageStage(site(59))).toBe(2);
    expect(damageStage(site(30))).toBe(2);
    expect(damageStage(site(29))).toBe(3);
    expect(damageStage(site(1))).toBe(3);
  });

  it('nicht gebaut oder ohne MaxHP: keine Überlagerung', () => {
    expect(damageStage(site(0, 90, 'unpaid'))).toBe(0);
    expect(damageStage(site(0, 0))).toBe(0);
  });

  it('nach voller Reparatur verschwindet die Überlagerung', () => {
    expect(damageStage(site(23))).toBe(3);
    expect(damageStage(site(90))).toBe(0);
  });
});

describe('showRubble (WZ14.6)', () => {
  it('nur an einer Ruine unter 50 % HP', () => {
    expect(showRubble(site(23, 90, 'built', true))).toBe(true);
    expect(showRubble(site(45, 90, 'built', true))).toBe(false);
    expect(showRubble(site(23, 90, 'built'))).toBe(false);
    expect(showRubble(site(0, 90, 'unpaid', true))).toBe(false);
  });
});
