// RD1.1: Pose je Frame (Phase n / frames), Keyframes linear und zyklisch.
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { Abgelehnt, Aufruf } from './fehler';
import { keyframeAusgeblendet, keyframeWinkel, posen } from './pose';
import { ladeStil } from './stil';
import { einfachesBild } from './bild';

const spec = (rig: Spec['rig'], animationen: Spec['animationen']): Spec => ({
  id: 'k', rig, version: 1, beschreibung: '', animationen, ausgabe: { hoehe_px: 32, rahmen: [0, 0, 1, 1] },
  teile: [{ id: 'k', pivot: [0, 0], lagen: [] }],
});

describe('sichtbar (B-714)', () => {
  const kf = [{ t: 0, winkel: {}, sichtbar: { p: true } }, { t: 0.7, winkel: {}, sichtbar: { p: false } }];

  it('keyframeAusgeblendet: letzter Eintrag mit t ≤ Phase gilt, vor dem ersten der letzte (zyklisch)', () => {
    expect([...keyframeAusgeblendet(kf, 0)]).toEqual([]);
    expect([...keyframeAusgeblendet(kf, 0.69)]).toEqual([]);
    expect([...keyframeAusgeblendet(kf, 0.7)]).toEqual(['p']);
    expect([...keyframeAusgeblendet(kf, 0.99)]).toEqual(['p']);
    expect([...keyframeAusgeblendet([{ t: 0.5, winkel: {}, sichtbar: { p: true } }, { t: 0.8, winkel: {}, sichtbar: { p: false } }], 0.2)]).toEqual(['p']);
    expect(keyframeAusgeblendet(undefined, 0.5).size).toBe(0);
    expect(keyframeAusgeblendet([{ t: 0, winkel: { p: 1 } }], 0.5).size).toBe(0);
  });

  it('Teil mit sichtbar: false fehlt im Frame, mit true steht es darin (requisite)', () => {
    const teile: Spec['teile'] = [
      { id: 'a', pivot: [0, 0], lagen: [{ art: 'flaeche', form: 'M0 0L1 0L1 1Z', material: 'stein' }] },
      { id: 'p', pivot: [0, 0], lagen: [{ art: 'flaeche', form: 'M5 5L6 5L6 6Z', material: 'gold' }] },
    ];
    const s: Spec = { ...spec('requisite', { an: { frames: 2, keyframes: kf.map((f) => ({ ...f, t: f.t === 0 ? 0 : 0.5 })) } }), teile };
    const frames = einfachesBild(s, { schluessel: 'an', animation: 'an', stufe: 1 }, { stil: ladeStil() }).frames;
    expect(frames[0].svg).toContain('M5 5L6 5L6 6Z');
    expect(frames[1].svg).not.toContain('M5 5L6 5L6 6Z');
    expect(frames[1].svg).toContain('M0 0L1 0L1 1Z');
    expect(frames[0].geometrie.map((g) => g.teil.id)).toEqual(['a', 'p']);
    expect(frames[1].geometrie.map((g) => g.teil.id)).toEqual(['a']);
  });
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
