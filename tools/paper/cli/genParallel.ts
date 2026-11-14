// gen:paper auf mehreren Kernen mit Cache je Asset (B-751). Das Ergebnis gleicht `gen` (nacheinander) Byte für Byte:
// Manifest und PNGs wie render/ausgabe.ts › rendereAsset, nur rastert resvg in Worker-Threads (pool.ts). Ändert sich dort
// der Aufbau des Manifests, muss er hier mit; der Gleichheitstest in genParallel.test.ts schlägt dann an.
import { readFileSync, renameSync, writeFileSync } from 'node:fs';
import { basename, join } from 'node:path';
import { type Asset, schreibe, specsIn } from '../render/ausgabe';
import { bildVon, type Kontext } from '../render/bild';
import { Abgelehnt } from '../render/fehler';
import { frameGroesse, frameHashes, type FrameGroesse, streifenSvg } from '../render/raster';
import { animationen, ausgaben } from '../render/stufen';
import { defs } from '../render/svg';
import { ausgabeFps } from '../rig/takt';
import { checkSpecFile } from '../schema/check';
import { RENDERER_VERSION, specHash, stilHash } from '../schema/hash';
import type { Spec, Stil } from '../schema/types';
import { dauer, type Melde } from './fortschritt';
import { schreibeGolden, wurzelLesen } from './gen';
import { aktuell, cacheGueltig, cacheMerken, codeStand, schluessel } from './genCache';
import { jobs, RasterPool } from './pool';

export type Ereignisse = { start?: () => void; ausgabe?: (nr: number, n: number, schluessel: string, ms: number) => void };
type Teil = { schluessel: string; animation: string; png: Buffer; hashes: string[]; frames: number; g: FrameGroesse };

/** Alle Ausgaben einer Spec über den Pool; Manifest wie rendereAsset (gleiche Schlüssel und Reihenfolge). */
export async function rendereAssetParallel(spec: Spec, k: Kontext, pool: RasterPool, name: string, e: Ereignisse = {}): Promise<Asset> {
  const basis = frameGroesse(spec.ausgabe), alle = ausgaben(spec);
  let fertig = 0, gestartet = false;
  const teile = await Promise.all(alle.map(async (a): Promise<Teil> => {
    let t0 = 0, frames = 0, g = basis;
    const r = await pool.raster(`${name}:${a.schluessel}`, () => {
      if (!gestartet) (gestartet = true, e.start?.());
      t0 = performance.now();
      const bild = bildVon(spec, a, k);
      (g = bild.groesse), (frames = bild.frames.length);
      return streifenSvg(bild.frames.map((f) => f.svg), g, defs(k.stil, spec.id, g.frameH / g.rahmen[3]));
    });
    const hashes = frameHashes(r.rgba, r.breite, g.frameW, g.frameH, frames);
    e.ausgabe?.(++fertig, alle.length, a.schluessel, performance.now() - t0);
    return { schluessel: a.schluessel, animation: a.animation, png: r.png, hashes, frames, g };
  }));
  const anim: Record<string, unknown> = {};
  for (const { schluessel: s, animation, hashes, frames, g } of teile) {
    const eigen = g.eigen || g.frameW !== basis.frameW || g.frameH !== basis.frameH ? { rahmen: g.rahmen, frameW: g.frameW, frameH: g.frameH } : {};
    anim[s] = { frames, fps: ausgabeFps(spec, animation, animationen(spec)[animation], frames), datei: `${s}.png`, hashes, ...eigen };
  }
  const manifest = { id: spec.id, version: spec.version, specHash: specHash(spec), stilHash: stilHash(k.stil), renderer: RENDERER_VERSION,
    renderKey: schluessel(spec, k.specPfad!, k.stil), ...basis, animationen: anim, varianten: Object.keys(spec.varianten ?? {}).sort() };
  return { manifest, pngs: teile.map((t): [string, Buffer] => [`${t.schluessel}.png`, t.png]) };
}

type Gruppe = { g: string; dir: string; pfade: string[]; specs: Spec[]; neu: number[] };

/** Specs aller Gruppen prüfen (wie renderOrdner, aber vor dem ersten Rendern) und lesen; `neu` = zu rendernde Indizes. */
function plane(wurzel: string, gruppen: string[], stil: Stil, mitCache: boolean): Gruppe[] {
  const plan = gruppen.map((g) => ({ g, dir: join(wurzel, g), pfade: specsIn(join(wurzel, g)) }));
  const fehler = plan.flatMap(({ pfade }) => pfade.flatMap((p) => checkSpecFile(p, stil).map((f) => ({ ...f, feld: `${basename(p)}:${f.feld}` }))));
  if (fehler.length) throw new Abgelehnt(fehler);
  return plan.map((x) => {
    const specs = x.pfade.map((p) => JSON.parse(readFileSync(p, 'utf8')) as Spec);
    const neu = specs.map((_, i) => i).filter((i) => !mitCache || !aktuell(join(x.dir, specs[i].id), schluessel(specs[i], x.pfade[i], stil)));
    return { ...x, specs, neu };
  });
}

function schreibeIndex(dir: string, specs: Spec[]): void {
  const tmp = join(dir, `.index.paper.json.tmp-${process.pid}`);
  writeFileSync(tmp, JSON.stringify(specs.map((s) => ({ id: s.id, rig: s.rig, manifest: `${s.id}/manifest.json` })), null, 2) + '\n');
  renameSync(tmp, join(dir, 'index.paper.json'));
}

export type ParallelOptionen = { stil?: Stil; melde?: Melde; /** Cache je Asset (nur im Arbeitsordner) */ cache?: boolean; jobs?: number };
export type ParallelErgebnis = { gruppen: string[]; gerendert: number; ausCache: number };

/** Wie `gen`, aber parallel und mit Cache; Fortschritt: ⏳ beim Start, ↳ je Ausgabe großer Assets, 🔨 je fertigem Asset. */
export async function genParallel(wurzel: string, o: ParallelOptionen = {}): Promise<ParallelErgebnis> {
  const { melde } = o, { stil, gruppen } = wurzelLesen(wurzel, o.stil);
  const code = o.cache ? codeStand() : '', plan = plane(wurzel, gruppen, stil, !!o.cache && cacheGueltig(wurzel, code));
  const gesamt = plan.reduce((n, x) => n + x.specs.length, 0), todo = plan.reduce((n, x) => n + x.neu.length, 0);
  // Worker nach Ausgaben, nicht nach Assets: ändert sich nur der Monarch, rastern trotzdem alle; ohne Arbeit keine Worker
  const auftraege = plan.reduce((n, x) => x.neu.reduce((m, i) => m + ausgaben(x.specs[i]).length, n), 0);
  const anzahl = Math.min(o.jobs ?? jobs(), auftraege);
  melde?.(`🚀 gen:paper: ${gruppen.length} Gruppen, ${gesamt} Assets (${todo} rendern, ${gesamt - todo} aus dem Cache, ${anzahl} Worker)`);
  const pool = new RasterPool(anzahl);
  try {
    await renderePlan(plan, stil, pool, todo, melde);
  } finally {
    await pool.schliessen();
  }
  if (o.cache) cacheMerken(wurzel, code);
  return { gruppen, gerendert: todo, ausCache: gesamt - todo };
}

/** Alle zu rendernden Assets über den Pool; je Gruppe schreiben, sobald ihre Assets fertig sind (Index und golden.json immer). */
async function renderePlan(plan: Gruppe[], stil: Stil, pool: RasterPool, todo: number, melde?: Melde): Promise<void> {
  let gestartet = 0, fertig = 0;
  const asset = async (x: Gruppe, i: number) => {
    const id = x.specs[i].id, name = `${x.g}/${id}`;
    let t0 = 0; // ab dem Start des ersten Auftrags, nicht ab dem Einreihen
    const a = await rendereAssetParallel(x.specs[i], { stil, specPfad: x.pfade[i] }, pool, name, {
      start: () => ((t0 = performance.now()), melde?.(`⏳ [${++gestartet}/${todo}] ${name} rendert …`)),
      // Assets mit vielen Ausgaben (Monarch: 195) melden jede einzelne, kleine nur sich selbst (B-750)
      ausgabe: (nr, n, key, ms) => n > 4 && melde?.(`   ↳ ${nr}/${n} ${name}: ${key} ${dauer(ms)}`),
    });
    melde?.(`🔨 [${++fertig}/${todo}] ${name} gerendert ✅ ${dauer(performance.now() - t0)}`);
    return { ordner: join(x.dir, id), asset: a };
  };
  await Promise.all(plan.map(async (x) => {
    const ziele = await Promise.all(x.neu.map((i) => asset(x, i)));
    if (x.neu.length < x.specs.length) melde?.(`📂 ${x.g}: ${x.specs.length - x.neu.length} Assets aus dem Cache`);
    schreibe(ziele);
    schreibeIndex(x.dir, x.specs);
    schreibeGolden(x.dir, x.g, melde);
  }));
}
