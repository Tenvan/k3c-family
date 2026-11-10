/**
 * Daten des Papierschnitt-Hintergrunds (B-768): Waldschichten, Sternlagen und die Form der Himmelsbahnen. Ohne Phaser, damit Tests sie
 * prüfen können; `paperBackdrop.ts` zeichnet.
 */
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { rng, type Pt } from './paperCard';

/** Waldschichten von hinten nach vorn: Datei, Farbton im Dämmerlicht, Fußlinie und Deckkraft */
export const FOREST_LAYERS: readonly { key: string; file: string; tint: number; y: number; alpha: number }[] = [
  { key: 'menu:wald-fern', file: 'hintergruende/hg-wald-fern-v1/stand.png', tint: 0x4a5aa0, y: GAME_HEIGHT - 20, alpha: 0.85 },
  { key: 'menu:wald-mitte', file: 'hintergruende/hg-wald-mitte-v1/stand.png', tint: 0x3a4a88, y: GAME_HEIGHT + 30, alpha: 0.95 },
  { key: 'menu:wald-nah', file: 'hintergruende/hg-wald-nah-v1/stand.png', tint: 0x28336a, y: GAME_HEIGHT + 90, alpha: 1 },
];

/** Sterne: Lage im Himmel (oben, über den Kacheln und im Rand) */
export const STAR_SPOTS: readonly Pt[] = [
  { x: 70, y: 60 }, { x: 330, y: 150 }, { x: 560, y: 70 }, { x: 790, y: 130 }, { x: 1130, y: 60 },
  { x: 1370, y: 150 }, { x: 1560, y: 70 }, { x: 1850, y: 210 }, { x: 60, y: 300 }, { x: 1860, y: 420 },
];

/** Wellenförmige Papierbahn von `top` bis zum unteren Rand; die Kante ist von Hand geschnitten (deterministisch aus `seed`) */
export function wavyBand(top: number, seed: number, amplitude = 14, step = 160): Pt[] {
  const rand = rng(seed);
  const edge: Pt[] = [];
  for (let x = -step; x <= GAME_WIDTH + step; x += step) edge.push({ x, y: top + (rand() * 2 - 1) * amplitude });
  return [...edge, { x: GAME_WIDTH + step, y: GAME_HEIGHT + 50 }, { x: -step, y: GAME_HEIGHT + 50 }];
}

