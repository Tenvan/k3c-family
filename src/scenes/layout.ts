import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';

/**
 * Bildschirmaufteilung für die lokalen Spieler eines Geräts (B-016, Entscheidung 🧑 2026-10-01):
 * 1 Spieler Vollbild (mit Mitspieler eines anderen Geräts oben im ersten Drittel), 2 Streifen, 3–4 ein 2×2-Raster.
 */
export interface Cell {
  x: number;
  y: number;
  w: number;
  h: number;
  /** `player`: Kamera auf den eigenen Monarchen, `partner`: Kamera auf den Mitspieler, `info`: freies Feld im Raster */
  kind: 'player' | 'partner' | 'info';
  /** Platz in der Liste der lokalen Spieler (bei `player`), sonst -1 */
  seat: number;
}

export const MAX_LOCAL_PLAYERS = 4;

export function computeLayout(seats: number, hasPartner: boolean): Cell[] {
  const W = GAME_WIDTH;
  const H = GAME_HEIGHT;
  const n = Math.max(1, Math.min(MAX_LOCAL_PLAYERS, seats));
  if (n === 1) {
    return hasPartner
      ? [
          { x: 0, y: 0, w: W, h: H / 3, kind: 'partner', seat: -1 },
          { x: 0, y: H / 3, w: W, h: (H * 2) / 3, kind: 'player', seat: 0 },
        ]
      : [{ x: 0, y: 0, w: W, h: H, kind: 'player', seat: 0 }];
  }
  if (n === 2) return [0, 1].map((i) => ({ x: 0, y: (i * H) / 2, w: W, h: H / 2, kind: 'player' as const, seat: i }));
  const cells: Cell[] = Array.from({ length: n }, (_, i) => ({ x: (i % 2) * (W / 2), y: Math.floor(i / 2) * (H / 2), w: W / 2, h: H / 2, kind: 'player', seat: i }));
  if (n === 3) cells.push({ x: W / 2, y: H / 2, w: W / 2, h: H / 2, kind: 'info', seat: -1 });
  return cells;
}
