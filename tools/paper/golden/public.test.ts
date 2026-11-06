// RD1.4 (AC-02): Die Gruppen unter public/grafik rendern im Temp-Ordner gleich ihrer golden.json (GOLDEN_REGEL).
import { cpSync, existsSync, mkdtempSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { gen } from '../cli/gen';
import { specsIn } from '../render/ausgabe';
import { ROOT } from '../render/stil';
import { gleich } from './regel';

const GRAFIK = join(ROOT, 'public/grafik');
const gruppen = readdirSync(GRAFIK).filter((g) => statSync(join(GRAFIK, g)).isDirectory() && specsIn(join(GRAFIK, g)).length > 0).sort();
const lies = (p: string) => JSON.parse(readFileSync(p, 'utf8'));

describe('Golden public/grafik', { timeout: 60_000 }, () => {
  it('Gruppen mit Specs: figuren und reittiere', () => {
    expect(gruppen).toEqual(['figuren', 'reittiere']);
  });

  it('monarch hat idle, lauf, schritt, trab, galopp; pferd hat schritt, trab, galopp', () => {
    expect(Object.keys(lies(join(GRAFIK, 'figuren/monarch.paper.json')).animationen)).toEqual(['idle', 'lauf', 'schritt', 'trab', 'galopp']);
    expect(Object.keys(lies(join(GRAFIK, 'reittiere/pferd.paper.json')).animationen)).toEqual(['schritt', 'trab', 'galopp']);
    expect(existsSync(join(GRAFIK, 'figuren/monarch/galopp.png'))).toBe(true);
  });

  it('Render im Temp-Ordner gleich golden.json (GOLDEN_REGEL) und gleich den eingecheckten PNGs', () => {
    const w = mkdtempSync(join(tmpdir(), 'paper-public-'));
    cpSync(join(GRAFIK, 'stil.paper.json'), join(w, 'stil.paper.json'));
    for (const g of gruppen) for (const s of specsIn(join(GRAFIK, g))) cpSync(s, join(w, g, s.slice(join(GRAFIK, g).length + 1)));
    gen(w);
    for (const g of gruppen) {
      const soll = lies(join(GRAFIK, g, 'golden.json')), ist = lies(join(w, g, 'golden.json'));
      expect(ist, g).toEqual(soll);
      for (const [id, e] of Object.entries(soll as Record<string, { dateien: Record<string, string> }>))
        for (const d of Object.keys(e.dateien))
          expect(gleich(readFileSync(join(w, g, id, d)), readFileSync(join(GRAFIK, g, id, d))), `${g}/${id}/${d}`).toBe(true);
    }
  }, 180_000);
});
