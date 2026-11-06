// Erzeugt die Fixture-Specs monarch und pferd aus dem Prototyp (docs/decisions/005/prototyp-monarch-rig.html):
// Die Zeichen-Helfer des Prototyps (part, S, shade, L, dot, ermineSpot) werden durch Varianten ersetzt, die Lagen statt
// SVG liefern; Farben werden über die Palette PAL zu Material-Namen. Pfade bleiben unverändert.
// Aufruf: bun tools/paper/fixtures/aus-prototyp.ts (schreibt fixtures/specs/*.paper.json, auch mauer und ungueltig/).
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { box } from '../schema/form';
import type { Lage, Spec, Teil } from '../schema/types';
import { mauerSpec } from './mauer';
import { negativSpecs } from './negativ';

const ROOT = join(import.meta.dirname, '../../..');
const OUT = join(import.meta.dirname, 'specs');

/** Ersatz-Helfer: liefern Marken \u0001{json}\u0002 statt SVG; Teile tragen ihren Inhalt als Text. */
const HELFER = `
const tok = (o) => '\\u0001' + JSON.stringify(o) + '\\u0002';
const part = (id, px, py, body, kids = '') => tok({ part: id, pivot: [px, py], inhalt: body + kids });
const S = (d, fill, inner = '') => tok({ art: 'flaeche', form: d, fill }) + inner;
const shade = (d, fill) => tok({ art: 'flaeche', form: d, fill, auflage: true });
const L = (d, w = 0.9, col = PAL.line) => tok({ art: 'falz', form: d, breite: w, fill: col });
const dot = (x, y, r, f) => tok({ art: 'punkt', mitte: [x, y], radius: r, fill: f });
const ermineSpot = (x, y) => tok({ art: 'punkt', mitte: [x, y + 0.5], radius: 1, fill: PAL.line });
`;

type Roh = { part?: string; pivot?: [number, number]; inhalt?: string; art?: Lage['art']; form?: string; fill?: string;
  auflage?: boolean; breite?: number; mitte?: [number, number]; radius?: number };
type Proto = { monarch: string; horseBack: string; horseFront: string; PAL: Record<string, string | string[]>;
  LEGW: { front: number[]; back: number[] }; CAPE_N: number; CAPE_SEG: number };

function ladePrototyp(): Proto {
  const html = readFileSync(join(ROOT, 'docs/decisions/005/prototyp-monarch-rig.html'), 'utf8').replace(/\r\n/g, '\n');
  let src = html.slice(html.indexOf('const C = {'), html.indexOf('// ---- Gangarten'));
  for (const name of ['part', 'shade', 'S', 'L', 'dot', 'ermineSpot']) src = src.replace(new RegExp(`^const ${name} = [\\s\\S]*?;$`, 'm'), '');
  const PAL = /const PAL = (\{[\s\S]*?\});/.exec(src)?.[1];
  const body = `const PAL = ${PAL};\n${HELFER}\n${src.replace(/const PAL = \{[\s\S]*?\};/, '')}\nreturn { monarch, horseBack, horseFront, PAL, LEGW, CAPE_N, CAPE_SEG };`;
  return new Function(body)() as Proto;
}

const MATERIAL: Record<string, string> = { skin: 'haut', hair: 'haar', gold: 'gold', royal: 'koenigsblau', red: 'koenigsrot',
  ermine: 'hermelin', leather: 'leder', cloth: 'stoff', horse: 'fell-fuchs', mane: 'maehne', hoof: 'huf', eye: 'auge', line: 'linie' };
const TOENE = ['licht', 'basis', 'schatten'] as const;

/** Hex → Material und Ton; erste Fundstelle gewinnt (#ffffff = hermelin licht). */
function farbTabelle(PAL: Proto['PAL']): Map<string, { material: string; ton?: (typeof TOENE)[number] }> {
  const map = new Map<string, { material: string; ton?: (typeof TOENE)[number] }>();
  for (const [k, v] of Object.entries(PAL)) {
    if (Array.isArray(v)) v.forEach((hex, i) => map.has(hex) || map.set(hex, { material: MATERIAL[k], ton: TOENE[i] }));
    else if (!map.has(v)) map.set(v, { material: MATERIAL[k] });
  }
  map.set('#5fcde4', { material: 'edelstein' });
  return map;
}

/** Rohes SVG zwischen den Marken: Linien-Gruppen (Zaumzeug), einzelne Pfade mit Füllung oder Strich. */
function rohLagen(raw: string): Roh[] {
  const out: Roh[] = [];
  const ohneGruppen = raw.replace(/<g fill="none" stroke="(#\w+)" stroke-width="([\d.]+)"[^>]*>([\s\S]*?)<\/g>/g, (_, col, w, inner: string) => {
    for (const m of inner.matchAll(/<path d="([^"]+)"/g)) out.push({ art: 'falz', form: m[1], breite: Number(w), fill: col });
    return '';
  });
  for (const m of ohneGruppen.matchAll(/<path d="([^"]+)"([^>]*)\/>/g)) {
    const fill = /fill="(#\w+)"/.exec(m[2])?.[1], stroke = /stroke="(#\w+)"/.exec(m[2])?.[1];
    if (fill) out.push({ art: 'flaeche', form: m[1], fill, auflage: true });
    else if (stroke) out.push({ art: 'falz', form: m[1], breite: Number(/stroke-width="([\d.]+)"/.exec(m[2])?.[1] ?? 1), fill: stroke });
  }
  return out;
}

/** Zerlegt Text in Marken und Roh-SVG in Zeichenreihenfolge; `bruch` markiert Gruppen-Grenzen (<g, </g>). */
function zerlege(text: string): (Roh | { bruch: true })[] {
  const out: (Roh | { bruch: true })[] = [];
  let pos = 0;
  // Marken nutzen Steuerzeichen, weil JSON.stringify sie in verschachtelten Teilen maskiert (\u0001 → "\\u0001").
  // oxlint-disable-next-line no-control-regex
  for (const m of text.matchAll(/\u0001(.*?)\u0002/gs)) {
    const raw = text.slice(pos, m.index);
    out.push(...rohLagen(raw));
    if (/<\/?g[\s>]/.test(raw)) out.push({ bruch: true });
    out.push(JSON.parse(m[1]) as Roh);
    pos = m.index + m[0].length;
  }
  out.push(...rohLagen(text.slice(pos)));
  return out;
}

type Ctx = { farben: ReturnType<typeof farbTabelle>; name: (id: string) => string; eltern: (id: string) => string | undefined;
  pivot: (id: string, p: [number, number]) => [number, number]; laeufe: string[]; teile: Teil[] };

function lage(r: Roh, ctx: Ctx): Lage | undefined {
  if (!r.fill || r.fill === 'none') return undefined;
  const f = ctx.farben.get(r.fill);
  if (!f) throw new Error(`Farbe ${r.fill} ohne Material`);
  const ton = f.ton && f.ton !== 'basis' ? { ton: f.ton } : {};
  if (r.art === 'punkt') return { art: 'punkt', mitte: r.mitte!, radius: r.radius!, material: f.material, ...ton };
  if (r.art === 'falz') return { art: 'falz', form: r.form!, breite: r.breite!, material: f.material, ...ton };
  return { art: 'flaeche', form: r.form!, material: f.material, ...ton, ...(r.auflage ? { auflage: true } : {}) };
}

/** Sammelt Teile in Zeichenreihenfolge; Lagen außerhalb von Teilen bilden auf oberster Ebene benannte Läufe. */
function sammle(text: string, ctx: Ctx, eltern?: string): Lage[] {
  const eigene: Lage[] = [];
  let lauf: Teil | undefined;
  for (const item of zerlege(text)) {
    if ('bruch' in item || item.part) {
      lauf = undefined;
      if ('bruch' in item) continue;
      const id = ctx.name(item.part!);
      const teil: Teil = { id, ...elternFeld(eltern ?? ctx.eltern(id)), pivot: ctx.pivot(id, item.pivot!), lagen: [] };
      ctx.teile.push(teil);
      teil.lagen = sammle(item.inhalt ?? '', ctx, id);
      continue;
    }
    const l = lage(item, ctx);
    if (!l) continue;
    if (eltern) eigene.push(l);
    else {
      if (!lauf) {
        const id = ctx.laeufe.shift() ?? 'rest';
        lauf = { id, ...elternFeld(ctx.eltern(id)), pivot: ctx.pivot(id, [0, 0]), lagen: [] };
        ctx.teile.push(lauf);
      }
      lauf.lagen.push(l);
    }
  }
  return eigene;
}

const elternFeld = (e: string | undefined) => (e ? { eltern: e } : {});

/** Rahmen [x, y, b, h] aus allen Formen plus Rand, ganzzahlig. */
function rahmen(teile: Teil[], rand: number, links = 0): [number, number, number, number] {
  const b = box(teile.flatMap((t) => t.lagen.map((l) => (l.art === 'punkt' ? `M${l.mitte[0]} ${l.mitte[1]}` : l.form))))!;
  const x = Math.floor(b.x0 - rand - links), y = Math.floor(b.y0 - rand);
  return [x, y, Math.ceil(b.x1 + rand) - x, Math.ceil(Math.max(b.y1, 0) + rand) - y];
}

export function monarchSpec(p: Proto): Spec {
  const namen: Record<string, string> = { upperB: 'oberarm-h', foreB: 'unterarm-h', thighB: 'oberschenkel-h', shinB: 'unterschenkel-h',
    thighF: 'oberschenkel-v', shinF: 'unterschenkel-v', head: 'kopf', crown: 'krone', upperF: 'oberarm-v', foreF: 'unterarm-v' };
  const pivots: Record<string, [number, number]> = { rumpf: [0, -50], schwert: [-7, -53] };
  const ctx: Ctx = { farben: farbTabelle(p.PAL), name: (id) => namen[id] ?? id, eltern: (id) => (id === 'rumpf' ? undefined : 'rumpf'),
    pivot: (id, pv) => pivots[id] ?? pv, laeufe: ['schwert', 'rumpf'], teile: [] };
  sammle(p.monarch, ctx);
  const laenge = Number(((p.CAPE_N - 1) * p.CAPE_SEG).toFixed(2));
  return {
    id: 'monarch', rig: 'figur', version: 1,
    beschreibung: 'König mit Bart, Hermelinkragen, Krone mit Edelsteinen, blauer Rock, Schwert an der Seite, roter Umhang (aus dem Prototyp 005).',
    teile: ctx.teile,
    umhang: { anker: 'rumpf', punkt: [-4, -91], material: 'koenigsrot', laenge, breite: [4, Number((4 + (p.CAPE_N - 1) * 1.9).toFixed(2))], segmente: p.CAPE_N },
    animationen: { idle: { frames: 8 }, lauf: { frames: 8 }, reiten: { frames: 8, reittier: 'pferd', gangart: 'galopp' } },
    varianten: { spieler2: { tausch: { koenigsblau: 'smaragd' } } },
    ausgabe: { hoehe_px: 192, rahmen: rahmen(ctx.teile, 4, 30) },
  };
}

export function pferdSpec(p: Proto): Spec {
  const seite: Record<string, string> = { fB: 'hf', nB: 'hn', fF: 'vf', nF: 'vn' };
  const name = (id: string) => {
    const m = /^(hu|hl|hh)(\w\w)$/.exec(id);
    if (m) return `${{ hu: 'oberbein', hl: 'unterbein', hh: 'huf' }[m[1]]}-${seite[m[2]]}`;
    return { tail: 'schweif', neck: 'hals' }[id] ?? id;
  };
  const eltern = (id: string): string | undefined =>
    id === 'rumpf-vorn' ? undefined : id === 'schweif' || id.startsWith('oberbein-h') ? 'rumpf-hinten' : 'rumpf-vorn';
  const pivots: Record<string, [number, number]> = { 'rumpf-hinten': [0, -66], 'rumpf-vorn': [0, -80], sattel: [0, -78] };
  const ctx: Ctx = { farben: farbTabelle(p.PAL), name, eltern, pivot: (id, pv) => pivots[id] ?? pv,
    laeufe: ['rumpf-hinten', 'rumpf-vorn', 'sattel'], teile: [] };
  sammle(p.horseBack + p.horseFront, ctx);
  return {
    id: 'pferd', rig: 'vierbeiner', version: 1,
    beschreibung: 'Fuchsfarbenes Reitpferd mit dunkler Mähne, blauer Satteldecke mit Goldborte, Ledersattel und Zaumzeug (aus dem Prototyp 005).',
    teile: ctx.teile,
    bein: { breiten_vorn: p.LEGW.front, breiten_hinten: p.LEGW.back },
    animationen: { schritt: { frames: 8, gangart: 'schritt' }, trab: { frames: 8, gangart: 'trab' }, galopp: { frames: 8, gangart: 'galopp' } },
    ausgabe: { hoehe_px: 192, rahmen: rahmen(ctx.teile, 4), massstab: 1.3 },
  };
}

/** JSON mit Einrückung 2, Zahlenlisten (Pivot, Mitte, Rahmen) in einer Zeile. */
export function jsonText(wert: object): string {
  return JSON.stringify(wert, null, 2).replace(/\[\s*(-?[\d.e+-]+(?:,\s*-?[\d.e+-]+)*)\s*\]/g, (_, n: string) => `[${n.split(/,\s*/).join(', ')}]`) + '\n';
}

function schreibe(datei: string, spec: object) {
  const schema = datei.includes('/') ? '../../../schema/paper-spec.schema.json' : '../../schema/paper-spec.schema.json';
  writeFileSync(join(OUT, datei), jsonText({ $schema: schema, ...spec }));
}

if (import.meta.main) {
  const p = ladePrototyp();
  schreibe('monarch.paper.json', monarchSpec(p));
  schreibe('pferd.paper.json', pferdSpec(p));
  schreibe('mauer.paper.json', mauerSpec());
  for (const [name, spec] of Object.entries(negativSpecs())) schreibe(`ungueltig/${name}.paper.json`, spec);
}
