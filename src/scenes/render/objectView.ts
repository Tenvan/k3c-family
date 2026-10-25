import Phaser from 'phaser';
import { GROUND_Y } from '../../core/constants';
import { BACKGROUND_ZOOM, WORLD_TEXTURES, backgroundFile, objectSprite, planKeys, type BackgroundLayerPlan, type ObjectSprite } from './worldSprites';

/** Lädt, bereitet vor und setzt die Welt-Grafiken aus worldSprites.ts (GR3.2). Zeichnet nur. */

/** Alle zugeordneten Welt-Grafiken laden (zusammen klein, daher nicht je Biom getrennt) */
export function preloadWorld(scene: Phaser.Scene): void {
  for (const [key, { file, frameWidth, frameHeight }] of Object.entries(WORLD_TEXTURES)) {
    if (frameWidth && frameHeight) scene.load.spritesheet(key, `grafik/${file}`, { frameWidth, frameHeight });
    else scene.load.image(key, `grafik/${file}`);
  }
}

/** Nach dem Laden: Pixel-Art scharf, Animationen (Stern, Münze) anlegen; Portal und Lager stehen still (GEB6) */
export function prepareWorld(scene: Phaser.Scene): void {
  for (const key of Object.keys(WORLD_TEXTURES)) {
    if (!scene.textures.exists(key)) continue;
    scene.textures.get(key).setFilter(Phaser.Textures.FilterMode.NEAREST);
  }
  for (const id of ['pickup:skillPoint', 'coin']) {
    const s = objectSprite(id);
    if (!s || typeof s.frame !== 'number' || !scene.textures.exists(s.key) || scene.anims.exists(id)) continue;
    const frames = scene.anims.generateFrameNumbers(s.key, { start: s.frame, end: s.frame + s.frames - 1 });
    scene.anims.create({ key: id, frames, frameRate: 8, repeat: -1 });
  }
}

/** Sprite eines Objekts mit Fußpunkt (x, y); `sprite` überschreibt die Zuordnung der ID (Biom-Look, Baumform: `natureSprites.ts`). `null` = keine Grafik, dann zeichnet der Aufrufer die Platzhalter-Form */
export function objectImage(scene: Phaser.Scene, id: string, x: number, y: number, sprite?: ObjectSprite | null): Phaser.GameObjects.Sprite | null {
  const s = sprite === undefined ? objectSprite(id) : sprite;
  if (!s || !scene.textures.exists(s.key)) return null;
  const image = scene.add.sprite(x, y, s.key, s.frame).setOrigin(0.5, 1).setScale(s.zoom);
  if (s.frames > 1 && scene.anims.exists(id)) image.play(id);
  return image;
}

/**
 * Lädt die Hintergrund-Bilder eines Plans, die noch fehlen (nur die Varianten dieser Stufe, GEB18), und ruft danach `done`;
 * sind alle schon da, sofort. Eine Datei, die nicht lädt, bleibt weg (Rückfall Variante 1 in `backgroundLayers`).
 */
export function loadBackground(scene: Phaser.Scene, plan: readonly BackgroundLayerPlan[], done: () => void): void {
  const missing = planKeys(plan).filter((key) => !scene.textures.exists(key));
  if (missing.length === 0) return done();
  for (const key of missing) {
    const f = backgroundFile(key);
    if (f) scene.load.image(key, `grafik/${f.file}`);
  }
  scene.load.once(Phaser.Loader.Events.COMPLETE, () => {
    for (const key of missing) if (scene.textures.exists(key)) scene.textures.get(key).setFilter(Phaser.Textures.FilterMode.NEAREST);
    done();
  });
  scene.load.start();
}

/** Eine Parallax-Schicht aus Einzelbildern: je Kachel ein Bild, Unterkante auf `y` (Boden oder Spiegelung); der Container heißt `bg:<biom>:<n>` */
export interface BackgroundView {
  scroll: number;
  /** Textur und x je Kachel, wie gezeichnet (Rückfall schon aufgelöst) */
  parts: { key: string; x: number }[];
  container: Phaser.GameObjects.Container;
}

/**
 * Parallax-Ebenen eines Plans als Container (Scroll-Faktor je Schicht, Maßstab `BACKGROUND_ZOOM`), Unterkante auf dem Boden.
 * Fehlt die Textur einer Variante, nimmt die Kachel Variante 1; fehlt auch die → `null`, dann bleiben die Silhouetten.
 */
export function backgroundLayers(scene: Phaser.Scene, plan: readonly BackgroundLayerPlan[]): BackgroundView[] | null {
  const resolved = plan.map((l) => ({ scroll: l.scroll, parts: l.parts.map((p) => ({ x: p.x, key: scene.textures.exists(p.key) ? p.key : p.key.replace(/:v\d+$/, ':v1') })) }));
  if (!resolved.every((l) => l.parts.every((p) => scene.textures.exists(p.key)))) return null;
  return resolved.map(({ scroll, parts }, i) => {
    const images = parts.map((p) => scene.add.image(p.x, 0, p.key).setOrigin(0, 1).setScale(BACKGROUND_ZOOM));
    const name = parts[0]?.key.replace(/:[1-3]:v\d+$/, `:${i + 1}`) ?? '';
    return { scroll, parts, container: scene.add.container(0, GROUND_Y, images).setScrollFactor(scroll, 1).setName(name) };
  });
}
