// Teile-Export des Rig-Prototyps (B-778/AC-01, `task native:rig:export`): Teil-PNGs, Rig-Dateien, Elemente und
// Sollwerte nach native/k3c-rig/assets/. Liest Specs und Stil nur; tools/paper und gen:paper bleiben unberührt.
// Aufruf: bun native/k3c-rig/export/export.ts [--pruefen]   (--pruefen: nichts schreiben, mit dem Bestand vergleichen)
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import type { Spec } from '../../../tools/paper/schema/types';
import { ladeStil, ROOT } from '../../../tools/paper/render/stil';
import { groesse } from '../../../tools/paper/render/png';
import { exportElemente } from './elements';
import { exportFigur, figurKontext } from './figure';
import { sollFigur, sollReiten, sollText } from './expected';
import type { Dateien } from './types';
import { exportVierbeiner } from './quadruped';

const ASSETS = resolve(import.meta.dirname, '../assets');
const lese = (pfad: string) => JSON.parse(readFileSync(join(ROOT, pfad), 'utf8')) as Spec;
const json = (wert: unknown) => JSON.stringify(wert, null, 2) + '\n';

function baue(): Dateien {
  const stil = ladeStil(), monarch = lese('public/grafik/figuren/monarch.paper.json'), pferd = lese('public/grafik/reittiere/pferd.paper.json');
  const dateien: Dateien = new Map();
  const figur = exportFigur(monarch, stil, dateien);
  dateien.set('monarch/rig.json', json(figur));
  dateien.set('pferd/rig.json', json(exportVierbeiner(pferd, monarch, stil, dateien)));
  dateien.set('elemente/elemente.json', json(exportElemente(monarch, figur, figurKontext(monarch, stil), dateien)));
  for (const a of ['idle', 'lauf']) dateien.set(`soll/monarch-${a}.json`, sollText(sollFigur(monarch, a)));
  dateien.set('soll/reiten-trab.json', sollText(sollReiten(monarch, pferd, 'trab')));
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

/** Kennzahlen: Anzahl und Bytes der PNGs, RGBA-Speicher (Breite · Höhe · 4) in MiB. */
function kennzahlen(dateien: Dateien) {
  const pngs = [...dateien].filter((e): e is [string, Buffer] => e[0].endsWith('.png'));
  const rgba = pngs.reduce((s, [, b]) => s + groesse(b).breite * groesse(b).hoehe * 4, 0);
  const bytes = [...dateien.values()].reduce((s, b) => s + Buffer.byteLength(b), 0);
  return { dateien: dateien.size, pngs: pngs.length, bytes, rgba_mib: Math.round((rgba / 1048576) * 100) / 100 };
}

const dateien = baue();
if (process.argv.includes('--pruefen')) {
  const fehler = pruefe(dateien);
  console.log(JSON.stringify({ ok: fehler.length === 0, fehler, ...kennzahlen(dateien) }));
  process.exit(fehler.length ? 1 : 0);
}
schreibe(dateien);
console.log(JSON.stringify({ ok: true, ziel: 'native/k3c-rig/assets', ...kennzahlen(dateien) }));
