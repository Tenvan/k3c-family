import Phaser from 'phaser';
import monarchData from '../data/monarch.json';
import { GROUND_Y, UNIT_PX } from '../core/constants';
import type { PlayerInput } from '../input/playerInput';

const SPRINT_MULTIPLIER = 1.8;
/** Wie schnell die Zielgeschwindigkeit erreicht wird (1/s). Höher = direkter. */
const ACCELERATION = 8;

/** Ein Monarch = ein Couch-Koop-Spieler. Nur horizontale Bewegung (kein Springen, wie K2C). */
export class Monarch extends Phaser.GameObjects.Container {
  velocityX = 0;
  private readonly speedPx = monarchData.base.speed * UNIT_PX;

  constructor(
    scene: Phaser.Scene,
    x: number,
    readonly playerIndex: number,
    readonly controls: PlayerInput,
    color: number,
  ) {
    super(scene, x, GROUND_Y);
    // Platzhalter-Grafik: Körper + Krone. Wird später durch Sprites ersetzt.
    const body = scene.add.rectangle(0, -40, 36, 80, color).setStrokeStyle(3, 0x000000);
    const crown = scene.add.rectangle(0, -88, 28, 12, 0xffd166).setStrokeStyle(2, 0x000000);
    this.add([body, crown]);
    scene.add.existing(this);
  }

  step(deltaSeconds: number, minX: number, maxX: number): void {
    const target = this.controls.moveX() * this.speedPx * (this.controls.sprint() ? SPRINT_MULTIPLIER : 1);
    const blend = 1 - Math.exp(-ACCELERATION * deltaSeconds);
    this.velocityX += (target - this.velocityX) * blend;
    this.x = Phaser.Math.Clamp(this.x + this.velocityX * deltaSeconds, minX, maxX);
    if (Math.abs(this.velocityX) > 1) this.scaleX = Math.sign(this.velocityX);
  }
}
