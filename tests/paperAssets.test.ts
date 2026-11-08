import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import {
  PAPER_BUILDINGS, PAPER_ENEMIES, PAPER_PLAYERS, PAPER_TROOPS, type PaperManifest, manifestAnim, paperBackgroundFile,
  paperBuildingFile, paperOrigin, paperScale, usedFigureAssets,
} from '../src/scenes/render/paperAssets';

const GRAFIK = resolve(__dirname, '../public/grafik');
const manifest = (key: string) => JSON.parse(readFileSync(join(GRAFIK, key, 'manifest.json'), 'utf8')) as PaperManifest;
const index = (gruppe: string) => (JSON.parse(readFileSync(join(GRAFIK, gruppe, 'index.paper.json'), 'utf8')) as { id: string }[]).map((e) => e.id);

describe('Papierschnitt-Zuordnung (US1)', () => {
  it('jede Truppe und jeder Gegner hat ein Papier-Asset und Maße (data/paper.json › figuren)', async () => {
    const { PAPER_FIGURES } = await import('../src/scenes/render/paperAssets');
    expect(Object.keys(PAPER_TROOPS).sort()).toEqual(Object.keys(PAPER_FIGURES.truppen).sort());
    expect(Object.keys(PAPER_ENEMIES).sort()).toEqual(Object.keys(PAPER_FIGURES.gegner).sort());
    for (const f of [...PAPER_FIGURES.spieler, ...Object.values(PAPER_FIGURES.truppen), ...Object.values(PAPER_FIGURES.gegner)]) expect(f.hoehe).toBeGreaterThan(0);
  });

  it('jedes Figuren-Asset steht im Gruppen-Index und hat idle samt Varianten', () => {
    for (const key of usedFigureAssets()) {
      const [gruppe, id] = key.split('/');
      expect(index(gruppe!), key).toContain(id);
      expect(manifest(key).animationen.idle, key).toBeDefined();
    }
    for (const p of PAPER_PLAYERS) expect(manifest(`${p.gruppe}/${p.id}`).animationen[manifestAnim('idle', p.variante)]).toBeDefined();
  });

  it('Sheets der Manifeste liegen auf der Platte', () => {
    for (const key of usedFigureAssets()) {
      for (const a of Object.values(manifest(key).animationen)) expect(existsSync(join(GRAFIK, key, a.datei)), `${key}/${a.datei}`).toBe(true);
    }
  });

  it('Gebäude und ihre Stufen aus dem Gruppen-Index', () => {
    expect([...PAPER_BUILDINGS].sort()).toEqual(index('gebaeude-paper').sort());
    expect(paperBuildingFile('mauer')).toBe('gebaeude-paper/mauer/stand.png');
    expect(paperBuildingFile('mauer', 3)).toBe('gebaeude-paper/mauer/stand.s3.png');
    expect(paperBuildingFile('geruest-2')).toBe('gebaeude-paper/geruest-2/stand.png');
    for (const id of PAPER_BUILDINGS) expect(existsSync(join(GRAFIK, paperBuildingFile(id)!)), id).toBe(true);
  });

  it('Hintergründe: Varianten wiederholen sich, Dateien liegen da', () => {
    expect(paperBackgroundFile('wald', 'fern', 3)).toBe('hintergruende/hg-wald-fern-v1/stand.png');
    for (const teil of ['wald', 'hoehle', 'mine', 'eisenstollen', 'kristall']) {
      for (const ebene of ['fern', 'mitte', 'nah']) for (const v of [1, 2]) expect(existsSync(join(GRAFIK, paperBackgroundFile(teil, ebene, v)))).toBe(true);
    }
  });

  it('Ursprung im Fußpunkt, Maßstab auf die Zielhöhe', () => {
    expect(paperOrigin([-60, -175, 150, 181])).toEqual({ x: 0.4, y: 175 / 181 });
    const m: PaperManifest = { id: 'x', rahmen: [-50, -100, 100, 100], frameW: 200, frameH: 200, animationen: { idle: { frames: 1, fps: 8, datei: 'idle.png' } } };
    expect(paperScale(m, 'idle', 50)).toEqual({ scale: 0.25, top: -50 });
  });
});


describe('Natur, Objekte, Münzen und Icons als Papierschnitt (US2/AC-01)', () => {

  it('Natur- und Objekt-Texturen kommen aus objekte/ und icons/', async () => {
    const { NATURE_TEXTURES } = await import('../src/scenes/render/natureSprites');
    const { WORLD_TEXTURES } = await import('../src/scenes/render/worldSprites');
    const files = { ...NATURE_TEXTURES, ...Object.fromEntries(Object.entries(WORLD_TEXTURES).map(([k, v]) => [k, v.file])) };
    for (const [key, file] of Object.entries(files)) {
      if (file.startsWith('gebaeude-paper/')) continue;
      expect(file, key).toMatch(/^(objekte|icons)\/[a-z-]+\/idle\.png$/);
      expect(existsSync(join(GRAFIK, file)), file).toBe(true);
    }
  });

  it('keine alten Pixel-Gebäude mehr (CU1.2)', async () => {
    const { BUILDING_TEXTURES } = await import('../src/scenes/render/buildingSprites');
    const alt = Object.entries(BUILDING_TEXTURES).filter(([, f]) => f.startsWith('gebaeude/')).map(([k]) => k);
    expect(alt).toEqual([]);
  });

  it('Maße in data/paper.json passen zu den Manifesten', async () => {
    const { PAPER_OBJECTS } = await import('../src/scenes/render/paperAssets');
    expect(Object.keys(PAPER_OBJECTS).length).toBeGreaterThan(20);
    for (const [id, o] of Object.entries(PAPER_OBJECTS)) {
      const m = manifest(`${o.gruppe}/${id}`);
      expect([o.frame, o.frames, o.rahmen], id).toEqual([[m.frameW, m.frameH], m.animationen.idle!.frames, m.rahmen]);
    }
  });
});

describe('Reittiere (US2/AC-02)', () => {
  it('jeder Reittier-Schlüssel zeigt auf ein Asset in reittiere/, auch das Standard-Reittier des Monarchen', async () => {
    const { PAPER_MOUNTS } = await import('../src/scenes/render/paperAssets');
    const { MONARCH } = await import('../src/model/data');
    expect(Object.keys(PAPER_MOUNTS)).toContain(MONARCH.mount.sprite);
    for (const id of Object.values(PAPER_MOUNTS)) expect(index('reittiere'), id).toContain(id);
  });
});
