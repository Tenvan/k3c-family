import { describe, expect, it } from 'vitest';
import { BIOMES } from '../../model/biome';
import { FELSEN, NATURE_TEXTURES, WURZELN, campSprite, decoRoots, natureLook, nodeSprite, rockTexture, treeTexture, veinSlot } from './natureSprites';

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

  it('Höhle (und Mine) zeigen Höhlen-Flora: alle fünf Wurzel-Formen, keine Bäume (GEB10, GEB15.1/AC-01)', () => {
    expect(WURZELN).toHaveLength(5);
    for (const biome of ['cave', 'mine', 'ironhold']) {
      const keys = new Set(xs.map((x) => treeTexture(x, biome)));
      expect([...keys].sort(), biome).toEqual(['grafik:wurzel', 'grafik:wurzel-2', 'grafik:wurzel-3', 'grafik:wurzel-4', 'grafik:wurzel-5']);
    }
  });

  it('Wald: Art und Form 1–3, fehlende Biom-ID wie Wald', () => {
    const keys = new Set(xs.map((x) => treeTexture(x, 'forest')));
    for (const k of keys) expect(k).toMatch(/^grafik:tree-(tanne|pappel|birke)-[123]$/);
    expect(treeTexture(7, undefined)).toBe(treeTexture(7, 'forest'));
  });

  it('jeder gewählte Schlüssel ist geladen und seine Datei existiert', () => {
    for (const biome of ['forest', 'cave', 'mine']) {
      for (const x of xs) expect(NATURE_TEXTURES).toHaveProperty([treeTexture(x, biome)]);
    }
    for (const file of Object.values(NATURE_TEXTURES)) expect(FILES).toContain(`../../../public/grafik/${file}`);
  });
});

describe('decoRoots (GEB15.1/AC-01)', () => {
  const xs = Array.from({ length: 200 }, (_, i) => i * 3.7 - 40);
  const dichte = (id: string) => BIOMES.find((b) => b.id === id)?.rootsPerNode;

  it('Zahl je Knoten kommt aus den Daten: Höhle 2 Wurzeln (eine Deko), ohne Feld 1, Wald keine', () => {
    expect(dichte('cave')).toBe(2);
    for (const x of xs) expect(decoRoots(x, 'cave', dichte('cave'))).toHaveLength(1);
    expect(decoRoots(5, 'mine', dichte('mine'))).toEqual([]);
    expect(decoRoots(5, 'cave', 4)).toHaveLength(3);
    expect(decoRoots(5, 'forest', 3)).toEqual([]);
  });

  it('deterministisch, Versatz 0,4–1,2 Units zu beiden Seiten, andere Form als der Knoten, Schlüssel geladen', () => {
    const seiten = new Set<number>();
    for (const x of xs) {
      const deko = decoRoots(x, 'cave', 3);
      expect(decoRoots(x, 'cave', 3)).toEqual(deko);
      for (const d of deko) {
        expect(Math.abs(d.dx)).toBeGreaterThanOrEqual(0.4);
        expect(Math.abs(d.dx)).toBeLessThanOrEqual(1.2);
        expect(d.key).not.toBe(treeTexture(x, 'cave'));
        expect(NATURE_TEXTURES).toHaveProperty([d.key]);
        seiten.add(Math.sign(d.dx));
      }
    }
    expect([...seiten].sort()).toEqual([-1, 1]);
  });
});

describe('rockTexture (GEB15.2/AC-02)', () => {
  const xs = Array.from({ length: 200 }, (_, i) => i * 3.7 - 40);

  it('drei Formen je Look nach Position, Wald bleibt Look 1, Dateien vorhanden', () => {
    expect(FELSEN).toHaveLength(3);
    for (const [biome, look] of [['forest', 1], [undefined, 1], ['cave', 2], ['mine', 3], ['ironhold', 3]] as const) {
      const keys = new Set(xs.map((x) => rockTexture(x, biome)));
      expect([...keys].sort(), String(biome)).toEqual(FELSEN.map((id) => `grafik:${id}-${look}`));
    }
    for (const id of FELSEN) {
      for (const look of [1, 2, 3]) {
        expect(NATURE_TEXTURES).toHaveProperty([`grafik:${id}-${look}`], `gebaeude/${id}-${look}-32.png`);
        expect(FILES).toContain(`../../../public/grafik/gebaeude/${id}-${look}-32.png`);
      }
    }
  });

  it('deterministisch: gleiche Position gleiche Form, in jedem Biom dieselbe Form, nodeSprite folgt', () => {
    const form = (key: string) => key.replace(/-\d$/, '');
    for (const x of [0, 1.4, -0.5, -137.25, 412, 99999.9]) {
      expect(rockTexture(x, 'cave')).toBe(rockTexture(x, 'cave'));
      expect(form(rockTexture(x, 'forest'))).toBe(form(rockTexture(x, 'mine')));
      expect(nodeSprite('rock', x, 'cave')?.key).toBe(rockTexture(x, 'cave'));
    }
    expect(rockTexture(412.2, 'forest')).toBe(rockTexture(412.4, 'forest')); // gerundet
  });
});

describe('nodeSprite (GEB6.5)', () => {
  it('Fels folgt dem Biom, Erzbrocken und Busch haben einen Look', () => {
    expect(nodeSprite('rock', 0, 'forest')?.key).toMatch(/^grafik:felsen(-[bc])?-1$/);
    expect(nodeSprite('rock', 0, 'cave')?.key).toMatch(/^grafik:felsen(-[bc])?-2$/);
    expect(nodeSprite('rock', 0, 'ironhold')?.key).toMatch(/^grafik:felsen(-[bc])?-3$/);
    expect(nodeSprite('copperOre', 0, 'mine')?.key).toBe('grafik:erzbrocken');
    expect(nodeSprite('bush', 0, 'forest')?.key).toBe('grafik:busch');
    expect(nodeSprite('bush', 0, 'cave')?.key).toBe('grafik:hoehlenbusch'); // Höhlen-Flora (GEB10)
    expect(nodeSprite('bush', 0, 'mine')?.key).toBe('grafik:hoehlenbusch');
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

describe('campSprite (B-570)', () => {
  it('Rekrutierungslager zeigt den Look des Bioms, Datei je Look vorhanden', () => {
    expect(campSprite('forest').key).toBe('grafik:lager');
    expect(campSprite(undefined).key).toBe('grafik:lager');
    expect(campSprite('cave').key).toBe('grafik:lager-2');
    for (const biome of ['mine', 'ironhold', 'crystal']) expect(campSprite(biome).key).toBe('grafik:lager-3');
    for (const [key, file] of [['grafik:lager', 1], ['grafik:lager-2', 2], ['grafik:lager-3', 3]] as const) {
      expect(NATURE_TEXTURES[key]).toBe(`gebaeude/rekrutierungslager-${file}-32.png`);
      expect(FILES).toContain(`../../../public/grafik/${NATURE_TEXTURES[key]}`);
    }
  });
});
