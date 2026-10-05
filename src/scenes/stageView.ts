import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, UNIT_PX } from '../core/constants';
import { t } from '../core/texts';
import type { World } from '../model/types';
import { StageEffects } from './effectsView';
import { backgroundLayers } from './objectView';
import { WorldRenderer } from './worldRenderer';

/** Hintergrundfarbe einer Zelle, deren Stufe noch nicht geladen ist */
export const PLACEHOLDER_BG = '#202030';
/** Schrift des Platzhalters; die endgültige Größe regelt S4.2 (Mindest-Schriftgröße) */
const PLACEHOLDER_TEXT = { fontSize: '40px', color: '#ffffff', stroke: '#000000', strokeThickness: 4, fontStyle: 'bold' };

function hex(color: string): number {
  return Phaser.Display.Color.HexStringToColor(color).color;
}

/**
 * Eine Stufe als eigene Ebene (Hintergrund, Parallax, Boden, Welt-Objekte). Jede Kamera blendet die Ebenen
 * fremder Stufen aus (`showOnly`), so zeigt jede Zelle die Stufe ihres Spielers.
 */
export class StageView {
  readonly layer: Phaser.GameObjects.Layer;
  readonly renderer: WorldRenderer;
  readonly effects: StageEffects;

  constructor(scene: Phaser.Scene, readonly world: World) {
    this.layer = scene.add.layer();
    const widthPx = world.widthUnits * UNIT_PX;
    const palette = world.biome.palette;
    // Parallax-Ebenen des Bioms (GR3.2); ohne Grafik zwei gezackte Silhouetten (Berge / Höhlenwände).
    const fallback = (): Phaser.GameObjects.GameObject[] => [ridge(scene, widthPx, hex(palette.far), 0.3, 520, 180), ridge(scene, widthPx, hex(palette.near), 0.6, 700, 120)];
    for (const o of backgroundLayers(scene, world.biome.id, widthPx) ?? fallback()) this.layer.add(o);
    this.layer.add(scene.add.rectangle(0, GROUND_Y, widthPx, GAME_HEIGHT - GROUND_Y, hex(palette.ground)).setOrigin(0, 0));
    this.renderer = new WorldRenderer(scene, world, this.layer);
    this.effects = new StageEffects(scene, this.layer);
  }

  destroy(): void {
    this.effects.destroy();
    this.layer.destroy();
  }
}

function ridge(scene: Phaser.Scene, widthPx: number, color: number, scrollFactor: number, baseY: number, amplitude: number): Phaser.GameObjects.Graphics {
  const g = scene.add.graphics().setScrollFactor(scrollFactor, 1);
  g.fillStyle(color, 1);
  const points = [new Phaser.Math.Vector2(0, GROUND_Y)];
  for (let x = 0; x <= widthPx * scrollFactor + GAME_WIDTH * 2; x += 160) {
    points.push(new Phaser.Math.Vector2(x, baseY - Math.abs(Math.sin(x * 0.0021) + Math.sin(x * 0.0057)) * amplitude * 0.6));
  }
  points.push(new Phaser.Math.Vector2(widthPx * scrollFactor + GAME_WIDTH * 2, GROUND_Y));
  g.fillPoints(points, true);
  return g;
}

/** Text „Stufe n wird geladen“ in der Mitte der Spielfläche; die Kamera der Zelle zentriert dort (`centerOn`). */
export function placeholderLayer(scene: Phaser.Scene, depth: number | null): Phaser.GameObjects.Layer {
  const text = depth === null ? t('stage.loading') : t('stage.loadingN', { depth });
  const layer = scene.add.layer();
  layer.add(scene.add.text(GAME_WIDTH / 2, GAME_HEIGHT / 2, text, PLACEHOLDER_TEXT).setOrigin(0.5));
  return layer;
}

/** Jede Kamera sieht nur ihre eigene Ebene; `own[i]` gehört zu `cams[i]`. Setzt die Filter aller Ebenen neu. */
export function showOnly(cams: readonly Phaser.Cameras.Scene2D.BaseCamera[], own: readonly Phaser.GameObjects.Layer[], all: readonly Phaser.GameObjects.Layer[]): void {
  for (const layer of all) layer.cameraFilter = 0;
  cams.forEach((cam, i) => {
    for (const layer of all) if (layer !== own[i]) layer.cameraFilter |= cam.id;
  });
}
