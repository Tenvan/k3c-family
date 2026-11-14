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
import { REITTIER_PRAEFIX } from '../rig/reitBoxen';
import type { VierbeinerKontext } from '../rig/vierbeinerBild';
import { ladeReittier, type ReitKontext, reitFeld } from '../rig/reiten';

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

const teilFehlt = (unbekannt: string[], ids: string[]) =>
  ablehnen('teile', 'teil-unbekannt', `Teil „${unbekannt.join('“, „')}“ fehlt (${ids.join(', ')})`);

/** Teile des Reittiers in `ohne` (Präfix `reittier:`, B-845), ohne Präfix; die übrigen gelten dem Reiter. */
const reittierTeile = (ohne: string[]) => ohne.filter((id) => id.startsWith(REITTIER_PRAEFIX)).map((id) => id.slice(REITTIER_PRAEFIX.length));

/** Auszublendende Teile (B-734) müssen in der Spec stehen, `reittier:<teil>` bei Reit-Animationen im Reittier (B-845). */
function pruefeOhne(spec: Spec, w: FrameWahl, stil: Stil, specPfad: string) {
  const ohne = w.ohne ?? [], ids = spec.teile.map((t) => t.id), anim = animationVon(spec, w.animation);
  const reitet = !!anim.reittier && !anim.ebene;
  const unbekannt = ohne.filter((id) => !ids.includes(id) && !(reitet && id.startsWith(REITTIER_PRAEFIX)));
  if (unbekannt.length) throw teilFehlt(unbekannt, ids);
  const fremd = reittierTeile(ohne);
  if (!reitet || !fremd.length) return;
  const tier = ladeReittier(specPfad, anim.reittier!, stil, reitFeld(w.animation)).teile.map((t) => t.id);
  const fehlt = fremd.filter((id) => !tier.includes(id));
  if (fehlt.length) throw teilFehlt(fehlt.map((id) => REITTIER_PRAEFIX + id), tier.map((id) => REITTIER_PRAEFIX + id));
}

function pruefeWahl(spec: Spec, w: FrameWahl, stil: Stil, specPfad: string) {
  const anim = animationVon(spec, w.animation);
  if (w.variante !== undefined && !Object.hasOwn(spec.varianten ?? {}, w.variante))
    throw ablehnen('varianten', 'variante-unbekannt', `Variante „${w.variante}“ fehlt (${Object.keys(spec.varianten ?? {}).join(', ') || 'keine'})`);
  const stufen = (spec.stufen?.length ?? 0) + 1;
  if (!Number.isInteger(w.stufe) || w.stufe < 1 || w.stufe > stufen)
    throw ablehnen('stufen', 'stufe-unbekannt', `Stufe ${w.stufe} gibt es nicht (1–${stufen})`);
  pruefeOhne(spec, w, stil, specPfad);
  if (!Number.isInteger(w.frame) || w.frame < 0 || w.frame >= anim.frames)
    throw ablehnen(`animationen.${w.animation}.frames`, 'frame-unbekannt', `Frame ${w.frame} gibt es nicht (0–${anim.frames - 1})`);
}

/** Rendert den Frame nach `out` und liefert die Teil-Boxen. */
export function frameRendern(spec: Spec, w: FrameWahl, stil: Stil, specPfad: string, out: string): Record<string, number[]> {
  pruefeWahl(spec, w, stil, specPfad);
  if (w.ohne?.length) spec = { ...spec, teile: spec.teile.map((t) => (w.ohne!.includes(t.id) ? { ...t, lagen: [] } : t)) };
  const a: Ausgabe = { schluessel: w.animation, animation: w.animation, stufe: w.stufe, ...(w.variante ? { variante: w.variante } : {}) };
  const k: ReitKontext & VierbeinerKontext = { stil, specPfad, reittierOhne: reittierTeile(w.ohne ?? []), beinOhne: w.ohne ?? [] };
  const bild = bildVon(spec, a, k), f = bild.frames[w.frame];
  writeFileSync(out, rastere(streifenSvg([f.svg], bild.groesse, defs(stil, spec.id, bild.groesse.frameH / bild.groesse.rahmen[3]))).png);
  return teilBoxen(f.geometrie, bild.groesse);
}
