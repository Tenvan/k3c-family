// RD1.1: render schreibt Ausgabe-Layout, Manifest und Gruppen-Index atomar (paper-spec.md › 7, 8).
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { RENDERER_VERSION, renderKey, specHash, stilHash } from '../schema/hash';
import { rel, renderOrdner, renderSpec, schreibe } from './ausgabe';
import { Abgelehnt, Aufruf } from './fehler';
import { ladeStil, ROOT } from './stil';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const stil = ladeStil();
const tmp = () => mkdtempSync(join(tmpdir(), 'paper-render-'));
const mitMauer = () => {
  const dir = tmp();
  copyFileSync(join(SPECS, 'mauer.paper.json'), join(dir, 'mauer.paper.json'));
  return dir;
};
const lies = (p: string) => JSON.parse(readFileSync(p, 'utf8'));

describe('render', () => {
  it('render mauer: manifest.json nach Vertrag, Dateien stand.png und stand.s2.png', () => {
    const dir = mitMauer();
    const r = renderSpec(join(dir, 'mauer.paper.json'), { stil });
    expect(readdirSync(join(dir, 'mauer')).sort()).toEqual(['manifest.json', 'stand.png', 'stand.s2.png']);
    const m = lies(join(dir, 'mauer/manifest.json'));
    const sh = specHash(readFileSync(join(dir, 'mauer.paper.json'), 'utf8')), st = stilHash(stil);
    expect(Object.keys(m)).toEqual(['id', 'version', 'specHash', 'stilHash', 'renderer', 'renderKey', 'rahmen', 'frameW', 'frameH', 'animationen', 'varianten']);
    expect(m).toMatchObject({ id: 'mauer', version: 1, specHash: sh, stilHash: st, renderer: RENDERER_VERSION, renderKey: renderKey(sh, st),
      rahmen: [-66, -112, 132, 116], frameW: 146, frameH: 128, varianten: [] });
    expect(m.animationen.stand).toMatchObject({ frames: 1, fps: 8, datei: 'stand.png' });
    expect(m.animationen['stand.s2'].hashes).toHaveLength(1);
    expect(m.animationen['stand.s2'].hashes[0]).not.toBe(m.animationen.stand.hashes[0]);
    expect(r.dateien.map((d) => d.split('/').pop())).toEqual(['stand.png', 'stand.s2.png']);
  });

  it('render ordner: alle Specs außer index.paper.json, schreibt Gruppen-Index [{id, rig, manifest}]', () => {
    const dir = mitMauer();
    writeFileSync(join(dir, 'index.paper.json'), '[]');
    const r = renderOrdner(dir, { stil });
    expect(r.manifeste).toHaveLength(1);
    expect(lies(join(dir, 'index.paper.json'))).toEqual([{ id: 'mauer', rig: 'gebaeude', manifest: 'mauer/manifest.json' }]);
  });

  it('render ist atomar: bei Fehler bleibt der alte Ausgabeordner', () => {
    const dir = mitMauer();
    renderOrdner(dir, { stil });
    const vorher = readFileSync(join(dir, 'mauer/manifest.json'), 'utf8');
    const m = lies(join(dir, 'mauer.paper.json'));
    m.version = 2;
    writeFileSync(join(dir, 'mauer.paper.json'), JSON.stringify(m));
    copyFileSync(join(SPECS, 'ungueltig/hex-farbe.paper.json'), join(dir, 'kaputt.paper.json'));
    expect(() => renderOrdner(dir, { stil })).toThrow(Abgelehnt);
    expect(readFileSync(join(dir, 'mauer/manifest.json'), 'utf8')).toBe(vorher);
    expect(readdirSync(dir).filter((f) => f.startsWith('.'))).toEqual([]);
  });

  it('zweiter Render ersetzt den alten Ordner ohne Reste', () => {
    const dir = mitMauer();
    renderSpec(join(dir, 'mauer.paper.json'), { stil });
    writeFileSync(join(dir, 'mauer/alt.png'), 'x');
    renderSpec(join(dir, 'mauer.paper.json'), { stil });
    expect(existsSync(join(dir, 'mauer/alt.png'))).toBe(false);
    expect(readdirSync(dir).sort()).toEqual(['mauer', 'mauer.paper.json']);
  });

  it('Pfade in der Antwort relativ zum Repo mit /', () => {
    expect(rel(join(ROOT, 'public', 'grafik', 'x.png'))).toBe('public/grafik/x.png');
    expect(rel(join(tmpdir(), 'a', 'b.png'))).not.toContain('\\');
  });
});

describe('render: Fehlerfälle beim Schreiben (Review RD1.1)', () => {
  it('Spec ohne Endung .paper.json: Aufruffehler, Datei bleibt', () => {
    const dir = tmp();
    copyFileSync(join(SPECS, 'mauer.paper.json'), join(dir, 'mauer'));
    expect(() => renderSpec(join(dir, 'mauer'), { stil })).toThrow(Aufruf);
    expect(existsSync(join(dir, 'mauer'))).toBe(true);
  });

  it('Tausch scheitert: alte Stände kommen zurück, keine Temp-Reste', () => {
    const dir = mitMauer();
    renderSpec(join(dir, 'mauer.paper.json'), { stil });
    const vorher = readFileSync(join(dir, 'mauer/manifest.json'), 'utf8');
    const asset = { manifest: { id: 'mauer' }, pngs: [] as [string, Buffer][] };
    let n = 0;
    const umbenennen = (a: string, b: string) => {
      if (++n === 2) throw new Error('EBUSY');
      renameSync(a, b);
    };
    expect(() => schreibe([{ ordner: join(dir, 'mauer'), asset }], umbenennen)).toThrow('EBUSY');
    expect(readFileSync(join(dir, 'mauer/manifest.json'), 'utf8')).toBe(vorher);
    expect(readdirSync(dir).filter((f) => f.startsWith('.'))).toEqual([]);
  });

  it('Reste früherer Läufe (.id.tmp-*, .id.alt-*) werden beim nächsten Render aufgeräumt', () => {
    const dir = mitMauer();
    mkdirSync(join(dir, '.mauer.tmp-999'));
    mkdirSync(join(dir, '.mauer.alt-999'));
    renderSpec(join(dir, 'mauer.paper.json'), { stil });
    expect(readdirSync(dir).filter((f) => f.startsWith('.'))).toEqual([]);
  });
});
