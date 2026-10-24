import { readFileSync } from 'node:fs';
import { buildView, emptyState, toggleChip } from '@webkit/lib/listFilter';
import { describe, expect, it } from 'vitest';
import { baumDatei, baumReihe, figureCard, gebaeudeKarten, GRAFIK_SCALE, grafikScale, hintergrundStreifen, hubWachstum, kachelAnzahl, stufenKarten, vorlageZeilen, licenseFlag, resolveSlot, sceneFilter, SCENES, slotsOf, type AssetSlot, type GebaeudeEintrag } from './scenesLogic';

const farm: AssetSlot = {
  id: 'farm', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Farm',
  wahl: [{ datei: 'grafik/gothicvania-town/props-einzeln/well.png', ausschnitt: { x: 0, y: 0, w: 64, h: 48 } }, { datei: 'grafik/gothicvania-town/props-einzeln/wagon.png' }],
};
const tavern: AssetSlot = { id: 'tavern', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Taverne', wahl: [] };
const coin: AssetSlot = { id: 'sfx:coin', art: 'sound', kategorie: 'sfx', beschreibung: 'Münze', bus: 'sfx', wahl: [{ datei: 'audio/atlas.ogg', sprite: 'coin' }] };

describe('Szenen: Liste (B-299)', () => {
  it('kennt fünf Grafik- und drei Sound-Szenen', () => {
    expect(SCENES.map((s) => s.id)).toEqual(['figuren', 'gebaeude', 'umgebung', 'objekte', 'hud', 'sfx', 'musik', 'ambient']);
  });
});

describe('Asset-Vorschau: Slot → aktives Asset (B-299/AC-03)', () => {
  it('nimmt die erste Präferenz samt Ausschnitt', () => {
    const r = resolveSlot(farm, { wahl: 1 });
    expect(r).toMatchObject({ pref: 1, placeholder: false, choice: farm.wahl[0] });
    expect(r.choice?.ausschnitt).toEqual({ x: 0, y: 0, w: 64, h: 48 });
  });

  it('wahl=n zeigt die n-te Präferenz nur für den genannten Slot', () => {
    expect(resolveSlot(farm, { slot: 'farm', wahl: 2 }).choice).toBe(farm.wahl[1]);
    expect(resolveSlot(farm, { slot: 'tower', wahl: 2 }).pref).toBe(1);
  });

  it('ohne Zuordnung oder ohne n-te Präferenz: Platzhalter', () => {
    expect(resolveSlot(tavern, { wahl: 1 })).toMatchObject({ placeholder: true, choice: undefined });
    expect(resolveSlot(farm, { slot: 'farm', wahl: 3 })).toMatchObject({ placeholder: true, pref: 3 });
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

  it('Nachbarn überlappen sich (Schritt 1 Kachel, schmalster Baum 2 Kacheln breit, GEB10)', () => {
    const reihe = baumReihe(index);
    for (let i = 1; i < reihe.length; i++) expect(reihe[i]!.x - reihe[i - 1]!.x).toBe(1);
  });

  it('Höhlen-Flora als eigene Reihe (GEB10): nur Klasse flora', () => {
    const flora = (id: string) => ({ id, klasse: 'flora', kacheln: { b: 2, h: 15 }, dateien: {} });
    const idx = [...index, flora('hoehlenbusch'), flora('wurzel')];
    expect(baumReihe(idx, undefined, 1, 'flora').map((b) => b.id)).toEqual(['hoehlenbusch', 'wurzel']);
    expect(baumReihe(idx).map((b) => b.id)).toEqual(['birke', 'pappel', 'tanne']);
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

describe('Asset-Vorschau: echte Grafik (GEB10)', () => {
  const index = JSON.parse(readFileSync(new URL('../../../../../public/grafik/gebaeude/index.json', import.meta.url), 'utf8')) as GebaeudeEintrag[];

  it('zeigt offene und geschlossene Truhe direkt nacheinander (B-455/AC-01)', () => {
    const ids = gebaeudeKarten(index).filter((k) => k.dichte === 32).map((k) => k.id);
    expect(ids.indexOf('truhe-geschlossen')).toBe(ids.indexOf('truhe') - 1); // Dateiname: `-` sortiert vor `.`
  });

  it('Bäume 11,25 Kacheln hoch (birke-3 9,5), Höhlen-Flora in eigener Reihe (B-454)', () => {
    const baeume = index.filter((e) => e.klasse === 'baum');
    expect(baeume).toHaveLength(10); // pappel-niedrig (B-568) ist bewusst nur 5 Kacheln hoch
    for (const e of baeume) expect(e.kacheln.h).toBe(e.id === 'birke-3' ? 9.5 : e.id === 'pappel-niedrig' ? 5 : 11.25);
    expect(baumReihe(index, undefined, 1, 'flora').map((b) => b.id)).toEqual(['hoehlenbusch', 'wurzel-2', 'wurzel-3', 'wurzel']);
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

describe('Asset-Vorschau: Stufenreihe der Hub-Gebäude (GEB9/AC-03)', () => {
  // Echter Index der Gebäude-Grafik; jede GEB9-Session ergänzt ihre Gebäude.
  const index = JSON.parse(readFileSync(new URL('../../../../../public/grafik/gebaeude/index.json', import.meta.url), 'utf8')) as GebaeudeEintrag[];
  const mitStufen = ['werkstatt', 'farm', 'kaserne', 'lager', 'taverne', 'schmiede', 'heilplatz', 'ruestkammer'];

  it.each(mitStufen)('%s zeigt die Stufen 2 bis 5 in 32 px', (id) => {
    const e = index.find((g) => g.id === id)!;
    expect(stufenKarten(e).map((k) => k.stufe)).toEqual([2, 3, 4, 5]);
    for (const k of stufenKarten(e)) {
      expect(k.datei).toBe(`grafik/gebaeude/${id}-${k.stufe}-32.png`);
      expect(k.aenderung).not.toBe('');
    }
  });
});

describe('Asset-Vorschau: Hub wächst (GEB11/AC-03)', () => {
  // Echter Index: die Reihe kommt aus dem Spielcode (HUB_GROWTH), die Dateien aus task gen:gebaeude.
  const index = JSON.parse(readFileSync(new URL('../../../../../public/grafik/gebaeude/index.json', import.meta.url), 'utf8')) as GebaeudeEintrag[];

  it('zeigt Lagerfeuer, Zelt, Hütte und die Burgen 1 bis 5 in Spielreihenfolge, dann die Steinburg mit Gerüst', () => {
    const karten = hubWachstum(index);
    expect(karten.map((k) => k.titel)).toEqual([
      'Lagerfeuer mit Grill', 'Zelt am Feuer', 'Holzhütte am Feuer',
      'Holzburg', 'Steinburg', 'Kupferburg', 'Eisenburg', 'Kristallburg', 'Steinburg im Ausbau',
    ]);
    expect(karten.map((k) => k.datei)).toEqual([
      ...[1, 2, 3].map((n) => `grafik/gebaeude/lagerfeuer-${n}-32.png`),
      ...[1, 2, 3, 4, 5, 2].map((n) => `grafik/gebaeude/burg-${n}-32.png`),
    ]);
    expect(karten.at(-1)?.geruest).toBe('grafik/gebaeude/hub-geruest-32.png');
    expect(karten.slice(0, -1).every((k) => k.geruest === undefined && k.wann !== '')).toBe(true);
    expect(karten.map((k) => k.h)).toEqual([5, 5, 5, 9, 9, 9, 9, 9, 9]);
  });

  it('lässt Looks ohne Datei weg; ohne Burg kein Gerüst-Beispiel', () => {
    expect(hubWachstum([])).toEqual([]);
    const ohneBurg = index.filter((e) => e.id !== 'burg');
    expect(hubWachstum(ohneBurg).map((k) => k.titel)).toEqual(['Lagerfeuer mit Grill', 'Zelt am Feuer', 'Holzhütte am Feuer']);
  });
});

describe('Asset-Vorschau: kachelbare Grafik (GEB3/AC-07)', () => {
  it('Hintergrundmauer dreimal nebeneinander, andere Gebäude einmal', () => {
    expect(kachelAnzahl({ klasse: 'kachel' })).toBe(3);
    expect(kachelAnzahl({ klasse: 'baum' })).toBe(1);
    expect(kachelAnzahl({})).toBe(1);
  });
});

describe('Asset-Vorschau: Hintergründe je Biom (GEB7/AC-03)', () => {
  const schicht = (id: string) => ({ id, klasse: 'hintergrund', kacheln: { b: 24, h: 15 }, dateien: { 16: `${id}-16.png`, 32: `${id}-32.png` } });
  const wald = ['fern', 'mitte', 'nah'].map((s) => schicht(`hg-wald-${s}`));
  const biome = [{ id: 'forest', palette: { sky: '#123456' } }, { id: 'cave', palette: { sky: '#654321' } }];

  it('stapelt je Biom die drei Schichten fern → nah auf der Himmelsfarbe des Bioms', () => {
    const s = hintergrundStreifen([...wald].reverse(), biome);
    expect(s).toHaveLength(1);
    expect(s[0]!.biom).toBe('forest');
    expect(s[0]!.himmel).toBe('#123456');
    expect(s[0]!.schichten.map((l) => l.id)).toEqual(['hg-wald-fern', 'hg-wald-mitte', 'hg-wald-nah']);
  });

  it('Spielmaßstab: Generator-Grafik nach Dichte im Namen (32/Dichte), sonst ×GRAFIK_SCALE (GEB18)', () => {
    expect(grafikScale('grafik/gebaeude/hg-hoehle-fern-v1-128.png')).toBe(0.25);
    expect(grafikScale('grafik/gebaeude/hg-wald-fern-32.png')).toBe(1);
    expect(grafikScale('grafik/gebaeude/castle-16.png')).toBe(2);
    expect(grafikScale('grafik/gothicvania-town/props-einzeln/well.png')).toBe(GRAFIK_SCALE);
  });

  it('zeigt nach der bisherigen Fassung jede vollständige Variante aufsteigend (GEB18)', () => {
    const v = (n: number, schicht: string): GebaeudeEintrag => ({ ...wald[0]!, id: `hg-wald-${schicht}-v${n}` });
    const index = [...wald, ...['fern', 'mitte', 'nah'].flatMap((s) => [v(10, s), v(2, s)]), v(3, 'fern')];
    const s = hintergrundStreifen(index, biome);
    expect(s.map((x) => x.variante)).toEqual(['', 'v2', 'v10']);
    expect(s[1]!.schichten.map((l) => l.id)).toEqual(['hg-wald-fern-v2', 'hg-wald-mitte-v2', 'hg-wald-nah-v2']);
  });

  it('lässt Biome ohne vollständige Schichten und unbekannte Biome weg', () => {
    expect(hintergrundStreifen(wald.slice(0, 2), biome)).toEqual([]);
    expect(hintergrundStreifen(wald, [{ id: 'sumpf', palette: { sky: '#000000' } }])).toEqual([]);
  });

  it('zeigt Hintergrund-Schichten nicht als Gebäude-Karten', () => {
    expect(gebaeudeKarten(wald)).toEqual([]);
  });
});

describe('Szenen: Filter (B-599)', () => {
  const credits = [{ root: 'audio', folders: ['nc'], license: 'CC-BY-NC' }];
  const nc: AssetSlot = { id: 'sfx:nc', art: 'sound', kategorie: 'sfx', beschreibung: 'Zischen', bus: 'ui', wahl: [{ datei: 'audio/nc/zisch.ogg' }] };
  const gap: AssetSlot = { id: 'skill:heal', art: 'sound', kategorie: 'sfx', beschreibung: 'Heilen', bus: 'sfx', wahl: [] };
  const items = [coin, nc, gap].map((s) => resolveSlot(s, { wahl: 1 }));
  const cfg = sceneFilter(items, credits);
  const ids = (state = emptyState('spiel')) => buildView(items, cfg, state).items.map((r) => r.slot.id);

  it('Reihenfolge im Spiel, Name und Status', () => {
    expect(ids()).toEqual(['sfx:coin', 'sfx:nc', 'skill:heal']);
    expect(ids(emptyState('name'))).toEqual(['sfx:coin', 'sfx:nc', 'skill:heal'].sort());
    expect(ids(emptyState('status'))[0]).toBe('skill:heal');
  });

  it('Lücke, Bus, Art und Lizenz filtern; Suche findet Beschreibung und Datei', () => {
    expect(ids(toggleChip(emptyState('spiel'), 'status', 'Lücke'))).toEqual(['skill:heal']);
    expect(ids(toggleChip(emptyState('spiel'), 'bus', 'ui'))).toEqual(['sfx:nc']);
    expect(ids(toggleChip(emptyState('spiel'), 'art', 'skill'))).toEqual(['skill:heal']);
    expect(ids(toggleChip(emptyState('spiel'), 'lizenz', 'NC'))).toEqual(['sfx:nc']);
    expect(ids({ ...emptyState('spiel'), query: 'zisch' })).toEqual(['sfx:nc']);
    expect(ids({ ...emptyState('spiel'), query: 'heilen' })).toEqual(['skill:heal']);
  });
});
