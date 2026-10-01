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

export interface Anchor {
  x: number;
  y: number;
  /** 1 = rechtsbündig, 0.5 = zentriert */
  originX: number;
}

/**
 * Wo die gemeinsamen Anzeigen (Vorrat, Tageszeit, Kampf) stehen (B-084): oben rechts bei Vollbild und Streifen,
 * im Info-Feld bei 3 Spielern, mittig am Kreuzpunkt bei 4 Spielern – nie in der Ecke eines Spielerfelds.
 */
export function sharedAnchor(cells: readonly Cell[]): Anchor {
  const info = cells.find((c) => c.kind === 'info');
  if (info) return { x: info.x + info.w / 2, y: info.y + 16, originX: 0.5 };
  if (cells.some((c) => c.w < GAME_WIDTH)) return { x: GAME_WIDTH / 2, y: GAME_HEIGHT / 2 - SHARED_BLOCK_HEIGHT / 2, originX: 0.5 };
  return { x: GAME_WIDTH - 24, y: 16, originX: 1 };
}

/** Höhe der drei gemeinsamen Textzeilen (Zeilenabstand 40 px) */
export const SHARED_LINE_HEIGHT = 40;
const SHARED_BLOCK_HEIGHT = SHARED_LINE_HEIGHT * 3;
