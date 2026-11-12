// B-751: gen:paper parallel und mit Cache je Asset; Ergebnis bitgleich zum Rendern nacheinander.
import { copyFileSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { Worker } from 'node:worker_threads';
import { describe, expect, it } from 'vitest';
import { rendereAsset, sha256 } from '../render/ausgabe';
import { ladeStil } from '../render/stil';
import type { Spec } from '../schema/types';
import { gen } from './gen';
import { cacheGueltig, cacheMerken, codeStand } from './genCache';
import { genParallel, rendereAssetParallel } from './genParallel';
import { jobs, RasterPool } from './pool';
import { genMitWork } from './work';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const STIL = join(import.meta.dirname, '../../../public/grafik/stil.paper.json');

function wurzel() {
  const w = mkdtempSync(join(tmpdir(), 'paper-par-'));
  for (const [gruppe, ids] of [['bauten', ['mauer', 'langarm']], ['figuren', ['monarch']], ['reittiere', ['pferd']]] as const) {
    mkdirSync(join(w, gruppe));
    for (const id of ids) copyFileSync(join(SPECS, `${id}.paper.json`), join(w, gruppe, `${id}.paper.json`));
  }
  copyFileSync(STIL, join(w, 'stil.paper.json'));
  return w;
}

function stand(dir: string, pre = ''): Record<string, string> {
  const out: Record<string, string> = {};
  for (const f of readdirSync(dir).sort()) {
    const p = join(dir, f);
    if (statSync(p).isDirectory()) Object.assign(out, stand(p, `${pre}${f}/`));
    else if (!f.startsWith('.')) out[`${pre}${f}`] = sha256(readFileSync(p));
  }
  return out;
}

describe('genParallel', { timeout: 120_000 }, () => {
  it('rendereAssetParallel gleicht rendereAsset: Manifest und PNGs bitgleich', async () => {
    const stil = ladeStil(STIL), pool = new RasterPool(3);
    try {
      for (const id of ['mauer', 'langarm', 'pferd', 'monarch']) {
        const pfad = join(SPECS, `${id}.paper.json`), spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
        const soll = rendereAsset(spec, { stil, specPfad: pfad }), ist = await rendereAssetParallel(spec, { stil, specPfad: pfad }, pool, id);
        expect(JSON.stringify(ist.manifest), id).toBe(JSON.stringify(soll.manifest));
        expect(ist.pngs.map(([d, b]) => [d, sha256(b)]), id).toEqual(soll.pngs.map(([d, b]) => [d, sha256(b)]));
      }
    } finally {
      await pool.schliessen();
    }
  });

  it('schreibt dieselben Dateien wie gen (nacheinander)', async () => {
    const a = wurzel(), b = wurzel();
    gen(a);
    expect(await genParallel(b, { jobs: 3 })).toEqual({ gruppen: ['bauten', 'figuren', 'reittiere'], gerendert: 4, ausCache: 0 });
    expect(stand(b)).toEqual(stand(a));
  });

  it('Fortschritt: ⏳ vor 🔨 je Asset, ↳ je Ausgabe nur bei mehr als 4 Ausgaben (B-749, B-750)', async () => {
    const zeilen: string[] = [];
    await genParallel(wurzel(), { jobs: 2, melde: (z) => zeilen.push(z) });
    expect(zeilen[0]).toBe('🚀 gen:paper: 3 Gruppen, 4 Assets (4 rendern, 0 aus dem Cache, 2 Worker)');
    for (const a of ['bauten/mauer', 'bauten/langarm', 'figuren/monarch', 'reittiere/pferd']) {
      const start = zeilen.findIndex((z) => z.startsWith('⏳') && z.endsWith(` ${a} rendert …`));
      expect(start, a).toBeGreaterThan(0);
      expect(zeilen.findIndex((z) => new RegExp(`^🔨 \\[\\d/4\\] ${a} gerendert ✅ `).test(z))).toBeGreaterThan(start);
    }
    const unter = zeilen.filter((z) => z.startsWith('   ↳'));
    expect(unter).toHaveLength(6);
    expect(unter.every((z) => /^ {3}↳ [1-6]\/6 figuren\/monarch: \S+ \d/.test(z))).toBe(true);
    expect(zeilen.filter((z) => z.startsWith('💾'))).toHaveLength(3);
  });

  it('Cache: zweiter Lauf rendert 0 Assets, geänderte Spec nur sich, neuer Renderer-Code alles (B-751/AC-01)', async () => {
    const t = mkdtempSync(join(tmpdir(), 'paper-cache-')), w = join(t, 'grafik'), work = join(t, 'work');
    mkdirSync(join(w, 'bauten'), { recursive: true });
    for (const id of ['mauer', 'langarm']) copyFileSync(join(SPECS, `${id}.paper.json`), join(w, 'bauten', `${id}.paper.json`));
    copyFileSync(STIL, join(w, 'stil.paper.json'));
    expect(await genMitWork(w, work)).toMatchObject({ gerendert: 2, ausCache: 0 });
    expect(await genMitWork(w, work)).toMatchObject({ gerendert: 0, ausCache: 2, geschrieben: 0 });
    const spec = JSON.parse(readFileSync(join(w, 'bauten/mauer.paper.json'), 'utf8'));
    writeFileSync(join(w, 'bauten/mauer.paper.json'), JSON.stringify({ ...spec, version: spec.version + 1 }));
    expect(await genMitWork(w, work)).toMatchObject({ gerendert: 1, ausCache: 1 });
    cacheMerken(work, 'anderer Code');
    expect(await genMitWork(w, work)).toMatchObject({ gerendert: 2, ausCache: 0 });
    expect(cacheGueltig(work, codeStand())).toBe(true);
  });

  it('jobs: Kerne minus 1, höchstens 8, mindestens 1; K3C_PAPER_JOBS übersteuert', () => {
    expect([jobs({}, 1), jobs({}, 4), jobs({}, 22)]).toEqual([1, 3, 8]);
    expect(jobs({ K3C_PAPER_JOBS: '12' }, 4)).toBe(12);
    expect(() => jobs({ K3C_PAPER_JOBS: 'viele' })).toThrow(/K3C_PAPER_JOBS/);
  });

  it('stürzt ein Worker ab, scheitert der Auftrag mit seinem Namen', async () => {
    const pool = new RasterPool(1, () => new Worker('process.exit(3)', { eval: true }));
    await expect(pool.raster('figuren/wolf:lauf', () => '<svg/>')).rejects.toThrow(/Worker abgestürzt bei figuren\/wolf:lauf: Exit 3/);
    await pool.schliessen();
  });
});
