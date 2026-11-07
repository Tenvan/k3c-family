// Rig figur: Posen idle und lauf wie im Prototyp (docs/decisions/005/prototyp-monarch-rig.html › POSES), Winkel in Grad,
// positiv = im Uhrzeigersinn. Keyframes der Spec addieren sich; andere Animationen brauchen Keyframes.
import type { Punkt, Spec } from '../schema/types';
import { Abgelehnt } from '../render/fehler';
import { animationVon, keyframeWinkel } from '../render/pose';
import { figurFaktor } from './proportionen';

const TAU = Math.PI * 2;
export type FigurPose = { winkel: Record<string, number>; versatz: Punkt; wind: number };
type PoseFn = (p: number) => FigurPose;

export const FIGUR_POSEN: Record<'idle' | 'lauf', { freq: number; f: PoseFn }> = {
  idle: { freq: 0.5, f(p) {
    const s = Math.sin(p);
    return { winkel: { 'oberarm-v': 5 + 2 * s, 'unterarm-v': -6 - 3 * s, 'oberarm-h': -3 - 2 * s, 'unterarm-h': -4,
      kopf: 2 * s, krone: -s }, versatz: [0, 0], wind: 0 };
  } },
  lauf: { freq: 1.6, f(p) {
    const s = Math.sin(p), c = Math.cos(p), s2 = Math.sin(2 * p);
    return { winkel: { 'oberschenkel-v': -30 * s, 'oberschenkel-h': 30 * s, 'unterschenkel-v': 45 * Math.max(0, c),
      'unterschenkel-h': 45 * Math.max(0, -c), 'oberarm-v': 25 * s, 'unterarm-v': -15 - 12 * Math.max(0, -s),
      'oberarm-h': -25 * s, 'unterarm-h': -15 - 12 * Math.max(0, s), kopf: 2 * s2, krone: -2 * s2 },
    versatz: [0, 2.5 * Math.abs(s) - 2], wind: 30 };
  } },
};

const istPose = (name: string): name is 'idle' | 'lauf' => Object.hasOwn(FIGUR_POSEN, name);

/** Frequenz einer Animation: Pose des Rigs, sonst ein Zyklus über alle Frames bei `fps`. */
export function figurFreq(spec: Spec, name: string): number {
  if (istPose(name)) return FIGUR_POSEN[name].freq;
  const a = animationVon(spec, name);
  return (a.fps ?? 8) / a.frames;
}

/** Pose der Figur zur Zeit t (Sekunden). */
export function figurPose(spec: Spec, name: string, t: number): FigurPose {
  const anim = animationVon(spec, name);
  if (!istPose(name) && !anim.keyframes?.length)
    throw new Abgelehnt([{ feld: `animationen.${name}`, regel: 'animation-unbekannt',
      text: `figur kennt ohne Keyframes nur idle und lauf, nicht „${name}“` }]);
  const freq = figurFreq(spec, name), phase = (((t * freq) % 1) + 1) % 1;
  const roh = istPose(name) ? FIGUR_POSEN[name].f(phase * TAU) : { winkel: {}, versatz: [0, 0] as Punkt, wind: 0 };
  const rumpf = spec.proportionen?.rumpf ?? 1;
  const basis = { ...roh, versatz: [roh.versatz[0], roh.versatz[1] * rumpf] as Punkt,
    winkel: Object.fromEntries(Object.entries(roh.winkel).map(([id, w]) => [id, w * figurFaktor(spec, id)])) };
  const kf = keyframeWinkel(anim.keyframes, phase);
  const winkel = { ...basis.winkel };
  for (const [id, w] of Object.entries(kf)) winkel[id] = (winkel[id] ?? 0) + w;
  return { ...basis, winkel };
}
