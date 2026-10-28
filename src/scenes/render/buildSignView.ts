import type Phaser from 'phaser';
import type { Site } from '../../model/types';

/**
 * Bauschild (B-584): Brett am Stock ohne Text statt des Umrisses am unbezahlten Bauplatz. Drei Varianten nach Art des Platzes
 * (Verteidigung, Siedlung, Treppen); Zeichnung nur, keine Spiel-Logik.
 */

export type SignVariant = 'defense' | 'settlement' | 'stairs';

const VARIANT: Partial<Record<Site['kind'], SignVariant>> = {
  wall: 'defense',
  tower: 'defense',
  stairsUp: 'stairs',
  stairsDown: 'stairs',
};

/** Variante je Platzart; alles Übrige sind Gebäude der Siedlung */
export const signVariant = (kind: Site['kind']): SignVariant => VARIANT[kind] ?? 'settlement';

const BOARD: Record<SignVariant, { wood: number; trim: number }> = {
  defense: { wood: 0x9c6644, trim: 0xc1121f },
  settlement: { wood: 0xb08968, trim: 0xf4d58d },
  stairs: { wood: 0x8d99ae, trim: 0x343a40 },
};

export const SIGN = { stick: 17, stickW: 8, w: 34, h: 22 };

/** Schild nur an einem ungebauten Platz, den der Server als bezahlbar meldet (`payable`, B-587); ohne Feld nie */
export const showsBuildSign = (s: Pick<Site, 'state' | 'payable'>): boolean => s.state !== 'built' && s.payable === true;

/** Zeichnet das Schild mit Fuß bei (0, 0); Oberkante des Bretts liegt bei `-(stick + h)` */
export function drawBuildSign(g: Phaser.GameObjects.Graphics, kind: Site['kind']): void {
  const c = BOARD[signVariant(kind)];
  const { stick, stickW, w, h } = SIGN;
  g.fillStyle(0x6f4518).fillRect(-stickW / 2, -stick, stickW, stick);
  g.fillStyle(c.wood).fillRect(-w / 2, -stick - h, w, h).lineStyle(2, 0x3d2b1f).strokeRect(-w / 2, -stick - h, w, h);
  g.fillStyle(c.trim).fillRect(-w / 2 + 3, -stick - h + 3, w - 6, 4);
  g.fillStyle(0x3d2b1f).fillCircle(-w / 2 + 5, -stick - 5, 1.5).fillCircle(w / 2 - 5, -stick - 5, 1.5);
}
