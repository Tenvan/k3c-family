// Render-CLI der Papierschnitt-Grafik (Vertrag docs/standards/paper-spec.md › 8). Antwort: ein JSON-Objekt auf stdout;
// Exit 0 ok, 1 fachlich abgelehnt ({ok: false, fehler}), 2 Aufruf- oder Systemfehler (Text auf stderr).
// In-Process: run(args, {stil?}) → {code, stdout, stderr}; Shell: bun tools/paper/bin/paper.ts <befehl> …
import { existsSync, statSync } from 'node:fs';
import { checkSpecFile } from '../schema/check';
import type { Stil } from '../schema/types';
import { renderOrdner, renderSpec } from '../render/ausgabe';
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

const validate: Befehl = (pos, _opt, o) => {
  const fehler = checkSpecFile(datei(pos[0], 'Spec'), o.stil ?? ladeStil());
  if (fehler.length) throw new Abgelehnt(fehler);
  return { fehler: [] };
};

const render: Befehl = (pos, _opt, o) => {
  const ziel = datei(pos[0], 'Spec oder Ordner');
  return statSync(ziel).isDirectory() ? renderOrdner(ziel, o) : renderSpec(ziel, o);
};

const folgt = (name: string): Befehl => () => {
  throw new Aufruf(`${name} folgt in RD1.4`);
};

export const BEFEHLE: Record<string, Befehl> = {
  validate, render, frame: folgt('frame'), diff: folgt('diff'), pixeldiff: folgt('pixeldiff'),
};

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
