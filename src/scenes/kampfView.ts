import type Phaser from 'phaser';
import { GAME_WIDTH, GROUND_Y } from '../core/constants';
import { fontStyle } from './render/fontRules';
import { BAR_HEIGHT } from './layout';
import { bossBar, eventBanner, warnCircles, type KampfWorld, type WarnCircleView } from './kampfAnzeige';

/**
 * Zeichnet die Kampf-Anzeige (K5.1, B-132) aus `kampfAnzeige.ts`: Boss-Leiste und Event-Banner bildschirmfest im HUD (einmal für
 * alle Spieler), den Warnkreis in der Welt jeder Stufe. Platzhalter-Gestaltung, Abnahme am Gerät in K5.4.
 */

export const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold', align: 'center' };
export const BAR = { w: 640, h: 22, gap: 12 } as const;
export const WARN_COLOR = 0xe63946;

/** Lebensleiste des Bosses bei (x, y); `share` 0..1. Gemeinsam für das Spiel und die Szene „Kampf-Anzeigen“ des Studios (B-602). */
export function drawBar(g: Phaser.GameObjects.Graphics, x: number, y: number, share: number): void {
  g.fillStyle(0x000000, 0.7).fillRect(x, y, BAR.w, BAR.h);
  g.fillStyle(WARN_COLOR, 1).fillRect(x + 3, y + 3, (BAR.w - 6) * share, BAR.h - 6);
  g.lineStyle(2, 0xffffff, 1).strokeRect(x, y, BAR.w, BAR.h);
}

/** Warnkreise um den Boden `groundY`; je kräftiger, desto näher der Schlag. */
export function drawWarn(g: Phaser.GameObjects.Graphics, circles: readonly WarnCircleView[], groundY: number): void {
  for (const c of circles) {
    g.fillStyle(WARN_COLOR, 0.15 + 0.35 * c.urgency).fillCircle(c.x, groundY, c.r);
    g.lineStyle(4, WARN_COLOR, 0.6 + 0.4 * c.urgency).strokeCircle(c.x, groundY, c.r);
  }
}

/** Boss-Leiste oben mittig unter der Leiste für alle, darunter die laufenden Events. */
export class KampfHud {
  private readonly g: Phaser.GameObjects.Graphics;
  private readonly title: Phaser.GameObjects.Text;
  private readonly events: Phaser.GameObjects.Text;

  constructor(scene: Phaser.Scene) {
    this.g = scene.add.graphics();
    this.title = scene.add.text(GAME_WIDTH / 2, BAR_HEIGHT + BAR.gap, '', { ...STYLE, ...fontStyle('bossBar') }).setOrigin(0.5, 0);
    this.events = scene.add.text(GAME_WIDTH / 2, 0, '', { ...STYLE, ...fontStyle('banner'), strokeThickness: 4 }).setOrigin(0.5, 0);
  }

  draw(world: KampfWorld | null): void {
    const boss = world ? bossBar(world) : null;
    const g = this.g.clear();
    this.title.setText(boss ? [boss.name, boss.phase].filter(Boolean).join('  ·  ') : '').setVisible(!!boss);
    let y = BAR_HEIGHT + BAR.gap;
    if (boss) {
      y += this.title.height + 4;
      const x = (GAME_WIDTH - BAR.w) / 2;
      drawBar(g, x, y, boss.share);
      y += BAR.h + BAR.gap;
    }
    const text = world ? eventBanner(world) : null;
    this.events.setText(text ?? '').setVisible(!!text).setY(y);
  }
}

/** Warnkreise vor dem Flächenschlag, um den Boden in der Ebene der Stufe; je kräftiger, desto näher der Schlag. */
export class WarnView {
  private readonly g: Phaser.GameObjects.Graphics;

  constructor(scene: Phaser.Scene, root: Phaser.GameObjects.Layer) {
    this.g = scene.add.graphics().setDepth(2);
    root.add(this.g);
  }

  sync(world: KampfWorld): void {
    const g = this.g.clear();
    drawWarn(g, warnCircles(world), GROUND_Y);
  }
}
