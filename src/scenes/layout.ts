import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';

/**
 * Bildschirmaufteilung für die lokalen Spieler eines Geräts (B-016, Entscheidung 🧑 2026-10-01):
 * 1 Spieler Vollbild (mit Mitspieler eines anderen Geräts oben im ersten Drittel), 2 Streifen,
 * 3 Spieler zwei oben und einer breit unten (Entscheidung 🧑 2026-10-01), 4 Spieler ein 2×2-Raster.
 */
export interface Cell {
  x: number;
  y: number;
  w: number;
  h: number;
  /** `player`: Kamera auf den eigenen Monarchen, `partner`: Kamera auf den Mitspieler */
  kind: 'player' | 'partner';
  /** Platz in der Liste der lokalen Spieler (bei `player`), sonst -1 */
  seat: number;
}

export const MAX_LOCAL_PLAYERS = 4;

export function computeLayout(seats: number, hasPartner: boolean): Cell[] {
  const W = GAME_WIDTH;
  const H = GAME_HEIGHT - BAR_HEIGHT; // unter der Leiste oben
  const Y = BAR_HEIGHT;
  const n = Math.max(1, Math.min(MAX_LOCAL_PLAYERS, seats));
  if (n === 1) {
    return hasPartner
      ? [
          { x: 0, y: Y, w: W, h: H / 3, kind: 'partner', seat: -1 },
          { x: 0, y: Y + H / 3, w: W, h: (H * 2) / 3, kind: 'player', seat: 0 },
        ]
      : [{ x: 0, y: Y, w: W, h: H, kind: 'player', seat: 0 }];
  }
  if (n === 2) return [0, 1].map((i) => ({ x: 0, y: Y + (i * H) / 2, w: W, h: H / 2, kind: 'player' as const, seat: i }));
  return Array.from({ length: n }, (_, i) => {
    const wide = n === 3 && i === 2; // dritter Spieler: untere Reihe über die ganze Breite
    return { x: wide ? 0 : (i % 2) * (W / 2), y: Y + Math.floor(i / 2) * (H / 2), w: wide ? W : W / 2, h: H / 2, kind: 'player', seat: i };
  });
}

/**
 * Leiste oben über die ganze Breite (zwei Zeilen): Raum, Vorrat und Hub, Tageszeit, Kampf, Bürger. Die Werte gelten für
 * alle Spieler und stehen nicht in den Spielerfeldern; die Felder beginnen darunter.
 */
export const BAR_HEIGHT = 96;

/** Weltzeile, ab der die Kameras zeigen: der Himmel darüber entfällt, das Bild wird entsprechend größer gezoomt (B-413) */
export const WORLD_TOP = 420;

/** Zoom einer Zellen-Kamera: die Zelle zeigt die Welt von `WORLD_TOP` bis `GAME_HEIGHT` */
export const viewZoom = (cellHeight: number): number => cellHeight / (GAME_HEIGHT - WORLD_TOP);
