// Zwei-Knochen-IK des Vierbeiners (Prototyp ik): Winkel in Grad, 0 = Bein hängt senkrecht, positiv = im Uhrzeigersinn.
export const HL1 = 25;
export const HL2 = 31;
const L1 = HL1, L2 = HL2;
const DEG = 180 / Math.PI;

/** Winkel [Oberbein, Unterbein relativ] für ein Ziel (dx, dy) ab Hüfte; knieVorn = Vorderbein (Knie zeigt nach vorn). */
export function ik(dx: number, dy: number, knieVorn: boolean, HL1 = L1, HL2 = L2): [number, number] {
  const d = Math.min(HL1 + HL2 - 0.01, Math.max(Math.abs(HL1 - HL2) + 0.01, Math.hypot(dx, dy)));
  const at = Math.atan2(-dx, dy), al = Math.acos((HL1 * HL1 + d * d - HL2 * HL2) / (2 * HL1 * d));
  const beuge = Math.PI - Math.acos((HL1 * HL1 + HL2 * HL2 - d * d) / (2 * HL1 * HL2));
  return knieVorn ? [(at - al) * DEG, beuge * DEG] : [(at + al) * DEG, -beuge * DEG];
}
