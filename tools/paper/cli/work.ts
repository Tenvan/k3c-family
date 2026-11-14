// gen:paper im Arbeitsordner (B-737): Specs und Stil werden nach `.work/grafik/` gespiegelt, dort wird gerendert, und nur
// geänderte Ergebnisdateien (PNG, manifest.json, index.paper.json, golden.json) werden nach `public/grafik/` kopiert.
// Die Asset-Ordner in `public/grafik/<gruppe>/<asset>/` sind in .gitignore; eingecheckt bleiben Specs, Indizes und golden.json.
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { ROOT } from '../render/stil';
import { dauer, schritt, type Melde } from './fortschritt';
import { genParallel, type ParallelErgebnis } from './genParallel';

export const WORK = join(ROOT, '.work/grafik');

/** Schreibt `ziel` nur, wenn sich der Inhalt unterscheidet; true = geschrieben. */
function kopiere(quelle: string, ziel: string): boolean {
  if (existsSync(ziel) && readFileSync(ziel).equals(readFileSync(quelle))) return false;
  mkdirSync(join(ziel, '..'), { recursive: true });
  cpSync(quelle, ziel);
  return true;
}

const ordner = (p: string) => existsSync(p) && statSync(p).isDirectory();
const gruppenIn = (wurzel: string) => readdirSync(wurzel).filter((g) => ordner(join(wurzel, g)) && !g.startsWith('.')).sort();

/** Specs und Stil nach `work` spiegeln (Ergebnisse aus früheren Läufen bleiben dort liegen). */
export function spiegeln(wurzel: string, work: string): void {
  mkdirSync(work, { recursive: true });
  kopiere(join(wurzel, 'stil.paper.json'), join(work, 'stil.paper.json'));
  for (const g of gruppenIn(wurzel)) {
    const specs = readdirSync(join(wurzel, g)).filter((f) => f.endsWith('.paper.json') && f !== 'index.paper.json');
    if (!specs.length) continue;
    for (const f of specs) kopiere(join(wurzel, g, f), join(work, g, f));
    // Specs, die es in `wurzel` nicht mehr gibt, auch im Arbeitsordner entfernen
    if (ordner(join(work, g)))
      for (const f of readdirSync(join(work, g)).filter((x) => x.endsWith('.paper.json') && x !== 'index.paper.json' && !specs.includes(x)))
        rmSync(join(work, g, f), { force: true });
  }
}

/** Ergebnisse von `work` nach `wurzel` kopieren, veraltete Dateien in den Asset-Ordnern löschen; Anzahl geschriebener Dateien. */
export function uebertragen(work: string, wurzel: string): { geschrieben: number; entfernt: number } {
  let geschrieben = 0, entfernt = 0;
  for (const g of gruppenIn(work)) {
    for (const f of ['golden.json', 'index.paper.json']) if (existsSync(join(work, g, f)) && kopiere(join(work, g, f), join(wurzel, g, f))) geschrieben++;
    const assets = readdirSync(join(work, g)).filter((a) => ordner(join(work, g, a)));
    for (const a of assets) {
      const quelle = join(work, g, a), ziel = join(wurzel, g, a), dateien = readdirSync(quelle);
      if (!existsSync(join(work, g, `${a}.paper.json`))) continue; // Ordner ohne Spec: Überbleibsel
      for (const d of dateien) if (kopiere(join(quelle, d), join(ziel, d))) geschrieben++;
      if (ordner(ziel))
        for (const d of readdirSync(ziel).filter((x) => !dateien.includes(x))) {
          rmSync(join(ziel, d), { force: true });
          entfernt++;
        }
    }
  }
  return { geschrieben, entfernt };
}

/** Spiegeln, rendern (parallel, unveränderte Assets aus dem Cache in `work`, B-751), übertragen. */
export async function genMitWork(wurzel: string, work = WORK, melde?: Melde, o: { jobs?: number } = {}): Promise<ParallelErgebnis & { geschrieben: number; entfernt: number }> {
  const t0 = performance.now();
  schritt(melde, '📂', 'Specs spiegeln', () => spiegeln(wurzel, work));
  const g = await genParallel(work, { melde, cache: true, jobs: o.jobs });
  const r = schritt(melde, '📦', 'nach public/grafik übertragen', () => uebertragen(work, wurzel));
  melde?.(`✅ gen:paper fertig in ${dauer(performance.now() - t0)}: ${g.gerendert} Assets gerendert, ${g.ausCache} aus dem Cache, ${r.geschrieben} Dateien geschrieben, ${r.entfernt} entfernt`);
  return { ...g, ...r };
}

/** Nur kopieren (B-748): `.work` ist vollständig, `public/grafik` nicht (frischer Checkout, gelöschte Ordner); kein Rendern. */
export function nurKopieren(wurzel: string, work = WORK, melde?: Melde): { geschrieben: number; entfernt: number } {
  const r = schritt(melde, '📦', 'nach public/grafik übertragen (ohne Rendern)', () => uebertragen(work, wurzel));
  melde?.(`✅ ${r.geschrieben} Dateien geschrieben, ${r.entfernt} entfernt`);
  return r;
}

/** Specs unter `wurzel` (`<gruppe>/<asset>.paper.json`), zu denen `<gruppe>/<asset>/manifest.json` fehlt (B-748). */
export function ohneManifest(wurzel: string): string[] {
  return gruppenIn(wurzel).flatMap((g) =>
    readdirSync(join(wurzel, g))
      .filter((f) => f.endsWith('.paper.json') && f !== 'index.paper.json')
      .map((f) => `${g}/${f.slice(0, -'.paper.json'.length)}`)
      .filter((a) => !existsSync(join(wurzel, g, a.split('/')[1]!, 'manifest.json'))));
}
