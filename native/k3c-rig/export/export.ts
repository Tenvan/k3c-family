// Teile-Export des Rig-Prototyps (B-778/AC-01, B-789/AC-03, `task native:rig:export`): Teil-PNGs, Rig-Dateien,
// Elemente und Sollwerte aller Figuren mit Teile-Baum (catalog.ts) nach native/k3c-rig/assets/<gruppe>/<id>/. Liest
// Specs und Stil nur; tools/paper und gen:paper bleiben unberührt.
// Aufruf: bun native/k3c-rig/export/export.ts [--pruefen]   (--pruefen: nichts schreiben, mit dem Bestand vergleichen)
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import type { Spec, Stil } from '../../../tools/paper/schema/types';
import { ladeStil } from '../../../tools/paper/render/stil';
import { groesse } from '../../../tools/paper/render/png';
import { type Eintrag, katalog, REITER, REITTIERE } from './catalog';
import { exportElemente } from './elements';
import { exportFigur, figurKontext } from './figure';
import { sollFigur, sollGang, sollReiten, sollText } from './expected';
import type { Dateien } from './types';
import { exportVierbeiner } from './quadruped';

const ASSETS = resolve(import.meta.dirname, '../assets');
const json = (wert: unknown) => JSON.stringify(wert, null, 2) + '\n';

/** figur: Rig und Sollwerte je eigener Animation; der Reiter dazu seine Elemente. */
function figur(e: Eintrag, stil: Stil, dateien: Dateien) {
  const reiter = e.ordner === REITER, rig = exportFigur(e.spec, stil, dateien, e.ordner, reiter);
  dateien.set(`${e.ordner}/rig.json`, json(rig));
  if (reiter) dateien.set('elemente/elemente.json', json(exportElemente(e.spec, rig, figurKontext(e.spec, stil), dateien)));
  for (const a of Object.keys(rig.animationen)) dateien.set(`soll/${e.ordner}/${a}.json`, sollText(sollFigur(e.spec, a)));
}

/** vierbeiner: Rig und Sollwerte je Gangart-Animation; ein Reittier dazu je Reit-Animation des Reiters (`reiten-<gangart>`). */
function vierbeiner(e: Eintrag, reiter: Spec, stil: Stil, dateien: Dateien) {
  const traegt = e.gruppe === REITTIERE, rig = exportVierbeiner(e.spec, traegt ? reiter : undefined, stil, dateien, e.ordner);
  dateien.set(`${e.ordner}/rig.json`, json(rig));
  for (const [a, anim] of Object.entries(rig.animationen)) dateien.set(`soll/${e.ordner}/${a}.json`, sollText(sollGang(e.spec, a, anim)));
  if (!traegt) return;
  const reiten = Object.entries(reiter.animationen ?? {}).filter(([, a]) => a.reittier === e.spec.id && !a.ebene && a.gangart);
  if (!reiten.length) throw new Error(`${e.ordner}: ${reiter.id} hat keine Reit-Animation für dieses Reittier`);
  for (const [a, anim] of reiten) dateien.set(`soll/${e.ordner}/reiten-${anim.gangart}.json`, sollText(sollReiten(reiter, e.spec, a)));
}

function baue(): Dateien {
  const stil = ladeStil(), alle = katalog(), dateien: Dateien = new Map();
  const reiter = alle.find((e) => e.ordner === REITER)?.spec;
  if (!reiter) throw new Error(`Reiter ${REITER} fehlt in public/grafik`);
  for (const e of alle) (e.spec.rig === 'figur' ? figur(e, stil, dateien) : vierbeiner(e, reiter, stil, dateien));
  return dateien;
}

/** Alle Dateien unter `ordner`, relativ mit `/`. */
function bestand(ordner: string, vor = ''): string[] {
  if (!existsSync(ordner)) return [];
  return readdirSync(ordner, { withFileTypes: true }).flatMap((e) =>
    (e.isDirectory() ? bestand(join(ordner, e.name), `${vor}${e.name}/`) : [`${vor}${e.name}`]));
}

/** Text ohne CRLF (Git checkt Text unter Windows mit CRLF aus), Bilder byte-genau. */
const inhalt = (b: Buffer | string) => (typeof b === 'string' ? b : b.toString('latin1'));
const gleich = (pfad: string, soll: Buffer | string) => {
  if (!existsSync(pfad)) return false;
  const ist = readFileSync(pfad);
  return typeof soll === 'string' ? ist.toString('utf8').replace(/\r\n/g, '\n') === soll : inhalt(ist) === inhalt(soll);
};

function pruefe(dateien: Dateien): string[] {
  const fehlt = [...dateien].filter(([p, b]) => !gleich(join(ASSETS, p), b)).map(([p]) => `weicht ab oder fehlt: ${p}`);
  const zuviel = bestand(ASSETS).filter((p) => !dateien.has(p)).map((p) => `nicht mehr im Export: ${p}`);
  return [...fehlt, ...zuviel];
}

/** Schreibt den Export und entfernt Dateien, die er nicht mehr enthält (der Ordner selbst bleibt: unter Windows oft gesperrt). */
function schreibe(dateien: Dateien) {
  for (const p of bestand(ASSETS)) if (!dateien.has(p)) rmSync(join(ASSETS, p));
  for (const [p, b] of dateien) {
    mkdirSync(dirname(join(ASSETS, p)), { recursive: true });
    writeFileSync(join(ASSETS, p), b);
  }
}

const mib = (b: number) => Math.round((b / 1048576) * 100) / 100;
const rgbaVon = (b: Buffer) => groesse(b).breite * groesse(b).hoehe * 4;

/** Kennzahlen: Anzahl und Bytes der PNGs, RGBA-Speicher (Breite · Höhe · 4) in MiB, gesamt und je Figur (Texturbudget). */
function kennzahlen(dateien: Dateien) {
  const pngs = [...dateien].filter((e): e is [string, Buffer] => e[0].endsWith('.png'));
  const bytes = [...dateien.values()].reduce((s, b) => s + Buffer.byteLength(b), 0);
  const je: Record<string, { pngs: number; bytes: number; rgba: number }> = {};
  for (const [p, b] of pngs) {
    const f = (je[p.split('/').slice(0, -1).join('/')] ??= { pngs: 0, bytes: 0, rgba: 0 });
    f.pngs++, (f.bytes += b.length), (f.rgba += rgbaVon(b));
  }
  const figuren = Object.fromEntries(Object.entries(je).map(([o, f]) => [o, { pngs: f.pngs, bytes: f.bytes, rgba_mib: mib(f.rgba) }]));
  const rgba = pngs.reduce((s, [, b]) => s + rgbaVon(b), 0);
  return { dateien: dateien.size, pngs: pngs.length, bytes, rgba_mib: mib(rgba), figuren };
}

const dateien = baue();
if (process.argv.includes('--pruefen')) {
  const fehler = pruefe(dateien);
  console.log(JSON.stringify({ ok: fehler.length === 0, fehler, ...kennzahlen(dateien) }));
  process.exit(fehler.length ? 1 : 0);
}
schreibe(dateien);
console.log(JSON.stringify({ ok: true, ziel: 'native/k3c-rig/assets', ...kennzahlen(dateien) }));
