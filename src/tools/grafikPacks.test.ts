import { describe, expect, it } from 'vitest';
import credits from '../../public/grafik/CREDITS.md?raw';
import index from '../../public/grafik/index.json';
import lizenzen from '../../lizenzen.html?raw';
import { GRAFIK_PACKS } from './grafikPacks';

/** Alle Dateien unter public/grafik (Schlüssel: Pfad ab dem Ordner) */
const files = Object.keys(import.meta.glob('../../public/grafik/**/*', { query: '?url', import: 'default' })).map((p) => p.replace('../../public/grafik/', ''));

/** Dateiendungen, die nicht ins Repo gehören: Musik, Code, Quelldateien, Skripte */
const FORBIDDEN = /\.(mp3|ogg|wav|js|html|psd|ase|aseprite|gif|iml|tps|json)$/i;

describe('Grafik-Packs (B-087)', () => {
  it('hat zwölf Packs mit eindeutiger ID', () => {
    expect(GRAFIK_PACKS).toHaveLength(12);
    expect(new Set(GRAFIK_PACKS.map((p) => p.id)).size).toBe(12);
  });

  it.each(GRAFIK_PACKS)('$id: Ordner, LICENSE.txt und Bilder liegen im Repo (AC-01)', (pack) => {
    expect(files, 'LICENSE.txt').toContain(`${pack.id}/LICENSE.txt`);
    const images = index.filter((i) => i.pack === pack.id);
    expect(images.length, 'Bilder').toBeGreaterThan(0);
    for (const image of images) {
      expect(files, image.file).toContain(`${pack.id}/${image.file}`);
      expect(image.file.endsWith('.png')).toBe(true);
      expect(image.w * image.h).toBeGreaterThan(0);
    }
  });

  it('enthält keine Musik, Code oder Quelldateien und keinen Ordner __MACOSX (AC-01)', () => {
    const bad = files.filter((f) => f !== 'index.json' && (FORBIDDEN.test(f) || f.includes('__MACOSX')));
    expect(bad).toEqual([]);
  });

  it('jede Datei gehört zu einem Pack oder ist CREDITS.md und index.json', () => {
    const ids = new Set(GRAFIK_PACKS.map((p) => p.id));
    for (const f of files) {
      if (f === 'CREDITS.md' || f === 'index.json') continue;
      expect(ids.has(f.split('/')[0]!), f).toBe(true);
    }
  });

  it('jedes Bild der Liste gehört zu einem bekannten Pack', () => {
    const ids = new Set(GRAFIK_PACKS.map((p) => p.id));
    expect(index.filter((i) => !ids.has(i.pack))).toEqual([]);
  });

  it.each(GRAFIK_PACKS)('$id: CREDITS.md und lizenzen.html nennen Quelle, Urheber und Lizenz (AC-02)', (pack) => {
    const row = credits.split('\n').find((l) => l.startsWith(`| ${pack.id} |`));
    expect(row, 'Zeile in CREDITS.md').toBeDefined();
    expect(row).toContain(pack.source);
    expect(row).toContain(pack.license);
    expect(row).toContain(pack.artist.split(' (')[0]!);
    expect(lizenzen, 'lizenzen.html').toContain(pack.source);
    expect(lizenzen).toContain(pack.artist.split(' (')[0]!);
    expect(lizenzen).toContain(pack.license === 'CC BY 3.0' ? 'CC BY 3.0' : 'CC0 1.0');
  });

  it('ein Pack mit abweichender Lizenz trägt einen Hinweis', () => {
    for (const pack of GRAFIK_PACKS.filter((p) => p.license !== 'CC0 1.0')) expect(pack.licenseNote, pack.id).toBeTruthy();
  });
});
