// Pose je Frame: Phase von Frame n = n / frames; Keyframes linear und zyklisch (paper-spec.md › 3 `keyframes`).
// Bewegung der Rig-Typen figur und vierbeiner liefern die Rigs (RD1.2, RD1.3).
import type { Animation, Spec } from '../schema/types';
import { Abgelehnt, Aufruf } from './fehler';
import { animationen } from './stufen';

export type Pose = { winkel: Record<string, number> };
type Keyframes = NonNullable<Animation['keyframes']>;

/** Winkel der Keyframes bei Phase p (0..1), zyklisch; Teile ohne Wert in einem Keyframe zählen dort 0. */
export function keyframeWinkel(kf: Keyframes | undefined, p: number): Record<string, number> {
  if (!kf?.length) return {};
  const k = [...kf].sort((a, b) => a.t - b.t);
  const ids = [...new Set(k.flatMap((f) => Object.keys(f.winkel)))].sort();
  const i = k.reduce((letzter, f, j) => (f.t <= p ? j : letzter), -1);
  const a = i >= 0 ? k[i] : { ...k[k.length - 1], t: k[k.length - 1].t - 1 };
  const b = i + 1 < k.length ? k[i + 1] : { ...k[0], t: k[0].t + 1 };
  const anteil = b.t === a.t ? 0 : (p - a.t) / (b.t - a.t);
  return Object.fromEntries(ids.map((id) => {
    const va = a.winkel[id] ?? 0, vb = b.winkel[id] ?? 0;
    return [id, va + (vb - va) * anteil];
  }));
}

/** Animation der Spec oder Ablehnung `animation-unbekannt`. */
export function animationVon(spec: Spec, name: string): Animation {
  const alle = animationen(spec);
  if (!Object.hasOwn(alle, name))
    throw new Abgelehnt([{ feld: 'animationen', regel: 'animation-unbekannt', text: `Animation „${name}“ fehlt (${Object.keys(alle).join(', ')})` }]);
  return alle[name];
}

/** Posen aller Frames für Rigs ohne eigene Bewegung (requisite, gebaeude, hintergrund). */
export function posen(spec: Spec, name: string): Pose[] {
  const anim = animationVon(spec, name);
  if (spec.rig === 'figur' || spec.rig === 'vierbeiner') throw new Aufruf(`Rig ${spec.rig} hat eigene Bewegung (rig/)`);
  return Array.from({ length: anim.frames }, (_, n) => ({ winkel: keyframeWinkel(anim.keyframes, n / anim.frames) }));
}
