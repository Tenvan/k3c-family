import { describe, expect, it } from 'vitest';
import { BIOMES } from '../../model/biome';
import { BACKGROUND_TILE_PX, BACKGROUND_ZOOM, CHEST_OPEN, WORLD_TEXTURES, backgroundFile, backgroundKey, backgroundPlan, objectSprite, planKeys } from './worldSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../../public/grafik/**/*.png'));
const ALL = ['forest', 'cave', 'mine', 'ironhold', 'crystal'];

describe('Welt-Grafik (GR3.2)', () => {
  it('zugeordnete Objekte bekommen ihr Bild; Portal, Ausgang, Truhe und Lager aus tools/buildings stehen still im Maßstab 1 (GEB6)', () => {
    expect(objectSprite('coin')).toMatchObject({ key: 'grafik:coins', frames: 8 });
    for (const [id, key] of [['portal', 'grafik:portal'], ['exit', 'grafik:ausgang'], ['harbor', 'grafik:hafen'], ['pickup:chest', 'grafik:truhe-geschlossen'], ['camp:recruit', 'grafik:lager']]) {
      expect(objectSprite(id!), id).toMatchObject({ key, frame: 0, frames: 1, zoom: 1 });
    }
    expect(objectSprite('camp:recruit:fire')).toBeNull(); // das Bild enthält die Feuerstelle
  });

  it('liegende Truhe geschlossen, beim Aufheben die offene; beide geladen (GEB10, B-455/AC-02)', () => {
    const closed = objectSprite('pickup:chest')!.key;
    expect(CHEST_OPEN[closed]).toBe('grafik:truhe');
    for (const key of [closed, 'grafik:truhe']) expect(WORLD_TEXTURES).toHaveProperty([key]);
  });

  it('ohne Zuordnung null → Platzhalter (AC-03)', () => {
    expect(objectSprite('plantation')).toBeNull();
    expect(objectSprite('node:unbekannt')).toBeNull();
    expect(backgroundPlan('sumpf', 1000, 1920)).toBeNull();
    expect(backgroundFile('bg:sumpf:1:v1')).toBeNull();
    expect(backgroundFile('grafik:coins')).toBeNull();
  });

  it('jeder Schlüssel ist geladen und jede Datei liegt unter public/grafik/', () => {
    const keys = ['portal', 'exit', 'harbor', 'pickup:chest', 'pickup:skillPoint', 'coin', 'camp:recruit'].map((id) => objectSprite(id)?.key);
    for (const key of keys) expect(WORLD_TEXTURES).toHaveProperty([key!]);
    for (const { file } of Object.values(WORLD_TEXTURES)) expect(FILES).toContain(`../../../public/grafik/${file}`);
    expect(Object.keys(WORLD_TEXTURES).some((k) => k.startsWith('bg:'))).toBe(false); // Hintergründe lädt die Stufe selbst (GEB18)
  });
});

describe('Hintergründe je Kachel und Variante (GEB18, B-618/AC-04)', () => {
  it('alle fünf Biome: drei Schichten, hinten langsamer, jede Variante 1 … backdropVariants als Datei in 64 px', () => {
    expect(BACKGROUND_ZOOM).toBe(0.5);
    for (const biome of ALL) {
      const plan = backgroundPlan(biome, 3000, 1920)!;
      const scrolls = plan.map((l) => l.scroll);
      expect(scrolls, biome).toEqual([...scrolls].sort((a, b) => a - b));
      expect(new Set(scrolls).size, biome).toBe(3);
      const n = (BIOMES.find((b) => b.id === biome) as unknown as { backdropVariants: number }).backdropVariants;
      expect(n, biome).toBe(5);
      for (let layer = 1; layer <= 3; layer++) {
        for (let v = 1; v <= n; v++) {
          const f = backgroundFile(backgroundKey(biome, layer, v))!;
          expect(f.file).toMatch(/^gebaeude\/hg-[a-z]+-(fern|mitte|nah)-v[1-5]-64\.png$/);
          expect(FILES, f.file).toContain(`../../../public/grafik/${f.file}`);
        }
      }
    }
  });

  it('Kachel k zeigt Variante backdrops[k], die Folge wiederholt sich, ohne Folge Variante 1; Kacheln decken die Welt beim Scroll-Faktor', () => {
    const plan = backgroundPlan('forest', 4800, 1920, [3, 1, 4])!;
    const nah = plan[2]!;
    expect(nah.parts.length).toBe(Math.ceil((4800 * nah.scroll + 1920 * 2) / BACKGROUND_TILE_PX));
    expect(nah.parts.slice(0, 4)).toEqual([
      { key: 'bg:forest:3:v3', x: 0 },
      { key: 'bg:forest:3:v1', x: BACKGROUND_TILE_PX },
      { key: 'bg:forest:3:v4', x: 2 * BACKGROUND_TILE_PX },
      { key: 'bg:forest:3:v3', x: 3 * BACKGROUND_TILE_PX },
    ]);
    expect(planKeys(plan).sort()).toEqual(['1', '2', '3'].flatMap((n) => [1, 3, 4].map((v) => `bg:forest:${n}:v${v}`)).sort());
    expect(planKeys(backgroundPlan('cave', 4800, 1920)!).every((k) => k.endsWith(':v1'))).toBe(true);
  });
});
