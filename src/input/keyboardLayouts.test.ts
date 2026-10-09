import { describe, expect, it } from 'vitest';
import { KEYBOARD_1, KEYBOARD_2, keyMatches, layoutKeys, type KeyboardLayout } from './keyboardLayouts';
import { SLOT_ACTIONS } from './slotBindings';

const ids = (l: KeyboardLayout): string[] => layoutKeys(l).map((k) => ('code' in k ? `code:${k.code}` : `key:${k.keyCode}`));
const keyCodes = (l: KeyboardLayout): number[] => layoutKeys(l).flatMap((k) => ('keyCode' in k ? [k.keyCode] : []));

describe('Zwei Spieler an einer Tastatur (B-316)', () => {
  it('die Layouts haben keine gemeinsame Taste', () => {
    const one = ids(KEYBOARD_1);
    for (const id of ids(KEYBOARD_2)) expect(one).not.toContain(id);
    // Strg (17), Enter (13) und Pfeile (37/39) gehören nur Spieler 2, auch über den keyCode
    for (const code of [13, 17, 37, 39]) expect(keyCodes(KEYBOARD_1)).not.toContain(code);
    expect(ids(KEYBOARD_2)).toEqual(expect.arrayContaining(['code:ArrowLeft', 'code:ArrowRight']));
  });

  it('Laufen, Sprint, Beitreten und alle Slot-Aktionen sind in beiden belegt', () => {
    for (const l of [KEYBOARD_1, KEYBOARD_2]) {
      expect(l.left.length).toBeGreaterThan(0);
      expect(l.right.length).toBeGreaterThan(0);
      expect(l.sprint.length).toBeGreaterThan(0);
      for (const a of ['confirm', ...SLOT_ACTIONS] as const) expect(l.actions[a]?.length ?? 0).toBeGreaterThan(0);
    }
  });

  it('keine Taste ist in einem Layout doppelt', () => {
    for (const l of [KEYBOARD_1, KEYBOARD_2]) expect(new Set(ids(l)).size).toBe(ids(l).length);
  });

  it('Spieler 2 belegt keine reservierte Taste (Pos1, F, Esc, Ö, Ä), Pause und Vollbild bleiben bei Spieler 1', () => {
    for (const reserved of [36, 70, 27, 192, 222]) expect(keyCodes(KEYBOARD_2)).not.toContain(reserved);
    expect(KEYBOARD_2.actions.pause).toBeUndefined();
    expect(KEYBOARD_2.actions.fullscreen).toBeUndefined();
    expect(KEYBOARD_1.actions.pause?.length).toBe(1);
  });

  it('Ziffernblock und Strg rechts zählen nach Ort, Enter in beiden Formen', () => {
    expect(keyMatches({ code: 'ControlRight' }, { keyCode: 17, code: 'ControlRight' })).toBe(true);
    expect(keyMatches({ code: 'ControlRight' }, { keyCode: 17, code: 'ControlLeft' })).toBe(false);
    expect(keyMatches({ code: 'Numpad1' }, { keyCode: 35, code: 'Numpad1' })).toBe(true); // NumLock aus
    expect(keyMatches({ keyCode: 13 }, { keyCode: 13, code: 'NumpadEnter' })).toBe(true);
    expect(keyMatches({ keyCode: 90 }, { keyCode: 90, code: 'KeyY' })).toBe(true); // QWERTZ: Z liegt auf KeyY
    // NumLock aus: Ziffernblock 4 meldet keyCode 37 – Skill 4 von Spieler 2 lässt ihn nicht nach links laufen (Review PL1.4)
    const numpad4 = { keyCode: 37, code: 'Numpad4' };
    expect(KEYBOARD_2.left.some((s) => keyMatches(s, numpad4))).toBe(false);
    expect(KEYBOARD_2.actions.skill4?.some((s) => keyMatches(s, numpad4))).toBe(true);
  });
});
