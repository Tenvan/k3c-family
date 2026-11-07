/**
 * Gangart des berittenen Monarchen als Papierschnitt (US1): Das Monarch-Asset enthält das Pferd in `schritt`, `trab` und
 * `galopp`; im Stand hält `schritt` den ersten Frame. Reine Logik ohne Phaser.
 */

export type Gait = 'schritt' | 'trab' | 'galopp';

/** Anteil der Reittier-Geschwindigkeit, ab dem Trab bzw. Galopp gilt */
export const TROT_FROM = 0.45;
export const GALLOP_FROM = 0.8;
const MOVE_EPS = 0.05;

/** Gangart und ob sie läuft (`moving`) bei Tempo `vx` und Referenz-Tempo `ref` */
export function mountGait(vx: number, ref: number): { gait: Gait; moving: boolean } {
  const v = Math.abs(vx);
  if (v <= MOVE_EPS) return { gait: 'schritt', moving: false };
  const r = ref > 0 ? v / ref : 1;
  return { gait: r >= GALLOP_FROM ? 'galopp' : r >= TROT_FROM ? 'trab' : 'schritt', moving: true };
}
