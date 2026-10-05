import { describe, expect, it } from 'vitest';
import type { Player, PlayerAction } from '../model/types';
import type { SlotAction } from '../input/slotBindings';
import { SkillMenus, menuEntries, slotViews, type MenuInput } from './skillMenuLogic';

const player = (index: number, over: Partial<Player> = {}): Player =>
  ({ id: index, index, x: 0, vx: 0, facing: 1, gold: 0, hp: 100, maxHp: 100, respawnIn: 0, payCooldown: 0, paying: false, payKey: null, payAmount: 0, points: 0, actions: [], ...over }) as Player;

const learn: PlayerAction[] = [{ action: 'learn' }];

class Input implements MenuInput {
  x = 0;
  pressed = new Set<string>();
  justPressed = (a: 'confirm' | SlotAction) => this.pressed.has(a);
  moveX = () => this.x;
  tap(...a: string[]) {
    this.pressed = new Set(a);
    return this;
  }
}

describe('menuEntries', () => {
  it('alle Skills nach Linie und Tier; lernbar nur mit learn in actions und freien Punkten', () => {
    const none = menuEntries(player(0));
    expect(none[0]).toMatchObject({ line: 'tank', tier: 1 });
    expect(none.every((e) => !e.learnable)).toBe(true);
    const tiers = none.filter((e) => e.line === 'mage').map((e) => e.tier);
    expect(tiers).toEqual([...tiers].sort());

    const p = player(0, { points: 2, actions: learn, skills: ['taunt'] });
    const entries = menuEntries(p);
    expect(entries.find((e) => e.id === 'taunt')).toMatchObject({ learned: true, learnable: false });
    expect(entries.find((e) => e.id === 'fireball')).toMatchObject({ learned: false, learnable: true });
    expect(entries.some((e) => e.id === 'respec')).toBe(false);
  });

  it('Respec nur, wenn der Server es anbietet (letzter Eintrag)', () => {
    const entries = menuEntries(player(0, { actions: [{ action: 'respec' }] }));
    expect(entries.at(-1)).toMatchObject({ id: 'respec', learnable: true });
  });
});

describe('slotViews', () => {
  it('vier Slots, fehlende Felder = frei und bereit', () => {
    expect(slotViews(player(0))).toEqual(Array.from({ length: 4 }, () => ({ skill: '', cooldown: 0 })));
    expect(slotViews(player(0, { slots: ['taunt', '', '', ''], cooldowns: [12.5, 0, 0, 0] }))[0]).toEqual({ skill: 'taunt', cooldown: 12.5 });
  });
});

describe('SkillMenus mit 2 Spielern', () => {
  it('jeder öffnet sein Menü, wählt und lernt; der andere spielt weiter', () => {
    const menus = new SkillMenus();
    const sent: string[] = [];
    const client = { learn: (s: number, k: string) => void sent.push(`learn ${s} ${k}`), respec: (s: number) => void sent.push(`respec ${s}`) };
    const a = new Input();
    const b = new Input();
    const players = [player(0, { points: 1, actions: learn }), player(1, { points: 1, actions: [...learn, { action: 'respec' }] })];
    const cmds = () => [0, 1].map((slot) => ({ slot, moveX: 1, sprint: true, pay: true, attack: true as const }));
    const route = () => menus.route(cmds(), [a, b], (s) => players[s], client);

    a.tap('skillMenu');
    expect(route()).toEqual([{ slot: 0, moveX: 0, sprint: false, pay: false }, cmds()[1]]);
    expect(menus.isOpen(0)).toBe(true);
    expect(menus.isOpen(1)).toBe(false);

    a.tap();
    a.x = 1; // ein Schritt je Auslenkung, Halten zählt nicht weiter
    route();
    route();
    expect(menus.cursor(0)).toBe(1);
    a.x = 0;
    a.tap('confirm');
    route();
    expect(sent).toEqual([`learn 0 ${menuEntries(players[0]!)[1]!.id}`]);

    b.tap('skillMenu');
    route();
    b.tap();
    b.x = -1; // links vom ersten Eintrag: zum letzten (Respec)
    route();
    b.tap('confirm');
    route();
    expect(sent.at(-1)).toBe('respec 1');

    a.tap('skillMenu');
    route();
    expect(menus.isOpen(0)).toBe(false);
  });

  it('nicht lernbarer Eintrag sendet nichts; Slot ohne Eingabe schließt sein Menü', () => {
    const menus = new SkillMenus();
    const sent: string[] = [];
    const client = { learn: () => void sent.push('learn'), respec: () => void sent.push('respec') };
    const a = new Input();
    menus.route([{ slot: 0, moveX: 0, sprint: false, pay: false }], [a.tap('skillMenu')], () => player(0), client);
    menus.route([{ slot: 0, moveX: 0, sprint: false, pay: false }], [a.tap('confirm')], () => player(0), client);
    expect(sent).toEqual([]);
    menus.route([], [null], () => undefined, client);
    expect(menus.isOpen(0)).toBe(false);
  });
});
