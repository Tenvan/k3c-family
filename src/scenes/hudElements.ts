import Phaser from 'phaser';
import { pauseButton } from './pauseButton';
import { DEBUG_AREA, type HAlign, type Rect, type Size } from './hudLayout';

/** Stil laut 🧑 (2026-10-10): Schwarz mit 55 % Deckkraft, Ecken 8 px, Innenabstand 8/4 px, ohne Rahmen. */
export const BOX = { fill: 0x000000, alpha: 0.55, radius: 8, padX: 8, padY: 4, frame: 0xffffff, frameAlpha: 0.35, frameWidth: 2 } as const;

export interface BoxStyle {
  background: boolean;
  frame: boolean;
}

/** Breite in Stufen, damit wechselnde Texte (Uhr, Abklingzeit) das Layout nicht jedes Bild neu anstoßen */
const WIDTH_STEP = 32;

/**
 * Hülle eines HUD-Elements: Text mit optionalem Hintergrund und Rahmen (je Element schaltbar). Inhalt und Sichtbarkeit
 * setzt das HUD am Text; die Lage kommt aus `hudLayout`, ein vom Layout ausgeblendetes Element verbirgt seinen Container.
 */
export class HudBox {
  private readonly g: Phaser.GameObjects.Graphics;
  private readonly box: Phaser.GameObjects.Container;

  constructor(
    scene: Phaser.Scene,
    readonly text: Phaser.GameObjects.Text,
    public style: BoxStyle = { background: true, frame: false },
  ) {
    this.g = scene.add.graphics();
    this.box = scene.add.container(0, 0, [this.g, text]);
  }

  /** Platzbedarf in Spielkoordinaten, `undefined` = gerade nichts zu zeigen */
  size(): Size | undefined {
    if (!this.text.visible || this.text.text === '') return undefined;
    return { w: Math.ceil((this.text.width + 2 * BOX.padX) / WIDTH_STEP) * WIDTH_STEP, h: Math.ceil(this.text.height + 2 * BOX.padY) };
  }

  /** Setzt Text und Hintergrund in das Rechteck, bündig wie der Anker; `null` blendet aus. */
  place(rect: Rect | null | undefined, align: HAlign): void {
    this.box.setVisible(!!rect);
    this.g.clear();
    if (!rect) return;
    const origin = align === 'left' ? 0 : align === 'right' ? 1 : 0.5;
    const x = align === 'left' ? rect.x + BOX.padX : align === 'right' ? rect.x + rect.w - BOX.padX : rect.x + rect.w / 2;
    this.text.setOrigin(origin, 0).setPosition(x, rect.y + BOX.padY);
    const w = this.text.width + 2 * BOX.padX;
    const bx = x - origin * this.text.width - BOX.padX;
    const h = this.text.height + 2 * BOX.padY;
    if (this.style.background) this.g.fillStyle(BOX.fill, BOX.alpha).fillRoundedRect(bx, rect.y, w, h, BOX.radius);
    if (this.style.frame) this.g.lineStyle(BOX.frameWidth, BOX.frame, BOX.frameAlpha).strokeRoundedRect(bx, rect.y, w, h, BOX.radius);
  }
}

/** Rechteck eines DOM-Elements in Spielkoordinaten (Canvas skaliert mit `Phaser.Scale.FIT`) */
function toGame(scene: Phaser.Scene, r: DOMRect): Rect {
  const x = scene.scale.transformX(r.left);
  const y = scene.scale.transformY(r.top);
  return { x, y, w: scene.scale.transformX(r.right) - x, h: scene.scale.transformY(r.bottom) - y };
}

/** Freiflächen (B-337): Home-Button, „☰ Optionen“, Touch-Knöpfe und, solange sichtbar, die Diagnose. DOM nur lesen. */
export function freeAreas(scene: Phaser.Scene, debugShown: boolean): Rect[] {
  const dom = [...document.querySelectorAll<HTMLElement>('.k3c-home, .k3c-touch .grp')].map((el) => el.getBoundingClientRect());
  const button = pauseButton().rect();
  if (button) dom.push(button);
  const rects = dom.filter((r) => r.width > 0 && r.height > 0).map((r) => toGame(scene, r));
  return debugShown ? [...rects, DEBUG_AREA] : rects;
}
