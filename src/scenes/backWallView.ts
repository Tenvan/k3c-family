import Phaser from 'phaser';
import { GROUND_Y, UNIT_PX } from '../core/constants';
import type { BackWall } from './backWallRules';
import { backWallTexture } from './buildingSprites';

/** Hintergrundmauer (B-442): ein in der Breite gekacheltes Bild hinter den Bauplätzen (Tiefe 1, Bauplätze 2), Maßstab 1. */
export function createBackWallView(scene: Phaser.Scene): Phaser.GameObjects.Container {
  const tile = scene.add.tileSprite(0, 0, UNIT_PX, UNIT_PX, '__DEFAULT').setOrigin(0, 1).setVisible(false);
  return scene.add.container(0, GROUND_Y, [tile]).setDepth(1);
}

/** Lage, Breite und Stufe setzen; fehlt die Textur, bleibt der Abschnitt unsichtbar (kein Platzhalter). */
export function updateBackWallView(scene: Phaser.Scene, v: Phaser.GameObjects.Container, b: BackWall): void {
  const tile = v.getAt(0) as Phaser.GameObjects.TileSprite;
  const key = backWallTexture(b.stage);
  if (!key || !scene.textures.exists(key)) {
    tile.setVisible(false);
    return;
  }
  const height = scene.textures.get(key).getSourceImage().height;
  const x0 = Math.round(b.from * UNIT_PX); // ganze Pixel, sonst flimmert die Naht an den Mauerkanten
  v.setX(x0);
  tile.setTexture(key).setSize(Math.round(b.to * UNIT_PX) - x0, height).setVisible(true);
}
