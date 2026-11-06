// Zeitregel der bewegten Rigs (README › D-04): Phase von Frame n = n / frames, Pose-Zeit = Phase / freq; Physik mit
// 1/120 s, 240 Schritten Einschwingen vor Frame 0, Frame n bei Schritt round(n · 120 / (freq · frames)).
import type { Punkt, Spec } from '../schema/types';
import { weltMatrizen } from '../render/baum';
import { Aufruf } from '../render/fehler';
import { anwenden, type M, verschiebe } from '../render/matrix';
import { animationVon } from '../render/pose';
import { figurFreq, type FigurPose, figurPose } from './figur';
import { Umhang } from './umhang';

export const EINSCHWINGEN = 240;
const SCHRITTE_JE_S = 120;

export const frameSchritte = (freq: number, frames: number) =>
  Array.from({ length: frames }, (_, n) => Math.round((n * SCHRITTE_JE_S) / (freq * frames)));

export type FigurFrame = FigurPose & { umhang?: Punkt[] };

/** Anker des Umhangs: umhang.punkt durch die Welt-Matrix des Anker-Teils (mit Versatz der Figur). */
export function umhangAnker(spec: Spec, pose: FigurPose, basis: M = verschiebe(...pose.versatz)): Punkt {
  const u = spec.umhang!;
  return anwenden(weltMatrizen(spec, pose.winkel, basis).get(u.anker)!, u.punkt);
}

/** Posen aller Frames einer figur-Animation samt Umhang-Kette. */
export function simuliere(spec: Spec, name: string): FigurFrame[] {
  const anim = animationVon(spec, name);
  if (anim.reittier) throw new Aufruf(`Reiten (${name}) braucht das Reit-Rig`);
  const freq = figurFreq(spec, name), schritte = frameSchritte(freq, anim.frames);
  const posen = schritte.map((_, n) => figurPose(spec, name, n / (freq * anim.frames)));
  if (!spec.umhang) return posen;
  const zuZeit = (k: number) => figurPose(spec, name, k / SCHRITTE_JE_S);
  const start = zuZeit(-EINSCHWINGEN);
  const kette = new Umhang(spec.umhang, umhangAnker(spec, start));
  const out: FigurFrame[] = [];
  for (let k = -EINSCHWINGEN + 1, n = 0; n < posen.length; k++) {
    const p = zuZeit(k);
    kette.schritt(umhangAnker(spec, p), p.wind, 0);
    while (n < posen.length && schritte[n] === k) {
      out.push({ ...posen[n], umhang: kette.punkte() });
      n++;
    }
  }
  return out;
}
