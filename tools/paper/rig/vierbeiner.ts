// Rig vierbeiner: Ride-Matrix translate(0, dy) · rotate(pitch um (0, −80)) in Welt-Einheiten, darin scale(massstab);
// die Neigung steckt in der Ride-Matrix, das Rückengelenk ist der Winkel von rumpf-hinten (Pivot 0, −66).
import { num } from '../schema/hash';
import type { Spec } from '../schema/types';
import { weltMatrizen } from '../render/baum';
import { drehung, type M, mul, skaliere, verschiebe } from '../render/matrix';

export const rideMatrix = (dy: number, pitch: number, massstab: number): M =>
  mul(verschiebe(0, dy), mul(drehung(pitch, 0, -80), skaliere(massstab)));

/** Ride-Transformation als SVG-Text (ohne Maßstab): translate(0 dy) rotate(pitch 0 -80). */
export const rideText = (dy: number, pitch: number) => `translate(0 ${num(dy)}) rotate(${num(pitch)} 0 -80)`;

/** Welt-Matrizen aller Teile des Vierbeiners in einer Pose. */
export function vierbeinerMatrizen(spec: Spec, p: { winkel: Record<string, number>; ride: { dy: number; pitch: number }; massstab: number }): Map<string, M> {
  return weltMatrizen(spec, p.winkel, rideMatrix(p.ride.dy, p.ride.pitch, p.massstab));
}
