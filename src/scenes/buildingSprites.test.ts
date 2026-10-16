import { describe, expect, it } from 'vitest';
import { BUILDING_TEXTURES, STAIRS_UP_TILES, backWallTexture, hubTexture, siteTexture, textureScale } from './buildingSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../public/grafik/**/*.png'));

describe('Gebäude-Grafik (GR3.1)', () => {
  it('gebaute Gebäude mit Zuordnung bekommen ihre Grafik', () => {
    expect(siteTexture('gate', 'built')).toBe('grafik:gate-1');
    expect(siteTexture('wall', 'built')).toBe('grafik:wall-1');
  });

  it('Treppe hoch steht aus Treppen-Kacheln: Anfang, Mitte, Ende', () => {
    expect(siteTexture('stairsUp', 'built')).toBe('grafik:stairs');
    expect(STAIRS_UP_TILES[0]).toBe('grafik:stairs-left');
    expect(STAIRS_UP_TILES.at(-1)).toBe('grafik:stairs-right');
    for (const key of STAIRS_UP_TILES) expect(BUILDING_TEXTURES).toHaveProperty([key]);
  });

  it('Baustellen und Lücken zeichnen den Platzhalter (AC-03)', () => {
    for (const state of ['unpaid', 'waitingMaterial', 'waitingWorker'] as const) expect(siteTexture('workshop', state)).toBeNull();
  });

  it('Gebäude aus tools/gebaeude: eigene 32-px-Grafik in Maßstab 1, Packs ×2 (GEB2 AC-02)', () => {
    const neu = { workshop: 'werkstatt', tavern: 'taverne', healer: 'heilplatz', smithy: 'schmiede', armory: 'ruestkammer' } as const;
    for (const [kind, id] of Object.entries(neu)) {
      const key = siteTexture(kind as keyof typeof neu, 'built');
      expect(key).not.toBeNull();
      expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/${id}-32.png`);
      expect(textureScale(key!)).toBe(1);
    }
    expect(textureScale('grafik:barracks')).toBe(2);
  });

  it('Material-Stufen wählen das Sprite je Stufe, Lücken sind null (AC-04)', () => {
    for (const [kind, id] of [['wall', 'mauer'], ['tower', 'turm'], ['gate', 'tor']] as const) {
      for (const s of [1, 2, 3, 4, 5]) {
        const key = siteTexture(kind, 'built', s);
        expect(key).toBe(`grafik:${kind}-${s}`);
        expect(BUILDING_TEXTURES[key!]).toBe(`gebaeude/${id}-${s}-32.png`); // GEB3 AC-02: alle zehn Stufen, eigene Grafik
        expect(textureScale(key!)).toBe(1);
      }
    }
    expect(siteTexture('wall', 'built', 9)).toBeNull();
    expect(siteTexture('farm', 'built', 3)).toBe('grafik:farm');
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

  it('jede genannte Datei liegt unter public/grafik/', () => {
    for (const file of Object.values(BUILDING_TEXTURES)) expect(FILES).toContain(`../../public/grafik/${file}`);
  });

  it('jeder gewählte Schlüssel ist ladbar', () => {
    const kinds = ['wall', 'tower', 'gate', 'workshop', 'tavern', 'healer', 'smithy', 'armory', 'storage', 'farm', 'barracks', 'stairsUp', 'stairsDown'] as const;
    const keys = [...kinds.map((k) => siteTexture(k, 'built')), ...[1, 2, 3, 4, 5].map((s) => siteTexture('tower', 'built', s)), ...[1, 2, 3, 4, 5].map((s) => hubTexture(s))];
    for (const key of keys.filter((k) => k !== null)) expect(BUILDING_TEXTURES).toHaveProperty([key]);
  });
});
