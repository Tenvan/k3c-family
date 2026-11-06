// RD1.1: Pose je Frame (Phase n / frames), Keyframes linear und zyklisch.
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { Abgelehnt, Aufruf } from './fehler';
import { keyframeWinkel, posen } from './pose';

const spec = (rig: Spec['rig'], animationen: Spec['animationen']): Spec => ({
  id: 'k', rig, version: 1, beschreibung: '', animationen, ausgabe: { hoehe_px: 32, rahmen: [0, 0, 1, 1] },
  teile: [{ id: 'k', pivot: [0, 0], lagen: [] }],
});

describe('Pose', () => {
  it('requisite mit keyframes: Winkel linear interpoliert, zyklisch', () => {
    const s = spec('requisite', { dreh: { frames: 4, keyframes: [{ t: 0, winkel: { k: 0 } }, { t: 0.5, winkel: { k: 90 } }] } });
    expect(posen(s, 'dreh').map((p) => p.winkel.k)).toEqual([0, 45, 90, 45]);
  });

  it('keyframeWinkel: vor dem ersten Keyframe zwischen letztem und erstem, fehlende Teile = 0', () => {
    const kf: { t: number; winkel: Record<string, number> }[] = [{ t: 0.25, winkel: { a: 40 } }, { t: 0.75, winkel: { a: 0, b: 10 } }];
    expect(keyframeWinkel(kf, 0)).toEqual({ a: 20, b: 5 });
    expect(keyframeWinkel(kf, 0.5)).toEqual({ a: 20, b: 5 });
    expect(keyframeWinkel(undefined, 0.3)).toEqual({});
  });

  it('ohne Keyframes: jeder Frame ohne Winkel; ohne animationen gilt stand', () => {
    expect(posen(spec('gebaeude', { stand: { frames: 2 } }), 'stand')).toEqual([{ winkel: {} }, { winkel: {} }]);
    expect(posen(spec('requisite', undefined), 'stand')).toHaveLength(1);
  });

  it('unbekannte Animation: abgelehnt mit Regel animation-unbekannt', () => {
    expect(() => posen(spec('requisite', { stand: { frames: 1 } }), 'lauf')).toThrow(Abgelehnt);
  });

  it('figur und vierbeiner folgen in RD1.2/RD1.3 (Aufruffehler)', () => {
    expect(() => posen(spec('vierbeiner', { schritt: { frames: 1 } }), 'schritt')).toThrow(Aufruf);
  });
});
