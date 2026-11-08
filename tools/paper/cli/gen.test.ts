// RD1.4: gen rendert alle Gruppen einer Wurzel, je Gruppe index.paper.json und golden.json, kein gemeinsamer Index.
import { copyFileSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { sha256 } from '../render/ausgabe';
import { dauer } from './fortschritt';
import { gen } from './gen';
import { genMitWork, nurKopieren, ohneManifest } from './work';

const SPECS = join(import.meta.dirname, '../fixtures/specs');

function wurzel() {
  const w = mkdtempSync(join(tmpdir(), 'paper-gen-'));
  for (const [gruppe, ids] of [['bauten', ['mauer']], ['reittiere', ['pferd']], ['leer', []]] as const) {
    mkdirSync(join(w, gruppe));
    for (const id of ids) copyFileSync(join(SPECS, `${id}.paper.json`), join(w, gruppe, `${id}.paper.json`));
  }
  mkdirSync(join(w, 'altes-pack'));
  writeFileSync(join(w, 'altes-pack', 'x.json'), '{}');
  writeFileSync(join(w, 'stil.paper.json'), readFileSync(join(import.meta.dirname, '../../../public/grafik/stil.paper.json')));
  return w;
}

/** Alle Dateien unter einem Ordner mit SHA-256 (für den Vergleich zweier Läufe). */
function stand(dir: string, pre = ''): Record<string, string> {
  const out: Record<string, string> = {};
  for (const f of readdirSync(dir).sort()) {
    const p = join(dir, f);
    if (statSync(p).isDirectory()) Object.assign(out, stand(p, `${pre}${f}/`));
    else out[`${pre}${f}`] = sha256(readFileSync(p));
  }
  return out;
}

describe('gen', { timeout: 60_000 }, () => {
  it('gen schreibt je Gruppe index.paper.json und golden.json, keinen gemeinsamen Index', () => {
    const w = wurzel(), r = gen(w);
    expect(r.gruppen).toEqual(['bauten', 'reittiere']);
    expect(readdirSync(join(w, 'bauten')).sort()).toEqual(['golden.json', 'index.paper.json', 'mauer', 'mauer.paper.json']);
    const golden = JSON.parse(readFileSync(join(w, 'bauten/golden.json'), 'utf8'));
    const manifest = JSON.parse(readFileSync(join(w, 'bauten/mauer/manifest.json'), 'utf8'));
    expect(golden).toEqual({ mauer: { renderKey: manifest.renderKey, dateien: {
      'stand.png': sha256(readFileSync(join(w, 'bauten/mauer/stand.png'))),
      'stand.s2.png': sha256(readFileSync(join(w, 'bauten/mauer/stand.s2.png'))) } } });
    expect(readdirSync(w).filter((f) => f.endsWith('.json'))).toEqual(['stil.paper.json']);
  }, 60_000);

  it('Gruppen ohne *.paper.json werden übersprungen', () => {
    const w = wurzel();
    gen(w);
    expect(readdirSync(join(w, 'leer'))).toEqual([]);
    expect(readdirSync(join(w, 'altes-pack'))).toEqual(['x.json']);
  }, 60_000);

  it('meldet je Asset genau eine Zeile [i/n] mit Dauer (B-740)', () => {
    const zeilen: string[] = [];
    gen(wurzel(), { melde: (z) => zeilen.push(z) });
    expect(zeilen[0]).toBe('🚀 gen:paper: 2 Gruppen, 2 Assets');
    const assets = zeilen.filter((z) => z.startsWith('🔨'));
    expect(assets).toHaveLength(2);
    expect(assets[0]).toMatch(/^🔨 \[1\/2\] bauten\/mauer gerendert ✅ \d+(,\d)? (ms|s)$/);
    expect(assets[1]).toMatch(/^🔨 \[2\/2\] reittiere\/pferd gerendert ✅ /);
    expect(zeilen.filter((z) => z.startsWith('💾'))).toHaveLength(2);
  }, 60_000);

  it('dauer formatiert ms, Sekunden und Minuten', () => {
    expect([dauer(80), dauer(2300), dauer(252_000)]).toEqual(['80 ms', '2,3 s', '4 min 12 s']);
  });

  it('fehlende Grafik in public wird ohne Neu-Rendern aus .work kopiert (B-748)', () => {
    const w = wurzel(), work = join(mkdtempSync(join(tmpdir(), 'paper-work-')), 'grafik');
    expect(ohneManifest(w).sort()).toEqual(['bauten/mauer', 'reittiere/pferd']);
    genMitWork(w, work);
    expect(ohneManifest(w)).toEqual([]);
    rmSync(join(w, 'bauten/mauer'), { recursive: true });
    expect(ohneManifest(w)).toEqual(['bauten/mauer']);
    expect(nurKopieren(w, work).geschrieben).toBeGreaterThan(0);
    expect(ohneManifest(w)).toEqual([]);
  }, 90_000);

  it('zwei Läufe hintereinander: keine Datei ändert sich', () => {
    const w = wurzel();
    gen(w);
    const vorher = stand(w);
    gen(w);
    expect(stand(w)).toEqual(vorher);
  }, 90_000);
});
