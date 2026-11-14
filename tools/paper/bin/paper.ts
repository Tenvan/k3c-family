// Render-CLI der Papierschnitt-Grafik (Vertrag docs/standards/paper-spec.md › 8). Antwort: ein JSON-Objekt auf stdout;
// Exit 0 ok, 1 fachlich abgelehnt ({ok: false, fehler}), 2 Aufruf- oder Systemfehler (Text auf stderr).
// In-Process: run(args, {stil?}) → {code, stdout, stderr}; Shell: bun tools/paper/bin/paper.ts <befehl> …
import { existsSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { basename } from 'node:path';
import { checkSpecFile, checkStil, readJson } from '../schema/check';
import { stilHash } from '../schema/hash';
import type { Spec, Stil } from '../schema/types';
import { batch, schritteAus } from '../cli/batch';
import { diffSpecs } from '../cli/diff';
import { frameRendern } from '../cli/frame';
import { pixeldiff } from '../cli/pixeldiff';
import { pruefeRenderRegeln } from '../rig/pruefen';
import { rel, renderOrdner, renderSpec } from '../render/ausgabe';
import { Abgelehnt, Aufruf } from '../render/fehler';
import { ladeStil } from '../render/stil';

export type Ergebnis = { code: 0 | 1 | 2; stdout: string; stderr: string };
export type Optionen = { stil?: Stil };
type Befehl = (pos: string[], opt: Record<string, string>, o: Optionen) => object;

const ok = (wert: object): Ergebnis => ({ code: 0, stdout: JSON.stringify({ ok: true, ...wert }), stderr: '' });

export function optionen(args: string[]): { pos: string[]; opt: Record<string, string> } {
  const pos: string[] = [], opt: Record<string, string> = {};
  for (let i = 0; i < args.length; i++) {
    if (!args[i].startsWith('--')) pos.push(args[i]);
    else if (i + 1 < args.length) opt[args[i].slice(2)] = args[++i];
    else throw new Aufruf(`Wert für ${args[i]} fehlt`);
  }
  return { pos, opt };
}

/** Pfad muss existieren, sonst Aufruffehler. */
export function datei(p: string | undefined, was: string): string {
  if (!p || !existsSync(p)) throw new Aufruf(`${was} fehlt: ${p ?? '–'}`);
  return p;
}

/** `--stil <datei>` (B-705, nur frame und render): fehlt → Exit 2, kein JSON oder Schema verletzt → Exit 1 mit Dateinamen im Feld. */
export function stilAus(pfad: string): Stil {
  const { wert, fehler } = readJson(datei(pfad, 'Stil'));
  const alle = fehler.length ? fehler : checkStil(wert);
  if (alle.length) throw new Abgelehnt(alle.map((f) => ({ ...f, feld: `${basename(pfad)}:${f.feld}` })));
  return wert as Stil;
}

/** Stil des Aufrufs: `--stil` vor dem In-Process-Stil vor `public/grafik/stil.paper.json`. */
const stilFuer = (opt: Record<string, string>, o: Optionen): Stil => (opt.stil ? stilAus(opt.stil) : o.stil ?? ladeStil());

/** Schema- und Fachregeln (check.ts), danach die Render-Regeln der Rigs als Trockenprüfung. */
const validate: Befehl = (pos, _opt, o) => {
  const pfad = datei(pos[0], 'Spec'), stil = o.stil ?? ladeStil(), fehler = checkSpecFile(pfad, stil);
  if (fehler.length) throw new Abgelehnt(fehler);
  const render = pruefeRenderRegeln(JSON.parse(readFileSync(pfad, 'utf8')) as Spec, pfad, stil);
  if (render.length) throw new Abgelehnt(render);
  return { fehler: [] };
};

const render: Befehl = (pos, opt, o) => {
  const ziel = datei(pos[0], 'Spec oder Ordner'), mit = { ...o, stil: stilFuer(opt, o) };
  return statSync(ziel).isDirectory() ? renderOrdner(ziel, mit) : renderSpec(ziel, mit);
};

/** Optionswert als ganze Zahl ≥ 0, sonst Aufruffehler (der Bereich wird fachlich geprüft). */
function ganzzahl(wert: string, name: string): number {
  if (!/^\d+$/.test(wert)) throw new Aufruf(`--${name} braucht eine ganze Zahl ≥ 0, nicht „${wert}“`);
  return Number(wert);
}

const frame: Befehl = (pos, opt, o) => {
  const pfad = datei(pos[0], 'Spec');
  if (!opt.animation || opt.frame === undefined || !opt.out) throw new Aufruf('frame braucht --animation, --frame und --out');
  const stil = stilFuer(opt, o), fehler = checkSpecFile(pfad, stil);
  if (fehler.length) throw new Abgelehnt(fehler);
  const spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
  const ohne = opt.ohne ? opt.ohne.split(',').filter(Boolean) : undefined;
  const wahl = { animation: opt.animation, frame: ganzzahl(opt.frame, 'frame'), variante: opt.variante, stufe: ganzzahl(opt.stufe ?? '1', 'stufe'), ohne };
  return { datei: rel(opt.out), teile: frameRendern(spec, wahl, stil, pfad, opt.out) };
};

const diff: Befehl = (pos, opt) => {
  const [a, n] = [readJson(datei(pos[0], 'alte Spec')), readJson(datei(pos[1], 'neue Spec'))];
  if (a.fehler.length || n.fehler.length) throw new Abgelehnt([...a.fehler, ...n.fehler]);
  const stilVon = (p: string) => stilHash(readFileSync(datei(p, 'Stil'), 'utf8'));
  const stil = opt['stil-alt'] && opt['stil-neu'] ? stilVon(opt['stil-alt']) !== stilVon(opt['stil-neu']) : false;
  return { ...diffSpecs(a.wert as Spec, n.wert as Spec), stil };
};

const pixel: Befehl = (pos, opt) => {
  if (!opt.out) throw new Aufruf('pixeldiff braucht --out');
  const d = pixeldiff(readFileSync(datei(pos[0], 'PNG a')), readFileSync(datei(pos[1], 'PNG b')));
  writeFileSync(opt.out, d.overlay);
  return { anteil: d.anteil, boxen: d.boxen, datei: rel(opt.out) };
};

/** Sammelaufruf (B-856): Schritte aus der Datei nacheinander in diesem Prozess, Antwort je Schritt. */
const sammel: Befehl = (pos, _opt, o) => ({ schritte: batch(schritteAus(datei(pos[0], 'Schritte')), (a) => run(a, o)) });

export const BEFEHLE: Record<string, Befehl> = { validate, render, frame, diff, pixeldiff: pixel, batch: sammel };

/** Führt einen Befehl aus; Aufruffehler → Exit 2 mit Text auf stderr, nichts auf stdout. */
export function run(args: string[], o: Optionen = {}): Ergebnis {
  try {
    const befehl = Object.hasOwn(BEFEHLE, args[0] ?? '') ? BEFEHLE[args[0]] : undefined;
    if (!befehl) throw new Aufruf(`unbekannter Befehl „${args[0] ?? ''}“ (${Object.keys(BEFEHLE).join(', ')})`);
    const { pos, opt } = optionen(args.slice(1));
    return ok(befehl(pos, opt, o));
  } catch (e) {
    if (e instanceof Abgelehnt) return { code: 1, stdout: JSON.stringify({ ok: false, fehler: e.fehler }), stderr: '' };
    return { code: 2, stdout: '', stderr: (e as Error).message };
  }
}

if ((import.meta as { main?: boolean }).main) {
  const r = run(process.argv.slice(2));
  if (r.stdout) process.stdout.write(r.stdout + '\n');
  if (r.stderr) process.stderr.write(r.stderr + '\n');
  process.exitCode = r.code;
}
