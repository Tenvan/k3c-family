// Prüft Paper-Specs und die Stil-Datei: erst JSON-Schema (ajv), dann fachliche Regeln, die ein Schema nicht ausdrückt
// (docs/standards/paper-spec.md › Regeln). Fehler `{feld, regel, text}`, `feld` als Pfad wie `teile[2].lagen[0].material`.
import { readFileSync } from 'node:fs';
import { basename } from 'node:path';
import Ajv2020Import from 'ajv/dist/2020.js';
import type { ErrorObject, ValidateFunction } from 'ajv';
import specSchema from './paper-spec.schema.json';
import stilSchema from './stil.schema.json';
import { box } from './form';
import type { Lage, Spec, Stil, Teil } from './types';

export type Regel = 'json' | 'schema' | 'hex-farbe' | 'material-unbekannt' | 'material-doppelt' | 'ton-ohne-toene'
  | 'pflicht-teil-fehlt' | 'teil-doppelt' | 'eltern-unbekannt' | 'teil-unbekannt' | 'zyklus' | 'id-dateiname' | 'proportion' | 'proportionen-rig';
export type Fehler = { feld: string; regel: Regel; text: string };

/** Pflicht-Teile je Rig-Typ; requisite und hintergrund brauchen mindestens ein Teil. */
export const PFLICHT_TEILE: Record<Spec['rig'], string[]> = {
  figur: ['rumpf', 'kopf', 'oberarm-v', 'unterarm-v', 'oberarm-h', 'unterarm-h', 'oberschenkel-v', 'unterschenkel-v',
    'oberschenkel-h', 'unterschenkel-h'],
  vierbeiner: ['rumpf-vorn', 'rumpf-hinten', 'hals', 'schweif'],
  gebaeude: ['korpus'],
  requisite: [],
  hintergrund: [],
};

type AjvCtor = new (o: object) => { compile: (s: object) => ValidateFunction };
const Ajv2020 = ((Ajv2020Import as unknown as { default?: AjvCtor }).default ?? Ajv2020Import) as unknown as AjvCtor;
let compiled: { spec: ValidateFunction; stil: ValidateFunction } | undefined;
function validators() {
  if (!compiled) {
    const ajv = new Ajv2020({ allErrors: true, strict: true });
    compiled = { spec: ajv.compile(specSchema), stil: ajv.compile(stilSchema) };
  }
  return compiled;
}

/** JSON-Pointer `/teile/0/id` → `teile[0].id`. */
const feldAus = (pointer: string) => pointer.split('/').slice(1).reduce((acc, p) => (/^\d+$/.test(p) ? `${acc}[${p}]` : acc ? `${acc}.${p}` : p), '');

function schemaFehler(errors: ErrorObject[] | null | undefined): Fehler[] {
  return (errors ?? []).map((e) => {
    const extra = (e.params as { additionalProperty?: string }).additionalProperty;
    return { feld: feldAus(e.instancePath) || '$', regel: 'schema', text: extra ? `unbekanntes Feld „${extra}“` : e.message ?? 'ungültig' };
  });
}

/** Hex-Farbe (`#` mit 3–8 Hex-Ziffern) in einem Wert außerhalb von `form`: Farben gehören in die Stil-Datei. */
function hexFehler(wert: unknown, feld = ''): Fehler[] {
  if (typeof wert === 'string') return /#[0-9a-fA-F]{3,8}\b/.test(wert) ? [{ feld, regel: 'hex-farbe', text: `Farbe „${wert}“ statt Material-Name` }] : [];
  if (Array.isArray(wert)) return wert.flatMap((w, i) => hexFehler(w, `${feld}[${i}]`));
  if (wert && typeof wert === 'object') {
    return Object.entries(wert).filter(([k]) => k !== 'form' && k !== '$schema').flatMap(([k, w]) => hexFehler(w, feld ? `${feld}.${k}` : k));
  }
  return [];
}

/** Alle Materialstellen: Lagen (mit Ton), Umhang, Tausch in Varianten und Stufen (Schlüssel und Werte). */
function materialStellen(spec: Spec): { feld: string; material: string; ton?: string }[] {
  const lagen = (teile: Teil[], pfad: string) => teile.flatMap((t, i) =>
    t.lagen.map((l: Lage, j) => ({ feld: `${pfad}[${i}].lagen[${j}].material`, material: l.material, ton: l.ton })));
  const tausch = (t: Record<string, string> | undefined, pfad: string) =>
    Object.entries(t ?? {}).flatMap(([a, b]) => [{ feld: `${pfad}.${a}`, material: a }, { feld: `${pfad}.${a}`, material: b }]);
  return [
    ...lagen(spec.teile, 'teile'),
    ...(spec.umhang ? [{ feld: 'umhang.material', material: spec.umhang.material }] : []),
    ...Object.entries(spec.varianten ?? {}).flatMap(([n, v]) => tausch(v.tausch, `varianten.${n}.tausch`)),
    ...(spec.stufen ?? []).flatMap((s, i) => [...tausch(s.tausch, `stufen[${i}].tausch`), ...lagen(s.zusaetze ?? [], `stufen[${i}].zusaetze`)]),
  ];
}

function materialFehler(spec: Spec, stil: Stil): Fehler[] {
  return materialStellen(spec).flatMap(({ feld, material, ton }): Fehler[] => {
    if (material in stil.materialien) return [];
    if (!(material in stil.einzeltoene)) return [{ feld, regel: 'material-unbekannt', text: `Material „${material}“ fehlt in der Stil-Datei` }];
    return ton && ton !== 'basis' ? [{ feld, regel: 'ton-ohne-toene', text: `Einzelton „${material}“ hat keinen Ton „${ton}“` }] : [];
  });
}

/** Pflicht-Teile, doppelte IDs, unbekannte Eltern und Zyklen; Stufen-Zusätze sehen die Basis-Teile. */
function teilFehler(spec: Spec): Fehler[] {
  const ids = spec.teile.map((t) => t.id);
  const pflicht = PFLICHT_TEILE[spec.rig];
  const fehler: Fehler[] = pflicht.filter((p) => !ids.includes(p))
    .map((p) => ({ feld: 'teile', regel: 'pflicht-teil-fehlt', text: `Rig ${spec.rig} braucht das Teil „${p}“` }));
  if (pflicht.length === 0 && ids.length === 0) fehler.push({ feld: 'teile', regel: 'pflicht-teil-fehlt', text: 'mindestens ein Teil' });
  const gruppen: [Teil[], string, string[]][] = [[spec.teile, 'teile', []],
    ...(spec.stufen ?? []).map((s, i): [Teil[], string, string[]] => [s.zusaetze ?? [], `stufen[${i}].zusaetze`, ids])];
  for (const [teile, pfad, basis] of gruppen) fehler.push(...elternFehler(teile, pfad, basis));
  return fehler;
}

function elternFehler(teile: Teil[], pfad: string, basis: string[]): Fehler[] {
  const fehler: Fehler[] = [];
  const eltern = new Map<string, string | undefined>(teile.map((t) => [t.id, t.eltern]));
  const bekannt = new Set([...basis, ...eltern.keys()]);
  teile.forEach((t, i) => {
    if (teile.findIndex((u) => u.id === t.id) !== i) fehler.push({ feld: `${pfad}[${i}].id`, regel: 'teil-doppelt', text: `Teil „${t.id}“ doppelt` });
    if (t.eltern && !bekannt.has(t.eltern)) fehler.push({ feld: `${pfad}[${i}].eltern`, regel: 'eltern-unbekannt', text: `Eltern „${t.eltern}“ fehlt` });
    if (hatZyklus(t.id, eltern)) fehler.push({ feld: `${pfad}[${i}].eltern`, regel: 'zyklus', text: `Teil „${t.id}“ ist sein eigener Vorfahr` });
  });
  return fehler;
}

function hatZyklus(start: string, eltern: Map<string, string | undefined>): boolean {
  const gesehen = new Set<string>();
  for (let id = eltern.get(start); id !== undefined; id = eltern.get(id)) {
    if (id === start || gesehen.has(id)) return true;
    gesehen.add(id);
  }
  return false;
}

/** Verweise auf Teile außerhalb von `eltern`: Umhang-Anker und Winkel in Keyframes. */
function verweisFehler(spec: Spec): Fehler[] {
  const ids = new Set(spec.teile.map((t) => t.id));
  const fehler: Fehler[] = [];
  if (spec.umhang && !ids.has(spec.umhang.anker)) fehler.push({ feld: 'umhang.anker', regel: 'teil-unbekannt', text: `Teil „${spec.umhang.anker}“ fehlt` });
  for (const [name, a] of Object.entries(spec.animationen ?? {}))
    (a.keyframes ?? []).forEach((k, i) => {
      for (const feld of ['winkel', 'sichtbar'] as const)
        Object.keys(k[feld] ?? {}).filter((id) => !ids.has(id)).forEach((id) =>
          fehler.push({ feld: `animationen.${name}.keyframes[${i}].${feld}.${id}`, regel: 'teil-unbekannt', text: `Teil „${id}“ fehlt` }));
    });
  return fehler;
}

/** `ebene` (B-752): `reiter` nur am figur mit reittier und gangart, `hinten`/`vorn` nur am vierbeiner mit reiter und gangart. */
function ebenenFehler(spec: Spec): Fehler[] {
  return Object.entries(spec.animationen ?? {}).flatMap(([name, a]): Fehler[] => {
    if (!a.ebene) return a.reiter ? [{ feld: `animationen.${name}.reiter`, regel: 'ebene-angabe', text: '„reiter“ gilt nur für Reittier-Ebenen (ebene hinten/vorn)' }] : [];
    const reiter = a.ebene === 'reiter', rig = reiter ? 'figur' : 'vierbeiner';
    const fehlt = [spec.rig !== rig && `ebene „${a.ebene}“ gilt nur für rig ${rig}`, !a.gangart && 'gangart fehlt',
      reiter && !a.reittier && 'reittier fehlt', !reiter && !a.reiter && 'reiter fehlt', reiter && a.reiter && '„reiter“ gilt nur für Reittier-Ebenen'].filter((t): t is string => !!t);
    return fehlt.map((text) => ({ feld: `animationen.${name}.ebene`, regel: 'ebene-angabe', text }));
  });
}

/** `proportionen`: vierbeiner-Felder nur am vierbeiner, figur-Felder nur an der figur (B-715). */
function proportionenFehler(spec: Spec): Fehler[] {
  const p = spec.proportionen;
  if (!p) return [];
  const erlaubt: string[] = spec.rig === 'vierbeiner' ? ['fuss', 'gangarten'] : spec.rig === 'figur' ? ['arme', 'beine', 'rumpf', 'kopf'] : [];
  return Object.keys(p).filter((k) => !erlaubt.includes(k)).map((k): Fehler =>
    ({ feld: `proportionen.${k}`, regel: 'proportionen-rig', text: `„${k}“ gilt nicht für rig ${spec.rig}` }));
}

/** Figur: Höhe des Teils `kopf` (eigene Lagen) zur Rahmenhöhe innerhalb kopf_anteil ± toleranz (absolut). */
function proportionFehler(spec: Spec, stil: Stil): Fehler[] {
  const kopf = spec.teile.find((t) => t.id === 'kopf');
  if (spec.rig !== 'figur' || !kopf) return [];
  const forms = kopf.lagen.map((l) => (l.art === 'punkt' ? `M${l.mitte[0]} ${l.mitte[1] - l.radius}V${l.mitte[1] + l.radius}` : l.form));
  const b = box(forms);
  if (!b) return [];
  const anteil = (b.y1 - b.y0) / spec.ausgabe.rahmen[3];
  const { kopf_anteil, toleranz } = stil.proportionen.figur;
  if (Math.abs(anteil - kopf_anteil) <= toleranz) return [];
  return [{ feld: 'teile.kopf', regel: 'proportion', text: `Kopf ${anteil.toFixed(3)} der Höhe, erlaubt ${kopf_anteil} ± ${toleranz}` }];
}

/** Prüft eine Spec; `datei` (Pfad) aktiviert die Regel id-dateiname. Stufen: Hex → Schema → fachlich. */
export function checkSpec(spec: unknown, stil: Stil, datei?: string): Fehler[] {
  const hex = hexFehler(spec);
  if (hex.length) return hex;
  const v = validators().spec;
  if (!v(spec)) return schemaFehler(v.errors);
  const s = spec as Spec;
  const fehler = [...materialFehler(s, stil), ...teilFehler(s), ...verweisFehler(s), ...proportionFehler(s, stil), ...proportionenFehler(s), ...ebenenFehler(s)];
  if (datei && basename(datei).replace(/\.paper\.json$/, '') !== s.id)
    fehler.push({ feld: 'id', regel: 'id-dateiname', text: `id „${s.id}“ passt nicht zu ${basename(datei)}` });
  return fehler;
}

/** Prüft die Stil-Datei: Schema, Namen nicht zugleich Material und Einzelton. */
export function checkStil(stil: unknown): Fehler[] {
  const v = validators().stil;
  if (!v(stil)) return schemaFehler(v.errors);
  const s = stil as Stil;
  return Object.keys(s.einzeltoene).filter((n) => n in s.materialien)
    .map((n) => ({ feld: `einzeltoene.${n}`, regel: 'material-doppelt', text: `„${n}“ ist schon ein Material` }));
}

/** Liest und prüft eine Datei; kaputtes JSON → Regel json. */
export function readJson(path: string): { wert?: unknown; fehler: Fehler[] } {
  try {
    return { wert: JSON.parse(readFileSync(path, 'utf8')), fehler: [] };
  } catch (e) {
    return { fehler: [{ feld: '$', regel: 'json', text: `${basename(path)}: ${(e as Error).message}` }] };
  }
}

export function checkSpecFile(path: string, stil: Stil): Fehler[] {
  const { wert, fehler } = readJson(path);
  return fehler.length ? fehler : checkSpec(wert, stil, path);
}
