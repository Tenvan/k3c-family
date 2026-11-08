import { beforeEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../../core/texts';
import type { Player } from '../../model/types';
import { slotBindings } from '../../input/slotBindings';
import { menuEntries, skillName } from './skillMenuLogic';
import { entryText, menuText, slotBar, slotKeys } from './skillMenuText';

const player = (over: Partial<Player> = {}): Player =>
  ({ id: 0, index: 0, x: 0, vx: 0, facing: 1, gold: 0, hp: 100, maxHp: 100, respawnIn: 0, payCooldown: 0, paying: false, payKey: null, payAmount: 0, points: 0, actions: [], ...over }) as Player;
const ausgestattet = player({ skills: ['taunt', 'fireball'], slots: ['fireball', 'taunt', '', ''], cooldowns: [0, 4.2, 0, 0] });

beforeEach(() => setLanguage('de'));

describe('Skill-Leiste ohne Tasten (B-755/AC-01)', () => {
  it('nennt Skill und Abklingzeit je Slot, keinen Tastennamen', () => {
    expect(slotBar(ausgestattet)).toBe(`${skillName('fireball')} bereit  ·  ${skillName('taunt')} 5 s  ·  –  ·  –`);
    for (const d of ['pad', 'keyboard', 'touch'] as const) {
      const text = slotBar(ausgestattet);
      for (const b of slotBindings(d).slice(1, 5)) expect(text).not.toContain(`${b.label} `);
    }
  });
});

describe('Skillmenü: Taste vor dem Skill (B-755/AC-02)', () => {
  it('slotKeys: Skill → Taste seines Slots je Gerät, aus slotBindings', () => {
    for (const d of ['pad', 'keyboard', 'touch'] as const) {
      const keys = slotKeys(ausgestattet, d), labels = slotBindings(d);
      expect(keys.get('fireball')).toBe(labels[1]!.label);
      expect(keys.get('taunt')).toBe(labels[2]!.label);
      expect(keys.size).toBe(2);
    }
  });

  it('die Zeile eines belegten Skills beginnt mit der Taste, dann Linie und Skillname', () => {
    const e = menuEntries(ausgestattet).find((x) => x.id === 'fireball')!;
    const zeile = entryText(e, false, 'Q');
    expect(zeile.trimStart().startsWith('[Q] ')).toBe(true);
    expect(zeile.indexOf('[Q]')).toBeLessThan(zeile.indexOf(skillName('fireball')));
    expect(entryText(e, false)).not.toContain('[');
  });

  it('menuText zeigt die Taste des Geräts vor dem Skill, unbelegte Skills ohne', () => {
    const cursor = menuEntries(ausgestattet).findIndex((x) => x.id === 'fireball');
    const zeilen = menuText(ausgestattet, cursor, 'keyboard').split('\n');
    const feuer = zeilen.find((z) => z.includes(skillName('fireball')))!;
    expect(feuer).toMatch(/^▶ \[Q\] /);
    const ohne = zeilen.filter((z) => z.startsWith('   ') && !z.includes('['));
    expect(ohne.length).toBeGreaterThan(0);
  });
});
