import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import { UNIT_PX } from '../core/constants';
import type { Enemy } from '../model/types';
import { bossBar, eventBanner, phaseText, warnCircles, WARN_WINDOW, type KampfWorld } from './kampfAnzeige';

beforeEach(() => setLanguage('de'));
afterEach(() => setLanguage('de'));

const enemy = (over: Partial<Enemy>): Enemy => ({ id: 1, kind: 'goblin', x: 10, hp: 10, maxHp: 10, ...over }) as Enemy;
const world = (over: Partial<KampfWorld> = {}): KampfWorld => ({ enemies: [], cycleSpeed: 1, ...over });
const endBoss = (over: Partial<Enemy> = {}) => enemy({ id: 7, kind: 'crystalHeartWarden', boss: true, hp: 50, maxHp: 100, phase: 2, ...over });

describe('Boss-Leiste (K5.1 AC-01)', () => {
  it('Anteil HP/Maximum, Name und Phasen-Text', () => {
    expect(bossBar(world({ enemies: [enemy({}), endBoss()] }))).toEqual({ id: 7, share: 0.5, name: 'Kristallherz-Wächter', phase: 'Phase 2 von 3' });
  });

  it('0 und Maximum, Maximum 0 ohne Division durch null', () => {
    expect(bossBar(world({ enemies: [endBoss({ hp: 0 })] }))?.share).toBe(0);
    expect(bossBar(world({ enemies: [endBoss({ hp: 100 })] }))?.share).toBe(1);
    expect(bossBar(world({ enemies: [endBoss({ maxHp: 0 })] }))?.share).toBe(0);
  });

  it('Endboss vor Miniboss, nur eine Leiste', () => {
    const mini = enemy({ id: 3, kind: 'goblinLeader', boss: true });
    expect(bossBar(world({ enemies: [mini, endBoss()] }))?.id).toBe(7);
    expect(bossBar(world({ enemies: [mini] }))).toEqual({ id: 3, share: 1, name: 'Goblin-Anführer', phase: '' });
  });

  it('unbekannte Boss-ID → ID als Name; englisch übersetzt', () => {
    expect(bossBar(world({ enemies: [enemy({ kind: 'neu', boss: true })] }))?.name).toBe('neu');
    setLanguage('en');
    expect(bossBar(world({ enemies: [endBoss()] }))).toMatchObject({ name: 'Crystal Heart Warden', phase: 'Phase 2 of 3' });
  });

  it('kein Boss oder Zustand ohne die Felder → null', () => {
    expect(bossBar(world({ enemies: [enemy({})] }))).toBeNull();
    expect(bossBar({} as KampfWorld)).toBeNull();
  });
});

describe('Phasen-Text', () => {
  it('Endboss „Phase n von m“, Miniboss und fehlende Phase leer', () => {
    expect(phaseText(endBoss({ phase: 3 }))).toBe('Phase 3 von 3');
    expect(phaseText(enemy({ kind: 'goblinLeader', boss: true }))).toBe('');
    expect(phaseText(endBoss({ phase: undefined }))).toBe('');
  });
});

describe('Warnkreis', () => {
  it('Mitte und Radius in Pixeln, Anteil der Restzeit', () => {
    const warn = { x: 20, r: 5, in: WARN_WINDOW / 2 };
    expect(warnCircles(world({ enemies: [endBoss({ warn })] }))).toEqual([{ x: 20 * UNIT_PX, r: 5 * UNIT_PX, urgency: 0.5 }]);
  });

  it('Restzeit über dem Fenster → 0, beim Schlag → 1', () => {
    expect(warnCircles(world({ enemies: [endBoss({ warn: { x: 0, r: 1, in: WARN_WINDOW * 4 } })] }))[0]?.urgency).toBe(0);
    expect(warnCircles(world({ enemies: [endBoss({ warn: { x: 0, r: 1, in: 0 } })] }))[0]?.urgency).toBe(1);
  });

  it('ohne Warnkreis oder ohne Felder → leer', () => {
    expect(warnCircles(world({ enemies: [endBoss()] }))).toEqual([]);
    expect(warnCircles({} as KampfWorld)).toEqual([]);
  });
});

describe('Event-Banner', () => {
  it('Text je Event mit Restzeit (Zeitraffer wie die Uhr)', () => {
    expect(eventBanner(world({ nightEvents: [{ id: 'fullMoon', secondsLeft: 90 }] }))).toBe('Vollmond  ·  1:30');
    expect(eventBanner(world({ nightEvents: [{ id: 'bloodMoon', secondsLeft: 120 }], cycleSpeed: 2 }))).toBe('Blutmond  ·  1:00');
  });

  it('zwei Events in einer Nacht, Händler-Überfall ohne Restzeit', () => {
    const both = [{ id: 'fullMoon', secondsLeft: 61 }, { id: 'bloodMoon', secondsLeft: 61 }];
    expect(eventBanner(world({ nightEvents: both }))).toBe('Vollmond  ·  1:01\nBlutmond  ·  1:01');
    const merchant = { resource: 'wood', leaves: 3, raid: true } as KampfWorld['merchant'];
    expect(eventBanner(world({ merchant }))).toBe('Händler-Überfall');
  });

  it('unbekanntes Event → ID; kein Event oder ohne Felder → null', () => {
    expect(eventBanner(world({ nightEvents: [{ id: 'neu', secondsLeft: 5 }] }))).toBe('neu  ·  0:05');
    expect(eventBanner(world({ nightEvents: [] }))).toBeNull();
    expect(eventBanner(world({ merchant: { resource: 'wood', leaves: 3 } }))).toBeNull();
    expect(eventBanner({} as KampfWorld)).toBeNull();
  });
});
