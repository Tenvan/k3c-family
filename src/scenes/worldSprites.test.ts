import { describe, expect, it } from 'vitest';
import { WORLD_TEXTURES, biomeBackground, objectSprite } from './worldSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../public/grafik/**/*.png'));

describe('Welt-Grafik (GR3.2)', () => {
  it('zugeordnete Objekte bekommen ihr Bild; Portal, Ausgang, Truhe und Lager aus tools/gebaeude stehen still im Maßstab 1 (GEB6)', () => {
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

  it('Wald, Höhle und Mine haben eigene Ebenen, hinten langsamer (AC-05)', () => {
    for (const biome of ['forest', 'cave', 'mine']) {
      const layers = biomeBackground(biome)?.layers ?? [];
      expect(layers.length, biome).toBeGreaterThan(0);
      const scrolls = layers.map((l) => l.scroll);
      expect(scrolls).toEqual([...scrolls].sort((a, b) => a - b));
    }
  });

  it('jeder Schlüssel ist geladen und jede Datei liegt unter public/grafik/', () => {
    const keys = [...['portal', 'exit', 'pickup:chest', 'pickup:skillPoint', 'coin', 'camp:recruit'].map((id) => objectSprite(id)?.key), ...['forest', 'cave', 'mine'].flatMap((b) => biomeBackground(b)!.layers.map((l) => l.key))];
    for (const key of keys) expect(WORLD_TEXTURES).toHaveProperty([key!]);
    for (const { file } of Object.values(WORLD_TEXTURES)) expect(FILES).toContain(`../../public/grafik/${file}`);
  });
});
