import Phaser from 'phaser';
import { BUILDING_TEXTURES, textureScale } from './buildingSprites';
import { buildingFit, buildingSource } from './buildingSize';

/**
 * Gebäude-Grafiken laden und ins Zielmaß stellen (B-685): Datei und Ausschnitt der aktiven Zuordnung, Maßstab und Filter
 * einmal nach dem Laden je Schlüssel (`prepareBuildingLooks`), beim Zeichnen nur nachschlagen. Nur Darstellung.
 */

const CUT_FRAME = 'b685-cut';

/** Je Schlüssel: Frame (Ausschnitt) und Maßstab; berechnet beim Vorbereiten, nie je Frame */
const LOOKS = new Map<string, { frame?: string; scale: number }>();

/** Zugeordnete Gebäude-Grafiken einreihen (LoadScene.preload, Studio-Bühne) */
export function preloadBuildingLooks(scene: Phaser.Scene): void {
  for (const key of Object.keys(BUILDING_TEXTURES)) {
    const src = buildingSource(key);
    if (src && !scene.textures.exists(key)) scene.load.image(key, `grafik/${src.file}`);
  }
}

/** Nach dem Laden: Ausschnitt als Frame, Maßstab ins Zielmaß, Filter scharf bei ganzzahligem Maßstab, sonst weich */
export function prepareBuildingLooks(scene: Phaser.Scene): void {
  for (const key of Object.keys(BUILDING_TEXTURES)) {
    if (!scene.textures.exists(key)) continue;
    const tex = scene.textures.get(key);
    const img = tex.getSourceImage() as { width: number; height: number };
    const cut = buildingSource(key)?.cut;
    const inside = cut && cut.w > 0 && cut.h > 0 && cut.x >= 0 && cut.y >= 0 && cut.x + cut.w <= img.width && cut.y + cut.h <= img.height;
    if (inside && !tex.has(CUT_FRAME)) tex.add(CUT_FRAME, 0, cut.x, cut.y, cut.w, cut.h);
    const fit = buildingFit(key, inside ? cut : { w: img.width, h: img.height });
    tex.setFilter(fit.smooth ? Phaser.Textures.FilterMode.LINEAR : Phaser.Textures.FilterMode.NEAREST);
    LOOKS.set(key, { frame: inside ? CUT_FRAME : undefined, scale: fit.scale });
  }
}

/** Bild auf die Gebäude-Grafik `key` setzen: Frame und Maßstab aus `prepareBuildingLooks` (sonst ganzes Bild, heutiger Maßstab) */
export function showBuildingLook(image: Phaser.GameObjects.Image, key: string): Phaser.GameObjects.Image {
  const look = LOOKS.get(key);
  return image.setTexture(key, look?.frame).setScale(look?.scale ?? textureScale(key)).setVisible(true);
}
