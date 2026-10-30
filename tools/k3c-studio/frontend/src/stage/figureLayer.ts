import type Phaser from 'phaser';
import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import { DEFAULT_CONTROL, SHEETS, figureIndexAt, type Control } from '../playback/figure';
import { FigureView, GROUND_Y, drawScale, type FigureItem } from '../playback/figureView';

// Figuren-Ebene der Bühne (WZ10.1, B-672): hält die gezeichneten Figuren, Klick-Treffer, Markierung, Kamera-Anpassung und
// Wiedergabe. Die Phaser-Szene (`PreviewScene`) ist der Host; die Ebene zeichnet nur, was sie bekommt.

export const FRAME_COLOR = 0x2f81f7;
export const FRAME_PX = 5;

/** Kamera-Zoom der Figuren-Ansicht, bei dem die Spielhöhe in die Höhe der Bühne passt. */
const stageZoom = (stageHeightPx: number): number => (stageHeightPx > 0 ? stageHeightPx / GAME_HEIGHT : 1);

export class FigureLayer {
  private figures: FigureView[] = [];
  private base = 1; // Kamera-Zoom, bei dem alle Figuren im Bild stehen; `factor` (Zoom der Seite) wirkt darauf
  private factor = 1;
  private extras: Phaser.GameObjects.GameObject[] = [];
  private control: Control = DEFAULT_CONTROL;
  // Auswahl einer Figur (B-672): Ansichten in der Reihenfolge der Liste (null = ohne Figur), Klick-Meldung und Markierung
  private slots: (FigureView | null)[] = [];
  private onClick: ((index: number | null) => void) | null = null;
  private mark: number | null = null;
  private markGfx: Phaser.GameObjects.Graphics | null = null;

  constructor(private readonly scene: Phaser.Scene) {}

  get count(): number {
    return this.figures.length;
  }

  /** Ansichten für die Einträge anlegen und die Kamera darauf einstellen (null = unbekanntes Sheet, nur eine Lücke) */
  build(items: readonly FigureItem[]): (FigureView | null)[] {
    // Ein unbekanntes Sheet ist eine Lücke ohne Figur (makeSprite würde abstürzen).
    const views = items.map((i) => (SHEETS[i.spec.sheet] ? new FigureView(this.scene, i) : null));
    this.slots = views;
    this.figures = views.filter((v): v is FigureView => v !== null);
    this.fit(items);
    this.setControl(this.control);
    return views;
  }

  clear(): void {
    for (const f of this.figures) f.destroy();
    for (const o of this.extras) o.destroy();
    this.figures = [];
    this.slots = [];
    this.markGfx?.clear();
    this.extras = [];
  }

  /** Wiedergabe (Schleife, Tempo, Einzelbild) aller Figuren */
  setControl(control: Control): void {
    this.control = control;
    for (const f of this.figures) f.apply(control);
  }

  follow(): void {
    for (const f of this.figures) f.follow();
  }

  /** Klick auf eine Figur melden (Index in der Liste, null = daneben); null schaltet den Treffer-Test ab (B-672). */
  setClick(onFigure: ((index: number | null) => void) | null): void {
    this.onClick = onFigure;
  }

  /** Klick melden, wenn die Ebene ihn auswertet; false = der Host behandelt den Klick selbst */
  click(p: Phaser.Input.Pointer): boolean {
    if (!this.onClick || this.figures.length === 0) return false;
    const cam = this.scene.cameras.main;
    const point = cam.getWorldPoint(p.x, p.y);
    this.onClick(figureIndexAt(this.slots.map((v) => v?.bounds() ?? null), point.x, point.y, 8 / cam.zoom));
    return true;
  }

  /** Die Figur mit diesem Index in der Liste blau umrahmen (null = keine) */
  setMark(index: number | null): void {
    this.mark = index;
  }

  drawMark(): void {
    this.markGfx ??= this.scene.add.graphics().setDepth(60);
    this.markGfx.clear();
    const box = this.mark === null ? null : this.slots[this.mark]?.bounds();
    if (!box) return;
    const pad = 6;
    this.markGfx.lineStyle(FRAME_PX / this.scene.cameras.main.zoom, FRAME_COLOR).strokeRect(box.left - pad, box.top - pad, box.right - box.left + 2 * pad, box.bottom - box.top + 2 * pad);
  }

  /** Kamera: alle Figuren und die Unit-Skala im Bild, Bodenlinie unten */
  private fit(items: readonly FigureItem[]): void {
    const xs = items.map((i) => i.x);
    const from = Math.floor(Math.min(...xs, 0) - 6);
    const to = Math.ceil(Math.max(...xs, 0) + 6);
    this.extras = drawScale(this.scene, from, to);
    const cam = this.scene.cameras.main;
    const zoom = Math.min(stageZoom(this.scene.scale.height), this.scene.scale.width / ((to - from) * UNIT_PX));
    this.base = zoom;
    cam.setZoom(zoom * this.factor);
    cam.removeBounds();
    cam.centerOn(((from + to) / 2) * UNIT_PX, GROUND_Y - 140);
  }

  /** Zoom der Seite (1 = alle Figuren im Bild, B-502) auf die Figuren-Ansicht anwenden; die Mitte bleibt. */
  setZoom(factor: number): void {
    this.factor = factor;
    if (this.figures.length > 0) this.scene.cameras.main.setZoom(this.base * factor);
  }
}
