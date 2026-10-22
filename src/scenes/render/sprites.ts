import Phaser from 'phaser';
import spritesJson from '../../../data/sprites.json';
import { MONARCH } from '../../model/data';

/**
 * Figuren-Sprites (LuizMelo und Gothicvania, beide CC0). Welche Figur wofür steht, Frame-Größen und Einfärbung stehen in
 * data/sprites.json, die Bilder in public/sprites/<sheet>/<anim>.png.
 */

export type AnimName = 'idle' | 'run' | 'attack' | 'give' | 'hit' | 'death' | 'respawn';

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

/** Reittier: Sheet, Zoom und Sattelpunkt je Frame (Frame-Pixel, Blick nach rechts). */
export interface MountData {
  label: string;
  sheet: string;
  scale: number;
  tint?: string;
  saddle: Partial<Record<'idle' | 'run', [number, number][]>>;
}

const DATA = spritesJson as unknown as {
  sheets: Record<string, SheetData>;
  mounts: Record<string, MountData>;
  riderWaist: number;
  players: SpriteSpec[];
  troops: Record<string, SpriteSpec>;
  enemies: Record<string, SpriteSpec>;
};

export const SHEETS = DATA.sheets;
export const MOUNTS = DATA.mounts;
/** Anteil der Figurhöhe, der über dem Sattel sichtbar ist */
export const RIDER_WAIST = DATA.riderWaist;
export const PLAYER_SPRITES = DATA.players;
export const TROOP_SPRITES = DATA.troops;
export const ENEMY_SPRITES = DATA.enemies;

/** Wie `anims` in tools/atlas/atlas.go. Nur idle und run laufen in Schleife, die übrigen einmal; death hält den letzten Frame. */
const ANIMS: AnimName[] = ['idle', 'run', 'attack', 'give', 'hit', 'death', 'respawn'];
const LOOPS = new Set<AnimName>(['idle', 'run']);
/** Nur Figuren mit einer Rolle laden, die übrigen sind nur für die Referenzseiten da */
const USED = new Set([...PLAYER_SPRITES, ...Object.values(TROOP_SPRITES), ...Object.values(ENEMY_SPRITES)].map((s) => s.sheet));
const textureKey = (sheet: string, anim: AnimName) => `${sheet}-${anim}`;
/** Sheet des Standard-Reittiers (Monarch): liegt nicht im Atlas, sondern als einzelne PNGs `sprites/<sheet>/<anim>.png`; die übrigen Reittiere kommen auf Anforderung (`preloadMount`) */
const MOUNT_SHEET = MOUNTS[MONARCH.mount.sprite]?.sheet;

/** Schlüssel des Figuren-Atlas (Phaser-Multiatlas aus `task gen:atlas`, Frames `<sheet>-<anim>/<i>`). */
export const ATLAS_KEY = 'atlas';

/** Im preload() der Lade-Szene aufrufen: ein Multiatlas statt je Sheet und Animation eine PNG; dazu das Standard-Reittier. */
export function preloadSprites(scene: Phaser.Scene): void {
  if (!scene.textures.exists(ATLAS_KEY)) scene.load.multiatlas(ATLAS_KEY, 'atlas/atlas.json', 'atlas/');
  if (MOUNT_SHEET) preloadSheet(scene, MOUNT_SHEET);
}

/** Reittier `key` (Schlüssel aus `data/sprites.json › mounts`) auf Anforderung laden, nicht beim Start (B-476); wie bei
 *  `preloadSheet` wird erst mit `scene.load.start()` geladen. Gibt die Zahl der eingereihten Dateien zurück. */
export function preloadMount(scene: Phaser.Scene, key: string): number {
  const sheet = MOUNTS[key]?.sheet;
  return sheet ? preloadSheet(scene, sheet) : 0;
}

/** Einzel-PNGs eines Sheets außerhalb des Atlas einreihen (`sprites/<sheet>/<anim>.png`, Textur `<sheet>-<anim>`);
 *  schon geladene und fehlende Animationen werden übersprungen. Gibt die Zahl der eingereihten Dateien zurück. */
export function preloadSheet(scene: Phaser.Scene, sheet: string): number {
  const d = SHEETS[sheet];
  if (!d) return 0;
  let queued = 0;
  for (const anim of ANIMS) {
    const key = textureKey(sheet, anim);
    if (d.anims[anim] && !scene.textures.exists(key)) {
      scene.load.spritesheet(key, `sprites/${sheet}/${anim}.png`, { frameWidth: d.frameWidth, frameHeight: d.frameHeight });
      queued++;
    }
  }
  return queued;
}

/** Nach dem Laden aufrufen. Legt die Animationen einmal global an (Schlüssel `<sheet>-<anim>` wie zuvor). */
export function createSpriteAnims(scene: Phaser.Scene): void {
  // Pixel-Art beim Vergrößern scharf halten
  scene.textures.get(ATLAS_KEY).setFilter(Phaser.Textures.FilterMode.NEAREST);
  for (const [sheet, d] of Object.entries(SHEETS)) {
    if (!USED.has(sheet)) continue;
    for (const anim of ANIMS) {
      const a = d.anims[anim];
      const key = textureKey(sheet, anim);
      if (!a || scene.anims.exists(key)) continue;
      scene.anims.create({
        key,
        frames: scene.anims.generateFrameNames(ATLAS_KEY, { prefix: `${key}/`, start: 0, end: a.frames - 1 }),
        frameRate: a.fps,
        repeat: LOOPS.has(anim) ? -1 : 0,
      });
    }
  }
  if (MOUNT_SHEET) createSheetAnims(scene, MOUNT_SHEET);
}

/** Animationen eines per `preloadSheet` oder `preloadMount` geladenen Sheets anlegen (einmal global, wie im Atlas). */
export function createSheetAnims(scene: Phaser.Scene, sheet: string): void {
  const d = SHEETS[sheet];
  if (!d) return;
  for (const anim of ANIMS) {
    const a = d.anims[anim];
    const key = textureKey(sheet, anim);
    if (!a || !scene.textures.exists(key) || scene.anims.exists(key)) continue;
    scene.textures.get(key).setFilter(Phaser.Textures.FilterMode.NEAREST);
    scene.anims.create({
      key,
      frames: scene.anims.generateFrameNumbers(key, { start: 0, end: a.frames - 1 }),
      frameRate: a.fps,
      repeat: LOOPS.has(anim) ? -1 : 0,
    });
  }
}

/** Figur mit Fußpunkt bei (0, 0) bzw. spec.y darüber. */
export function makeSprite(scene: Phaser.Scene, spec: SpriteSpec): Phaser.GameObjects.Sprite {
  const d = SHEETS[spec.sheet];
  const sprite = scene.add.sprite(0, -(spec.y ?? 0), ATLAS_KEY, `${textureKey(spec.sheet, 'idle')}/0`);
  sprite.setOrigin(d.originX, d.originY).setScale(d.scale * (spec.scale ?? 1));
  if (spec.tint) sprite.setTint(Phaser.Display.Color.HexStringToColor(spec.tint).color);
  if (spec.alpha !== undefined) sprite.setAlpha(spec.alpha);
  sprite.setData('sheet', spec.sheet);
  sprite.setData('top', -(spec.y ?? 0) - d.height * d.scale * (spec.scale ?? 1));
  sprite.play(textureKey(spec.sheet, 'idle'));
  return sprite;
}

/** Hat das Sheet des Sprites die Animation `anim`? */
export function hasAnim(sprite: Phaser.GameObjects.Sprite, anim: AnimName): boolean {
  return SHEETS[sprite.getData('sheet') as string]?.anims[anim] !== undefined;
}

/**
 * Spielt eine Animation, ohne eine laufende neu zu starten. Eine Einmal-Animation (Angriff, Wurf, Treffer, Respawn)
 * läuft zu Ende, bevor wieder Laufen/Stehen gezeigt wird; der Tod hält seinen letzten Frame. Fehlt eine Animation
 * (z. B. der Geist hat kein Laufen), bleibt es bei idle.
 */
export function playAnim(sprite: Phaser.GameObjects.Sprite, anim: AnimName, restart = false): void {
  const sheet = sprite.getData('sheet') as string;
  const d = SHEETS[sheet];
  const name = d.anims[anim] ? anim : 'idle';
  const key = textureKey(sheet, name);
  const current = sprite.anims.currentAnim?.key;
  const once = current !== undefined && !LOOPS.has(current.slice(sheet.length + 1) as AnimName);
  if (once && sprite.anims.isPlaying && LOOPS.has(name)) return;
  if (current === key && (sprite.anims.isPlaying || name === 'death') && !restart) return;
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
