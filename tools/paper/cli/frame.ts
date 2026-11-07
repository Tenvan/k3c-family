// frame (paper-spec.md › 8): ein Frame als PNG und die Teil-Boxen [x, y, b, h] in Pixeln des Frames.
import { writeFileSync } from 'node:fs';
import { points } from '../schema/form';
import type { Lage, Spec, Stil } from '../schema/types';
import { bildVon, type Geometrie } from '../render/bild';
import { Abgelehnt } from '../render/fehler';
import { anwenden } from '../render/matrix';
import { animationVon } from '../render/pose';
import { type FrameGroesse, rastere, streifenSvg } from '../render/raster';
import type { Ausgabe } from '../render/stufen';
import { defs } from '../render/svg';

/** `ohne`: Teile, die ohne Lagen gerendert werden (B-734: Teil in der Vorschau ausblenden); Gelenke bleiben. */
export type FrameWahl = { animation: string; frame: number; variante?: string; stufe: number; ohne?: string[] };

const ablehnen = (feld: string, regel: string, text: string) => new Abgelehnt([{ feld, regel, text }]);

/** Punkte einer Lage für die Box (Punkt-Lage als Quadrat um die Mitte). */
const lagePunkte = (l: Lage): [number, number][] => (l.art === 'punkt'
  ? [[l.mitte[0] - l.radius, l.mitte[1] - l.radius], [l.mitte[0] + l.radius, l.mitte[1] + l.radius]]
  : points(l.form));

export function teilBoxen(geo: Geometrie, g: FrameGroesse): Record<string, number[]> {
  const [rx, ry, , rh] = g.rahmen, s = g.frameH / rh, out: Record<string, number[]> = {};
  for (const { teil, m } of geo) {
    const pts = teil.lagen.flatMap(lagePunkte).map((p) => anwenden(m, p));
    if (!pts.length) continue;
    const xs = pts.map((p) => p[0]), ys = pts.map((p) => p[1]);
    const x0 = Math.min(...xs), y0 = Math.min(...ys), x1 = Math.max(...xs), y1 = Math.max(...ys);
    out[teil.id] = [x0 - rx, y0 - ry, x1 - x0, y1 - y0].map((v) => Math.round(v * s));
  }
  return out;
}

/** Auszublendende Teile (B-734) müssen in der Spec stehen. */
function pruefeOhne(spec: Spec, ohne: string[]) {
  const unbekannt = ohne.filter((id) => !spec.teile.some((t) => t.id === id));
  if (unbekannt.length)
    throw ablehnen('teile', 'teil-unbekannt', `Teil „${unbekannt.join('“, „')}“ fehlt (${spec.teile.map((t) => t.id).join(', ')})`);
}

function pruefeWahl(spec: Spec, w: FrameWahl) {
  const anim = animationVon(spec, w.animation);
  if (w.variante !== undefined && !Object.hasOwn(spec.varianten ?? {}, w.variante))
    throw ablehnen('varianten', 'variante-unbekannt', `Variante „${w.variante}“ fehlt (${Object.keys(spec.varianten ?? {}).join(', ') || 'keine'})`);
  const stufen = (spec.stufen?.length ?? 0) + 1;
  if (!Number.isInteger(w.stufe) || w.stufe < 1 || w.stufe > stufen)
    throw ablehnen('stufen', 'stufe-unbekannt', `Stufe ${w.stufe} gibt es nicht (1–${stufen})`);
  pruefeOhne(spec, w.ohne ?? []);
  if (!Number.isInteger(w.frame) || w.frame < 0 || w.frame >= anim.frames)
    throw ablehnen(`animationen.${w.animation}.frames`, 'frame-unbekannt', `Frame ${w.frame} gibt es nicht (0–${anim.frames - 1})`);
}

/** Rendert den Frame nach `out` und liefert die Teil-Boxen. */
export function frameRendern(spec: Spec, w: FrameWahl, stil: Stil, specPfad: string, out: string): Record<string, number[]> {
  pruefeWahl(spec, w);
  if (w.ohne?.length) spec = { ...spec, teile: spec.teile.map((t) => (w.ohne!.includes(t.id) ? { ...t, lagen: [] } : t)) };
  const a: Ausgabe = { schluessel: w.animation, animation: w.animation, stufe: w.stufe, ...(w.variante ? { variante: w.variante } : {}) };
  const bild = bildVon(spec, a, { stil, specPfad }), f = bild.frames[w.frame];
  writeFileSync(out, rastere(streifenSvg([f.svg], bild.groesse, defs(stil, spec.id, bild.groesse.frameH / bild.groesse.rahmen[3]))).png);
  return teilBoxen(f.geometrie, bild.groesse);
}
