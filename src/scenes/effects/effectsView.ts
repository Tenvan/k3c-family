import Phaser from 'phaser';
import { UNIT_PX } from '../../core/constants';
import { fontStyle } from '../render/fontRules';
import { EFFECT_CONFIG, MAX_LIVE_EFFECTS, type Effect } from './effects';
import { particleOffset } from './skillLooks';

/**
 * Zeichnet Effekte einer Stufe in deren Ebene (nur Optik, kein Spielzustand). Jeder Effekt ist ein kurzer Ring bzw. Blitz
 * plus Partikel, die gleichmäßig im Kreis auseinanderfliegen (Skills: Bewegung aus ihrem Look, B-578); nach Ablauf wird alles zerstört, höchstens `MAX_LIVE_EFFECTS` zugleich.
 */
export class StageEffects {
  private live = 0;
  private readonly shapes = new Set<Phaser.GameObjects.Arc>();

  private readonly texts = new Set<Phaser.GameObjects.Text>();

  constructor(private readonly scene: Phaser.Scene, private readonly layer: Phaser.GameObjects.Layer) {}

  spawn(effect: Effect): void {
    if (effect.text) return this.label(effect);
    const base = EFFECT_CONFIG[effect.kind];
    const cfg = effect.look ?? { ...base, motion: 'burst' as const };
    const count = 1 + cfg.particles;
    if (this.live + count > MAX_LIVE_EFFECTS) return;
    const color = effect.look?.color ?? effect.color ?? base.color;
    this.piece(effect, color, cfg.radius, cfg.durationMs, 0, 0, base.flash);
    for (let i = 0; i < cfg.particles; i++) {
      const [dx, dy] = particleOffset(cfg.motion, i, cfg.particles, cfg.radius);
      this.piece(effect, color, 5, cfg.durationMs, dx, dy, false);
    }
  }

  /**
   * Kurzer Text am Monarchen, steigt auf und blendet aus; zählt wie eine Form gegen die Höchstzahl. Ein gehaltener Skill
   * meldet `castFailed` je Tick (B-321, keine Abklingzeit): Steht derselbe Text schon an dieser Stelle, kein zweiter (ein
   * Element je Weltposition, B-319).
   */
  private label(effect: Effect): void {
    if (this.live + 1 > MAX_LIVE_EFFECTS) return;
    for (const t of this.texts) if (t.text === effect.text && Math.abs(t.x - effect.x) < UNIT_PX) return;
    const text = this.scene.add.text(effect.x, effect.y, effect.text ?? '', { ...fontStyle('controlsHint'), fontStyle: 'bold', stroke: '#000000', strokeThickness: 4 }).setOrigin(0.5).setDepth(1000);
    this.layer.add(text);
    this.live++;
    this.texts.add(text);
    this.scene.tweens.add({
      targets: text,
      y: effect.y - 24,
      alpha: 0,
      duration: EFFECT_CONFIG[effect.kind].durationMs,
      onComplete: () => {
        this.texts.delete(text);
        text.destroy();
        this.live--;
      },
    });
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
    for (const text of this.texts) this.scene.tweens.killTweensOf(text);
    this.shapes.clear();
    this.texts.clear();
  }
}
