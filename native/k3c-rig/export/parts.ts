// Bilder der Teile einer Figur: je Teil ein PNG in Ruhelage, abweichende Varianten (Spielerfarben) daneben.
import type { Spec, Stil, Teil } from '../../../tools/paper/schema/types';
import { tausche } from '../../../tools/paper/render/stil';
import { lageSvg } from '../../../tools/paper/render/svg';
import { type Kontext, rastere } from './raster';
import type { BildOrt, Dateien, Rig, RigTeil } from './types';

/** Spec in der Grundform und je Variante (Tausch der Materialien). */
export type Quelle = { basis: Spec; varianten: [string, Spec][] };
export type Ziel = { ordner: string; dateien: Dateien; varianten: Rig['varianten'] };

export function quelle(spec: Spec, varianten: Spec['varianten']): Quelle {
  const namen = Object.keys(varianten ?? {}).sort();
  return { basis: spec, varianten: namen.map((n) => [n, tausche(spec, varianten![n].tausch)]) };
}

export const neuesZiel = (ordner: string, dateien: Dateien, q: Quelle): Ziel =>
  ({ ordner, dateien, varianten: Object.fromEntries(q.varianten.map(([n]) => [n, {}])) });

/** Standardbild `<id>.png`; eine Variante bekommt nur dann `<id>@<variante>.png`, wenn ihr Inhalt abweicht. */
export function bildMitVarianten(k: Kontext, q: Quelle, id: string, inhalt: (s: Spec) => string, z: Ziel): BildOrt {
  const text = inhalt(q.basis), roh = rastere(k, text, id);
  z.dateien.set(`${z.ordner}/${id}.png`, roh.png);
  for (const [name, spec] of q.varianten) {
    const t = inhalt(spec);
    if (t === text) continue;
    const v = rastere(k, t, `${id}@${name}`);
    if (v.versatz.join() !== roh.versatz.join() || v.groesse.join() !== roh.groesse.join())
      throw new Error(`${id}@${name}: Variante hat einen anderen Ausschnitt als das Standardbild`);
    z.dateien.set(`${z.ordner}/${id}@${name}.png`, v.png);
    z.varianten[name][id] = `${id}@${name}.png`;
  }
  return { bild: `${id}.png`, versatz: roh.versatz, groesse: roh.groesse };
}

export const lagenSvg = (t: Teil, stil: Stil) => `<g>${t.lagen.map((l) => lageSvg(l, stil)).join('')}</g>`;

/** Alle Teile in Listenreihenfolge (ebene = Index · 10); Teile ohne Lagen bleiben ohne Bild, `ohne` entfällt ganz. */
export function rigTeile(k: Kontext, q: Quelle, z: Ziel, ohne = new Set<string>()): RigTeil[] {
  const out: RigTeil[] = [];
  q.basis.teile.forEach((t, i) => {
    if (ohne.has(t.id)) return;
    const kopf = { id: t.id, ...(t.eltern ? { eltern: t.eltern } : {}), pivot: t.pivot, ebene: i * 10 };
    if (t.lagen.length === 0) return void out.push(kopf);
    const teil = (s: Spec) => lagenSvg(s.teile.find((x) => x.id === t.id)!, k.stil);
    out.push({ ...kopf, ...bildMitVarianten(k, q, t.id, teil, z) });
  });
  return out;
}

export const ebeneVon = (spec: Spec, id: string): number => {
  const i = spec.teile.findIndex((t) => t.id === id);
  if (i < 0) throw new Error(`Teil ${id} fehlt in ${spec.id}`);
  return i * 10;
};
