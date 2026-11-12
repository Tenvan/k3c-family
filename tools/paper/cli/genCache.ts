// Cache je Asset (B-751): Ein Asset rendert nicht neu, wenn sein Manifest denselben renderKey trägt (Spec, Stil, Reittier,
// Renderer-Version) und alle PNGs da sind. Zusätzlich gilt der Cache nur für denselben Stand des Renderer-Codes: Die
// Renderer-Version wird nicht bei jeder Code-Änderung erhöht, ein Hash über die Quellen fängt das ab.
import { createHash } from 'node:crypto';
import { existsSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { reittierHash } from '../rig/reiten';
import { renderKey, RENDERER_VERSION, specHash, stilHash } from '../schema/hash';
import type { Spec, Stil } from '../schema/types';
import { ROOT } from '../render/stil';

/** renderKey wie im Manifest (render/ausgabe.ts › rendereAsset), ohne zu rendern. */
export function schluessel(spec: Spec, specPfad: string, stil: Stil): string {
  const sh = specHash(spec), reittier = reittierHash(spec, specPfad);
  return renderKey(reittier ? `${sh}+${reittier}` : sh, stilHash(stil));
}

/** Liegt im Asset-Ordner ein Manifest mit `key` und sind alle seine PNGs da? */
export function aktuell(ordner: string, key: string): boolean {
  try {
    const m = JSON.parse(readFileSync(join(ordner, 'manifest.json'), 'utf8')) as { renderKey?: string; animationen?: Record<string, { datei: string }> };
    return m.renderKey === key && Object.values(m.animationen ?? {}).every((a) => existsSync(join(ordner, a.datei)));
  } catch {
    return false;
  }
}

const PAPER = join(ROOT, 'tools/paper');
const QUELLEN = ['render', 'rig', 'schema', 'cli'];
// Aus cli/ zählen nur die Dateien, die Pixel oder Manifest bestimmen (nicht Fortschritt, Kopieren, golden.json)
const zaehlt = (ordner: string, f: string) =>
  /\.(ts|json)$/.test(f) && !f.endsWith('.test.ts') && (ordner !== 'cli' || /^(genParallel|pool\.worker)\.ts$/.test(f));

/** Hash über die Renderer-Quellen und die resvg-Version (Zeilenenden egal). */
export function codeStand(paper = PAPER): string {
  const h = createHash('sha256').update(RENDERER_VERSION);
  for (const o of QUELLEN)
    for (const f of readdirSync(join(paper, o), { recursive: true, encoding: 'utf8' }).map((x) => x.replace(/\\/g, '/')).filter((x) => zaehlt(o, x)).sort())
      h.update(`${o}/${f}\0`).update(readFileSync(join(paper, o, f), 'utf8').replace(/\r\n/g, '\n'));
  try {
    h.update(JSON.parse(readFileSync(join(ROOT, 'node_modules/@resvg/resvg-js/package.json'), 'utf8')).version);
  } catch {
    // ohne node_modules (sollte nicht vorkommen): nur die Quellen
  }
  return h.digest('hex');
}

const DATEI = '.gen-cache.json';

/** Gilt der Cache unter `wurzel` für `code`? Sonst wird die Marke gelöscht, bis ein Lauf mit diesem Code fertig ist. */
export function cacheGueltig(wurzel: string, code: string): boolean {
  try {
    if ((JSON.parse(readFileSync(join(wurzel, DATEI), 'utf8')) as { code?: string }).code === code) return true;
  } catch {
    // fehlt oder kaputt
  }
  rmSync(join(wurzel, DATEI), { force: true });
  return false;
}

export function cacheMerken(wurzel: string, code: string): void {
  writeFileSync(join(wurzel, DATEI), JSON.stringify({ code }) + '\n');
}
