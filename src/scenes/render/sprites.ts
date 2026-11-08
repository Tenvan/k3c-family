import Phaser from 'phaser';
import {
  PAPER_ENEMIES, PAPER_FIGURES, PAPER_PLAYERS, PAPER_TROOPS, type PaperFigure, type PaperManifest, type PaperRef, animFrame, assetKey, manifestAnim, paperAnimName,
  paperOrigin, paperScale, paperTextureKey, usedFigureAssets,
} from './paperAssets';

/**
 * Figuren-Sprites als Papierschnitt (US1): Welche Rolle welches Asset zeigt, steht in `data/paper.json`, Manifest und Sheets
 * unter `public/grafik/<gruppe>/<id>/`. Größe (Zielhöhe), Höhe über dem Boden, Tönung und Deckkraft je Rolle stehen in
 * `data/paper.json › figuren` (CU1.2; vorher `data/sprites.json`).
 */

export type AnimName = 'idle' | 'run' | 'attack' | 'give' | 'hit' | 'death' | 'respawn';

/** Rolle eines Sprites: Maße aus `data/paper.json › figuren` samt Papier-Asset */
export interface SpriteSpec extends PaperFigure {
  /** Papierschnitt-Asset der Rolle (`data/paper.json`) */
  paper?: PaperRef;
}

const withPaper = (spec: PaperFigure, paper: PaperRef | undefined): SpriteSpec => (paper ? { ...spec, paper } : spec);
/** Truppen und Gegner haben je ein eigenes Asset: die alte Einfärbung gleicher Pixel-Sheets entfällt (Review US1, W4) */
const ownAsset = (spec: PaperFigure, paper: PaperRef | undefined): SpriteSpec => {
  const s = withPaper(spec, paper);
  if (!paper) return s;
  const { tint: _tint, ...rest } = s;
  return rest;
};
export const PLAYER_SPRITES: SpriteSpec[] = PAPER_FIGURES.spieler.map((s, i) => withPaper(s, PAPER_PLAYERS[i % PAPER_PLAYERS.length]));
export const TROOP_SPRITES: Record<string, SpriteSpec> = Object.fromEntries(Object.entries(PAPER_FIGURES.truppen).map(([k, s]) => [k, ownAsset(s, PAPER_TROOPS[k])]));
export const ENEMY_SPRITES: Record<string, SpriteSpec> = Object.fromEntries(Object.entries(PAPER_FIGURES.gegner).map(([k, s]) => [k, ownAsset(s, PAPER_ENEMIES[k])]));

/** Spiel-Animationen. Nur idle und run laufen in Schleife, die übrigen einmal; death hält den letzten Frame. */
const LOOPS = new Set<AnimName>(['idle', 'run']);

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

/** Zielhöhe des Papier-Assets in Spiel-Pixeln */
export function targetHeight(spec: SpriteSpec): number {
  return spec.hoehe;
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
  sprite.setData('paper', key).setData('variante', paper?.variante).setData('target', target);
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
 * Laufen/Stehen gezeigt wird. Fehlt eine Einmal-Animation (z. B. Schlag des Monarchen, Tod, Wurf), läuft die aktuelle
 * weiter, statt idle neu zu starten (Review US1, W1).
 */
export function playAnim(sprite: Phaser.GameObjects.Sprite, anim: AnimName, restart = false): void {
  if (!LOOPS.has(anim) && !(paperAnimName(anim) && paperAnim(sprite, anim))) return;
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
