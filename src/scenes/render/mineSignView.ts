import type Phaser from 'phaser';
import type { ResourceNode } from '../../model/types';

/**
 * Abbau-Schild (B-588, GEB16): Brett am Stock am Fuß eines Knotens, den der Server als abbaubar meldet (`minable`), mit Axt
 * (Baum) bzw. Spitzhacke (Fels, Erz, Ader). Unbekannte Art oder fehlende Grafik: Brett ohne Symbol, nie ein falsches.
 */

export const MINE_SYMBOLS = { axe: 'grafik:symbol-axt', pickaxe: 'grafik:symbol-spitzhacke' } as const;

/** Textur-Schlüssel → Datei unter `public/grafik/` (`NATURE_TEXTURES` nimmt sie auf) */
export const MINE_SYMBOL_TEXTURES: Record<string, string> = {
  [MINE_SYMBOLS.axe]: 'objekte/schild-axt/idle.png', // ganzes Schild am Pfahl als Papierschnitt (MG8, US2)
  [MINE_SYMBOLS.pickaxe]: 'objekte/schild-spitzhacke/idle.png',
};

/** Symbol je Knotenart: Baum → Axt, Fels, Erzbrocken und Adern → Spitzhacke, sonst keins */
export function mineSymbol(kind: string): string | null {
  if (kind === 'tree') return MINE_SYMBOLS.axe;
  if (kind === 'rock' || kind === 'copperOre' || kind.endsWith('Vein')) return MINE_SYMBOLS.pickaxe;
  return null;
}

/** Schild nur an einem zum Abbau markierten Knoten, anstelle des früheren Wimpels (B-653); unmarkiert nie */
export const showsMineSign = (n: Pick<ResourceNode, 'marked'>): boolean => n.marked === true;

const BOARD = 40;
const STICK = 14;

/** Schild mit Fuß bei (`x`, 0): Papier-Schild der Knotenart, ohne Grafik ein gezeichnetes Brett ohne Symbol; Sichtbarkeit setzt der Aufrufer */
export function createMineSign(scene: Phaser.Scene, kind: string, x: number): Phaser.GameObjects.Container {
  const key = mineSymbol(kind);
  // Papierschnitt-Schild samt Pfahl (MG8, US2): ersetzt Brett und Stock, Fuß am Boden
  if (key && scene.textures.exists(key)) return scene.add.container(x, 0, [scene.add.image(0, 0, key, 0).setOrigin(0.5, 1)]).setVisible(false);
  const g = scene.add.graphics();
  g.fillStyle(0x6f4518).fillRect(-4, -STICK, 8, STICK);
  g.fillStyle(0xb08968).fillRect(-BOARD / 2, -STICK - BOARD, BOARD, BOARD).lineStyle(2, 0x3d2b1f).strokeRect(-BOARD / 2, -STICK - BOARD, BOARD, BOARD);
  return scene.add.container(x, 0, [g]).setVisible(false);
}
