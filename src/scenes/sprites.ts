import Phaser from 'phaser';
import spritesJson from '../../data/sprites.json';

/**
 * Figuren-Sprites (LuizMelo und Gothicvania, beide CC0). Welche Figur wofür steht, Frame-Größen und Einfärbung stehen in
 * data/sprites.json, die Bilder in public/sprites/<sheet>/<anim>.png.
 */

export type AnimName = 'idle' | 'run' | 'attack';

export interface SheetData {
  frameWidth: number;
  frameHeight: number;
  /** Fußpunkt der Figur im Frame (0..1), wird zum Ursprung des Sprites */
  originX: number;
  originY: number;
  /** Figurhöhe in Frame-Pixeln (für Balken über dem Kopf) */
  height: number;
  scale: number;
  anims: Partial<Record<AnimName, { frames: number; fps: number }>>;
}

export interface SpriteSpec {
  sheet: string;
  tint?: string;
  scale?: number;
  /** Höhe über dem Boden in Pixeln (fliegende Gegner) */
  y?: number;
  alpha?: number;
}

const DATA = spritesJson as unknown as {
  sheets: Record<string, SheetData>;
  players: SpriteSpec[];
  troops: Record<string, SpriteSpec>;
  enemies: Record<string, SpriteSpec>;
};

export const SHEETS = DATA.sheets;
export const PLAYER_SPRITES = DATA.players;
export const TROOP_SPRITES = DATA.troops;
export const ENEMY_SPRITES = DATA.enemies;

const ANIMS: AnimName[] = ['idle', 'run', 'attack'];
/** Nur Figuren mit einer Rolle laden, die übrigen sind nur für die Referenzseiten da */
const USED = new Set([...PLAYER_SPRITES, ...Object.values(TROOP_SPRITES), ...Object.values(ENEMY_SPRITES)].map((s) => s.sheet));
const textureKey = (sheet: string, anim: AnimName) => `${sheet}-${anim}`;

/** Im preload() der Szene aufrufen. */
export function preloadSprites(scene: Phaser.Scene): void {
  for (const [sheet, d] of Object.entries(SHEETS)) {
    if (!USED.has(sheet)) continue;
    for (const anim of ANIMS) {
      if (!d.anims[anim] || scene.textures.exists(textureKey(sheet, anim))) continue;
      scene.load.spritesheet(textureKey(sheet, anim), `sprites/${sheet}/${anim}.png`, { frameWidth: d.frameWidth, frameHeight: d.frameHeight });
    }
  }
}

/** Im create() nach dem Laden aufrufen. Legt die Animationen einmal global an. */
export function createSpriteAnims(scene: Phaser.Scene): void {
  for (const [sheet, d] of Object.entries(SHEETS)) {
    for (const anim of ANIMS) {
      const a = d.anims[anim];
      const key = textureKey(sheet, anim);
      if (!a || !scene.textures.exists(key) || scene.anims.exists(key)) continue;
      // Pixel-Art beim Vergrößern scharf halten
      scene.textures.get(key).setFilter(Phaser.Textures.FilterMode.NEAREST);
      scene.anims.create({
        key,
        frames: scene.anims.generateFrameNumbers(key, { start: 0, end: a.frames - 1 }),
        frameRate: a.fps,
        repeat: anim === 'attack' ? 0 : -1,
      });
    }
  }
}

/** Figur mit Fußpunkt bei (0, 0) bzw. spec.y darüber. */
export function makeSprite(scene: Phaser.Scene, spec: SpriteSpec): Phaser.GameObjects.Sprite {
  const d = SHEETS[spec.sheet];
  const sprite = scene.add.sprite(0, -(spec.y ?? 0), textureKey(spec.sheet, 'idle'));
  sprite.setOrigin(d.originX, d.originY).setScale(d.scale * (spec.scale ?? 1));
  if (spec.tint) sprite.setTint(Phaser.Display.Color.HexStringToColor(spec.tint).color);
  if (spec.alpha !== undefined) sprite.setAlpha(spec.alpha);
  sprite.setData('sheet', spec.sheet);
  sprite.setData('top', -(spec.y ?? 0) - d.height * d.scale * (spec.scale ?? 1));
  sprite.play(textureKey(spec.sheet, 'idle'));
  return sprite;
}

/**
 * Spielt eine Animation, ohne eine laufende neu zu starten. Ein Angriff läuft zu Ende, bevor wieder
 * Laufen/Stehen gezeigt wird. Fehlt eine Animation (z. B. der Geist hat kein Laufen), bleibt es bei idle.
 */
export function playAnim(sprite: Phaser.GameObjects.Sprite, anim: AnimName, restart = false): void {
  const sheet = sprite.getData('sheet') as string;
  const d = SHEETS[sheet];
  const name = d.anims[anim] ? anim : 'idle';
  const key = textureKey(sheet, name);
  const current = sprite.anims.currentAnim?.key;
  if (current === textureKey(sheet, 'attack') && sprite.anims.isPlaying && name !== 'attack') return;
  if (current === key && sprite.anims.isPlaying && !restart) return;
  sprite.play(key);
}

/** Oberkante der Figur relativ zum Fußpunkt (negativ), für Balken und Anzeigen über dem Kopf. */
export function spriteTop(sprite: Phaser.GameObjects.Sprite): number {
  return sprite.getData('top') as number;
}

/**
 * Figur zeigt nach links oder rechts (die Sprites schauen von sich aus nach rechts).
 * Der Ursprung wird mitgespiegelt, sonst springt die Figur beim Umdrehen zur Seite.
 */
export function face(sprite: Phaser.GameObjects.Sprite, dir: number): void {
  if (dir === 0) return;
  const d = SHEETS[sprite.getData('sheet') as string];
  const left = dir < 0;
  sprite.setFlipX(left).setOrigin(left ? 1 - d.originX : d.originX, d.originY);
}
