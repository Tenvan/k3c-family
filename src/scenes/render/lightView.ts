import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, UNIT_PX } from '../../core/constants';
import { clientLog } from '../../core/clientLog';
import { LIGHT } from '../../model/data';
import type { World } from '../../model/types';
import { brightnessOf, glowAlpha, lightSources } from './nightLight';

/** Pixel der Welt je Pixel der Lichtkarte; die Karte wird weich hochskaliert, das gibt den weichen Rand ohne Shader je Pixel */
const MAP_SCALE = 8;
/** Mitte der Lichter über dem Boden (Höhe der Flammen) */
const LIFT_PX = 40;
/** Über Figuren und Gebäuden, damit die Karte sie abdunkelt bzw. im Licht wieder zeigt */
const MAP_DEPTH = 12;
/** Rand der Karte links und rechts der Welt, damit die Kamera am Weltende nicht über sie hinausschaut */
const PAD_PX = GAME_WIDTH;
/** Helligkeit wird auf Stufen gerundet, die Karte zeichnet nur bei Änderung neu */
const STEPS = 64;
/** Fackeln (B-622) über der Lichtkarte: Halter aus Holz, die Flamme in `glowColor` leuchtet nur mit ihrem Schein */
const TORCH_DEPTH = MAP_DEPTH + 1;
const TORCH_STICK = { w: 4, h: 16, color: 0x5a3a22 };
const TORCH_FLAME_R = 5;

let counter = 0;

/**
 * Licht und Dunkelheit der Nacht (B-472, W13.1): eine Lichtkarte in Weltkoordinaten mit Mischmodus MULTIPLY über der Welt. Sie ist
 * außerhalb des Lichts so dunkel wie `nightBrightness` (Farben und Kontrast bleiben, nur dunkler) und wird um jede Quelle hell bis
 * Weiß, dort erscheinen die normalen Farben. Der Hintergrund wird zusätzlich in `StageView` getönt. Jede Kamera sieht die Karte
 * als Weltobjekt, Split-Screen und Welt-Betrachter des Studios zeichnen sie gleich.
 */
export class LightView {
  private readonly key = `light-map-${counter++}`;
  private image: Phaser.GameObjects.Image | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private tex: Phaser.Textures.CanvasTexture | null = null;
  private last = '';
  private lastTorches = '';
  private torches: Phaser.GameObjects.Graphics | null = null;
  private warned = false;

  constructor(
    private readonly scene: Phaser.Scene,
    private readonly root: Phaser.GameObjects.Layer,
  ) {}

  sync(world: World): void {
    const { lit, missing } = lightSources(world);
    if (missing.length > 0 && !this.warned) {
      this.warned = true;
      clientLog('warn', '🤒 Licht-Werte fehlen in data/light.json, Quelle bleibt dunkel', { missing });
    }
    const base = Math.round(brightnessOf(world.cycle) * STEPS);
    const glows = lit.map((s) => ({ x: s.x, radius: s.radius, a: Math.round(glowAlpha(s.strength, world.cycle) * STEPS) }));
    this.drawTorches(glows.filter((_, i) => lit[i]!.kind === 'torch'));
    const stamp = `${world.widthUnits}|${base}|${glows.map((g) => `${g.x}:${g.radius}:${g.a}`).join(',')}`;
    if (stamp === this.last) return;
    this.last = stamp;
    if (base >= STEPS) {
      this.image?.setVisible(false); // Tag: nichts zu multiplizieren
      return;
    }
    this.ensure(world.widthUnits);
    this.paint(base / STEPS, glows.map((g) => ({ ...g, a: g.a / STEPS })));
    this.image?.setVisible(true);
  }

  private ensure(widthUnits: number): void {
    const w = Math.ceil((widthUnits * UNIT_PX + 2 * PAD_PX) / MAP_SCALE);
    if (this.tex && this.tex.width === w) return;
    this.image?.destroy();
    if (this.scene.textures.exists(this.key)) this.scene.textures.remove(this.key);
    this.tex = this.scene.textures.createCanvas(this.key, w, Math.ceil(GAME_HEIGHT / MAP_SCALE))!;
    this.tex.setFilter(Phaser.Textures.FilterMode.LINEAR);
    this.ctx = this.tex.getContext();
    this.image = this.scene.add
      .image(-PAD_PX, 0, this.key)
      .setOrigin(0, 0)
      .setScale(MAP_SCALE)
      .setBlendMode(Phaser.BlendModes.MULTIPLY)
      .setDepth(MAP_DEPTH);
    this.root.add(this.image);
  }

  private paint(base: number, glows: { x: number; radius: number; a: number }[]): void {
    const c = this.ctx;
    const tex = this.tex;
    if (!c || !tex) return;
    const grey = Math.round(base * 255);
    c.globalCompositeOperation = 'source-over';
    c.fillStyle = `rgb(${grey},${grey},${grey})`;
    c.fillRect(0, 0, tex.width, tex.height);
    c.globalCompositeOperation = 'lighter';
    const glow = Phaser.Display.Color.HexStringToColor(LIGHT.glowColor);
    const { red: r, green: g, blue: b } = glow;
    for (const s of glows) {
      if (s.a <= 0) continue;
      const cx = (s.x * UNIT_PX + PAD_PX) / MAP_SCALE;
      const cy = (GROUND_Y - LIFT_PX) / MAP_SCALE;
      const rad = (s.radius * UNIT_PX) / MAP_SCALE;
      // Anhebung (1 − base) × a in der Mitte, linear auf 0 am Rand; Farbe warm nach glowColor, die Mitte erreicht Weiß-nah
      const lift = (1 - base) * s.a;
      const grad = c.createRadialGradient(cx, cy, 0, cx, cy, rad);
      grad.addColorStop(0, `rgba(${r},${g},${b},${lift})`);
      grad.addColorStop(1, `rgba(${r},${g},${b},0)`);
      c.fillStyle = grad;
      c.fillRect(cx - rad, cy - rad, 2 * rad, 2 * rad);
    }
    tex.refresh();
  }

  /** Fackeln als Objekte an Mauer und Turm: der Halter immer, die Flamme mit der Deckkraft ihres Scheins (am Tag aus) */
  private drawTorches(torches: { x: number; a: number }[]): void {
    const stamp = torches.map((t) => `${t.x}:${t.a}`).join(',');
    if (stamp === this.lastTorches) return;
    this.lastTorches = stamp;
    if (!this.torches) {
      this.torches = this.scene.add.graphics().setDepth(TORCH_DEPTH);
      this.root.add(this.torches);
    }
    const g = this.torches.clear();
    const flame = Phaser.Display.Color.HexStringToColor(LIGHT.glowColor).color;
    const top = GROUND_Y - LIFT_PX;
    for (const t of torches) {
      const px = t.x * UNIT_PX;
      g.fillStyle(TORCH_STICK.color, 1).fillRect(px - TORCH_STICK.w / 2, top, TORCH_STICK.w, TORCH_STICK.h);
      if (t.a > 0) g.fillStyle(flame, t.a / STEPS).fillCircle(px, top - TORCH_FLAME_R / 2, TORCH_FLAME_R);
    }
  }

  destroy(): void {
    this.torches?.destroy();
    this.image?.destroy();
    if (this.scene.textures.exists(this.key)) this.scene.textures.remove(this.key);
  }
}
