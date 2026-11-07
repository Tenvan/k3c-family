// Stil-Datei laden, Farben auflösen und Materialien tauschen (Varianten, Gebäude-Stufen).
// Vertrag: docs/standards/paper-spec.md › 2 (Stil-Datei) und › 3 (Lagen, Varianten, Stufen).
import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import type { Lage, Spec, Stil, Teil, Ton } from '../schema/types';

/** Repo-Wurzel (tools/paper/render → ../../..). */
export const ROOT = resolve(import.meta.dirname, '../../..');
export const STIL_PFAD = join(ROOT, 'public/grafik/stil.paper.json');

export function ladeStil(pfad = STIL_PFAD): Stil {
  return JSON.parse(readFileSync(pfad, 'utf8')) as Stil;
}

/** Farbe eines Materials (Ton fehlt = basis) oder eines Einzeltons (ohne Töne). */
export function farbe(stil: Stil, material: string, ton: Ton = 'basis'): string {
  if (Object.hasOwn(stil.materialien, material)) return stil.materialien[material][ton];
  if (Object.hasOwn(stil.einzeltoene, material)) return stil.einzeltoene[material];
  throw new Error(`Material „${material}“ fehlt in der Stil-Datei`);
}

const tauscheLage = (l: Lage, t: Record<string, string>): Lage =>
  (Object.hasOwn(t, l.material) ? { ...l, material: t[l.material] } : l);

export const tauscheTeil = (teil: Teil, t: Record<string, string>): Teil =>
  ({ ...teil, lagen: teil.lagen.map((l) => tauscheLage(l, t)) });

/** Kopie der Spec mit getauschten Materialien in Lagen, Umhang und Stufen-Zusätzen; das Original bleibt. */
export function tausche(spec: Spec, t: Record<string, string>): Spec {
  const neu: Spec = structuredClone(spec);
  neu.teile = neu.teile.map((teil) => tauscheTeil(teil, t));
  if (neu.umhang && Object.hasOwn(t, neu.umhang.material)) neu.umhang.material = t[neu.umhang.material];
  if (neu.stufen) neu.stufen = neu.stufen.map((s) => ({ ...s, zusaetze: s.zusaetze?.map((z) => tauscheTeil(z, t)) }));
  return neu;
}
