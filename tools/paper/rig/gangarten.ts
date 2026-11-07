// Gangarten des Vierbeiners wie im Prototyp (docs/decisions/005/prototyp-monarch-rig.html › GAITS, gait): Standphase
// am Boden, Schwungbahn, Zwei-Knochen-IK, Rückengelenk, Kopfnicken, Schweif und die Pose des Reiters.
import type { GangartName, Punkt, Spec } from '../schema/types';
import { anwenden, drehung, invers, type M, mul } from '../render/matrix';
import { ik } from './ik';
import { spline } from './kurve';
import { beinLaengen } from './proportionen';
import { rideMatrix } from './vierbeiner';

const TAU = Math.PI * 2;
const c1 = (ph: number, at: number) => Math.cos(TAU * (ph - at));
const c2 = (ph: number, at: number) => Math.cos(2 * TAU * (ph - at));

export type Bein = 'hf' | 'hn' | 'vf' | 'vn';
export const BEINE: Record<Bein, { x: number; vorn: boolean }> = {
  hf: { x: -26, vorn: false }, hn: { x: -30, vorn: false }, vf: { x: 30, vorn: true }, vn: { x: 26, vorn: true },
};

type Gang = {
  freq: number; stance: number; reach: number; lift: [number, number]; curl: [number, number]; offs: Record<Bein, number>;
  path?: { front: Punkt[]; back: Punkt[] }; riderLevel?: number;
  dy(ph: number): number; pitch(ph: number): number; nod(ph: number): number; lean(ph: number): number;
  spine(ph: number): number; riderY(dy: number, ph: number): number; tail(ph: number): number;
};

export const GANGARTEN: Record<GangartName, Gang> = {
  // Stand (B-717, Reit-Idle): alle Hufe am Boden (reach 0, stance 1), nur Atmen, leichtes Nicken und Schweifpendeln.
  stand: { freq: 0.5, stance: 1, reach: 0, lift: [0, 0], curl: [0, 0], offs: { hn: 0, vn: 0, hf: 0, vf: 0 },
    dy: (ph) => 1 + 0.6 * Math.sin(TAU * ph), pitch: () => 0, nod: (ph) => 1.5 * Math.sin(TAU * (ph - 0.2)), lean: () => 2,
    spine: (ph) => 0.4 * Math.sin(TAU * ph), riderY: (dy) => -0.8 * (dy - 1), tail: (ph) => 6 + 3 * Math.sin(TAU * ph) },
  schritt: { freq: 0.9, stance: 0.65, reach: 11, lift: [9, 7], curl: [35, 15], offs: { hn: 0, vn: 0.25, hf: 0.5, vf: 0.75 },
    dy: (ph) => 1 + c2(ph, 0.3), pitch: (ph) => Math.sin(2 * TAU * ph), nod: (ph) => 6 * c2(ph, 0.3), lean: () => 3,
    spine: (ph) => 1.5 * Math.sin(2 * TAU * ph), riderY: (dy, ph) => -0.7 * (dy - 1) + 0.6 * Math.sin(2 * TAU * ph),
    tail: (ph) => 6 + 4 * Math.sin(TAU * ph) },
  trab: { freq: 1.5, stance: 0.45, reach: 12, lift: [16, 12], curl: [45, 20], offs: { hn: 0, vf: 0, hf: 0.5, vn: 0.5 },
    dy: (ph) => 1 + 2.5 * c2(ph, 0.225), pitch: (ph) => Math.sin(2 * TAU * ph), nod: (ph) => 4 * c2(ph, 0.25), lean: () => 6,
    spine: (ph) => 2.5 * c2(ph, 0.1), riderY: (dy, ph) => -0.4 * (dy - 1) - 4 * Math.max(0, Math.sin(TAU * (ph - 0.1))),
    tail: (ph) => 12 + 6 * Math.sin(2 * TAU * ph) },
  galopp: { freq: 2.1, stance: 0.25, reach: 22, lift: [26, 18], curl: [80, 35], offs: { hf: 0, hn: 0.08, vf: 0.3, vn: 0.38 },
    path: {
      front: [[-1, 0], [-1.35, 0.45], [-0.7, 1], [0.3, 0.95], [1.35, 0.55], [1.15, 0.12], [1, 0]],
      back: [[-1, 0], [-1.45, 0.35], [-0.9, 0.85], [0.2, 0.75], [1.1, 0.35], [1, 0]] },
    dy: (ph) => 8 - 6 * c1(ph, 0.8), spine: (ph) => 7 * c1(ph, 0.2), pitch: (ph) => 6 * Math.sin(TAU * (ph - 0.2)),
    nod: (ph) => 10 * c1(ph, 0.45), lean: (ph) => 14 + 3 * Math.sin(TAU * (ph - 0.95)), riderY: (dy) => -0.55 * (dy - 8),
    riderLevel: 0.7, tail: (ph) => 28 + 10 * Math.sin(TAU * (ph - 0.2)) },
};
export type Gangart = keyof typeof GANGARTEN;

export type GangPose = {
  winkel: Record<string, number>; ride: { dy: number; pitch: number }; massstab: number; wind: number;
  reiter: { versatz: Punkt; drehung: number; winkel: Record<string, number> };
};

/** Ziel des Hufs in Welt-Einheiten (Boden y = 0) und Hufkrümmung, nach Stand- oder Schwungphase. */
function hufZiel(G: Gang, hip: Punkt, r: number, u: number, vorn: boolean): { t: Punkt; curl: number } {
  const fi = vorn ? 0 : 1;
  if (u < G.stance) return { t: [hip[0] + r * (1 - (2 * u) / G.stance), 0], curl: 0 };
  const k = (u - G.stance) / (1 - G.stance);
  if (G.path) {
    const [px, py] = spline(G.path[vorn ? 'front' : 'back'], k);
    return { t: [hip[0] + r * px, -py * G.lift[fi]], curl: G.curl[fi] * Math.sin(Math.PI * Math.pow(k, 0.7)) };
  }
  const e = 0.5 - 0.5 * Math.cos(Math.PI * k);
  return { t: [hip[0] + r * (2 * e - 1), -Math.sin(Math.PI * k) * G.lift[fi]], curl: G.curl[fi] * Math.sin(Math.PI * k) };
}

function beinWinkel(G: Gang, ph: number, M: M, Mh: M, r: number, pitch: number, spine: number, l: { ober: number; unter: number }) {
  const a: Record<string, number> = {};
  for (const [id, b] of Object.entries(BEINE) as [Bein, (typeof BEINE)[Bein]][]) {
    const u = (((ph - G.offs[id]) % 1) + 1) % 1, B = b.vorn ? M : Mh;
    const { t, curl } = hufZiel(G, anwenden(B, [b.x, -56]), r, u, b.vorn);
    const lt = anwenden(invers(B), t), [u1, u2] = ik(lt[0] - b.x, lt[1] + 56, b.vorn, l.ober, l.unter);
    a[`oberbein-${id}`] = u1;
    a[`unterbein-${id}`] = u2;
    a[`huf-${id}`] = -(u1 + u2) - pitch - (b.vorn ? 0 : spine) + curl;
  }
  return a;
}

/** Pose des Vierbeiners (und des Reiters) bei Phase ph (0..1) einer Gangart. */
export function gangPose(spec: Spec, gangart: Gangart, ph: number): GangPose {
  const G = GANGARTEN[gangart], m = spec.ausgabe.massstab ?? 1;
  const dy = G.dy(ph), pitch = G.pitch(ph), spine = G.spine(ph), r = G.reach * m;
  const M = rideMatrix(dy, pitch, m), Mh = mul(M, drehung(spine, 0, -66));
  const lean = G.lean(ph) - (G.riderLevel ?? 0) * pitch, nod = G.nod(ph), s = Math.sin(TAU * ph);
  const winkel = { ...beinWinkel(G, ph, M, Mh, r, pitch, spine, beinLaengen(spec)), hals: nod, schweif: G.tail(ph), 'rumpf-hinten': spine };
  const reiter = { 'oberschenkel-v': -40 - lean, 'unterschenkel-v': 50, 'oberarm-v': -35 - lean + 0.6 * nod, 'unterarm-v': -40,
    'oberarm-h': -30 - lean, 'unterarm-h': -40, kopf: -lean * 0.7 + 2 * s, krone: -2 * s };
  return { winkel, ride: { dy, pitch }, massstab: m, wind: (r * 2) / (G.stance / G.freq),
    reiter: { versatz: [-2, -54 + G.riderY(dy, ph)], drehung: lean, winkel: reiter } };
}
