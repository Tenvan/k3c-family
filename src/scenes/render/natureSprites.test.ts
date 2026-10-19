import { describe, expect, it } from 'vitest';
import { NATURE_TEXTURES, natureLook, nodeSprite, treeTexture, veinSlot } from './natureSprites';

/** Nur die Pfade, nichts wird geladen */
const FILES = Object.keys(import.meta.glob('../../../public/grafik/gebaeude/*.png'));

describe('Natur-Looks (GEB6.5)', () => {
  it('natureLook: Wald 1, Höhle 2, Mine und Biome ohne eigenen Look 3, fehlende ID Wald', () => {
    expect(natureLook('forest')).toBe(1);
    expect(natureLook('cave')).toBe(2);
    expect(natureLook('mine')).toBe(3);
    expect(natureLook('ironhold')).toBe(3);
    expect(natureLook('crystal')).toBe(3);
    expect(natureLook('neues-biom')).toBe(3);
    expect(natureLook(undefined)).toBe(1);
    expect(natureLook('')).toBe(1);
  });

  it('veinSlot: alle vier Arten, unbekannt null', () => {
    expect(veinSlot('stoneVein')).toBe('vein:stone');
    expect(veinSlot('copperVein')).toBe('vein:copper');
    expect(veinSlot('ironVein')).toBe('vein:iron');
    expect(veinSlot('crystalVein')).toBe('vein:crystal');
    expect(veinSlot('rock')).toBeNull();
    expect(veinSlot('goldVein')).toBeNull();
  });
});

describe('treeTexture (GEB6.5, B-438)', () => {
  const xs = Array.from({ length: 200 }, (_, i) => i * 3.7 - 40);

  it('gleiche Position gibt gleiche Wahl, auch gebrochen und negativ', () => {
    for (const x of [0, 1.4, -0.5, -137.25, 412, 99999.9]) expect(treeTexture(x, 'forest')).toBe(treeTexture(x, 'forest'));
    expect(treeTexture(412.2, 'forest')).toBe(treeTexture(412.4, 'forest')); // gerundet
  });

  it('Nachbarn wechseln: im Wald erscheinen alle drei Arten und alle drei Formen', () => {
    const keys = new Set(xs.map((x) => treeTexture(x, 'forest')));
    expect(keys.size).toBe(9);
    for (const art of ['tanne', 'pappel', 'birke']) expect([...keys].filter((k) => k.includes(art))).toHaveLength(3);
  });

  it('Höhle (und Mine) nur Formen 2 und 3 im Look Höhle', () => {
    for (const biome of ['cave', 'mine', 'ironhold']) {
      const keys = new Set(xs.map((x) => treeTexture(x, biome)));
      expect(keys.size, biome).toBe(6);
      for (const k of keys) expect(k, biome).toMatch(/^grafik:tree-(tanne|pappel|birke)-[23]-2$/);
    }
  });

  it('Wald: Form 1 ohne Look-Suffix, Formen 2 und 3 im Look Wald; fehlende Biom-ID wie Wald', () => {
    const keys = new Set(xs.map((x) => treeTexture(x, 'forest')));
    for (const k of keys) expect(k).toMatch(/^grafik:tree-(tanne|pappel|birke)-(1|[23]-1)$/);
    expect(treeTexture(7, undefined)).toBe(treeTexture(7, 'forest'));
  });

  it('jeder gewählte Schlüssel ist geladen und seine Datei existiert', () => {
    for (const biome of ['forest', 'cave', 'mine']) {
      for (const x of xs) expect(NATURE_TEXTURES).toHaveProperty([treeTexture(x, biome)]);
    }
    for (const file of Object.values(NATURE_TEXTURES)) expect(FILES).toContain(`../../../public/grafik/${file}`);
  });
});

describe('nodeSprite (GEB6.5)', () => {
  it('Fels folgt dem Biom, Erzbrocken und Busch haben einen Look', () => {
    expect(nodeSprite('rock', 0, 'forest')?.key).toBe('grafik:felsen-1');
    expect(nodeSprite('rock', 0, 'cave')?.key).toBe('grafik:felsen-2');
    expect(nodeSprite('rock', 0, 'ironhold')?.key).toBe('grafik:felsen-3');
    expect(nodeSprite('copperOre', 0, 'mine')?.key).toBe('grafik:erzbrocken');
    expect(nodeSprite('bush', 0, 'forest')?.key).toBe('grafik:busch');
    expect(nodeSprite('bush', 0, 'cave')?.key).toBe('grafik:busch'); // Busch hat nur Wald-Look
  });

  it('Adern zeigen ihr Material, unabhängig vom Biom', () => {
    expect(nodeSprite('stoneVein', 5, 'cave')?.key).toBe('grafik:ader-1');
    expect(nodeSprite('copperVein', 5, 'mine')?.key).toBe('grafik:ader-2');
    expect(nodeSprite('ironVein', 5, 'ironhold')?.key).toBe('grafik:ader-3');
    expect(nodeSprite('crystalVein', 5, 'crystal')?.key).toBe('grafik:ader-4');
  });

  it('alles im Maßstab 1; unbekannte Art null (Platzhalter)', () => {
    for (const kind of ['tree', 'rock', 'copperOre', 'bush', 'stoneVein']) expect(nodeSprite(kind, 3, 'forest')?.zoom).toBe(1);
    expect(nodeSprite('goldVein', 3, 'forest')).toBeNull();
  });
});
