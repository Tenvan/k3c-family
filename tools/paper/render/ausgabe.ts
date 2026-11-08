// render: Ausgabe-Layout, Manifest und Gruppen-Index (paper-spec.md › 7, 8). Schreibt atomar: erst in einen
// Temp-Ordner neben der Spec, dann Umbenennen; bei Fehler bleibt der alte Stand.
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, relative, resolve } from 'node:path';
import { checkSpecFile } from '../schema/check';
import { RENDERER_VERSION, renderKey, specHash, stilHash } from '../schema/hash';
import type { Spec, Stil } from '../schema/types';
import '../rig/register';
import { reittierHash } from '../rig/reiten';
import { bildVon, type Kontext } from './bild';
import { Abgelehnt, Aufruf } from './fehler';
import { animationen, ausgaben } from './stufen';
import { frameGroesse, frameHashes, rastere, streifenSvg } from './raster';
import { defs } from './svg';
import { ladeStil, ROOT } from './stil';

/** Pfad relativ zum Repo mit `/`; außerhalb des Repos absolut (ebenfalls mit `/`). */
export function rel(p: string): string {
  const r = relative(ROOT, resolve(p));
  return (r.startsWith('..') || resolve(r) === r ? resolve(p) : r).replace(/\\/g, '/');
}

export const sha256 = (b: Buffer | string) => createHash('sha256').update(b).digest('hex');

export type Optionen = { stil?: Stil; /** nach jedem gerenderten Asset einer Gruppe (B-740): Id und Dauer in ms */ beiAsset?: (id: string, ms: number) => void };
export type Asset = { manifest: Record<string, unknown>; pngs: [string, Buffer][] };

/** Rendert alle Ausgaben einer Spec im Speicher. */
export function rendereAsset(spec: Spec, k: Kontext): Asset {
  const sh = specHash(spec), st = stilHash(k.stil), reittier = reittierHash(spec, k.specPfad), pngs: [string, Buffer][] = [];
  const anim: Record<string, unknown> = {};
  const basis = frameGroesse(spec.ausgabe);
  for (const a of ausgaben(spec)) {
    const bild = bildVon(spec, a, k), g = bild.groesse;
    const r = rastere(streifenSvg(bild.frames.map((f) => f.svg), g, defs(k.stil, spec.id, g.frameH / g.rahmen[3])));
    const datei = `${a.schluessel}.png`;
    pngs.push([datei, r.png]);
    const eigen = g.eigen || g.frameW !== basis.frameW || g.frameH !== basis.frameH ? { rahmen: g.rahmen, frameW: g.frameW, frameH: g.frameH } : {};
    anim[a.schluessel] = { frames: bild.frames.length, fps: animationen(spec)[a.animation].fps ?? 8, datei,
      hashes: frameHashes(r.rgba, r.breite, g.frameW, g.frameH, bild.frames.length), ...eigen };
  }
  const manifest = { id: spec.id, version: spec.version, specHash: sh, stilHash: st, renderer: RENDERER_VERSION,
    renderKey: renderKey(reittier ? `${sh}+${reittier}` : sh, st), ...basis, animationen: anim, varianten: Object.keys(spec.varianten ?? {}).sort() };
  return { manifest, pngs };
}

function pruefe(pfade: string[], stil: Stil, mitDatei: boolean) {
  const fehler = pfade.flatMap((p) => checkSpecFile(p, stil).map((f) => (mitDatei ? { ...f, feld: `${basename(p)}:${f.feld}` } : f)));
  if (fehler.length) throw new Abgelehnt(fehler);
}

type Umbenennen = (von: string, nach: string) => void;
const versteckt = (ordner: string, art: 'tmp' | 'alt') => join(dirname(ordner), `.${basename(ordner)}.${art}-${process.pid}`);
const weg = (p: string) => {
  try {
    rmSync(p, { recursive: true, force: true });
  } catch {
    // gesperrt (Windows); der nächste Lauf räumt Reste auf
  }
};

/** Läuft der Prozess noch? Reste laufender Prozesse (paralleler Render) bleiben stehen. */
function laeuft(pid: number): boolean {
  if (pid === process.pid) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (e) {
    return (e as NodeJS.ErrnoException).code === 'EPERM';
  }
}

/** Reste abgebrochener Läufe (.<id>.tmp-*, .<id>.alt-*) neben den Zielordnern löschen. */
function aufraeumen(ordner: string[]) {
  for (const o of ordner) {
    const praefixe = [`.${basename(o)}.tmp-`, `.${basename(o)}.alt-`];
    const rest = (f: string) => praefixe.some((p) => f.startsWith(p) && /^\d+$/.test(f.slice(p.length)) && !laeuft(Number(f.slice(p.length))));
    for (const f of readdirSync(dirname(o))) if (rest(f)) weg(join(dirname(o), f));
  }
}

/** Schreibt alle Assets in Temp-Ordner und tauscht sie erst danach aus; scheitert ein Schritt, kommt der alte Stand zurück. */
export function schreibe(ziele: { ordner: string; asset: Asset }[], umbenennen: Umbenennen = renameSync): string[][] {
  aufraeumen(ziele.map((z) => z.ordner));
  const tmp = ziele.map(({ ordner }) => versteckt(ordner, 'tmp'));
  const getauscht: { ordner: string; alt?: string }[] = [];
  try {
    ziele.forEach(({ asset }, i) => {
      mkdirSync(tmp[i], { recursive: true });
      for (const [datei, png] of asset.pngs) writeFileSync(join(tmp[i], datei), png);
      writeFileSync(join(tmp[i], 'manifest.json'), JSON.stringify(asset.manifest, null, 2) + '\n');
    });
    ziele.forEach(({ ordner }, i) => {
      const alt = existsSync(ordner) ? versteckt(ordner, 'alt') : undefined;
      if (alt) umbenennen(ordner, alt);
      getauscht.push({ ordner, alt });
      umbenennen(tmp[i], ordner);
    });
  } catch (e) {
    for (const { ordner, alt } of getauscht.reverse()) {
      if (!alt) continue;
      if (existsSync(ordner)) weg(ordner);
      renameSync(alt, ordner);
    }
    tmp.forEach(weg);
    throw e;
  }
  getauscht.forEach(({ alt }) => alt && weg(alt));
  return ziele.map(({ ordner, asset }) => [join(ordner, 'manifest.json'), ...asset.pngs.map(([d]) => join(ordner, d))]);
}

const lese = (p: string) => JSON.parse(readFileSync(p, 'utf8')) as Spec;
const zielOrdner = (pfad: string) => join(dirname(pfad), basename(pfad, '.paper.json'));
const antwort = (geschrieben: string[][]) =>
  ({ manifeste: geschrieben.map((g) => rel(g[0])), dateien: geschrieben.flatMap((g) => g.slice(1).map(rel)) });

export function renderSpec(pfad: string, opt: Optionen = {}) {
  if (!pfad.endsWith('.paper.json')) throw new Aufruf(`Spec muss auf .paper.json enden: ${pfad}`);
  const stil = opt.stil ?? ladeStil();
  pruefe([pfad], stil, false);
  const asset = rendereAsset(lese(pfad), { stil, specPfad: pfad });
  return antwort(schreibe([{ ordner: zielOrdner(pfad), asset }]));
}

export const specsIn = (ordner: string) => readdirSync(ordner)
  .filter((f) => f.endsWith('.paper.json') && f !== 'index.paper.json').sort().map((f) => join(ordner, f));

/** Alle Specs eines Ordners (ohne index.paper.json) und den Gruppen-Index [{id, rig, manifest}]. */
export function renderOrdner(ordner: string, opt: Optionen = {}) {
  const stil = opt.stil ?? ladeStil(), pfade = specsIn(ordner);
  pruefe(pfade, stil, true);
  const specs = pfade.map(lese);
  const ziele = pfade.map((p, i) => {
    const t0 = performance.now();
    const asset = rendereAsset(specs[i], { stil, specPfad: p });
    opt.beiAsset?.(specs[i].id, performance.now() - t0);
    return { ordner: zielOrdner(p), asset };
  });
  const geschrieben = schreibe(ziele);
  const index = specs.map((s) => ({ id: s.id, rig: s.rig, manifest: `${s.id}/manifest.json` }));
  const indexTmp = join(ordner, `.index.paper.json.tmp-${process.pid}`);
  writeFileSync(indexTmp, JSON.stringify(index, null, 2) + '\n');
  renameSync(indexTmp, join(ordner, 'index.paper.json'));
  return antwort(geschrieben);
}
