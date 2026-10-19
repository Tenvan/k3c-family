import { describe, expect, it } from 'vitest';
import { HOLD_IDLE, HOLD_MS, holdStep, tapStep, type HoldState } from './debugGestures';

function hold(steps: [boolean, boolean, number][]): (string | null)[] {
  let s: HoldState = HOLD_IDLE;
  return steps.map(([lb, rb, t]) => {
    const r = holdStep(s, lb, rb, t);
    s = r.state;
    return r.fire;
  });
}

describe('holdStep (B-231)', () => {
  it('LB + RB 3 s = Cheat-Dialog, genau einmal je Halten', () => {
    expect(hold([[true, true, 0], [true, true, HOLD_MS - 1], [true, true, HOLD_MS], [true, true, HOLD_MS + 500]])).toEqual([null, null, 'cheats', null]);
  });

  it('nur RB 3 s = Diagnose; LB allein löst nichts aus', () => {
    expect(hold([[false, true, 0], [false, true, HOLD_MS]])).toEqual([null, 'diag']);
    expect(hold([[true, false, 0], [true, false, HOLD_MS * 2]])).toEqual([null, null]);
  });

  it('RB zuerst, dann LB: die Zeit beginnt neu, keine Diagnose dazwischen', () => {
    expect(hold([[false, true, 0], [true, true, 1000], [true, true, HOLD_MS + 500], [true, true, HOLD_MS + 1000]])).toEqual([null, null, null, 'cheats']);
  });

  it('Loslassen setzt zurück', () => {
    expect(hold([[false, true, 0], [false, false, 2000], [false, true, 2500], [false, true, HOLD_MS + 1000]])).toEqual([null, null, null, null]);
  });
});

describe('tapStep (B-231)', () => {
  const tap = (fingers: number, start: number, end = start + 100) => ({ fingers, start, end });

  it('zwei Finger doppelt = Cheat-Dialog, ein Finger doppelt = Diagnose', () => {
    expect(tapStep(tapStep(null, tap(2, 0)).prev, tap(2, 300)).fire).toBe('cheats');
    expect(tapStep(tapStep(null, tap(1, 0)).prev, tap(1, 300)).fire).toBe('diag');
  });

  it('gemischte Fingerzahl, zu langsam oder langes Halten löst nichts aus', () => {
    expect(tapStep(tap(1, 0), tap(2, 300)).fire).toBeNull();
    expect(tapStep(tap(2, 0), tap(2, 1000)).fire).toBeNull();
    expect(tapStep(tap(1, 0), tap(1, 300, 2000))).toEqual({ prev: null, fire: null }); // laufen (Finger liegt)
  });
});
