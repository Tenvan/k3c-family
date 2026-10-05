import Phaser from 'phaser';
import { GAME_WIDTH, GROUND_Y, UNIT_PX } from '../core/constants';
import type { Device } from '../input/slotBindings';
import { playerHints } from './actionHints';
import { fontStyle } from './fontRules';
import type { Cell } from './layout';
import type { RadarCell } from './radarView';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
/** Höhe des Overlays über dem Boden in Welt-Pixeln (über Münzbeutel und Lebensbalken des Monarchen) */
const ABOVE_GROUND_PX = 280;
/** Oben frei für den Home-Button */
const TOP_FREE_PX = 70;

/** Bildschirm-Punkt über dem Monarchen bei `xUnits` in der Zelle; außerhalb des Felds `null` */
function screenAt(cell: Cell, view: Phaser.Geom.Rectangle, xUnits: number): { x: number; y: number } | null {
  if (view.width === 0) return null;
  const x = cell.x + ((xUnits * UNIT_PX - view.x) * cell.w) / view.width;
  const y = Math.max(cell.y + TOP_FREE_PX + 40, cell.y + ((GROUND_Y - ABOVE_GROUND_PX - view.y) * cell.h) / view.height);
  return x < cell.x || x > cell.x + cell.w ? null : { x, y };
}

/**
 * Aktionen-Overlay (S3.3): je Spielerfeld über dem eigenen Monarchen die wichtigste gültige Aktion groß, die übrigen
 * kleiner darunter, mit den Tasten des Geräts dieses Spielers. Gezeichnet im HUD-Raum, damit die Schrift fest bleibt (Q03).
 */
export class ActionOverlay {
  private main: Phaser.GameObjects.Text[] = [];
  private rest: Phaser.GameObjects.Text[] = [];

  constructor(private readonly scene: Phaser.Scene) {}

  /** `cameras[i]` gehört zu `cells[i]` (layoutCameras); `slotOf`/`deviceOf` wie bei der Skill-Leiste */
  draw(cells: readonly RadarCell[], cameras: readonly Phaser.Cameras.Scene2D.Camera[], slotOf: (seat: number) => number | undefined, deviceOf: (slot: number) => Device): void {
    for (const list of [this.main, this.rest]) list.forEach((o) => o.setVisible(false)); // forEach überspringt Lücken (Feld ohne Overlay)
    const compact = cells.some((c) => c.cell.w < GAME_WIDTH); // 3 bis 4 Spieler: nur die wichtigste Aktion (kürzen statt verkleinern)
    cells.forEach(({ cell, monarch, world }, i) => {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      const slot = slotOf(cell.seat);
      const at = p && cameras[i] ? screenAt(cell, cameras[i].worldView, p.x) : null;
      if (!p || !world || slot === undefined || !at) return;
      const hints = playerHints(world, p, deviceOf(slot));
      if (hints.length === 0) return;
      const { x, y } = at;
      const main = (this.main[i] ??= this.scene.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue') }).setOrigin(0.5, 1));
      main.setText(hints[0]!.text).setPosition(x, y).setVisible(true);
      if (compact || hints.length < 2) return;
      const rest = (this.rest[i] ??= this.scene.add.text(0, 0, '', { ...STYLE, ...fontStyle('controlsHint'), strokeThickness: 4 }).setOrigin(0.5, 0));
      rest.setText(hints.slice(1).map((h) => h.text).join('  ·  ')).setPosition(x, y + 4).setVisible(true);
    });
  }
}
