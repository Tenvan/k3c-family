// Frames einer Ausgabe als SVG-Inhalt (Koordinaten im Rahmen) samt Geometrie je Teil (für `frame`-Boxen).
// Rigs ohne eigene Bewegung hier; figur und vierbeiner liefern ihre Frames aus rig/ (RD1.2, RD1.3).
import type { Spec, Stil, Teil } from '../schema/types';
import { weltMatrizen } from './baum';
import type { M } from './matrix';
import { posen } from './pose';
import { type FrameGroesse, frameGroesse } from './raster';
import { type Ausgabe, ausgabeSpec } from './stufen';
import { teileSvg } from './svg';

export type Kontext = { stil: Stil; specPfad?: string };
export type Geometrie = { teil: Teil; m: M }[];
export type Frame = { svg: string; geometrie: Geometrie };
export type Bild = { groesse: FrameGroesse; frames: Frame[] };

export const geometrie = (teile: Teil[], m: Map<string, M>, ohne?: Set<string>): Geometrie =>
  teile.filter((t) => !ohne?.has(t.id)).map((teil) => ({ teil, m: m.get(teil.id)! }));

/** Frames ohne Rig-Bewegung: Keyframes der Animation, Teile mit Papier-Filter. */
export function einfachesBild(spec: Spec, a: Ausgabe, k: Kontext): Bild {
  const s = ausgabeSpec(spec, a);
  const frames = posen(s, a.animation).map(({ winkel }) => ({
    svg: `<g filter="url(#paper)">${teileSvg(s, k.stil, winkel)}</g>`,
    geometrie: geometrie(s.teile, weltMatrizen(s, winkel)),
  }));
  return { groesse: frameGroesse(spec.ausgabe), frames };
}

/** Rigs mit eigener Bewegung tragen sich hier ein (rig/register.ts), damit render/ nicht von rig/ abhängt. */
export const RIG_BILDER: Partial<Record<Spec['rig'], (spec: Spec, a: Ausgabe, k: Kontext) => Bild>> = {};

export function bildVon(spec: Spec, a: Ausgabe, k: Kontext): Bild {
  return (RIG_BILDER[spec.rig] ?? einfachesBild)(spec, a, k);
}
