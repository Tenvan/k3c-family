/**
 * Hintergrund der Kachelseiten im Papierschnitt-Stil (B-768): Nachthimmel aus drei geschnittenen Papierbahnen, ein Papiermond,
 * funkelnde Sterne (die Stern-Grafik des Spiels) und die drei Waldschichten des Spiels (`hintergruende/hg-wald-*`) im Dämmerlicht
 * davor. Die Bilder laden hier selbst (klein, einmal je Seite); fehlen sie, bleibt der Himmel. Zeichnet nur.
 */
import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { objectImage } from './render/objectView';
import { MATERIAL, SHADOW, SHADOW_LAYERS, cutPolygon, type Pt } from './paperCard';
import { FOREST_LAYERS, STAR_SPOTS, wavyBand } from './paperBackdropData';
import { HUB_COLORS } from './pauseHub';

const vec = (points: readonly Pt[]): Phaser.Math.Vector2[] => points as unknown as Phaser.Math.Vector2[];

export class PaperBackdrop {
  private readonly dim: Phaser.GameObjects.Rectangle;

  /** `depth`: alles dieses Hintergrunds liegt in den Tiefen `depth` bis `depth + 5` (unter den Kacheln) */
  constructor(
    private readonly scene: Phaser.Scene,
    private readonly depth = -100,
  ) {
    this.drawSky();
    this.addStars();
    FOREST_LAYERS.forEach((layer, i) => this.addForest(layer, i));
    this.dim = scene.add.rectangle(0, 0, GAME_WIDTH, GAME_HEIGHT, HUB_COLORS.night, 0).setOrigin(0).setDepth(depth + 4);
  }

  /** Leinen-Banner für den Titel: geschnittenes Papier mit Schatten, `cx`/`cy` Mitte (liegt über dem Hintergrund, unter dem Text) */
  banner(cx: number, cy: number, w: number, h: number): Phaser.GameObjects.Graphics {
    const g = this.scene.add.graphics();
    const poly = cutPolygon(cx - w / 2, cy - h / 2, w, h, 23, 150, 1.8, 8);
    for (const { d, alpha } of SHADOW_LAYERS) g.fillStyle(SHADOW, alpha).fillPoints(vec(poly.map((p) => ({ x: p.x + d * 0.5, y: p.y + d * 0.8 }))), true, true);
    g.fillStyle(MATERIAL.leinen.licht, 1).fillPoints(vec(poly), true, true);
    g.lineStyle(2, MATERIAL.leinen.schatten, 0.5).strokePoints(vec(poly), true, true);
    return g;
  }

  /** Abdunkeln (0–1), wenn Listen oder Optionen mit heller Schrift auf dem Hintergrund stehen */
  setDim(alpha: number): void {
    this.dim.setAlpha(alpha);
  }

  private drawSky(): void {
    const g = this.scene.add.graphics().setDepth(this.depth);
    g.fillStyle(HUB_COLORS.night, 1).fillRect(0, 0, GAME_WIDTH, GAME_HEIGHT);
    for (const [top, seed, color] of [[430, 3, 0x1b2550], [690, 7, 0x3b3a6b]] as const) {
      const band = wavyBand(top, seed);
      g.fillStyle(color, 1).fillPoints(vec(band), true, true);
      g.lineStyle(3, 0xffffff, 0.14).strokePoints(vec(band.slice(0, -2)), false);
    }
    // Papiermond: Schatten, heller Kreis, ausgeschnittene Sichel in der Farbe des Himmels
    const mx = 1660;
    const my = 150;
    for (const { d, alpha } of SHADOW_LAYERS) g.fillStyle(SHADOW, alpha).fillCircle(mx + d * 0.6, my + d, 56);
    g.fillStyle(MATERIAL.leinen.licht, 1).fillCircle(mx, my, 56);
    g.fillStyle(HUB_COLORS.night, 1).fillCircle(mx - 22, my - 12, 50);
    const moonRim = cutPolygon(mx - 56, my - 56, 112, 112, 11, 56, 1, 28);
    g.lineStyle(2, MATERIAL.leinen.schatten, 0.2).strokePoints(vec(moonRim), true, true);
  }

  private addStars(): void {
    STAR_SPOTS.forEach((spot, i) => {
      const star = objectImage(this.scene, 'pickup:skillPoint', spot.x, spot.y);
      if (!star) return;
      star.setDepth(this.depth + 1).setScale(star.scaleX * (0.7 + (i % 3) * 0.18)).setAlpha(0.9);
      star.anims.setProgress((i * 0.137) % 1); // nicht alle im Gleichschritt
    });
  }

  /** Waldschicht laden (falls nötig) und im Dämmerlicht davorstellen */
  private addForest(layer: (typeof FOREST_LAYERS)[number], index: number): void {
    const show = (): void => {
      if (!this.scene.textures.exists(layer.key) || !this.scene.sys.isActive()) return;
      const image = this.scene.add.image(GAME_WIDTH / 2, layer.y, layer.key).setOrigin(0.5, 1).setDepth(this.depth + 2 + index * 0.1).setTint(layer.tint).setAlpha(layer.alpha);
      image.setScale(GAME_WIDTH / image.width);
    };
    if (this.scene.textures.exists(layer.key)) return show();
    this.scene.load.image(layer.key, `grafik/${layer.file}`);
    this.scene.load.once(`filecomplete-image-${layer.key}`, show);
    if (!this.scene.load.isLoading()) this.scene.load.start();
  }
}
