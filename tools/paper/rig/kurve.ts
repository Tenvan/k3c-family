// Catmull-Rom wie im Prototyp (docs/decisions/005/prototyp-monarch-rig.html › crPath, spline).
import { num } from '../schema/hash';
import type { Punkt } from '../schema/types';

/** Kurve durch alle Punkte als kubische Bézier-Segmente; cont = an einen vorhandenen Pfad anschließen (L statt M). */
export function crPath(pts: Punkt[], cont = false): string {
  if (pts.length === 0) return '';
  const p = (x: number, y: number) => `${num(x)} ${num(y)}`;
  let d = (cont ? 'L' : 'M') + p(pts[0][0], pts[0][1]);
  for (let i = 0; i < pts.length - 1; i++) {
    const p0 = pts[Math.max(i - 1, 0)], p1 = pts[i], p2 = pts[i + 1], p3 = pts[Math.min(i + 2, pts.length - 1)];
    d += `C${p(p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6)} ${p(p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6)} ${p(p2[0], p2[1])}`;
  }
  return d;
}

/** Punkt auf der Catmull-Rom-Kurve durch alle Stützpunkte, k = 0..1. */
export function spline(P: Punkt[], k: number): Punkt {
  const n = P.length - 1, s = Math.min(k, 0.9999) * n, i = Math.floor(s), t = s - i;
  const p0 = P[Math.max(i - 1, 0)], p1 = P[i], p2 = P[i + 1], p3 = P[Math.min(i + 2, n)];
  const f = (j: 0 | 1) => 0.5 * (2 * p1[j] + (p2[j] - p0[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2[j] - p3[j]) * t * t
    + (3 * p1[j] - p0[j] - 3 * p2[j] + p3[j]) * t * t * t);
  return [f(0), f(1)];
}
