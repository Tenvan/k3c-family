import { describe, expect, it } from 'vitest';
import { BUILDINGS } from '../../model/data';
import { BUILDING_TEXTURES, HUB_SCAFFOLD_TEXTURE, backWallTexture, hubLookTexture, hubTexture, isHubBuilding, siteTexture, textureScale } from './buildingSprites';
import { NATURE_TEXTURES, nodeSprite } from './natureSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../../public/grafik/**/*.png'));

describe('Gebäude-Grafik (GR3.1)', () => {
  it('gebaute Gebäude mit Zuordnung bekommen ihre Grafik', () => {
    expect(siteTexture('wall', 'built')).toBe('grafik:wall-1');
  });

  it('Treppe hoch ist ein Bild statt Kacheln (GEB5 AC-05)', () => {
    expect(siteTexture('stairsUp', 'built')).toBe('grafik:stairsUp');
    expect(BUILDING_TEXTURES['grafik:stairsUp']).toBe('gebaeude/treppe-hoch-32.png');
    expect(Object.keys(BUILDING_TEXTURES).filter((k) => k === 'grafik:stairs' || k.startsWith('grafik:stairs-'))).toEqual([]);
  });

  it('Baustellen und Lücken zeichnen den Platzhalter (AC-03)', () => {
    for (const state of ['unpaid', 'waitingMaterial', 'waitingWorker'] as const) expect(siteTexture('workshop', state)).toBeNull();
  });

  it('Gebäude aus tools/buildings: eigene 32-px-Grafik in Maßstab 1 (GEB2 AC-02, GEB5 AC-05)', () => {
    const neu = { workshop: 'werkstatt-1', tavern: 'taverne-1', healer: 'heilplatz-1', smithy: 'schmiede-1', armory: 'ruestkammer-1', storage: 'lager-1', barracks: 'kaserne-1', nursery: 'farm-1', stairsUp: 'treppe-hoch', stairsDown: 'treppe-runter' } as const;
    for (const [kind, id] of Object.entries(neu)) {
      const key = siteTexture(kind as keyof typeof neu, 'built');
      expect(key).not.toBeNull();
      expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/${id}-32.png`); // Hub-Gebäude ohne Stufe: Grundform = Stufe 1 (GEB9)
      expect(textureScale(key!)).toBe(1);
    }
    for (const [key, id] of [['grafik:setzling', 'setzling'], ['grafik:pappel-niedrig', 'pappel-niedrig']] as const) {
      expect(BUILDING_TEXTURES[key]).toBe(`gebaeude/${id}-32.png`); // Plantage (GEB5 AC-02)
      expect(textureScale(key)).toBe(1);
    }
  });

  it('Adern, Plantage und Lager zeigen Figuren statt Platzhalter, die Dateien liegen vor (B-412, W11.4)', () => {
    const datei = (key: string | null | undefined) => `../../../public/grafik/${BUILDING_TEXTURES[key!] ?? NATURE_TEXTURES[key!]}`;
    const figuren = [
      siteTexture('storage', 'built'), siteTexture('storage', 'built', 3), siteTexture('nursery', 'built'), 'grafik:setzling', 'grafik:pappel-niedrig',
      ...['stoneVein', 'copperVein', 'ironVein', 'crystalVein'].map((kind) => nodeSprite(kind, 0, 'cave')?.key),
    ];
    for (const key of figuren) expect(FILES).toContain(datei(key));
  });

  it('Material-Stufen wählen das Sprite je Stufe, Lücken sind null (AC-04)', () => {
    for (const [kind, id] of [['wall', 'mauer'], ['tower', 'turm']] as const) {
      for (const s of [1, 2, 3, 4, 5]) {
        const key = siteTexture(kind, 'built', s);
        expect(key).toBe(`grafik:${kind}-${s}`);
        expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/${id}-${s}-32.png`); // GEB3 AC-02: alle zehn Stufen, eigene Grafik
        expect(textureScale(key!)).toBe(1);
      }
    }
    expect(siteTexture('wall', 'built', 9)).toBeNull();
  });

  it('Hub-Gebäude zeigen die Grafik der Hub-Stufe 1–5, fehlende Stufe = höchste, ohne Stufe Grundform (GEB9 AC-04)', () => {
    const hub = { workshop: 'werkstatt', tavern: 'taverne', healer: 'heilplatz', smithy: 'schmiede', armory: 'ruestkammer', storage: 'lager', barracks: 'kaserne', nursery: 'farm' } as const;
    for (const [kind, id] of Object.entries(hub)) {
      const k = kind as keyof typeof hub;
      expect(isHubBuilding(k)).toBe(!BUILDINGS[kind].levels?.length); // B-569: mit eigenen `levels` folgt die Grafik der Gebäudestufe
      for (const s of [1, 2, 3, 4, 5]) {
        const key = siteTexture(k, 'built', s);
        expect(key).toBe(`grafik:${kind}-${s}`);
        expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/${id}-${s}-32.png`);
        expect(textureScale(key!)).toBe(1);
      }
      expect(siteTexture(k, 'built', 6)).toBe(`grafik:${kind}-5`); // B-458: andere Stufenzahl später
      expect(siteTexture(k, 'built', 0)).toBe(`grafik:${kind}-1`);
      expect(siteTexture(k, 'built')).toBe(`grafik:${kind}-1`);
      expect(siteTexture(k, 'unpaid', 3)).toBeNull(); // Baustelle: Platzhalter, unabhängig von der Stufe
    }
    for (const kind of ['wall', 'tower', 'stairsUp', 'stairsDown'] as const) expect(isHubBuilding(kind)).toBe(false);
  });

  it('Gebäude mit `levels` in den Daten: Stufe 1 = Grundwerte, fünf Stufen, Grafik je Gebäudestufe (B-569 AC-01)', () => {
    const withLevels = Object.entries(BUILDINGS).filter(([k, d]) => k !== 'ship' && d.levels?.length); // das Schiff hat drei Abschnitte aus data/ship.json (B-637)
    expect(withLevels.map(([k]) => k).sort()).toEqual(['barracks', 'nursery', 'healer', 'storage', 'tavern', 'tower', 'wall'].sort());
    for (const [kind, d] of withLevels) {
      expect(d.levels).toHaveLength(5);
      expect(d.levels![0]).toEqual({ cost: d.cost, hp: d.hp, buildSeconds: d.buildSeconds });
      for (let n = 1; n < 5; n++) expect(d.levels![n].hp).toBeGreaterThan(d.levels![n - 1].hp);
      expect(siteTexture(kind as 'wall', 'built', 3)).toBe(`grafik:${kind}-3`);
    }
  });

  it('Schiff: je Abschnitt (Kiel, Rumpf, Segel) eine eigene 32-px-Grafik, Baustelle und Lücken Platzhalter (BS10 AC-03)', () => {
    for (const n of [1, 2, 3]) {
      const key = siteTexture('ship', 'built', n);
      expect(key).toBe(`grafik:ship-${n}`);
      expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/schiff-${n}-32.png`);
      expect(textureScale(key!)).toBe(1);
      expect(FILES).toContain(`../../../public/grafik/${BUILDING_TEXTURES[key!]}`);
    }
    expect(siteTexture('ship', 'built')).toBe('grafik:ship-1');
    expect(siteTexture('ship', 'built', 4)).toBe('grafik:ship-3'); // andere Abschnittszahl später: höchster vorhandener
    expect(siteTexture('ship', 'unpaid', 2)).toBeNull();
    expect(isHubBuilding('ship')).toBe(false);
    expect(BUILDINGS.ship.levels).toHaveLength(3); // Abschnitte aus data/ship.json = Grafiken
  });

  it('Hintergrundmauer je Stufe eigene 32-px-Grafik, unbekannte Stufe null (GEB3 AC-08)', () => {
    for (const n of [1, 2, 3, 4, 5]) {
      const key = backWallTexture(n);
      expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/hintergrundmauer-${n}-32.png`);
      expect(textureScale(key!)).toBe(1);
    }
    expect(backWallTexture(6)).toBeNull();
  });

  it('Hub-Stufen 1–5: Burg je Stufe eigene 32-px-Grafik, ohne Stufe 1, unbekannte Stufe null (GEB4 AC-05)', () => {
    for (const n of [1, 2, 3, 4, 5]) {
      const key = hubTexture(n);
      expect(key).toBe(`grafik:hub-${n}`);
      expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/burg-${n}-32.png`);
      expect(textureScale(key!)).toBe(1);
    }
    expect(hubTexture()).toBe('grafik:hub-1');
    expect(hubTexture(6)).toBeNull();
  });

  it('Wachstumsreihe (GEB11): Lagerfeuer, Zelt, Hütte und Gerüst je eigene 32-px-Grafik, Burg über hubTexture, unbekannt null', () => {
    for (const n of [1, 2, 3]) {
      const key = hubLookTexture({ id: 'lagerfeuer', stufe: n });
      expect(key).toBe(`grafik:lagerfeuer-${n}`);
      expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/lagerfeuer-${n}-32.png`);
      expect(textureScale(key!)).toBe(1);
    }
    expect(hubLookTexture({ id: 'burg', stufe: 2 })).toBe('grafik:hub-2');
    expect(hubLookTexture({ id: 'burg', stufe: 6 })).toBe('grafik:burg-6'); // Ausbauschritte (GEB17.6)
    expect(BUILDING_TEXTURES['grafik:burg-13']).toBe('gebaeude/burg-13-32.png');
    expect(hubLookTexture({ id: 'lagerfeuer', stufe: 4 })).toBeNull();
    expect(hubLookTexture({ id: 'burg', stufe: 14 })).toBeNull();
    expect(BUILDING_TEXTURES[HUB_SCAFFOLD_TEXTURE]).toBe('gebaeude/hub-geruest-32.png');
    expect(textureScale(HUB_SCAFFOLD_TEXTURE)).toBe(1);
  });

  it('jede genannte Datei liegt unter public/grafik/', () => {
    for (const file of Object.values(BUILDING_TEXTURES)) expect(FILES).toContain(`../../../public/grafik/${file}`);
  });

  it('jeder gewählte Schlüssel ist ladbar', () => {
    const kinds = ['wall', 'tower', 'workshop', 'tavern', 'healer', 'smithy', 'armory', 'storage', 'nursery', 'barracks', 'stairsUp', 'stairsDown'] as const;
    const keys = [...kinds.map((k) => siteTexture(k, 'built')), ...[1, 2, 3, 4, 5].map((s) => siteTexture('tower', 'built', s)), ...[1, 2, 3, 4, 5].map((s) => hubTexture(s))];
    for (const key of keys.filter((k) => k !== null)) expect(BUILDING_TEXTURES).toHaveProperty([key]);
  });
});
