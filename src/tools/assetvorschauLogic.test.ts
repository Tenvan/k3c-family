import { describe, expect, it } from 'vitest';
import { baumDatei, baumReihe, figureCard, gebaeudeKarten, kachelAnzahl, stufenKarten, vorlageZeilen, licenseFlag, parseQuery, resolveSlot, SCENES, slotsOf, type AssetSlot, type GebaeudeEintrag } from './assetvorschauLogic';

const farm: AssetSlot = {
  id: 'farm', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Farm',
  wahl: [{ datei: 'grafik/gothicvania-town/props-einzeln/well.png', ausschnitt: { x: 0, y: 0, w: 64, h: 48 } }, { datei: 'grafik/gothicvania-town/props-einzeln/wagon.png' }],
};
const tavern: AssetSlot = { id: 'tavern', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Taverne', wahl: [] };
const coin: AssetSlot = { id: 'sfx:coin', art: 'sound', kategorie: 'sfx', beschreibung: 'Münze', bus: 'sfx', wahl: [{ datei: 'audio/atlas.ogg', sprite: 'coin' }] };

describe('Asset-Vorschau: Aufruf (B-299)', () => {
  it('liest szene, slot und wahl; wahl ist mindestens 1', () => {
    expect(parseQuery('?szene=gebaeude&slot=farm&wahl=2')).toEqual({ szene: 'gebaeude', slot: 'farm', wahl: 2 });
    expect(parseQuery('')).toEqual({ szene: undefined, slot: undefined, wahl: 1 });
    expect(parseQuery('?szene=gebaeude&wahl=0').wahl).toBe(1);
    expect(parseQuery('?szene=nichts').szene).toBeUndefined();
  });

  it('kennt fünf Grafik- und drei Sound-Szenen', () => {
    expect(SCENES.map((s) => s.id)).toEqual(['figuren', 'gebaeude', 'umgebung', 'objekte', 'hud', 'sfx', 'musik', 'ambient']);
  });
});

describe('Asset-Vorschau: Slot → aktives Asset (B-299/AC-03)', () => {
  it('nimmt die erste Präferenz samt Ausschnitt', () => {
    const r = resolveSlot(farm, parseQuery('?szene=gebaeude'));
    expect(r).toMatchObject({ pref: 1, placeholder: false, choice: farm.wahl[0] });
    expect(r.choice?.ausschnitt).toEqual({ x: 0, y: 0, w: 64, h: 48 });
  });

  it('wahl=n zeigt die n-te Präferenz nur für den genannten Slot', () => {
    expect(resolveSlot(farm, parseQuery('?szene=gebaeude&slot=farm&wahl=2')).choice).toBe(farm.wahl[1]);
    expect(resolveSlot(farm, parseQuery('?szene=gebaeude&slot=tower&wahl=2')).pref).toBe(1);
  });

  it('ohne Zuordnung oder ohne n-te Präferenz: Platzhalter', () => {
    expect(resolveSlot(tavern, parseQuery(''))).toMatchObject({ placeholder: true, choice: undefined });
    expect(resolveSlot(farm, parseQuery('?slot=farm&wahl=3'))).toMatchObject({ placeholder: true, pref: 3 });
  });

  it('ordnet Slots ihrer Szene zu', () => {
    expect(slotsOf([farm, tavern, coin], 'gebaeude').map((s) => s.id)).toEqual(['farm', 'tavern']);
    expect(slotsOf([farm, tavern, coin], 'sfx')).toEqual([coin]);
  });
});

describe('Asset-Vorschau: Lizenz und Figuren', () => {
  const credits = [{ root: 'sprites', folders: ['horse'], license: 'CC BY-NC-SA 4.0' }, { root: 'grafik', folders: ['town'], license: 'CC0 1.0' }] as const;

  it('kennzeichnet NC nach Q70', () => {
    expect(licenseFlag('sprites/horse/idle.png', credits)).toBe('NC');
    expect(licenseFlag('grafik/town/a.png', credits)).toBe('');
    expect(licenseFlag('audio/atlas.ogg', credits)).toBe('');
  });

  it('übernimmt Tönung und Reittier aus data/sprites.json, wenn das Sheet passt', () => {
    expect(figureCard({ id: 'greed', art: 'grafik', kategorie: 'figuren', beschreibung: 'g', wahl: [] }, { datei: 'sprites/goblin/idle.png', sheet: 'goblin' }).spec)
      .toEqual({ sheet: 'goblin', tint: '#c77dff' });
    expect(figureCard({ id: 'mount:horse', art: 'grafik', kategorie: 'figuren', beschreibung: 'h', wahl: [] }, { datei: 'sprites/horse/idle.png', sheet: 'horse' }).mount).toBe('horse');
    expect(figureCard({ id: 'greed', art: 'grafik', kategorie: 'figuren', beschreibung: 'g', wahl: [] }, { datei: 'sprites/wolf/idle.png', sheet: 'wolf' }).spec)
      .toEqual({ sheet: 'wolf' });
  });
});

describe('Asset-Vorschau: neue Gebäude (GEB1/AC-03, AC-06)', () => {
  it('zeigt je Gebäude 32 px ×1 neben 16 px ×2, beide im Spielmaßstab', () => {
    const index = [{ id: 'werkstatt', kacheln: { b: 6, h: 4 }, dateien: { 16: 'w-16.png', 32: 'w-32.png', 64: 'w-64.png' } }];
    const karten = gebaeudeKarten(index);
    expect(karten).toEqual([
      { id: 'werkstatt', dichte: 32, datei: 'w-32.png', scale: 1 },
      { id: 'werkstatt', dichte: 16, datei: 'w-16.png', scale: 2 },
    ]);
    for (const k of karten) expect(k.dichte * k.scale).toBe(32);
  });

  it('lässt fehlende Dichten weg', () => {
    expect(gebaeudeKarten([{ id: 'x', kacheln: { b: 1, h: 1 }, dateien: { 32: 'x-32.png' } }])).toHaveLength(1);
  });
});

describe('Asset-Vorschau: Baum-Reihe (GEB1/AC-05)', () => {
  const baum = (id: string) => ({ id, klasse: 'baum', kacheln: { b: 1, h: 2.25 }, dateien: {} });
  const index = [baum('birke'), { id: 'werkstatt', kacheln: { b: 6, h: 4 }, dateien: {} }, baum('pappel'), baum('tanne')];

  it('wechselt die Bäume ab und lässt Gebäude weg', () => {
    expect(baumReihe(index, 4).map((b) => b.id)).toEqual(['birke', 'pappel', 'tanne', 'birke']);
  });

  it('Nachbarn überlappen sich (Abstand kleiner als 1 Kachel Breite)', () => {
    const reihe = baumReihe(index);
    for (let i = 1; i < reihe.length; i++) expect(reihe[i]!.x - reihe[i - 1]!.x).toBeLessThan(1);
  });

  it('ohne Bäume keine Reihe', () => {
    expect(baumReihe([])).toEqual([]);
  });

  it('zeigt ohne Anzahl jeden Baum einmal (GEB6.1: alle neun Formen)', () => {
    const neun = ['birke', 'birke-2', 'birke-3', 'pappel', 'pappel-2', 'pappel-3', 'tanne', 'tanne-2', 'tanne-3'].map(baum);
    expect(baumReihe(neun).map((b) => b.id)).toEqual(neun.map((b) => b.id));
  });

  it('Look Höhle (Stufe 2) enthält nur Bäume mit dieser Stufe und die Höhle-Dateien', () => {
    const mitHoehle = (id: string) => ({ ...baum(id), dateien: { 32: `${id}-1-32.png` }, stufen: [{ stufe: 2, aenderung: 'Höhle', dateien: { 32: `${id}-2-32.png` } }] });
    const idx = [baum('tanne'), mitHoehle('tanne-2'), mitHoehle('birke-3')];
    expect(baumReihe(idx, undefined, 2).map((b) => b.id)).toEqual(['tanne-2', 'birke-3']);
    expect(baumDatei(idx[1]!, 2, 32)).toBe('tanne-2-2-32.png');
    expect(baumDatei(idx[1]!, 1, 32)).toBe('tanne-2-1-32.png');
    expect(baumDatei(idx[0]!, 2, 32)).toBeUndefined();
  });
});

describe('Asset-Vorschau: Gebäude-Vorlage (GEB8/AC-03)', () => {
  const eintrag = (vorlage?: GebaeudeEintrag['vorlage']): GebaeudeEintrag => ({ id: 'taverne', kacheln: { b: 6, h: 4 }, dateien: {}, vorlage });

  it('Motiv als Titel, Teile als Zeilen, danach Materialien, Stimmung und fein', () => {
    const v = vorlageZeilen(eintrag({ motiv: 'Taverne', teile: ['Dach', 'Tür'], materialien: ['Putz (Fachwerk)', 'Dachziegel'], stimmung: 'warm', fein: 'Fugen' }));
    expect(v).toEqual({ titel: 'Taverne', zeilen: ['Dach', 'Tür', 'Materialien: Putz (Fachwerk), Dachziegel', 'Stimmung: warm', 'fein: Fugen'] });
  });

  it('leere Zusatzfelder entfallen', () => {
    expect(vorlageZeilen(eintrag({ motiv: 'Tanne', teile: ['Kegel'] }))).toEqual({ titel: 'Tanne', zeilen: ['Kegel'] });
  });

  it('ohne Vorlage nichts', () => {
    expect(vorlageZeilen(eintrag())).toBeNull();
  });
});

describe('Asset-Vorschau: Stufenreihe (GEB3/AC-01)', () => {
  const stufe = (n: number, aenderung: string) => ({ stufe: n, aenderung, dateien: { 32: `m-${n}-32.png`, 16: `m-${n}-16.png` } });
  const mauer: GebaeudeEintrag = { id: 'mauer', kacheln: { b: 1, h: 5 }, dateien: { 32: 'm-1-32.png' }, stufen: [stufe(1, 'Grundform'), stufe(2, 'Stein'), stufe(3, 'Kupfer')] };

  it('Stufen ab 2 in 32 px mit Änderungstext; Stufe 1 zeigt schon die Hauptkarte', () => {
    expect(stufenKarten(mauer)).toEqual([
      { id: 'mauer', stufe: 2, datei: 'm-2-32.png', aenderung: 'Stein' },
      { id: 'mauer', stufe: 3, datei: 'm-3-32.png', aenderung: 'Kupfer' },
    ]);
  });

  it('ohne Stufen keine Karten', () => {
    expect(stufenKarten({ id: 'x', kacheln: { b: 1, h: 1 }, dateien: {} })).toEqual([]);
  });
});

describe('Asset-Vorschau: kachelbare Grafik (GEB3/AC-07)', () => {
  it('Hintergrundmauer dreimal nebeneinander, andere Gebäude einmal', () => {
    expect(kachelAnzahl({ klasse: 'kachel' })).toBe(3);
    expect(kachelAnzahl({ klasse: 'baum' })).toBe(1);
    expect(kachelAnzahl({})).toBe(1);
  });
});
