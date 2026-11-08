import { describe, expect, it } from 'vitest';
import { KEYBOARD_1, keyMatches, layoutKeys, type KeyboardLayout } from './keyboardLayouts';
import { SLOT_ACTIONS } from './slotBindings';

const ids = (l: KeyboardLayout): string[] => layoutKeys(l).map((k) => ('code' in k ? `code:${k.code}` : `key:${k.keyCode}`));

describe('Eine Tastatur-Belegung (Leertaste, 🧑 2026-11-08)', () => {
  it('Laufen, Sprint, Beitreten und alle Slot-Aktionen sind belegt', () => {
    expect(KEYBOARD_1.left.length).toBeGreaterThan(0);
    expect(KEYBOARD_1.right.length).toBeGreaterThan(0);
    expect(KEYBOARD_1.sprint.length).toBeGreaterThan(0);
    for (const a of ['confirm', ...SLOT_ACTIONS] as const) expect(KEYBOARD_1.actions[a]?.length ?? 0).toBeGreaterThan(0);
  });

  it('keine Taste ist doppelt; Pause und Vollbild liegen auf der Tastatur', () => {
    expect(new Set(ids(KEYBOARD_1)).size).toBe(ids(KEYBOARD_1).length);
    expect(KEYBOARD_1.actions.pause?.length).toBe(1);
    expect(KEYBOARD_1.actions.fullscreen?.length).toBe(1);
  });

  it('keyMatches: Ort (code) oder keyCode', () => {
    expect(keyMatches({ code: 'ControlRight' }, { keyCode: 17, code: 'ControlRight' })).toBe(true);
    expect(keyMatches({ code: 'ControlRight' }, { keyCode: 17, code: 'ControlLeft' })).toBe(false);
    expect(keyMatches({ keyCode: 13 }, { keyCode: 13, code: 'NumpadEnter' })).toBe(true);
    expect(keyMatches({ keyCode: 90 }, { keyCode: 90, code: 'KeyY' })).toBe(true); // QWERTZ: Z liegt auf KeyY
  });
});
