// RD1.4: gen rendert alle Gruppen einer Wurzel, je Gruppe index.paper.json und golden.json, kein gemeinsamer Index.
import { copyFileSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { sha256 } from '../render/ausgabe';
import { gen } from './gen';

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

  it('zwei Läufe hintereinander: keine Datei ändert sich', () => {
    const w = wurzel();
    gen(w);
    const vorher = stand(w);
    gen(w);
    expect(stand(w)).toEqual(vorher);
  }, 90_000);
});
