import Phaser from 'phaser';
import { GROUND_Y, UNIT_PX } from '../core/constants';
import type { HintDevice } from './glyphs';
import { HINT_FONT, playerHint, type HintView } from './actionHints';
import { fontStyle } from './fontRules';
import { GlyphRow, type RowPart } from './glyphView';
import type { Cell } from './layout';
import type { RadarCell } from './radarView';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
/** Höhe des Overlays über dem Boden in Welt-Pixeln (über Münzbeutel und Lebensbalken des Monarchen) */
const ABOVE_GROUND_PX = 280;
/** Oben frei für den Home-Button */
const TOP_FREE_PX = 70;

/** Hinweis als Zeilenteile: Text vor der Taste, Glyph, Text danach */
const partsOf = (h: HintView): RowPart[] => [h.around[0], h.glyph, h.around[1]];

/** Bildschirm-Punkt `abovePx` über dem Boden bei `xUnits` in der Zelle (Standard: über dem Monarchen); außerhalb des Felds `null` */
export function screenAt(cell: Cell, view: Phaser.Geom.Rectangle, xUnits: number, abovePx = ABOVE_GROUND_PX): { x: number; y: number } | null {
  if (view.width === 0) return null;
  const x = cell.x + ((xUnits * UNIT_PX - view.x) * cell.w) / view.width;
  const y = Math.max(cell.y + TOP_FREE_PX + 40, cell.y + ((GROUND_Y - abovePx - view.y) * cell.h) / view.height);
  return x < cell.x || x > cell.x + cell.w ? null : { x, y };
}

/**
 * Aktionen-Overlay (S3.3, B-319): je Spielerfeld über dem eigenen Monarchen genau ein Hinweis in Nebeninfo-Größe, mit den
 * Tasten des Geräts dieses Spielers; am Preisschild keiner. Gezeichnet im HUD-Raum, damit die Schrift fest bleibt (Q03).
 */
export class ActionOverlay {
  private rows: GlyphRow[] = [];

  constructor(private readonly scene: Phaser.Scene) {}

  /** `cameras[i]` gehört zu `cells[i]` (layoutCameras); `slotOf`/`deviceOf` wie bei der Skill-Leiste */
  draw(cells: readonly RadarCell[], cameras: readonly Phaser.Cameras.Scene2D.Camera[], slotOf: (seat: number) => number | undefined, deviceOf: (slot: number) => HintDevice): void {
    this.rows.forEach((o) => o.box.setVisible(false)); // forEach überspringt Lücken (Feld ohne Overlay)
    cells.forEach(({ cell, monarch, world }, i) => {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      const slot = slotOf(cell.seat);
      const at = p && cameras[i] ? screenAt(cell, cameras[i].worldView, p.x) : null;
      if (!p || !world || slot === undefined || !at) return;
      const hint = playerHint(world, p, deviceOf(slot));
      if (!hint) return;
      (this.rows[i] ??= new GlyphRow(this.scene, { ...STYLE, ...fontStyle(HINT_FONT), strokeThickness: 4 }, 'bottom')).set(partsOf(hint), at.x, at.y);
    });
  }
}
