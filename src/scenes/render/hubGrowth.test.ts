import { describe, expect, it } from 'vitest';
import type { HubUpgrade } from '../../model/types';
import { HUB_GROWTH, hubLookOf } from './hubGrowth';

/** Ausbau auf Hub-Stufe 2 (data/hub.json: 50 Gold, 100 Stein) mit `paid` bezahltem Gold */
const up = (paid: number, state?: HubUpgrade['state']): HubUpgrade => ({ gold: 50, material: { stone: 100 }, paid, ...(state ? { state } : {}) });

describe('Wachstumsreihe der Burg (GEB11, materialien-gebaeude.md § 2.1)', () => {
  it('Hub-Stufe 1: Lagerfeuer, Zelt, Holzhütte nach bezahltem Gold, Holzburg bei vollem Gold', () => {
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(0) })).toEqual({ id: 'lagerfeuer', stufe: 1, scaffold: false });
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(1) })).toEqual({ id: 'lagerfeuer', stufe: 2, scaffold: false });
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(24) })).toMatchObject({ id: 'lagerfeuer', stufe: 2 });
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(25) })).toMatchObject({ id: 'lagerfeuer', stufe: 3 });
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(49) })).toMatchObject({ id: 'lagerfeuer', stufe: 3 });
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(50, 'waitingMaterial') })).toEqual({ id: 'burg', stufe: 1, scaffold: false });
  });

  it('Gerüst, solange ein Bauer den Ausbau baut, auf jeder Stufe', () => {
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(50, 'waitingWorker') })).toEqual({ id: 'burg', stufe: 1, scaffold: true });
    expect(hubLookOf({ hubLevel: 3, hubUpgrade: { ...up(200, 'waitingWorker'), gold: 200 } })).toEqual({ id: 'burg', stufe: 3, scaffold: true });
  });

  it('kein Gerüst bei Gefahr: der Bau wartet nachts, kein Bauer arbeitet (GEB12 D8)', () => {
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: up(50, 'waitingWorker'), danger: true })).toEqual({ id: 'burg', stufe: 1, scaffold: false });
    expect(hubLookOf({ hubLevel: 2, hubUpgrade: up(50, 'waitingWorker'), danger: false })).toEqual({ id: 'burg', stufe: 2, scaffold: true });
  });

  it('ab Hub-Stufe 2 die Burg der Stufe, auch während Gold fließt', () => {
    for (const n of [2, 3, 4]) expect(hubLookOf({ hubLevel: n, hubUpgrade: up(0) })).toEqual({ id: 'burg', stufe: n, scaffold: false });
    expect(hubLookOf({ hubLevel: 5 })).toEqual({ id: 'burg', stufe: 5, scaffold: false });
  });

  it('Rückfälle: ohne hubUpgrade (letzte Stufe, älterer Server) und ohne Goldkosten die Burg; ohne hubLevel Stufe 1', () => {
    expect(hubLookOf({ hubLevel: 1 })).toEqual({ id: 'burg', stufe: 1, scaffold: false });
    expect(hubLookOf({})).toEqual({ id: 'burg', stufe: 1, scaffold: false });
    expect(hubLookOf({ hubLevel: 1, hubUpgrade: { ...up(0), gold: 0 } })).toMatchObject({ id: 'burg', stufe: 1 });
    expect(hubLookOf({ hubUpgrade: up(0) })).toMatchObject({ id: 'lagerfeuer', stufe: 1 });
    expect(hubLookOf({ hubLevel: 0, hubUpgrade: up(10) })).toMatchObject({ id: 'lagerfeuer', stufe: 2 });
  });

  it('jeder Look der Spielableitung steht in der Reihe der Vorschau', () => {
    const looks = [0, 1, 25].map((p) => hubLookOf({ hubLevel: 1, hubUpgrade: up(p) }));
    for (const l of looks) expect(HUB_GROWTH.some((g) => g.id === l.id && g.stufe === l.stufe)).toBe(true);
    expect(HUB_GROWTH.map((g) => `${g.id}-${g.stufe}`)).toEqual(['lagerfeuer-1', 'lagerfeuer-2', 'lagerfeuer-3', 'burg-1', 'burg-2', 'burg-3', 'burg-4', 'burg-5']);
  });
});
