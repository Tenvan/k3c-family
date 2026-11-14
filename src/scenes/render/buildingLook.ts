import Phaser from 'phaser';
import { BUILDING_TEXTURES } from './buildingSprites';
import { buildingFit, buildingSource } from './buildingSize';

/**
 * Gebäude-Grafiken laden und darstellen: Filter einmal nach dem Laden je Schlüssel (`prepareBuildingLooks`), beim Zeichnen
 * nur nachschlagen. Nur Darstellung.
 */

/** Je Schlüssel: Maßstab; berechnet beim Vorbereiten, nie je Frame */
const LOOKS = new Map<string, { scale: number }>();

/** Zugeordnete Gebäude-Grafiken einreihen (LoadScene.preload, Studio-Bühne) */
export function preloadBuildingLooks(scene: Phaser.Scene): void {
  for (const key of Object.keys(BUILDING_TEXTURES)) {
    const src = buildingSource(key);
    if (src && !scene.textures.exists(key)) scene.load.image(key, `grafik/${src.file}`);
  }
}

/** Nach dem Laden: Maßstab und Filter (Papier: weich) je Schlüssel */
export function prepareBuildingLooks(scene: Phaser.Scene): void {
  for (const key of Object.keys(BUILDING_TEXTURES)) {
    if (!scene.textures.exists(key)) continue;
    const tex = scene.textures.get(key);
    const img = tex.getSourceImage() as { width: number; height: number };
    const fit = buildingFit(key, { w: img.width, h: img.height });
    tex.setFilter(fit.smooth ? Phaser.Textures.FilterMode.LINEAR : Phaser.Textures.FilterMode.NEAREST);
    LOOKS.set(key, { scale: fit.scale });
  }
}

/** Bild auf die Gebäude-Grafik `key` setzen: Maßstab aus `prepareBuildingLooks` (sonst 1) */
export function showBuildingLook(image: Phaser.GameObjects.Image, key: string): Phaser.GameObjects.Image {
  const look = LOOKS.get(key);
  return image.setTexture(key).setScale(look?.scale ?? 1).setVisible(true);
}
