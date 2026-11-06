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

export type UmhangSpec = NonNullable<Spec['umhang']>;

/** Umhang-Kette je Frame: Physik ab Schritt −240 bis zum letzten Frame; zustand(k) liefert Anker und Wind bei Schritt k. */
export function umhangLauf(u: UmhangSpec, schritte: number[], zustand: (k: number) => { anker: Punkt; wind: number }, boden: number): Punkt[][] {
  const kette = new Umhang(u, zustand(-EINSCHWINGEN).anker), out: Punkt[][] = [];
  for (let k = -EINSCHWINGEN + 1, n = 0; n < schritte.length; k++) {
    const z = zustand(k);
    kette.schritt(z.anker, z.wind, boden);
    while (n < schritte.length && schritte[n] === k) {
      out.push(kette.punkte());
      n++;
    }
  }
  return out;
}

/** Posen aller Frames einer figur-Animation samt Umhang-Kette. */
export function simuliere(spec: Spec, name: string): FigurFrame[] {
  const anim = animationVon(spec, name);
  if (anim.reittier) throw new Aufruf(`Reiten (${name}) braucht das Reit-Rig`);
  const freq = figurFreq(spec, name), schritte = frameSchritte(freq, anim.frames);
  const posen = schritte.map((_, n) => figurPose(spec, name, n / (freq * anim.frames)));
  if (!spec.umhang) return posen;
  const zustand = (k: number) => {
    const p = figurPose(spec, name, k / SCHRITTE_JE_S);
    return { anker: umhangAnker(spec, p), wind: p.wind };
  };
  const ketten = umhangLauf(spec.umhang, schritte, zustand, 0);
  return posen.map((p, n) => ({ ...p, umhang: ketten[n] }));
}
