import Phaser from 'phaser';
import { GROUND_Y as GAME_GROUND_Y, UNIT_PX } from '@game/core/constants';
import type { KampfWorld } from '@game/scenes/kampfAnzeige';
import { WarnView } from '@game/scenes/kampfView';
import type { Enemy } from '@game/model/types';
import type { Control } from './figure';
import { PROJECTILE, advance, appearAlpha, burstPieces, cycleTime, projectileOffset, warnIn, type TimedEffect } from './timeline';

// Zeichnet die Zeitachsen-Effekte einer Figur (B-671, B-672) mit den Darstellungen des Spiels: Ring und Partikel nach
// `skillFrame`, das Geschoss wie `worldRenderer` (Rechteck 26 × 3 in 70 px Höhe), den Warnkreis mit `WarnView`. Die Zeit
// läuft über die Wiedergabe (Tempo, Schleife, Einzelbild), nicht über Tweens. Nicht headless prüfbar; die Logik steht in
// `timeline.ts` mit Tests.

export class TimedEffects {
  private readonly g: Phaser.GameObjects.Graphics;
  private readonly warnLayer: Phaser.GameObjects.Layer | null;
  private readonly warn: WarnView | null;
  private elapsed = 0;
  private control: Control | null = null;

  /** `root` ist die Figur (Lage x, Bodenlinie y), `frames` die Bildzahl ihrer Animation (für das Einzelbild). */
  constructor(
    scene: Phaser.Scene, private readonly root: Phaser.GameObjects.Container, private readonly effects: readonly TimedEffect[], private readonly frames: () => number,
  ) {
    this.g = scene.add.graphics().setDepth(1000);
    const warns = effects.some((e) => e.kind === 'warn');
    this.warnLayer = warns ? scene.add.layer().setDepth(5) : null;
    this.warn = this.warnLayer ? new WarnView(scene, this.warnLayer) : null;
    // Layer haben keine Lage: die Zeichnung auf die Bodenlinie der Figuren verschieben (wie KampfOverlay)
    for (const o of this.warnLayer?.list ?? []) (o as Phaser.GameObjects.Graphics).setY(root.y - GAME_GROUND_Y);
  }

  /** Wiedergabe übernehmen; ein neues Einzelbild zeichnet sofort. */
  apply(control: Control): void {
    this.control = control;
    this.draw();
  }

  /** Pro Bild der Szene: Zeit weiterlaufen lassen und zeichnen */
  tick(deltaMs: number): void {
    if (!this.control) return;
    this.elapsed = advance(this.elapsed, deltaMs, this.control);
    this.draw();
  }

  private draw(): void {
    const c = this.control;
    if (!c) return;
    this.g.clear();
    let alpha = 1;
    for (const e of this.effects) {
      const t = cycleTime(e, c, this.elapsed, this.frames());
      if (e.kind === 'appear') alpha = appearAlpha(e.cycleMs, t);
      else this.drawOne(e, t);
    }
    this.root.setAlpha(alpha);
  }

  private drawOne(e: TimedEffect, t: number): void {
    const x = this.root.x;
    const ground = this.root.y;
    switch (e.kind) {
      case 'burst': {
        const cx = x + (e.at ?? 0) * UNIT_PX;
        const cy = ground - (e.y ?? 50);
        for (const p of burstPieces(e.look, t, e.delayMs)) {
          this.g.fillStyle(e.look.color, p.alpha).fillCircle(cx + p.x, cy + p.y, p.radius * p.scale);
        }
        return;
      }
      case 'projectile': {
        const dx = projectileOffset(e, t);
        if (dx !== null) this.g.fillStyle(e.color, 1).fillRect(x + dx - PROJECTILE.width / 2, ground - PROJECTILE.y - PROJECTILE.height / 2, PROJECTILE.width, PROJECTILE.height);
        return;
      }
      case 'warn': {
        const world: KampfWorld = { enemies: t < e.cycleMs ? [{ id: 1, warn: { x: x / UNIT_PX, r: e.radius, in: warnIn(e.cycleMs, t) } } as unknown as Enemy] : [] };
        this.warn?.sync(world);
        return;
      }
      case 'frame': {
        const a = appearAlpha(e.cycleMs, t);
        this.g.lineStyle(3, 0xffd166, a).strokeRect(x + e.at * UNIT_PX - 24, ground - 96, 48, 96);
        return;
      }
      default:
    }
  }

  destroy(): void {
    this.g.destroy();
    this.warnLayer?.destroy(true);
  }
}
