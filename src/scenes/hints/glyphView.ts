import Phaser from 'phaser';
import { GLYPH_COLORS, glyphPx, type Glyph } from './glyphs';

/** Bild aus einer geladenen Textur (Pixel-Art); ohne `scale` ganzzahlig auf Zeilenhöhe skaliert */
export interface RowImage {
  image: string;
  frame?: number;
  scale?: number;
}

/** Teil einer Zeile: Text, Tasten-Glyph oder Bild */
export type RowPart = string | Glyph | RowImage;

/** Abstand zwischen Bild und Text */
const IMAGE_GAP = 8;

const OUTLINE = 0x000000;
const FONT = { fontStyle: 'bold' };
const hex = (c: number): string => `#${c.toString(16).padStart(6, '0')}`;

function label(scene: Phaser.Scene, text: string, px: number, ink: number, x = 0): Phaser.GameObjects.Text {
  return scene.add.text(x, 0, text, { ...FONT, fontSize: `${Math.round(px)}px`, color: hex(ink) }).setOrigin(0.5);
}

/** Kreis mit Buchstabe (A/X/Y), Menu-Taste (drei Striche) oder Touch-Taste (mit Hand) */
function round(scene: Phaser.Scene, g: Phaser.GameObjects.Graphics, name: string, h: number): Phaser.GameObjects.GameObject[] {
  const c = name in GLYPH_COLORS ? GLYPH_COLORS[name as 'A' | 'X' | 'Y'] : GLYPH_COLORS.pad;
  g.fillStyle(c.fill).fillCircle(h / 2, 0, h / 2).lineStyle(2, OUTLINE).strokeCircle(h / 2, 0, h / 2);
  if (name !== 'MENU') return [label(scene, name, h * 0.6, c.ink, h / 2)];
  g.fillStyle(c.ink);
  for (const dy of [-0.18, 0, 0.18]) g.fillRect(h * 0.28, dy * h - 1.5, h * 0.44, 3);
  return [];
}

/** D-Pad als Kreuz, die gemeinte Richtung hell */
function dpad(g: Phaser.GameObjects.Graphics, dir: string, h: number): void {
  const s = h / 3;
  g.fillStyle(OUTLINE).fillRect(s - 2, -h / 2, s + 4, h).fillRect(-2, -s / 2 - 2, h + 4, s + 4);
  g.fillStyle(GLYPH_COLORS.pad.fill).fillRect(s, -h / 2 + 2, s, h - 4).fillRect(0, -s / 2, h, s);
  g.fillStyle(GLYPH_COLORS.pad.ink).fillRect(s + 3, dir === 'UP' ? -h / 2 + 5 : s / 2 + 1, s - 6, s - 6);
}

/** Abgerundetes Rechteck mit Beschriftung: Schultertasten (LB/RB/LT/RT) und Tastenkappen */
function cap(scene: Phaser.Scene, g: Phaser.GameObjects.Graphics, text: string, h: number, colors: { fill: number; ink: number }): [Phaser.GameObjects.Text, number] {
  const t = label(scene, text, h * 0.5, colors.ink);
  const w = Math.max(h, t.width + h * 0.5);
  t.setX(w / 2);
  g.fillStyle(colors.fill).fillRoundedRect(0, -h / 2, w, h, h * 0.25).lineStyle(2, OUTLINE).strokeRoundedRect(0, -h / 2, w, h, h * 0.25);
  return [t, w];
}

/** Zeichnet eine Glyph mit Höhe `h`, links an x = 0, mittig an y = 0; liefert Objekte und Breite */
export function drawGlyph(scene: Phaser.Scene, glyph: Glyph & { key: string }, h: number): [Phaser.GameObjects.GameObject[], number] {
  const g = scene.add.graphics();
  const [kind, name = ''] = glyph.key.split(':');
  if (kind === 'key' || ['LB', 'RB', 'LT', 'RT'].includes(name)) {
    const [t, w] = cap(scene, g, kind === 'key' ? glyph.label : name, h, kind === 'key' ? GLYPH_COLORS.key : GLYPH_COLORS.pad);
    return [[g, t], w];
  }
  if (name === 'UP' || name === 'DOWN') {
    dpad(g, name, h);
    return [[g], h];
  }
  if (kind === 'pad') return [[g, ...round(scene, g, name, h)], h];
  g.fillStyle(GLYPH_COLORS.touch.fill).fillCircle(h / 2, 0, h / 2).lineStyle(2, GLYPH_COLORS.touch.ink).strokeCircle(h / 2, 0, h / 2 - 1);
  const hand = label(scene, '👆', h * 0.45, GLYPH_COLORS.touch.ink, h * 0.95).setY(h * 0.3);
  return [[g, label(scene, glyph.label, h * 0.55, GLYPH_COLORS.touch.ink, h / 2), hand], h * 1.1];
}

/**
 * Eine Zeile aus Text und Glyphen, mittig an (x, y); `anchor` = welche Kante der Zeile auf y liegt.
 * Baut die Objekte nur neu, wenn sich der Inhalt ändert.
 */
export class GlyphRow {
  readonly box: Phaser.GameObjects.Container;
  private shown = '';

  constructor(
    private readonly scene: Phaser.Scene,
    private readonly style: Phaser.Types.GameObjects.Text.TextStyle,
    private readonly anchor: 'top' | 'bottom',
  ) {
    this.box = scene.add.container();
  }

  set(parts: readonly RowPart[], x: number, y: number): this {
    const px = parseInt(String(this.style.fontSize), 10);
    const h = glyphPx(px);
    this.box.setPosition(x, this.anchor === 'top' ? y + h / 2 : y - h / 2).setVisible(true);
    const sig = JSON.stringify(parts);
    if (sig === this.shown) return this;
    this.shown = sig;
    this.box.removeAll(true);
    let cx = 0;
    for (const part of parts) {
      if (typeof part !== 'string' && 'image' in part) {
        const img = this.scene.add.image(cx, 0, part.image, part.frame).setOrigin(0, 0.5);
        img.setScale(part.scale ?? Math.max(1, Math.floor(h / img.height)));
        this.box.add(img);
        cx += img.displayWidth + IMAGE_GAP;
      } else if (typeof part !== 'string' && part.key !== null) {
        const [objs, w] = drawGlyph(this.scene, { key: part.key, label: part.label }, h);
        this.box.add(objs.map((o) => (o as Phaser.GameObjects.Graphics).setX((o as Phaser.GameObjects.Graphics).x + cx)));
        cx += w;
      } else if (part !== '') {
        const text = this.scene.add.text(cx, 0, typeof part === 'string' ? part : part.label, this.style).setOrigin(0, 0.5);
        this.box.add(text);
        cx += text.width;
      }
    }
    this.box.each((o: Phaser.GameObjects.Components.Transform) => o.setX(o.x - cx / 2));
    return this;
  }
}
