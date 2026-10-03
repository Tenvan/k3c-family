import { describe, expect, it } from 'vitest';
import { DEV_ACTIONS } from './debugActions';
import { focusStep, muteFocused, PAD_FOCUS, type FocusState } from './debugOverlayPanel';

const off: FocusState = { focus: false, index: 0, seat: 0 };
const on: FocusState = { focus: true, index: 0, seat: 0 };

describe('focusStep (B-179)', () => {
  it('RB schaltet den Fokus nur bei sichtbarer Liste', () => {
    expect(focusStep(off, { RB: true }, true, 1).state.focus).toBe(true);
    expect(focusStep(on, { RB: true }, true, 1).state.focus).toBe(false);
    expect(focusStep(off, { RB: true }, false, 1).state.focus).toBe(false);
    expect(focusStep(on, {}, false, 1).state.focus).toBe(false); // Liste verschwindet (Overlay zu, kein Dev-Mode) → Fokus aus
  });

  it('D-Pad wählt die Zeile (rundum), A löst sie aus', () => {
    expect(focusStep(on, { DOWN: true }, true, 1).state.index).toBe(1);
    expect(focusStep(on, { UP: true }, true, 1).state.index).toBe(DEV_ACTIONS.length - 1);
    expect(focusStep({ ...on, index: 4 }, { A: true }, true, 1).fire).toBe(4);
    expect(focusStep(off, { A: true, DOWN: true }, true, 1)).toEqual({ state: off, fire: null }); // ohne Fokus gehört A dem Spiel
  });

  it('links/rechts wechselt den lokalen Spieler (2 Spieler am Gerät)', () => {
    expect(focusStep(on, { RIGHT: true }, true, 2).state.seat).toBe(1);
    expect(focusStep({ ...on, seat: 1 }, { RIGHT: true }, true, 2).state.seat).toBe(0);
    expect(focusStep(on, { LEFT: true }, true, 1).state.seat).toBe(0);
  });

  it('B, X, View und Menu gehören nicht zur Fokus-Bedienung', () => {
    expect(Object.values(PAD_FOCUS)).not.toContain(1); // B
    expect(Object.values(PAD_FOCUS)).not.toContain(2); // X
    expect(Object.values(PAD_FOCUS)).not.toContain(8); // View
    expect(Object.values(PAD_FOCUS)).not.toContain(9); // Menu
  });
});

describe('muteFocused', () => {
  const cmds = [
    { slot: 0, moveX: 1, sprint: true, pay: true },
    { slot: 1, moveX: -1, sprint: false, pay: true },
  ];

  it('im Fokus stehen nur die Controller-Spieler still', () => {
    expect(muteFocused(cmds, true, (s) => s === 0)).toEqual([{ slot: 0, moveX: 0, sprint: false, pay: false }, cmds[1]]);
  });

  it('ohne Fokus unverändert', () => {
    expect(muteFocused(cmds, false, () => true)).toBe(cmds);
  });
});
