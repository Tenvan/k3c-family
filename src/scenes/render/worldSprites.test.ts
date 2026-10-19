import { describe, expect, it } from 'vitest';
import { WORLD_TEXTURES, biomeBackground, objectSprite } from './worldSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../../public/grafik/**/*.png'));

describe('Welt-Grafik (GR3.2)', () => {
  it('zugeordnete Objekte bekommen ihr Bild; Portal, Ausgang, Truhe und Lager aus tools/buildings stehen still im Maßstab 1 (GEB6)', () => {
    expect(objectSprite('coin')).toMatchObject({ key: 'grafik:coins', frames: 8 });
    for (const [id, key] of [['portal', 'grafik:portal'], ['exit', 'grafik:ausgang'], ['pickup:chest', 'grafik:truhe'], ['camp:recruit', 'grafik:lager']]) {
      expect(objectSprite(id!), id).toMatchObject({ key, frame: 0, frames: 1, zoom: 1 });
    }
    expect(objectSprite('camp:recruit:fire')).toBeNull(); // das Bild enthält die Feuerstelle
  });

  it('ohne Zuordnung null → Platzhalter (AC-03)', () => {
    expect(objectSprite('plantation')).toBeNull();
    expect(objectSprite('node:unbekannt')).toBeNull();
    expect(biomeBackground('sumpf')).toBeNull();
  });

  it('alle fünf Biome haben drei Schichten aus tools/buildings in Maßstab 1, hinten langsamer (GEB7 AC-05)', () => {
    for (const biome of ['forest', 'cave', 'mine', 'ironhold', 'crystal']) {
      const bg = biomeBackground(biome);
      expect(bg?.zoom, biome).toBe(1);
      const layers = bg?.layers ?? [];
      expect(layers.map((l) => l.key), biome).toEqual([1, 2, 3].map((n) => `bg:${biome}:${n}`));
      const scrolls = layers.map((l) => l.scroll);
      expect(scrolls, biome).toEqual([...scrolls].sort((a, b) => a - b));
      expect(new Set(scrolls).size, biome).toBe(3);
      for (const l of layers) expect(WORLD_TEXTURES[l.key]!.file, l.key).toMatch(/^gebaeude\/hg-[a-z]+-(fern|mitte|nah)-32\.png$/);
    }
  });

  it('jeder Schlüssel ist geladen und jede Datei liegt unter public/grafik/', () => {
    const keys = [...['portal', 'exit', 'pickup:chest', 'pickup:skillPoint', 'coin', 'camp:recruit'].map((id) => objectSprite(id)?.key), ...['forest', 'cave', 'mine', 'ironhold', 'crystal'].flatMap((b) => biomeBackground(b)!.layers.map((l) => l.key))];
    for (const key of keys) expect(WORLD_TEXTURES).toHaveProperty([key!]);
    for (const { file } of Object.values(WORLD_TEXTURES)) expect(FILES).toContain(`../../../public/grafik/${file}`);
  });
});
