import type Phaser from 'phaser';
import { drawMark, statusMarks, type StatusMark, type StatusWorld } from './statusLooks';

/**
 * Zeichnet die Status-Effekte einer Stufe (B-675) in deren Ebene: Bodenflächen (Auren, Sturm) unter den Figuren, Symbole,
 * Schild und Last Stand darüber. Zwei Graphics je Stufe, je Bild geleert und neu gezeichnet: keine neuen Objekte je Takt,
 * beide Kameras im Split-Screen sehen dieselbe Ebene.
 */
export class StatusView {
  private readonly ground: Phaser.GameObjects.Graphics;
  private readonly over: Phaser.GameObjects.Graphics;
  private readonly marks: StatusMark[] = [];

  constructor(scene: Phaser.Scene, layer: Phaser.GameObjects.Layer) {
    this.ground = scene.add.graphics().setDepth(2); // wie der Warnkreis, unter Figuren
    this.over = scene.add.graphics().setDepth(999);
    layer.add(this.ground);
    layer.add(this.over);
  }

  /** Zustand zeichnen; `timeMs` treibt Drehung, Puls und Blitze. Ohne Effekte bleiben beide Flächen leer. */
  sync(world: StatusWorld, timeMs: number): void {
    this.ground.clear();
    this.over.clear();
    for (const m of statusMarks(world, this.marks)) drawMark(m.kind === 'aura' || m.kind === 'storm' ? this.ground : this.over, m, timeMs);
  }

  destroy(): void {
    this.ground.destroy();
    this.over.destroy();
  }
}
