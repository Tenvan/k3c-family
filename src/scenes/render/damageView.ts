import type Phaser from 'phaser';
import type { DamageStage } from './damageRules';

/**
 * Schadens-Überlagerung über dem Gebäude (WZ14.6): dunkle Tönung und Risse je Stufe, bei Ruinen ein Trümmerhaufen am Fuß.
 * Feste Muster (kein Zufall), gezeichnet in die Graphics des Platzes; neu nur, wenn sich der Cache-Schlüssel ändert.
 */

/** Tönung je Stufe (Deckkraft Schwarz) */
const SHADE: Record<DamageStage, number> = { 0: 0, 1: 0.12, 2: 0.25, 3: 0.4 };

/** Risse als Polylinien in Anteilen der Breite/Höhe (x von −0.5 bis 0.5, y von 0 = Boden bis −1 = oben); je Stufe mehr */
const CRACKS: number[][][] = [
  [[-0.2, -0.95], [-0.12, -0.75], [-0.22, -0.6], [-0.15, -0.45]],
  [[0.25, -0.85], [0.18, -0.65], [0.28, -0.5]],
  [[-0.38, -0.4], [-0.3, -0.28], [-0.36, -0.15]],
  [[0.05, -0.55], [0.12, -0.4], [0.04, -0.25], [0.1, -0.1]],
  [[0.36, -0.35], [0.3, -0.2]],
];
const CRACKS_PER_STAGE: Record<DamageStage, number> = { 0: 0, 1: 2, 2: 4, 3: 5 };

/** Steine des Trümmerhaufens: Mitte x (Anteil der Breite), Breite und Höhe in px */
const STONES: [number, number, number][] = [
  [-0.42, 22, 12], [-0.3, 18, 16], [-0.18, 26, 10], [0.32, 20, 14], [0.44, 16, 10], [0.2, 24, 9],
];

/** Tönung als Tint-Farbe für ein Sprite (Grau, je Stufe dunkler); 0xffffff = ungetönt */
export function damageTint(stage: DamageStage): number {
  const c = Math.round(255 * (1 - SHADE[stage]));
  return (c << 16) | (c << 8) | c;
}

/**
 * Risse über dem Gebäude; `shade`: Tönung als Rechteck (Platzhalter-Form). Ein Sprite tönt stattdessen `damageTint`,
 * damit durchsichtige Pixel nicht als dunkler Kasten erscheinen.
 */
export function drawDamage(g: Phaser.GameObjects.Graphics, stage: DamageStage, w: number, h: number, shade: boolean): void {
  if (stage === 0) return;
  if (shade) g.fillStyle(0x000000, SHADE[stage]).fillRect(-w / 2, -h, w, h);
  g.lineStyle(stage === 3 ? 4 : 3, 0x1b1b1b, 0.85);
  for (const crack of CRACKS.slice(0, CRACKS_PER_STAGE[stage])) {
    g.beginPath();
    crack.forEach(([x, y], i) => (i === 0 ? g.moveTo(x * w, y * h) : g.lineTo(x * w, y * h)));
    g.strokePath();
  }
}

export function drawRubble(g: Phaser.GameObjects.Graphics, w: number): void {
  for (const [x, sw, sh] of STONES) {
    g.fillStyle(0x6c757d).fillRect(x * w - sw / 2, -sh, sw, sh);
    g.lineStyle(2, 0x343a40).strokeRect(x * w - sw / 2, -sh, sw, sh);
  }
}
