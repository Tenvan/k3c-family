import { describe, expect, it } from 'vitest';
import type { AssetSlot } from '../api';
import { addChoice, countStatus, filterSlots, move, previewSize, removeChoice, sceneUrl, withCut } from './assets';

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

describe('Assets: Mini-Szene (B-298/AC-07)', () => {
  it('baut die Adresse der Dev-Seite, ohne Vite keine', () => {
    expect(sceneUrl('http://localhost:5173/', farm, 1)).toBe('http://localhost:5173/assetvorschau.html?szene=gebaeude&slot=farm');
    expect(sceneUrl('http://localhost:5173/', farm, 2)).toBe('http://localhost:5173/assetvorschau.html?szene=gebaeude&slot=farm&wahl=2');
    expect(sceneUrl('', farm, 1)).toBe('');
  });
});
