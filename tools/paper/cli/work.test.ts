// B-737: gen im Arbeitsordner: rendert in `work`, überträgt nur geänderte Dateien nach `wurzel`, räumt Veraltetes ab.
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { genMitWork } from './work';

const SPECS = join(import.meta.dirname, '../fixtures/specs');

function aufbau() {
  const t = mkdtempSync(join(tmpdir(), 'paper-work-')), wurzel = join(t, 'grafik'), work = join(t, 'work');
  mkdirSync(join(wurzel, 'bauten'), { recursive: true });
  copyFileSync(join(SPECS, 'mauer.paper.json'), join(wurzel, 'bauten', 'mauer.paper.json'));
  copyFileSync(join(import.meta.dirname, '../../../public/grafik/stil.paper.json'), join(wurzel, 'stil.paper.json'));
  return { wurzel, work };
}

describe('genMitWork', { timeout: 60_000 }, () => {
  it('rendert im Arbeitsordner und legt Bilder, Manifest, Index und golden.json in der Wurzel ab; die Spec bleibt unverändert', async () => {
    const { wurzel, work } = aufbau(), spec = readFileSync(join(wurzel, 'bauten', 'mauer.paper.json'));
    const r = await genMitWork(wurzel, work);
    expect(r.gruppen).toEqual(['bauten']);
    expect(r.geschrieben).toBeGreaterThan(0);
    for (const f of ['bauten/mauer/manifest.json', 'bauten/mauer/stand.png', 'bauten/index.paper.json', 'bauten/golden.json']) {
      expect(existsSync(join(wurzel, f)), f).toBe(true);
      expect(existsSync(join(work, f)), `work/${f}`).toBe(true);
    }
    expect(readFileSync(join(wurzel, 'bauten', 'mauer.paper.json')).equals(spec)).toBe(true);
  });

  it('zweiter Lauf schreibt nichts mehr (nur geänderte Dateien)', async () => {
    const { wurzel, work } = aufbau();
    await genMitWork(wurzel, work);
    const m = statSync(join(wurzel, 'bauten/mauer/stand.png')).mtimeMs;
    const r = await genMitWork(wurzel, work);
    expect(r).toMatchObject({ geschrieben: 0, entfernt: 0 });
    expect(statSync(join(wurzel, 'bauten/mauer/stand.png')).mtimeMs).toBe(m);
  });

  it('veraltete Dateien im Asset-Ordner der Wurzel werden entfernt, fremde Ordner ohne Spec bleiben', async () => {
    const { wurzel, work } = aufbau();
    await genMitWork(wurzel, work);
    writeFileSync(join(wurzel, 'bauten/mauer/alt.png'), 'x');
    mkdirSync(join(wurzel, 'bauten/fremd'));
    writeFileSync(join(wurzel, 'bauten/fremd/a.txt'), 'x');
    const r = await genMitWork(wurzel, work);
    expect(r.entfernt).toBe(1);
    expect(existsSync(join(wurzel, 'bauten/mauer/alt.png'))).toBe(false);
    expect(existsSync(join(wurzel, 'bauten/fremd/a.txt'))).toBe(true);
  });
});
