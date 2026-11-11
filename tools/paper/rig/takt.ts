// takt: Abspielrate einer Animation im Manifest. Posen des Rigs (idle, lauf) und Gangarten haben eine feste Frequenz; ihr
// Sheet enthält genau einen Zyklus, also gilt fps = Frequenz × Frames. Nennt die Spec ein `fps`, gilt das.
import type { Animation, Spec } from '../schema/types';
import { FIGUR_POSEN } from './figur';
import { GANGARTEN } from './gangarten';

/** Abspielrate ohne eigene Frequenz (Keyframe-Animationen, Einzelbilder). */
export const STANDARD_FPS = 8;

const runde = (x: number) => Math.round(x * 1000) / 1000;

/** fps einer Ausgabe mit `frames` Bildern: Spec-Wert, sonst Frequenz der Gangart bzw. der Pose × Frames, sonst 8. */
export function ausgabeFps(spec: Spec, name: string, anim: Animation, frames: number): number {
  if (anim.fps !== undefined) return anim.fps;
  if (frames <= 1) return STANDARD_FPS;
  if (anim.gangart) return runde(GANGARTEN[anim.gangart].freq * frames);
  if (spec.rig === 'figur' && Object.hasOwn(FIGUR_POSEN, name)) return runde(FIGUR_POSEN[name as keyof typeof FIGUR_POSEN].freq * frames);
  return STANDARD_FPS;
}
