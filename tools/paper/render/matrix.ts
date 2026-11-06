// 2D-Affin-Matrizen im SVG-Format matrix(a b c d e f): x' = a·x + c·y + e, y' = b·x + d·y + f.
// Winkel in Grad, positiv = im Uhrzeigersinn (y wächst nach unten), wie SVG rotate().
import type { Punkt } from '../schema/types';

export type M = [number, number, number, number, number, number];

export const EINHEIT: M = [1, 0, 0, 1, 0, 0];

/** p · q: erst q, dann p anwenden. */
export function mul(p: M, q: M): M {
  return [
    p[0] * q[0] + p[2] * q[1], p[1] * q[0] + p[3] * q[1],
    p[0] * q[2] + p[2] * q[3], p[1] * q[2] + p[3] * q[3],
    p[0] * q[4] + p[2] * q[5] + p[4], p[1] * q[4] + p[3] * q[5] + p[5],
  ];
}

export const verschiebe = (x: number, y: number): M => [1, 0, 0, 1, x, y];
export const skaliere = (s: number): M => [s, 0, 0, s, 0, 0];

/** Drehung um (px, py); 0° liefert genau die Einheit (keine Rundungsreste). */
export function drehung(grad: number, px = 0, py = 0): M {
  if (grad === 0) return EINHEIT;
  const r = (grad * Math.PI) / 180, c = Math.cos(r), s = Math.sin(r);
  return [c, s, -s, c, px - c * px + s * py, py - s * px - c * py];
}

export const anwenden = (m: M, [x, y]: Punkt): Punkt => [m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]];

/** Inverse einer invertierbaren Matrix. */
export function invers(m: M): M {
  const det = m[0] * m[3] - m[1] * m[2];
  const [a, b, c, d] = [m[3] / det, -m[1] / det, -m[2] / det, m[0] / det];
  return [a, b, c, d, -(a * m[4] + c * m[5]), -(b * m[4] + d * m[5])];
}
