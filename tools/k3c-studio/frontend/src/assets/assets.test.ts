import { describe, expect, it } from 'vitest';
import type { AssetSlot, AssetsView } from '../api';
import { isStale, STALE } from '../api/stale';
import { addChoice, applySaved, countStatus, filterSlots, move, previewSize, removeChoice, sceneLayout, SCENE, selectionOf, typen, typOf, withCut } from './assets';

const farm: AssetSlot = {
  id: 'farm', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: farm', status: 'zugeordnet',
  wahl: [{ datei: 'grafik/town/well.png' }, { datei: 'grafik/town/wagon.png' }],
};
const tavern: AssetSlot = { id: 'tavern', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: tavern', status: 'luecke', wahl: [] };
const coin: AssetSlot = { id: 'sfx:coin', art: 'sound', kategorie: 'sfx', beschreibung: 'Münze', status: 'zugeordnet', wahl: [{ datei: 'audio/atlas.ogg', sprite: 'coin' }] };

describe('Assets: Filter (B-298/AC-01)', () => {
  const all = [farm, tavern, coin];
  it('trennt Grafik und Sound', () => {
    expect(filterSlots(all, { art: 'grafik', kategorie: '', status: '', query: '' })).toEqual([farm, tavern]);
    expect(filterSlots(all, { art: 'sound', kategorie: '', status: '', query: '' })).toEqual([coin]);
  });
  it('filtert nach Status, Kategorie und Suchwörtern (auch in Dateien)', () => {
    expect(filterSlots(all, { art: 'grafik', kategorie: '', status: 'luecke', query: '' })).toEqual([tavern]);
    expect(filterSlots(all, { art: 'grafik', kategorie: 'figuren', status: '', query: '' })).toEqual([]);
    expect(filterSlots(all, { art: 'grafik', kategorie: '', status: '', query: 'WAGON' })).toEqual([farm]);
  });
  it('zählt je Status', () => {
    expect(countStatus(all)).toEqual({ zugeordnet: 2, luecke: 1, fehlt: 0 });
  });
});

describe('Assets: Präferenz und Ausschnitt (B-298/AC-02, AC-03)', () => {
  it('verschiebt per Ziehen und lässt ungültige Indizes unverändert', () => {
    expect(move(['a', 'b', 'c'], 2, 0)).toEqual(['c', 'a', 'b']);
    expect(move(['a', 'b', 'c'], 0, 2)).toEqual(['b', 'c', 'a']);
    expect(move(['a', 'b'], 0, 5)).toEqual(['a', 'b']);
  });
  it('hängt an, ohne doppelt, und entfernt', () => {
    expect(addChoice(farm.wahl, 'grafik/x.png').map((c) => c.datei)).toEqual(['grafik/town/well.png', 'grafik/town/wagon.png', 'grafik/x.png']);
    expect(addChoice(farm.wahl, 'grafik/town/well.png')).toHaveLength(2);
    expect(removeChoice(farm.wahl, 0).map((c) => c.datei)).toEqual(['grafik/town/wagon.png']);
  });
  it('setzt und entfernt einen Ausschnitt nur an der gewählten Präferenz', () => {
    const cut = withCut(farm.wahl, 1, { x: 0, y: 0, w: 16, h: 16 });
    expect(cut[1]).toEqual({ datei: 'grafik/town/wagon.png', ausschnitt: { x: 0, y: 0, w: 16, h: 16 } });
    expect(cut[0]).toBe(farm.wahl[0]);
    expect(withCut(cut, 1, undefined)[1]).toEqual({ datei: 'grafik/town/wagon.png' });
  });
  it('rechnet die Vorschau im Spielmaßstab (×2, Units à 32 px)', () => {
    expect(previewSize({ x: 0, y: 0, w: 64, h: 48 })).toEqual({ w: 128, h: 96, unitsW: 4, unitsH: 3 });
  });
});

describe('Assets: Typ-Filter', () => {
  const gegner: AssetSlot = { id: 'greed', art: 'grafik', kategorie: 'figuren', beschreibung: 'Gegner: greed', status: 'zugeordnet', wahl: [] };
  const monarch: AssetSlot = { id: 'm2', art: 'grafik', kategorie: 'figuren', beschreibung: 'Monarch 2', status: 'zugeordnet', wahl: [] };
  it('leitet den Typ aus der Beschreibung ab', () => {
    expect(typOf(farm)).toBe('Gebäude');
    expect(typOf(monarch)).toBe('Sonstige');
    expect(typOf({ beschreibung: 'Hub-Stufen: 1' })).toBe('Hub-Stufen');
    expect(typen([farm, tavern, gegner, monarch])).toEqual(['Gebäude', 'Gegner', 'Sonstige']);
  });
  it('filtert nach Typ', () => {
    expect(filterSlots([farm, gegner], { art: 'grafik', kategorie: '', typ: 'Gegner', status: '', query: '' })).toEqual([gegner]);
  });
});

describe('Assets: Mini-Szene', () => {
  it('ordnet Zuordnungen im Spielmaßstab auf einer Bodenlinie an', () => {
    const l = sceneLayout([{ w: 32, h: 160 }, { w: 16, h: 16 }]);
    expect(l.items[0]).toEqual({ x: SCENE.pad, y: l.floor - 320, w: 64, h: 320 });
    expect(l.items[1]!.x).toBe(SCENE.pad + 64 + SCENE.gap);
    expect(l.items[1]!.y + l.items[1]!.h).toBe(l.floor);
    expect(l.h).toBe(l.floor + SCENE.ground);
  });
  it('bleibt ohne Bilder gültig', () => {
    expect(sceneLayout([]).items).toEqual([]);
  });
});

describe('Asset-Studio: Neuladen und Schreibregel (B-459/AC-07)', () => {
  const view: AssetsView = { slots: [farm, tavern, coin], error: '', preview: '/', stand: 'a' };
  it('applySaved ersetzt nur den gespeicherten Slot und übernimmt den neuen Stand', () => {
    const neu = { ...tavern, wahl: [{ datei: 'grafik/x.png' }], status: 'zugeordnet' as const };
    const out = applySaved(view, 'tavern', { slot: neu, stand: 'b' });
    expect(out.stand).toBe('b');
    expect(out.slots).toEqual([farm, neu, coin]);
    expect(out.slots[0]).toBe(farm);
  });
  it('selectionOf: gefunden, verschwunden (Hinweis) oder keiner gewählt', () => {
    expect(selectionOf(view.slots, 'grafik', 'farm')).toEqual({ slot: farm, missing: false });
    expect(selectionOf(view.slots, 'grafik', 'weg')).toEqual({ slot: undefined, missing: true });
    expect(selectionOf(view.slots, 'grafik', 'sfx:coin')).toEqual({ slot: undefined, missing: true });
    expect(selectionOf(view.slots, 'grafik', '')).toEqual({ slot: undefined, missing: false });
  });
  it('isStale erkennt die Ablehnung als Text (Wails) und als Error (Mock)', () => {
    expect(isStale(`Error: ${STALE}`)).toBe(true);
    expect(isStale(new Error(STALE))).toBe(true);
    expect(isStale('präferenz 1: x fehlt unter public/')).toBe(false);
  });
});
