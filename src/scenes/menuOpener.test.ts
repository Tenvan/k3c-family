import { describe, expect, it } from 'vitest';
import { findOpener, removeTargets } from './menuOpener';

const pad1 = { name: 'pad1' };
const pad2 = { name: 'pad2' };
const kb1 = { name: 'kb1' };
const kb2 = { name: 'kb2' };
const touch = { name: 'touch' };
const keyboards = [kb1, kb2] as const;

describe('findOpener (B-411/AC-03)', () => {
  it('erkennt Pad, Tastatur 1 und Touch als Öffner', () => {
    expect(findOpener([{ input: kb1, opens: false }, { input: pad1, opens: true }])?.input).toBe(pad1);
    expect(findOpener([{ input: kb1, opens: true }, { input: pad1, opens: false }])?.input).toBe(kb1);
    expect(findOpener([{ input: kb1, opens: false }, { input: touch, opens: true }])?.input).toBe(touch);
  });

  it('niemand löst aus: kein Öffner; zwei Pads im selben Frame: das erste gilt', () => {
    expect(findOpener([{ input: kb1, opens: false }])).toBeNull();
    expect(findOpener([{ input: pad1, opens: true }, { input: pad2, opens: true }])?.input).toBe(pad1);
  });
});

describe('removeTargets (B-411/AC-01, AC-03, AC-04)', () => {
  it('der Öffner trifft nur seinen eigenen Slot, nicht den höchsten', () => {
    const bound = [pad1, pad2, touch, null];
    expect(removeTargets(pad2, bound, keyboards, [0, 1, 2])).toEqual([1]);
    expect(removeTargets(pad1, bound, keyboards, [0, 1, 2])).toEqual([0]);
    expect(removeTargets(touch, bound, keyboards, [0, 1, 2])).toEqual([2]);
  });

  it('Eingabe ohne Slot oder ohne Platz: ausgegraut (leer)', () => {
    expect(removeTargets(pad2, [pad1, null, null, null], keyboards, [0])).toEqual([]);
    expect(removeTargets(null, [pad1, null, null, null], keyboards, [0])).toEqual([]);
    expect(removeTargets(pad1, [pad1, null, null, null], keyboards, [1])).toEqual([]);
  });

  it('zwei Tastatur-Spieler: Esc (Tastatur 1) lässt zwischen beiden wählen', () => {
    expect(removeTargets(kb1, [kb1, kb2, null, null], keyboards, [0, 1])).toEqual([0, 1]);
    expect(removeTargets(kb1, [kb2, kb1, null, null], keyboards, [0, 1])).toEqual([1, 0]);
  });

  it('Tastatur 1 allein oder ein Pad als Öffner: keine Auswahl', () => {
    expect(removeTargets(kb1, [kb1, pad1, null, null], keyboards, [0, 1])).toEqual([0]);
    expect(removeTargets(pad1, [kb1, kb2, pad1, null], keyboards, [0, 1, 2])).toEqual([2]);
  });
});
