import Phaser from 'phaser';
import { PLAYER_COLORS } from '../core/constants';
import type { World } from '../model/types';
import { cellRadar, radarRect, type RadarFeed, type RadarKind, type RadarMarker, type RadarRect } from './radar';
import type { Cell } from './layout';

const COLORS: Record<Exclude<RadarKind, 'player'>, number> = {
  castle: 0xffd166,
  portal: 0xb388ff,
  exit: 0x6bd96b,
  stairs: 0x6bd96b,
  enemy: 0xff4d4d,
};
const BACKGROUND = 0x000000;
const FRAME = 0xffffff;

/**
 * Ein Feld, für das das HUD Radar und Werte zeigt: Zelle, Monarch des Feldes (`null` = Feld des Mitspielers),
 * Kamera-Ausschnitt, Stufe der Zelle (`depth`) und die Welt dieser Stufe (`world`, `null` = noch nicht geladen).
 */
export interface RadarCell extends RadarFeed {
  cell: Cell;
  depth: number | null;
  world: World | null;
}

/**
 * Zeichnet das Radar (B-090): je Feld ein `Graphics`-Objekt, das jeden Frame neu gefüllt wird. Nur Zeichnen: was wo
 * steht, kommt aus `radar.ts`. Liegt hinter den Texten des HUD, damit Meldungen lesbar bleiben.
 */
export class RadarLayer {
  private graphics: Phaser.GameObjects.Graphics[] = [];

  constructor(private readonly scene: Phaser.Scene) {}

  /** Zeichnet alle Felder aus der Welt ihrer Stufe; Felder ohne Kamera-Ausschnitt (Layout wechselt) oder ohne geladene Stufe bleiben leer. */
  draw(cells: readonly RadarCell[]): void {
    cells.forEach((c, i) => {
      const g = (this.graphics[i] ??= this.scene.add.graphics().setDepth(-1));
      const model = cellRadar(c);
      g.clear().setVisible(model !== null);
      if (!model) return;
      const rect = radarRect(c.cell);
      this.background(g, rect);
      for (const m of model.markers) this.marker(g, rect, m);
      this.window(g, rect, model.view);
    });
    this.graphics.slice(cells.length).forEach((g) => g.clear().setVisible(false));
  }

  private background(g: Phaser.GameObjects.Graphics, r: RadarRect): void {
    g.fillStyle(BACKGROUND, 0.5).fillRect(r.x, r.y, r.w, r.h);
    g.lineStyle(1, FRAME, 0.35).strokeRect(r.x, r.y, r.w, r.h);
  }

  private marker(g: Phaser.GameObjects.Graphics, r: RadarRect, m: RadarMarker): void {
    const x = r.x + m.pos * r.w;
    const y = r.y + r.h / 2;
    if (m.kind === 'player') {
      g.fillStyle(PLAYER_COLORS[(m.index ?? 0) % PLAYER_COLORS.length], m.down ? 0.4 : 1).fillCircle(x, y, m.own ? 8 : 6);
      if (m.own) g.lineStyle(2, FRAME, m.down ? 0.4 : 1).strokeCircle(x, y, 8);
    } else if (m.kind === 'enemy') {
      g.fillStyle(COLORS.enemy, 1).fillCircle(x, y, 3);
    } else if (m.kind === 'castle') {
      g.fillStyle(COLORS.castle, 1).fillRect(x - 5, y - 5, 10, 10);
    } else {
      g.fillStyle(COLORS[m.kind], 1).fillTriangle(x, y - 7, x - 6, y + 6, x + 6, y + 6);
    }
  }

  private window(g: Phaser.GameObjects.Graphics, r: RadarRect, view: { from: number; to: number }): void {
    const x = r.x + view.from * r.w;
    g.lineStyle(2, FRAME, 0.9).strokeRect(x, r.y + 1, Math.max(2, (view.to - view.from) * r.w), r.h - 2);
  }
}
