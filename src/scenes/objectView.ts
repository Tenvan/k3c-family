import Phaser from 'phaser';
import { GAME_WIDTH, GROUND_Y } from '../core/constants';
import { WORLD_FRAMES, WORLD_TEXTURES, biomeBackground, objectSprite } from './worldSprites';

/** Lädt, bereitet vor und setzt die Welt-Grafiken aus worldSprites.ts (GR3.2). Zeichnet nur. */

/** Alle zugeordneten Welt-Grafiken laden (zusammen klein, daher nicht je Biom getrennt) */
export function preloadWorld(scene: Phaser.Scene): void {
  for (const [key, { file, frameWidth, frameHeight }] of Object.entries(WORLD_TEXTURES)) {
    if (frameWidth && frameHeight) scene.load.spritesheet(key, `grafik/${file}`, { frameWidth, frameHeight });
    else scene.load.image(key, `grafik/${file}`);
  }
}

/** Nach dem Laden: Pixel-Art scharf, Ausschnitte und Animationen (Portal, Stern, Münze, Lagerfeuer) anlegen */
export function prepareWorld(scene: Phaser.Scene): void {
  for (const key of Object.keys(WORLD_TEXTURES)) {
    if (!scene.textures.exists(key)) continue;
    const texture = scene.textures.get(key);
    texture.setFilter(Phaser.Textures.FilterMode.NEAREST);
    for (const [name, x, y, w, h] of WORLD_FRAMES[key] ?? []) if (!texture.has(name)) texture.add(name, 0, x, y, w, h);
  }
  for (const id of ['portal', 'pickup:skillPoint', 'coin', 'camp:recruit:fire']) {
    const s = objectSprite(id);
    if (!s || typeof s.frame !== 'number' || !scene.textures.exists(s.key) || scene.anims.exists(id)) continue;
    const frames = scene.anims.generateFrameNumbers(s.key, { start: s.frame, end: s.frame + s.frames - 1 });
    scene.anims.create({ key: id, frames, frameRate: 8, repeat: -1 });
  }
}

/** Sprite eines Objekts mit Fußpunkt (x, y); `null` = keine Grafik, dann zeichnet der Aufrufer die Platzhalter-Form */
export function objectImage(scene: Phaser.Scene, id: string, x: number, y: number): Phaser.GameObjects.Sprite | null {
  const s = objectSprite(id);
  if (!s || !scene.textures.exists(s.key)) return null;
  const image = scene.add.sprite(x, y, s.key, s.frame).setOrigin(0.5, 1).setScale(s.zoom);
  if (s.frames > 1 && scene.anims.exists(id)) image.play(id);
  return image;
}

/**
 * Parallax-Ebenen eines Bioms als TileSprites, Unterkante auf dem Boden. Breit genug für die ganze Welt beim jeweiligen
 * Scroll-Faktor; jede Kamera verschiebt sie selbst. `null` = Biom ohne Zuordnung oder Textur fehlt → Silhouetten.
 */
export function backgroundLayers(scene: Phaser.Scene, biome: string, widthPx: number): Phaser.GameObjects.TileSprite[] | null {
  const bg = biomeBackground(biome);
  if (!bg || !bg.layers.every((l) => scene.textures.exists(l.key))) return null;
  return bg.layers.map(({ key, scroll }) => {
    const frame = scene.textures.getFrame(key);
    const width = Math.ceil((widthPx * scroll + GAME_WIDTH * 2) / bg.zoom);
    return scene.add.tileSprite(0, GROUND_Y, width, frame.height, key).setOrigin(0, 1).setScale(bg.zoom).setScrollFactor(scroll, 1);
  });
}
