import { describe, expect, it } from 'vitest';
import type { HubUpgrade } from '../../model/types';
import { BUILDING_TEXTURES, hubLookTexture } from './buildingSprites';
import { HUB_GROWTH, hubLookOf } from './hubGrowth';

/** Ausbau auf Hub-Stufe 2 (data/hub.json: 50 Gold, 100 Stein) mit `paid` bezahltem Gold */
const up = (paid: number, state?: HubUpgrade['state']): HubUpgrade => ({ gold: 50, material: { stone: 100 }, paid, ...(state ? { state } : {}) });

/** 32-px-Grafiken unter public/grafik/gebaeude/, als Pfad relativ zu public/grafik/ */
const files = new Set(Object.keys(import.meta.glob('../../../public/grafik/gebaeude/*-32.png')).map((p) => p.replace('../../../public/grafik/', '')));

/** Alle 15 Kombinationen aus Hub-Stufe 1–5 und Ausbauschritt 1–3 */
const combos = [1, 2, 3, 4, 5].flatMap((hubLevel) => [1, 2, 3].map((hubStep) => ({ hubLevel, hubStep })));

describe('Wachstumsreihe der Burg (GEB17.6, materialien-gebaeude.md § 2.1)', () => {
  it('15 Kombinationen aus Hub-Stufe und Ausbauschritt geben 15 verschiedene Texturen, deterministisch (AC-05)', () => {
    const keys = combos.map((c) => hubLookTexture(hubLookOf(c)));
    expect(keys.every((k) => k !== null)).toBe(true);
    expect(new Set(keys).size).toBe(15);
    expect(combos.map((c) => hubLookTexture(hubLookOf(c)))).toEqual(keys);
    for (const k of keys) expect(files.has(BUILDING_TEXTURES[k!]!), k!).toBe(true);
  });

  it('Hub-Stufe 1 folgt dem Ausbauschritt, nicht dem bezahlten Gold (GEB17.4 begann je Schritt neu)', () => {
    expect(hubLookOf({ hubLevel: 1, hubStep: 1, hubUpgrade: up(49) })).toEqual({ id: 'lagerfeuer', stufe: 1, level: 1, scaffold: false });
    expect(hubLookOf({ hubLevel: 1, hubStep: 2, hubUpgrade: up(0) })).toEqual({ id: 'lagerfeuer', stufe: 2, level: 1, scaffold: false });
    expect(hubLookOf({ hubLevel: 1, hubStep: 3, hubUpgrade: up(0) })).toEqual({ id: 'lagerfeuer', stufe: 3, level: 1, scaffold: false });
    expect(hubLookOf({ hubLevel: 1, hubStep: 3, hubUpgrade: up(50, 'waitingMaterial') })).toMatchObject({ id: 'lagerfeuer', stufe: 3 });
  });

  it('ab Hub-Stufe 2: Schritt 1 die Burg der Stufe, Schritte 2 und 3 ihre Ausbauschritte (burg 6–13)', () => {
    expect(combos.slice(3).map((c) => hubLookOf(c).stufe)).toEqual([2, 6, 7, 3, 8, 9, 4, 10, 11, 5, 12, 13]);
    expect(hubLookOf({ hubLevel: 5 })).toEqual({ id: 'burg', stufe: 5, level: 5, scaffold: false });
  });

  it('Gerüst bei jedem waitingWorker, auch bei Gefahr (B-572)', () => {
    expect(hubLookOf({ hubLevel: 1, hubStep: 2, hubUpgrade: up(50, 'waitingWorker'), danger: true })).toEqual({ id: 'lagerfeuer', stufe: 2, level: 1, scaffold: true });
    expect(hubLookOf({ hubLevel: 3, hubStep: 3, hubUpgrade: up(50, 'waitingWorker'), danger: false })).toEqual({ id: 'burg', stufe: 9, level: 3, scaffold: true });
  });

  it('Rückfälle: ohne hubStep Schritt 1, ohne hubLevel Stufe 1, Werte außerhalb begrenzt', () => {
    expect(hubLookOf({})).toEqual({ id: 'lagerfeuer', stufe: 1, level: 1, scaffold: false });
    expect(hubLookOf({ hubLevel: 2 })).toMatchObject({ id: 'burg', stufe: 2 });
    expect(hubLookOf({ hubLevel: 0, hubStep: 9 })).toMatchObject({ id: 'lagerfeuer', stufe: 3 });
    expect(hubLookOf({ hubLevel: 7, hubStep: 0 })).toMatchObject({ id: 'burg', stufe: 5, level: 5 });
  });

  it('die Reihe der Vorschau ist genau die Spielableitung in Spielreihenfolge', () => {
    expect(HUB_GROWTH.map((g) => `${g.id}-${g.stufe}`)).toEqual(combos.map((c) => `${hubLookOf(c).id}-${hubLookOf(c).stufe}`));
  });
});
