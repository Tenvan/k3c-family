// Attrappe der Render-CLI (Vertrag docs/standards/paper-spec.md › CLI-Vertrag, docs/standards/paper-mcp.md › Attrappen):
// beantwortet validate, render, frame, diff, pixeldiff (und batch) deterministisch, ohne Renderer. Bekannte Fixtures (monarch, pferd,
// mauer) bekommen die PNGs aus fixtures/render/, gültige unbekannte Specs ein Platzhalter-PNG, ungültige Exit 1.
// In-Process: run(args) → {code, stdout, stderr}; Shell: bun tools/paper/fixtures/cli-stub.ts <befehl> …
import { copyFileSync, existsSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { basename, join, relative, resolve } from 'node:path';
import { batch, schritteAus } from '../cli/batch';
import { checkSpecFile, readJson } from '../schema/check';
import { box } from '../schema/form';
import { stilHash } from '../schema/hash';
import type { Spec, Stil } from '../schema/types';
import { pixeldiff, platzhalter } from './cli-pixel';

export const ROOT = resolve(import.meta.dirname, '../../..');
const RENDER = join(import.meta.dirname, 'render');
export const PLATZHALTER = join(RENDER, 'platzhalter.png');
/** Pfad relativ zum Repo mit `/`; außerhalb des Repos absolut. */
const rel = (p: string) => {
  const r = relative(ROOT, resolve(p));
  return (r.startsWith('..') ? resolve(p) : r).replace(/\\/g, '/');
};

export type Ergebnis = { code: 0 | 1 | 2; stdout: string; stderr: string };
const ok = (wert: object): Ergebnis => ({ code: 0, stdout: JSON.stringify({ ok: true, ...wert }), stderr: '' });
const abgelehnt = (fehler: object[]): Ergebnis => ({ code: 1, stdout: JSON.stringify({ ok: false, fehler }), stderr: '' });
class Aufruf extends Error {}

/** Fixture-PNG je Asset und Animation (nur Frame 0 vorhanden). */
const BILDER: Record<string, (animation: string, stufe: number) => string> = {
  monarch: (a) => `monarch/${a === 'reiten' ? 'monarch-galopp-0' : 'monarch-idle-0'}.png`,
  pferd: () => 'pferd/pferd-galopp-0.png',
  mauer: (_, s) => `mauer/mauer-${s >= 2 ? 2 : 1}.png`,
};

function optionen(args: string[]): { pos: string[]; opt: Record<string, string> } {
  const pos: string[] = [], opt: Record<string, string> = {};
  for (let i = 0; i < args.length; i++) {
    if (!args[i].startsWith('--')) pos.push(args[i]);
    else if (i + 1 < args.length) opt[args[i].slice(2)] = args[++i];
    else throw new Aufruf(`Wert für ${args[i]} fehlt`);
  }
  return { pos, opt };
}

const stil = (pfad = join(ROOT, 'public/grafik/stil.paper.json')) => JSON.parse(readFileSync(pfad, 'utf8')) as Stil;
const datei = (p: string | undefined, was: string) => {
  if (!p || !existsSync(p)) throw new Aufruf(`${was} fehlt: ${p ?? '–'}`);
  return p;
};

function validate(pos: string[]): Ergebnis {
  const fehler = checkSpecFile(datei(pos[0], 'Spec'), stil());
  return fehler.length ? abgelehnt(fehler) : ok({ fehler: [] });
}

function render(pos: string[]): Ergebnis {
  const ziel = datei(pos[0], 'Spec oder Ordner');
  const specs = statSync(ziel).isDirectory()
    ? readdirSync(ziel).filter((f) => f.endsWith('.paper.json') && f !== 'index.paper.json').sort().map((f) => join(ziel, f))
    : [ziel];
  const fehler = specs.flatMap((s) => checkSpecFile(s, stil()).map((f) => ({ ...f, feld: `${basename(s)}:${f.feld}` })));
  if (fehler.length) return abgelehnt(fehler);
  const ids = specs.map((s) => basename(s, '.paper.json'));
  const manifeste = ids.filter((id) => Object.hasOwn(BILDER, id)).map((id) => `tools/paper/fixtures/render/${id}/manifest.json`);
  const ausgabe = (id: string) => Object.values(JSON.parse(readFileSync(join(RENDER, id, 'manifest.json'), 'utf8')).animationen as Record<string, { datei: string }>)
    .map((a) => `tools/paper/fixtures/render/${id}/${a.datei}`);
  const dateien = ids.flatMap((id) => (Object.hasOwn(BILDER, id) ? ausgabe(id) : [rel(PLATZHALTER)]));
  return ok({ manifeste, dateien: [...new Set(dateien)] });
}

/** Teil-Boxen ohne Pose: Rahmen der eigenen Lagen, linear vom Spec-Rahmen auf die Frame-Pixel. */
function teilBoxen(spec: Spec): Record<string, number[]> {
  const [rx, ry, , rh] = spec.ausgabe.rahmen, s = spec.ausgabe.hoehe_px / rh;
  const out: Record<string, number[]> = {};
  for (const t of spec.teile) {
    const b = box(t.lagen.map((l) => (l.art === 'punkt' ? `M${l.mitte[0]} ${l.mitte[1]}` : l.form)));
    if (b) out[t.id] = [b.x0 - rx, b.y0 - ry, b.x1 - b.x0, b.y1 - b.y0].map((v) => Math.round(v * s));
  }
  return out;
}

function frame(pos: string[], opt: Record<string, string>): Ergebnis {
  const pfad = datei(pos[0], 'Spec');
  if (!opt.animation || opt.frame === undefined || !opt.out) throw new Aufruf('frame braucht --animation, --frame und --out');
  const fehler = checkSpecFile(pfad, stil());
  if (fehler.length) return abgelehnt(fehler);
  const spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
  if (!spec.animationen?.[opt.animation])
    return abgelehnt([{ feld: 'animationen', regel: 'animation-unbekannt', text: `Animation „${opt.animation}“ fehlt (${Object.keys(spec.animationen ?? {}).join(', ') || 'keine'})` }]);
  const bild = Object.hasOwn(BILDER, spec.id) ? BILDER[spec.id] : undefined;
  if (bild) copyFileSync(join(RENDER, bild(opt.animation, Number(opt.stufe ?? 1))), opt.out);
  else writeFileSync(opt.out, platzhalter(Math.round(spec.ausgabe.hoehe_px * spec.ausgabe.rahmen[2] / spec.ausgabe.rahmen[3]), spec.ausgabe.hoehe_px));
  return ok({ datei: rel(opt.out), teile: teilBoxen(spec) });
}

type Feld = { pfad: string; alt: unknown; neu: unknown };

/** Feld-Diff: Objekte je Schlüssel, `teile` über `id` (Pfad mit Index im neuen bzw. alten Stand), sonst Listen je Index. */
function felder(alt: unknown, neu: unknown, pfad: string, out: Feld[]) {
  const istObj = (v: unknown) => v !== null && typeof v === 'object' && !Array.isArray(v);
  if (Array.isArray(alt) && Array.isArray(neu)) {
    for (let i = 0; i < Math.max(alt.length, neu.length); i++) felder(alt[i], neu[i], `${pfad}[${i}]`, out);
  } else if (istObj(alt) && istObj(neu)) {
    const a = alt as Record<string, unknown>, n = neu as Record<string, unknown>;
    for (const k of [...new Set([...Object.keys(a), ...Object.keys(n)])].filter((k) => k !== '$schema' && !(pfad === '' && k === 'teile')))
      felder(a[k], n[k], pfad ? `${pfad}.${k}` : k, out);
  } else if (JSON.stringify(alt) !== JSON.stringify(neu)) out.push({ pfad, alt: alt ?? null, neu: neu ?? null });
}

function teileDiff(alt: Spec, neu: Spec, out: Feld[]): string[] {
  const geaendert = new Set<string>();
  const ids = [...new Set([...neu.teile.map((t) => t.id), ...alt.teile.map((t) => t.id)])];
  for (const id of ids) {
    const ia = alt.teile.findIndex((t) => t.id === id), ineu = neu.teile.findIndex((t) => t.id === id);
    const vorher = out.length;
    felder(alt.teile[ia], neu.teile[ineu], `teile[${ineu >= 0 ? ineu : ia}]`, out);
    if (out.length > vorher) geaendert.add(id);
  }
  return [...geaendert];
}

function diff(pos: string[], opt: Record<string, string>): Ergebnis {
  const [a, n] = [readJson(datei(pos[0], 'alte Spec')), readJson(datei(pos[1], 'neue Spec'))];
  if (a.fehler.length || n.fehler.length) return abgelehnt([...a.fehler, ...n.fehler]);
  const out: Feld[] = [];
  felder(a.wert, n.wert, '', out);
  const teile = teileDiff(a.wert as Spec, n.wert as Spec, out);
  const st = opt['stil-alt'] && opt['stil-neu'] ? stilHash(stil(datei(opt['stil-alt'], 'Stil'))) !== stilHash(stil(datei(opt['stil-neu'], 'Stil'))) : false;
  return ok({ felder: out, teile, stil: st });
}

function pixel(pos: string[], opt: Record<string, string>): Ergebnis {
  if (!opt.out) throw new Aufruf('pixeldiff braucht --out');
  const d = pixeldiff(readFileSync(datei(pos[0], 'PNG a')), readFileSync(datei(pos[1], 'PNG b')));
  writeFileSync(opt.out, d.overlay);
  return ok({ anteil: d.anteil, boxen: d.boxen, datei: rel(opt.out) });
}

/** Sammelaufruf wie paper.ts (B-856). */
const sammel = (pos: string[]): Ergebnis => ok({ schritte: batch(schritteAus(datei(pos[0], 'Schritte')), run) });

const BEFEHLE: Record<string, (pos: string[], opt: Record<string, string>) => Ergebnis> = { validate, render, frame, diff, pixeldiff: pixel, batch: sammel };

/** Führt einen Befehl aus; Aufruffehler → Exit 2 mit Text auf stderr, nichts auf stdout. */
export function run(args: string[]): Ergebnis {
  try {
    const befehl = BEFEHLE[args[0]];
    if (!befehl) throw new Aufruf(`unbekannter Befehl „${args[0] ?? ''}“ (${Object.keys(BEFEHLE).join(', ')})`);
    const { pos, opt } = optionen(args.slice(1));
    return befehl(pos, opt);
  } catch (e) {
    return { code: 2, stdout: '', stderr: (e as Error).message };
  }
}

if (import.meta.main) {
  const r = run(process.argv.slice(2));
  if (r.stdout) process.stdout.write(r.stdout + '\n');
  if (r.stderr) process.stderr.write(r.stderr + '\n');
  process.exit(r.code);
}
