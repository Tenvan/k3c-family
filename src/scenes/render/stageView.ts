import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, UNIT_PX } from '../../core/constants';
import { t } from '../../core/texts';
import type { World } from '../../model/types';
import type { Effect } from '../effects/effects';
import { StageEffects } from '../effects/effectsView';
import { fontStyle, OUTLINE_COLOR } from './fontRules';
import { backgroundLayers } from './objectView';
import { WorldRenderer } from './worldRenderer';

/** Hintergrundfarbe einer Zelle, deren Stufe noch nicht geladen ist */
export const PLACEHOLDER_BG = '#202030';
/** Schrift des Platzhalters: Rolle `placeholder` des Katalogs, Kontur wie alle Texte */
const placeholderStyle = () => ({ ...fontStyle('placeholder'), stroke: OUTLINE_COLOR, strokeThickness: 4, fontStyle: 'bold' });

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
  /** Endlos laufende Tweens des Flusses; enden mit der Stufe */
  private river: Phaser.Tweens.Tween[] = [];

  constructor(scene: Phaser.Scene, readonly world: World) {
    this.layer = scene.add.layer();
    const widthPx = world.widthUnits * UNIT_PX;
    const palette = world.biome.palette;
    // Parallax-Ebenen des Bioms (GR3.2); ohne Grafik zwei gezackte Silhouetten (Berge / Höhlenwände).
    const fallback = (): Phaser.GameObjects.GameObject[] => RIDGES.map((r) => ridge(scene, widthPx, hex(palette[r.color]), r.scroll, r.baseY, r.amplitude));
    const tiles = backgroundLayers(scene, world.biome.id, widthPx);
    for (const o of tiles ?? fallback()) this.layer.add(o);
    this.layer.add(scene.add.rectangle(0, GROUND_Y, widthPx, RIVER_Y - GROUND_Y, hex(palette.ground)).setOrigin(0, 0));
    this.river = addRiver(scene, this.layer, widthPx, tiles, palette);
    this.renderer = new WorldRenderer(scene, world, this.layer);
    this.effects = new StageEffects(scene, this.layer);
  }

  /** Effekt zeichnen; ein Schlag lässt zusätzlich den Monarchen ausholen (S9.7, B-396) */
  spawn(effect: Effect): void {
    this.effects.spawn(effect);
    if (effect.by !== undefined) this.renderer.swing(effect.by);
  }

  /**
   * Gleicht den Kamera-Zoom für alle Texte der Stufe aus (Schrift erscheint in ihrer Größe, unabhängig von Zoom und Layout).
   * `hideFrom` ist die Kamera-ID der Mitspieler-Zelle, in der die Texte ausgeblendet sind (B-197); 0 = überall sichtbar.
   */
  scaleTexts(factor: number, hideFrom = 0): void {
    const walk = (list: Phaser.GameObjects.GameObject[]) => {
      for (const o of list) {
        if (o instanceof Phaser.GameObjects.Text) {
          if (o.scale !== factor) o.setScale(factor);
          o.cameraFilter = hideFrom;
        } else if (o instanceof Phaser.GameObjects.Container) walk(o.list);
      }
    };
    walk(this.layer.list);
  }

  destroy(): void {
    this.river.forEach((t) => t.stop());
    this.effects.destroy();
    this.layer.destroy();
  }
}

/** Silhouetten ohne Grafik: ferne Berge und nahe Hügel */
const RIDGES = [
  { color: 'far', scroll: 0.3, baseY: 520, amplitude: 180 },
  { color: 'near', scroll: 0.6, baseY: 700, amplitude: 120 },
] as const;

/** Oberkante des Flusses: darüber ein schmaler Uferstreifen, auf dem die Figuren stehen (POC, wie in K2C) */
const RIVER_Y = GROUND_Y + 36;
/** Wasserfarbe, Tönung und Deckkraft der Spiegelung, Weg und Dauer der Drift */
const WATER = { base: 0x0d2a40, tint: 0x4a78a8, mirror: 0.45, driftPx: 14, driftMs: 5200, flowMs: 9000 } as const;
const RIPPLE_W = 256;
const RIPPLE_KEY = 'river-ripple';

/** Wellenmuster als kleine, nahtlos kachelnde Textur (einmal erzeugt): helle Striche in drei Tiefen */
function rippleTexture(scene: Phaser.Scene): string {
  if (scene.textures.exists(RIPPLE_KEY)) return RIPPLE_KEY;
  const h = GAME_HEIGHT - RIVER_Y;
  const tex = scene.textures.createCanvas(RIPPLE_KEY, RIPPLE_W, h)!;
  const c = tex.getContext();
  c.strokeStyle = '#bcd8ff';
  c.lineCap = 'round';
  for (let i = 0; i < 9; i++) {
    const y = 10 + ((i * 53) % (h - 20));
    const x = (i * 97) % RIPPLE_W;
    const w = 28 + (i % 3) * 18;
    c.globalAlpha = 0.25 + 0.15 * (i % 3);
    c.lineWidth = 2 + (y / h) * 2; // nach unten (näher) dicker
    for (const dx of [0, -RIPPLE_W]) {
      c.beginPath();
      c.moveTo(x + dx, y);
      c.lineTo(x + dx + w, y);
      c.stroke();
    }
  }
  tex.refresh();
  return RIPPLE_KEY;
}

/**
 * Fluss unter dem Boden: dunkles Wasser mit dem Hintergrund kopfüber darin (gleiche Parallax-Faktoren, getönt), das
 * langsam hin und her treibt. Nur der Hintergrund spiegelt sich; Gebäude und Figuren stehen noch nicht im Wasser.
 * `tiles` = die Parallax-Ebenen der Stufe, `null` = Silhouetten ohne Grafik. Gibt die endlosen Tweens zurück.
 */
function addRiver(scene: Phaser.Scene, layer: Phaser.GameObjects.Layer, widthPx: number, tiles: Phaser.GameObjects.TileSprite[] | null, palette: World['biome']['palette']): Phaser.Tweens.Tween[] {
  layer.add(scene.add.rectangle(0, RIVER_Y, widthPx, GAME_HEIGHT - RIVER_Y, WATER.base).setOrigin(0, 0));
  const tweens: Phaser.Tweens.Tween[] = [];
  if (tiles) {
    for (const src of tiles) {
      const rows = (GAME_HEIGHT - RIVER_Y) / src.scaleY; // nur der Streifen direkt über dem Boden wird sichtbar
      const m = scene.add
        .tileSprite(0, RIVER_Y, src.width, Math.min(src.frame.height, rows), src.texture.key)
        .setOrigin(0, 0)
        .setScale(src.scaleX, src.scaleY)
        .setScrollFactor(src.scrollFactorX, 1)
        .setFlipY(true)
        .setTint(WATER.tint)
        .setAlpha(WATER.mirror);
      m.tilePositionY = src.frame.height - m.height; // der untere Rand der Textur liegt an der Bodenlinie
      layer.add(m);
      tweens.push(scene.tweens.add({ targets: m, tilePositionX: `+=${WATER.driftPx}`, duration: WATER.driftMs, yoyo: true, repeat: -1, ease: 'Sine.easeInOut' }));
    }
    return [...tweens, flow(scene, layer, widthPx)];
  }
  for (const r of RIDGES) {
    const g = ridge(scene, widthPx, hex(palette[r.color]), r.scroll, r.baseY, r.amplitude);
    g.setY(RIVER_Y + GROUND_Y).setScale(1, -1).setAlpha(WATER.mirror);
    layer.add(g);
  }
  return [flow(scene, layer, widthPx)];
}

/** Wellenstriche über Spiegelung und Wasser, strömen endlos nach links */
function flow(scene: Phaser.Scene, layer: Phaser.GameObjects.Layer, widthPx: number): Phaser.Tweens.Tween {
  const ripples = scene.add.tileSprite(0, RIVER_Y, widthPx, GAME_HEIGHT - RIVER_Y, rippleTexture(scene)).setOrigin(0, 0).setAlpha(0.6);
  layer.add(ripples);
  return scene.tweens.add({ targets: ripples, tilePositionX: RIPPLE_W, duration: WATER.flowMs, repeat: -1 });
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

/** Text „Stufe n wird geladen“ in der Mitte der Spielfläche; die Kamera der Zelle zentriert dort (`centerOn`), `zoom` ist ihr Zoom (Schrift erscheint in ihrer Größe). */
export function placeholderLayer(scene: Phaser.Scene, depth: number | null, zoom: number): Phaser.GameObjects.Layer {
  const text = depth === null ? t('stage.loading') : t('stage.loadingN', { depth });
  const layer = scene.add.layer();
  layer.add(scene.add.text(GAME_WIDTH / 2, GAME_HEIGHT / 2, text, placeholderStyle()).setOrigin(0.5).setScale(1 / zoom));
  return layer;
}

/** Jede Kamera sieht nur ihre eigene Ebene; `own[i]` gehört zu `cams[i]`. Setzt die Filter aller Ebenen neu. */
export function showOnly(cams: readonly Phaser.Cameras.Scene2D.BaseCamera[], own: readonly Phaser.GameObjects.Layer[], all: readonly Phaser.GameObjects.Layer[]): void {
  for (const layer of all) layer.cameraFilter = 0;
  cams.forEach((cam, i) => {
    for (const layer of all) if (layer !== own[i]) layer.cameraFilter |= cam.id;
  });
}
