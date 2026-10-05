import { describe, expect, it } from 'vitest';
import { KEY_ACTIONS, PAD, PAD_ACTIONS, SLOT_ACTIONS, heldSkill, slotBindings, type Device } from './slotBindings';

const DEVICES: Device[] = ['pad', 'keyboard', 'touch'];

describe('slotBindings', () => {
  it('belegt Schlag, Skill-Slot 1–4 und Skill-Menü nach Q06', () => {
    expect(slotBindings('pad').map((b) => b.key)).toEqual(['X', 'LB', 'RB', 'LT', 'UP', 'DOWN']);
    expect(slotBindings('keyboard').map((b) => b.key)).toEqual(['E', 'Q', 'R', 'T', 'Z', 'K']);
    expect(slotBindings('pad').map((b) => b.label)).toEqual(['X', 'LB', 'RB', 'LT', 'D-Pad ↑', 'D-Pad ↓']);
  });

  it.each(DEVICES)('%s: jede Aktion genau einmal, keine Taste doppelt, jede mit Anzeigetext', (device) => {
    const bindings = slotBindings(device);
    expect(bindings.map((b) => b.action)).toEqual(SLOT_ACTIONS);
    expect(new Set(bindings.map((b) => b.key)).size).toBe(bindings.length);
    expect(bindings.every((b) => b.label.length > 0)).toBe(true);
  });

  it('Controller: keine Taste doppelt, B nie, View und Menu nie allein (Menu kurz = Optionen)', () => {
    const buttons = Object.values(PAD_ACTIONS);
    expect(new Set(buttons).size).toBe(buttons.length);
    expect(buttons).not.toContain(PAD.B);
    expect(buttons).not.toContain(PAD.VIEW);
    expect(Object.entries(PAD_ACTIONS).filter(([, b]) => b === PAD.MENU)).toEqual([['pause', PAD.MENU]]);
    expect(buttons).not.toContain(PAD.LEFT); // D-Pad links/rechts läuft
    expect(buttons).not.toContain(PAD.RIGHT);
    expect(buttons).not.toContain(PAD.RT); // Sprint
  });

  it('Tastatur: keine Taste doppelt, Lauf- und Sprinttasten frei', () => {
    const keys = Object.values(KEY_ACTIONS);
    expect(new Set(keys).size).toBe(keys.length);
    for (const k of ['A', 'D', 'LEFT', 'RIGHT', 'SHIFT']) expect(keys).not.toContain(k);
  });

  it('heldSkill: gehaltener Slot 1–4, der niedrigste gewinnt, sonst 0', () => {
    expect(heldSkill(() => false)).toBe(0);
    expect(heldSkill((a) => a === 'skill3')).toBe(3);
    expect(heldSkill((a) => a === 'skill2' || a === 'skill4')).toBe(2);
    expect(heldSkill((a) => a === 'attack' || a === 'skillMenu')).toBe(0);
  });
});
