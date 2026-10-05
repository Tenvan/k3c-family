import { describe, expect, it } from 'vitest';
import { WORLD_FRAMES, WORLD_TEXTURES, biomeBackground, objectSprite } from './worldSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../public/grafik/**/*.png'));

describe('Welt-Grafik (GR3.2)', () => {
  it('zugeordnete Objekte bekommen ihr Bild, Erz-Sheet nach Zeile und Spalte', () => {
    expect(objectSprite('node:tree')?.key).toBe('grafik:trees');
    expect(objectSprite('node:rock')).toMatchObject({ key: 'grafik:stones', frame: 1 });
    expect(objectSprite('node:copperOre')).toMatchObject({ key: 'grafik:stones', frame: 7 });
    expect(objectSprite('coin')).toMatchObject({ key: 'grafik:coins', frames: 8 });
    expect(objectSprite('portal')).toMatchObject({ key: 'grafik:portals', frames: 4, zoom: 1 });
    expect(objectSprite('camp:recruit')).toMatchObject({ key: 'grafik:tent', frame: 'tent' });
    expect(objectSprite('camp:recruit:fire')).toMatchObject({ key: 'grafik:campfire', frames: 4 });
  });

  it('ohne Zuordnung null → Platzhalter (AC-03)', () => {
    expect(objectSprite('plantation')).toBeNull();
    expect(objectSprite('node:unbekannt')).toBeNull();
    expect(biomeBackground('sumpf')).toBeNull();
  });

  it('Wald, Höhle und Mine haben eigene Ebenen, hinten langsamer (AC-05)', () => {
    for (const biome of ['forest', 'cave', 'mine']) {
      const layers = biomeBackground(biome)?.layers ?? [];
      expect(layers.length, biome).toBeGreaterThan(0);
      const scrolls = layers.map((l) => l.scroll);
      expect(scrolls).toEqual([...scrolls].sort((a, b) => a - b));
    }
  });

  it('jeder Schlüssel ist geladen und jede Datei liegt unter public/grafik/', () => {
    const keys = [...['node:tree', 'node:rock', 'node:copperOre', 'node:bush', 'portal', 'exit', 'pickup:chest', 'pickup:skillPoint', 'coin', 'camp:recruit', 'camp:recruit:fire'].map((id) => objectSprite(id)?.key), ...['forest', 'cave', 'mine'].flatMap((b) => biomeBackground(b)!.layers.map((l) => l.key))];
    for (const key of keys) expect(WORLD_TEXTURES).toHaveProperty([key!]);
    for (const key of Object.keys(WORLD_FRAMES)) expect(WORLD_TEXTURES).toHaveProperty([key]);
    for (const { file } of Object.values(WORLD_TEXTURES)) expect(FILES).toContain(`../../public/grafik/${file}`);
  });
});
