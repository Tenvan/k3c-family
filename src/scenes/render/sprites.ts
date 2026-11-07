import Phaser from 'phaser';
import spritesJson from '../../../data/sprites.json';
import {
  PAPER_ENEMIES, PAPER_PLAYERS, PAPER_TROOPS, type PaperManifest, type PaperRef, animFrame, assetKey, manifestAnim, paperAnimName,
  paperOrigin, paperScale, paperTextureKey, usedFigureAssets,
} from './paperAssets';

/**
 * Figuren-Sprites als Papierschnitt (US1): Welche Rolle welches Asset zeigt, steht in `data/paper.json`, Manifest und Sheets
 * unter `public/grafik/<gruppe>/<id>/`. Größe, Höhe über dem Boden und Deckkraft kommen weiter aus `data/sprites.json`
 * (bisherige Figurhöhe = Zielhöhe), bis CU1 die alten Sheets entfernt.
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
  /** Papierschnitt-Asset der Rolle (`data/paper.json`) */
  paper?: PaperRef;
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
const withPaper = (spec: SpriteSpec, paper: PaperRef | undefined): SpriteSpec => (paper ? { ...spec, paper } : spec);
export const PLAYER_SPRITES: SpriteSpec[] = DATA.players.map((s, i) => withPaper(s, PAPER_PLAYERS[i % PAPER_PLAYERS.length]));
export const TROOP_SPRITES: Record<string, SpriteSpec> = Object.fromEntries(Object.entries(DATA.troops).map(([k, s]) => [k, withPaper(s, PAPER_TROOPS[k])]));
export const ENEMY_SPRITES: Record<string, SpriteSpec> = Object.fromEntries(Object.entries(DATA.enemies).map(([k, s]) => [k, withPaper(s, PAPER_ENEMIES[k])]));

/** Spiel-Animationen. Nur idle und run laufen in Schleife, die übrigen einmal; death hält den letzten Frame. */
const ANIMS: AnimName[] = ['idle', 'run', 'attack', 'give', 'hit', 'death', 'respawn'];
const LOOPS = new Set<AnimName>(['idle', 'run']);
/** Nur Figuren mit einer Rolle laden, die übrigen sind nur für die Referenzseiten da */
const textureKey = (sheet: string, anim: AnimName) => `${sheet}-${anim}`;

/** Geladene Manifeste je Asset (`<gruppe>/<id>`) */
const MANIFESTS = new Map<string, PaperManifest>();
const manifestKey = (key: string) => `paper-manifest:${key}`;

/** Manifest eines geladenen Assets (sonst `undefined`) */
export const paperManifest = (key: string): PaperManifest | undefined => MANIFESTS.get(key);

/**
 * Im preload() der Lade-Szene aufrufen: je Figuren-Asset das Manifest, nach dessen Eintreffen je Animation ein Sheet
 * (`paper:<gruppe>/<id>:<animation>`); der Loader nimmt die Sheets in denselben Lauf auf.
 */
export function preloadSprites(scene: Phaser.Scene): void {
  for (const key of usedFigureAssets()) {
    if (MANIFESTS.has(key)) continue;
    scene.load.once(`filecomplete-json-${manifestKey(key)}`, (_k: string, _t: string, m: PaperManifest) => {
      MANIFESTS.set(key, m);
      for (const [anim, a] of Object.entries(m.animationen)) {
        const f = animFrame(m, anim);
        const tex = paperTextureKey(key, anim);
        if (!scene.textures.exists(tex)) scene.load.spritesheet(tex, `grafik/${key}/${a.datei}`, { frameWidth: f.frameW, frameHeight: f.frameH });
      }
    });
    scene.load.json(manifestKey(key), `grafik/${key}/manifest.json`);
  }
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
  for (const [key, m] of MANIFESTS) {
    for (const [anim, a] of Object.entries(m.animationen)) {
      const tex = paperTextureKey(key, anim);
      if (!scene.textures.exists(tex) || scene.anims.exists(tex)) continue;
      scene.anims.create({
        key: tex,
        frames: scene.anims.generateFrameNumbers(tex, { start: 0, end: a.frames - 1 }),
        frameRate: a.fps,
        repeat: anim.startsWith('angriff') ? 0 : -1,
      });
    }
  }
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

/** Bisherige Figurhöhe in Spiel-Pixeln (Zielhöhe des Papier-Assets) */
export function targetHeight(spec: SpriteSpec): number {
  const d = SHEETS[spec.sheet];
  return d ? d.height * d.scale * (spec.scale ?? 1) : 100;
}

/** Papier-Animation zum Spiel-Namen samt Variante, `null` = gibt es nicht */
function paperAnim(sprite: Phaser.GameObjects.Sprite, anim: string): string | null {
  const m = MANIFESTS.get(sprite.getData('paper') as string);
  const name = paperAnimName(anim) ?? anim;
  const full = manifestAnim(name, sprite.getData('variante') as string | undefined);
  if (m?.animationen[full]) return full;
  return m?.animationen[name] ? name : null;
}

/** Textur, Ursprung und Maßstab der Papier-Animation setzen und abspielen */
export function playPaper(sprite: Phaser.GameObjects.Sprite, anim: string, restart = false): void {
  const key = sprite.getData('paper') as string;
  const m = MANIFESTS.get(key)!;
  const tex = paperTextureKey(key, anim);
  if (sprite.anims.currentAnim?.key === tex && !restart) return;
  const f = animFrame(m, anim);
  const o = paperOrigin(f.rahmen);
  const left = sprite.flipX;
  sprite.setOrigin(left ? 1 - o.x : o.x, o.y).setScale(paperScale(m, anim, sprite.getData('target') as number).scale);
  if (sprite.scene.anims.exists(tex)) sprite.play(tex);
  else sprite.setTexture(tex, 0);
}

/** Schatten auf Boden und Hintergrund (B-701 D-09): Phaser-Filter, ohne WebGL keiner */
function addShadow(sprite: Phaser.GameObjects.Sprite): void {
  if (sprite.scene.game.config.renderType === Phaser.CANVAS) return;
  sprite.enableFilters().filters?.internal.addShadow(0, 0, 0.1, 1, 0x000000, 6, 0.5);
}

/** Figur mit Fußpunkt bei (0, 0) bzw. spec.y darüber; Papier-Asset aus `spec.paper` (Variante je Spieler). */
export function makeSprite(scene: Phaser.Scene, spec: SpriteSpec): Phaser.GameObjects.Sprite {
  const paper = spec.paper;
  const key = paper ? assetKey(paper) : '';
  const sprite = scene.add.sprite(0, -(spec.y ?? 0), '__MISSING');
  const target = targetHeight(spec);
  sprite.setData('sheet', spec.sheet).setData('paper', key).setData('variante', paper?.variante).setData('target', target);
  sprite.setData('top', -(spec.y ?? 0) - target);
  if (spec.tint) sprite.setTint(Phaser.Display.Color.HexStringToColor(spec.tint).color);
  if (spec.alpha !== undefined) sprite.setAlpha(spec.alpha);
  const idle = MANIFESTS.has(key) ? paperAnim(sprite, 'idle') : null;
  if (idle) {
    playPaper(sprite, idle);
    addShadow(sprite);
  }
  return sprite;
}

/** Hat das Asset des Sprites die Animation `anim`? */
export function hasAnim(sprite: Phaser.GameObjects.Sprite, anim: AnimName): boolean {
  return paperAnim(sprite, anim) !== null && (anim === 'idle' || paperAnimName(anim) !== null);
}

/**
 * Spielt eine Animation, ohne eine laufende neu zu starten. Eine Einmal-Animation (Angriff) läuft zu Ende, bevor wieder
 * Laufen/Stehen gezeigt wird. Fehlt eine Animation (z. B. Tod, Wurf), bleibt es bei idle.
 */
export function playAnim(sprite: Phaser.GameObjects.Sprite, anim: AnimName, restart = false): void {
  const name = (paperAnimName(anim) && paperAnim(sprite, anim)) || paperAnim(sprite, 'idle');
  if (!name) return;
  const loop = LOOPS.has(anim) || name === paperAnim(sprite, 'idle');
  const current = sprite.anims.currentAnim?.key;
  const once = current !== undefined && current.includes(':angriff');
  if (once && sprite.anims.isPlaying && loop) return;
  playPaper(sprite, name, restart);
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
  const left = dir < 0;
  if (sprite.flipX === left) return;
  sprite.setFlipX(left).setOrigin(1 - sprite.originX, sprite.originY);
}
