import { describe, expect, it } from 'vitest';
import { DEV_ACTIONS } from './debugActions';
import { CHEAT_CSS, diagGesture, focusEdges, focusStep, isGamepadKey, KEY_FOCUS, muteFocused, PAD_FOCUS, type FocusState } from './debugOverlayPanel';

const on: FocusState = { index: 0, seat: 0 };

describe('focusStep (B-179, B-231)', () => {
  it('bei geschlossenem Dialog gehören die Tasten dem Spiel', () => {
    expect(focusStep(on, { A: true, DOWN: true }, false, 1)).toEqual({ state: on, fire: null });
  });

  it('D-Pad wählt die Zeile (rundum), A löst sie aus', () => {
    expect(focusStep(on, { DOWN: true }, true, 1).state.index).toBe(1);
    expect(focusStep(on, { UP: true }, true, 1).state.index).toBe(DEV_ACTIONS.length - 1);
    expect(focusStep({ ...on, index: 4 }, { A: true }, true, 1).fire).toBe(4);
  });

  it('links/rechts wechselt den lokalen Spieler (2 Spieler am Gerät)', () => {
    expect(focusStep(on, { RIGHT: true }, true, 2).state.seat).toBe(1);
    expect(focusStep({ ...on, seat: 1 }, { RIGHT: true }, true, 2).state.seat).toBe(0);
    expect(focusStep(on, { LEFT: true }, true, 1).state.seat).toBe(0);
  });

  it('B, X, View, Menu und die Schultertasten gehören nicht zur Bedienung', () => {
    expect(Object.values(PAD_FOCUS)).not.toContain(1); // B
    expect(Object.values(PAD_FOCUS)).not.toContain(2); // X
    expect(Object.values(PAD_FOCUS)).not.toContain(8); // View
    expect(Object.values(PAD_FOCUS)).not.toContain(9); // Menu
    expect(Object.values(PAD_FOCUS)).not.toContain(5); // RB (Geste)
  });
});

describe('muteFocused', () => {
  const cmds = [
    { slot: 0, moveX: 1, sprint: true, pay: true },
    { slot: 1, moveX: -1, sprint: false, pay: true },
  ];

  it('im Fokus stehen alle Spieler des Geräts still, auch die an der Tastatur (B-317)', () => {
    expect(muteFocused(cmds, true)).toEqual([
      { slot: 0, moveX: 0, sprint: false, pay: false },
      { slot: 1, moveX: 0, sprint: false, pay: false },
    ]);
  });

  it('ohne Fokus unverändert', () => {
    expect(muteFocused(cmds, false)).toBe(cmds);
  });
});

describe('Ö und verborgener Dialog (B-192)', () => {
  it('verborgener Dialog hat display:none, keine spätere Regel am Wurzel-Element setzt display', () => {
    const rules = [...CHEAT_CSS.matchAll(/([^{}]+)\{([^}]*)\}/g)].map((m) => ({ sel: m[1].trim(), body: m[2] }));
    const hidden = rules.findIndex((r) => r.sel === '.k3c-cheat[hidden]');
    expect(hidden).toBeGreaterThan(-1);
    expect(rules[hidden].body).toBe('display:none');
    const later = rules.slice(hidden + 1).filter((r) => !r.sel.includes(' ') && r.sel.startsWith('.k3c-cheat') && /display\s*:/.test(r.body));
    expect(later).toEqual([]);
  });

  it('Ö schließt bei offenem Dialog zuerst den Dialog, die Info-Zeilen bleiben', () => {
    expect(diagGesture(true, true)).toEqual({ shown: true, open: false });
    expect(diagGesture(false, true)).toEqual({ shown: false, open: false });
  });

  it('Ö schaltet ohne Dialog die Info-Zeilen', () => {
    expect(diagGesture(false, false)).toEqual({ shown: true, open: false });
    expect(diagGesture(true, false)).toEqual({ shown: false, open: false });
  });
});

describe('Tastatur, Klick und Xbox-Tasten im Cheat-Dialog (B-317)', () => {
  const none = (): boolean => false;
  const pad = (b: number) => focusEdges((x) => x === b, none);
  const key = (k: number) => focusEdges(none, (x) => x === k);

  it('Pfeile und Leertaste/Enter wirken wie D-Pad und A', () => {
    expect(key(40)).toEqual(pad(PAD_FOCUS.DOWN));
    expect(key(38)).toEqual(pad(PAD_FOCUS.UP));
    expect(key(37)).toEqual(pad(PAD_FOCUS.LEFT));
    expect(key(39)).toEqual(pad(PAD_FOCUS.RIGHT));
    expect(key(32)).toEqual(pad(PAD_FOCUS.A));
    expect(key(13)).toEqual(pad(PAD_FOCUS.A));
    expect(focusStep(on, key(40), true, 1).state.index).toBe(1);
    expect(focusStep(on, key(39), true, 2).state.seat).toBe(1);
    expect(focusStep({ ...on, index: 2 }, key(32), true, 1).fire).toBe(2);
    expect(focusStep({ ...on, index: 2 }, key(13), true, 1).fire).toBe(2);
  });

  it('jede Taste wird genau einmal abgefragt (Phaser JustDown verbraucht die Kante)', () => {
    const asked: number[] = [];
    focusEdges(none, (k) => (asked.push(k), true));
    expect(asked.sort((a, b) => a - b)).toEqual(Object.values(KEY_FOCUS).flat().sort((a, b) => a - b));
  });

  it('ein Klick setzt die Markierung auf die geklickte Schaltfläche und löst sie aus', () => {
    expect(focusStep(on, {}, true, 1, 3)).toEqual({ state: { index: 3, seat: 0 }, fire: 3 });
    expect(focusStep(on, { DOWN: true }, true, 1, 3).state.index).toBe(3);
    expect(focusStep(on, {}, false, 1, 3)).toEqual({ state: on, fire: null });
  });

  it('Controller-Tasten von Edge auf der Xbox (195–218) werden erkannt, Pfeile und Leertaste nicht', () => {
    expect([195, 203, 206, 218].every(isGamepadKey)).toBe(true);
    expect([13, 32, 37, 38, 39, 40, 194, 219].some(isGamepadKey)).toBe(false);
  });
});
