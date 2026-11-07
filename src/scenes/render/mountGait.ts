import { PAPER_MOUNTS, manifestAnim } from './paperAssets';

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

/** Das Reittier, auf dem der Monarch im Asset ohne Zusatz sitzt (`schritt`, `trab`, `galopp`) */
const BASE_MOUNT = 'pferd';

/**
 * Animation des Monarch-Assets für Reittier `mountKey` und Gangart (US2): Pferd ohne Zusatz, andere Reittiere
 * `<gangart>-<reittier>` (der Papier-Renderer setzt den Reiter per `animationen.<name>.reittier` auf `reittiere/<id>`); fehlt
 * sie im Asset, das Pferd (`fallback`). Variante je Spieler (`@spieler2`), wenn das Asset sie hat.
 */
export function mountedAnim(mountKey: string, gait: Gait, variante: string | undefined, has: (anim: string) => boolean): { anim: string; reittier: string; fallback: boolean } {
  const wanted = PAPER_MOUNTS[mountKey] ?? '';
  const pick = (name: string) => (variante && has(manifestAnim(name, variante)) ? manifestAnim(name, variante) : name);
  if (wanted === BASE_MOUNT) return { anim: pick(gait), reittier: BASE_MOUNT, fallback: false };
  const own = `${gait}-${wanted}`;
  if (wanted && has(own)) return { anim: pick(own), reittier: wanted, fallback: false };
  return { anim: pick(gait), reittier: BASE_MOUNT, fallback: true };
}
