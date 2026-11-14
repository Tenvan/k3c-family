// gen:paper: rendert jede Gruppe (Unterordner mit *.paper.json) einer Wurzel und schreibt je Gruppe den Gruppen-Index
// und golden.json (paper-spec.md › 1, 6). Es gibt keinen gemeinsamen Index. Stil-Datei: <wurzel>/stil.paper.json.
import { existsSync, readdirSync, readFileSync, renameSync, statSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import type { Stil } from '../schema/types';
import { renderOrdner, sha256, specsIn } from '../render/ausgabe';
import { ladeStil } from '../render/stil';
import { dauer, schritt, type Melde } from './fortschritt';

type Manifest = { id: string; renderKey: string; animationen: Record<string, { datei: string }> };

/** golden.json einer Gruppe aus den Manifesten und PNG-Dateien neben den Specs. */
function golden(gruppe: string): Record<string, { renderKey: string; dateien: Record<string, string> }> {
  const out: Record<string, { renderKey: string; dateien: Record<string, string> }> = {};
  for (const spec of specsIn(gruppe)) {
    const ordner = spec.slice(0, -'.paper.json'.length);
    const m = JSON.parse(readFileSync(join(ordner, 'manifest.json'), 'utf8')) as Manifest;
    const dateien = Object.values(m.animationen).map((a) => a.datei);
    out[m.id] = { renderKey: m.renderKey, dateien: Object.fromEntries(dateien.map((d) => [d, sha256(readFileSync(join(ordner, d)))])) };
  }
  return out;
}

/** golden.json einer Gruppe atomar schreiben, als Schritt `💾` gemeldet. */
export function schreibeGolden(dir: string, g: string, melde?: Melde): void {
  schritt(melde, '💾', `${g}: golden.json`, () => {
    const tmp = join(dir, `.golden.json.tmp-${process.pid}`);
    writeFileSync(tmp, JSON.stringify(golden(dir), null, 2) + '\n');
    renameSync(tmp, join(dir, 'golden.json'));
  });
}

/** Stil der Wurzel (`<wurzel>/stil.paper.json`, sonst der Standard) und die Gruppen mit Specs, sortiert. */
export function wurzelLesen(wurzel: string, stil?: Stil): { stil: Stil; gruppen: string[] } {
  const stilPfad = join(wurzel, 'stil.paper.json');
  return {
    stil: stil ?? (existsSync(stilPfad) ? ladeStil(stilPfad) : ladeStil()),
    gruppen: readdirSync(wurzel).sort().filter((g) => statSync(join(wurzel, g)).isDirectory() && specsIn(join(wurzel, g)).length > 0),
  };
}

/** Nacheinander auf einem Kern (Referenz für den Golden-Test); gen:paper nutzt genParallel. Fortschritt (B-740):
 * `[i/n] gruppe/id gerendert ✅ Dauer` je Asset, je Gruppe eine Zeile für golden/Index. */
export function gen(wurzel: string, o: { stil?: Stil; melde?: Melde } = {}): { gruppen: string[] } {
  const { melde } = o;
  const { stil, gruppen } = wurzelLesen(wurzel, o.stil);
  const gesamt = gruppen.reduce((n, g) => n + specsIn(join(wurzel, g)).length, 0);
  melde?.(`🚀 gen:paper: ${gruppen.length} Gruppen, ${gesamt} Assets`);
  let nr = 0;
  for (const g of gruppen) {
    const dir = join(wurzel, g);
    renderOrdner(dir, {
      stil,
      vorAsset: (id) => melde?.(`⏳ [${nr + 1}/${gesamt}] ${g}/${id} rendert …`),
      // Assets mit vielen Ausgaben (Monarch: über 2 min) melden jede einzelne, kleine nur sich selbst
      beiAusgabe: (id, nr, n, key, ms) => n > 4 && melde?.(`   ↳ ${nr}/${n} ${g}/${id}: ${key} ${dauer(ms)}`),
      beiAsset: (id, ms) => melde?.(`🔨 [${++nr}/${gesamt}] ${g}/${id} gerendert ✅ ${dauer(ms)}`),
    });
    schreibeGolden(dir, g, melde);
  }
  return { gruppen };
}
