import Phaser from 'phaser';
import { EFFECT_CONFIG, MAX_LIVE_EFFECTS, type Effect } from './effects';

/**
 * Zeichnet Effekte einer Stufe in deren Ebene (nur Optik, kein Spielzustand). Jeder Effekt ist ein kurzer Ring bzw. Blitz
 * plus Partikel, die gleichmäßig im Kreis auseinanderfliegen; nach Ablauf wird alles zerstört, höchstens `MAX_LIVE_EFFECTS` zugleich.
 */
export class StageEffects {
  private live = 0;
  private readonly shapes = new Set<Phaser.GameObjects.Arc>();

  constructor(private readonly scene: Phaser.Scene, private readonly layer: Phaser.GameObjects.Layer) {}

  spawn(effect: Effect): void {
    const cfg = EFFECT_CONFIG[effect.kind];
    const count = 1 + cfg.particles;
    if (this.live + count > MAX_LIVE_EFFECTS) return;
    this.piece(effect, cfg.color, cfg.radius, cfg.durationMs, 0, 0, cfg.flash);
    for (let i = 0; i < cfg.particles; i++) {
      const angle = (i / cfg.particles) * Math.PI * 2;
      this.piece(effect, cfg.color, 5, cfg.durationMs, Math.cos(angle) * cfg.radius * 1.4, Math.sin(angle) * cfg.radius * 1.4 - 12, false);
    }
  }

  private piece(effect: Effect, color: number, radius: number, durationMs: number, dx: number, dy: number, flash: boolean): void {
    const shape = this.scene.add.circle(effect.x, effect.y, radius, color, flash ? 0.9 : 0.8).setDepth(1000);
    this.layer.add(shape);
    this.live++;
    this.shapes.add(shape);
    this.scene.tweens.add({
      targets: shape,
      x: effect.x + dx,
      y: effect.y + dy,
      alpha: 0,
      scale: dx === 0 && dy === 0 ? 1.8 : 0.5,
      duration: durationMs,
      onComplete: () => {
        this.shapes.delete(shape);
        shape.destroy();
        this.live--;
      },
    });
  }

  /** Stufe wird abgebaut: laufende Tweens beenden, bevor die Ebene ihre Objekte zerstört. */
  destroy(): void {
    for (const shape of this.shapes) this.scene.tweens.killTweensOf(shape);
    this.shapes.clear();
  }
}
