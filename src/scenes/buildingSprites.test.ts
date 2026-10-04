import { describe, expect, it } from 'vitest';
import { BUILDING_TEXTURES, STAIRS_UP_TILES, hubTexture, siteTexture } from './buildingSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../public/grafik/**/*.png'));

describe('Gebäude-Grafik (GR3.1)', () => {
  it('gebaute Gebäude mit Zuordnung bekommen ihre Grafik', () => {
    expect(siteTexture('workshop', 'built')).toBe('grafik:workshop');
    expect(siteTexture('gate', 'built')).toBe('grafik:gate');
    expect(siteTexture('wall', 'built')).toBe('grafik:wall-2');
  });

  it('Treppe hoch steht aus Treppen-Kacheln: Anfang, Mitte, Ende', () => {
    expect(siteTexture('stairsUp', 'built')).toBe('grafik:stairs');
    expect(STAIRS_UP_TILES[0]).toBe('grafik:stairs-left');
    expect(STAIRS_UP_TILES.at(-1)).toBe('grafik:stairs-right');
    for (const key of STAIRS_UP_TILES) expect(BUILDING_TEXTURES).toHaveProperty([key]);
  });

  it('Baustellen und Lücken zeichnen den Platzhalter (AC-03)', () => {
    for (const state of ['unpaid', 'waitingMaterial', 'waitingWorker'] as const) expect(siteTexture('workshop', state)).toBeNull();
    for (const kind of ['tavern', 'healer', 'smithy', 'armory'] as const) expect(siteTexture(kind, 'built')).toBeNull();
  });

  it('Material-Stufen wählen das Sprite je Stufe, Lücken sind null (AC-04)', () => {
    expect([1, 2, 3, 4, 5].map((s) => siteTexture('wall', 'built', s))).toEqual([null, 'grafik:wall-2', 'grafik:wall-3', 'grafik:wall-4', 'grafik:wall-5']);
    expect([1, 2, 3, 4, 5].map((s) => siteTexture('tower', 'built', s))).toEqual([null, 'grafik:tower-2', 'grafik:tower-3', 'grafik:tower-4', null]);
    expect(siteTexture('wall', 'built', 9)).toBeNull();
    expect(siteTexture('farm', 'built', 3)).toBe('grafik:farm');
  });

  it('Hub-Stufen: 1 Burg, 2 Palisade, 3–5 noch Platzhalter', () => {
    expect([1, 2, 3, 4, 5].map((s) => hubTexture(s))).toEqual(['grafik:fort-tileset', 'grafik:hub-2', null, null, null]);
    expect(hubTexture()).toBe('grafik:fort-tileset');
  });

  it('jede genannte Datei liegt unter public/grafik/', () => {
    for (const file of Object.values(BUILDING_TEXTURES)) expect(FILES).toContain(`../../public/grafik/${file}`);
  });

  it('jeder gewählte Schlüssel ist ladbar', () => {
    const kinds = ['wall', 'tower', 'gate', 'workshop', 'storage', 'farm', 'barracks', 'stairsUp', 'stairsDown'] as const;
    const keys = [...kinds.map((k) => siteTexture(k, 'built')), ...[1, 2, 3, 4, 5].map((s) => siteTexture('tower', 'built', s)), hubTexture(2)];
    for (const key of keys.filter((k) => k !== null)) expect(BUILDING_TEXTURES).toHaveProperty([key]);
  });
});
