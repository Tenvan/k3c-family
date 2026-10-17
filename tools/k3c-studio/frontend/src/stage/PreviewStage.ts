import Phaser from 'phaser';
import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import type { World } from '@game/model/types';
import { StageView } from '@game/scenes/stageView';
import { prepareWorld, preloadWorld } from '@game/scenes/objectView';
import { prepareBuildings, preloadBuildings } from '@game/scenes/siteView';
import { createSpriteAnims, preloadSprites } from '@game/scenes/sprites';
import { DEFAULT_CONTROL, SHEETS, resolveAnim, type Control, type Gap } from '../abspiel/figure';
import { FigureView, GROUND_Y, drawScale, loadFigureSheets, type FigureItem } from '../abspiel/figureView';
import { levelKey, stageZoom } from './stageLogic';

// Die wiederverwendbare Phaser-Bühne des Asset-Studios (B-460, WZ7.3; WZ10/WZ11 nutzen sie wieder, eine zweite entsteht
// nicht): lädt die Grafiken wie das Spiel (LoadScene) und zeichnet eine `World` mit `StageView`/`WorldRenderer` aus
// `src/scenes/` (unverändert, über @game). Die Bühne rechnet nichts: sie zeichnet, was sie bekommt.

/** Scroll-Schritt je Mausrad-Tick und Pfeiltaste (Bildschirm-Pixel) */
const SCROLL_STEP = 120;

/** Darstellung der Bühne: Kamerahelligkeit (Tageszeit) und ausgeblendete Hintergrund-Schichten (Textur-Schlüssel) */
export interface Look {
  brightness: number;
  hiddenLayers: ReadonlySet<string>;
}

const DEFAULT_LOOK: Look = { brightness: 1, hiddenLayers: new Set() };

class PreviewScene extends Phaser.Scene {
  private ready = false;
  private pending: World | null = null;
  private view: StageView | null = null;
  private centeredFor = '';
  private look: Look = DEFAULT_LOOK;
  private fx: Phaser.Filters.ColorMatrix | null = null;
  private drag: { x: number; scroll: number } | null = null;
  // Figuren-Modus (WZ10.1): statt der Welt zeichnet die Bühne Figuren auf einer Bodenlinie mit Unit-Skala
  private figures: FigureView[] = [];
  private extras: Phaser.GameObjects.GameObject[] = [];
  private figureItems: readonly FigureItem[] | null = null;
  private control: Control = DEFAULT_CONTROL;

  constructor(private readonly onLoadError: (files: string[]) => void) {
    super('preview');
  }

  preload(): void {
    this.load.on('loaderror', (file: Phaser.Loader.File) => this.failed.push(String(file.url)));
    preloadSprites(this);
    preloadBuildings(this);
    preloadWorld(this);
  }

  private readonly failed: string[] = [];
  private resolveReady: () => void = () => {};
  /** Erfüllt, sobald die Grafiken geladen sind und die Szene zeichnen kann */
  readonly whenReady = new Promise<void>((resolve) => { this.resolveReady = resolve; });

  create(): void {
    // Fehlende Dateien melden, die Bühne zeichnet trotzdem, was da ist: der Figuren-Atlas (public/atlas/) ist erzeugt und
    // fehlt in einem frischen Checkout; die Vorschau zeigt ohnehin keine Figuren.
    if (this.failed.length > 0) this.onLoadError(this.failed);
    createSpriteAnims(this);
    prepareBuildings(this);
    prepareWorld(this);
    this.cameras.main.filters.internal.clear(); // wie GameScene: die Hauptkamera überlebt einen Neustart der Szene
    this.fx = this.cameras.main.filters.internal.addColorMatrix();
    this.ready = true;
    this.resolveReady();
    this.scale.on('resize', () => this.fit());
    this.input.on('pointerdown', (p: Phaser.Input.Pointer) => { this.drag = { x: p.x, scroll: this.cameras.main.scrollX }; });
    this.input.on('pointermove', (p: Phaser.Input.Pointer) => {
      if (this.drag && p.isDown) this.cameras.main.scrollX = this.drag.scroll - (p.x - this.drag.x) / this.cameras.main.zoom;
    });
    this.input.on('pointerup', () => { this.drag = null; });
    this.input.on('wheel', (_p: unknown, _o: unknown, dx: number, dy: number) => this.scrollBy((Math.abs(dx) > Math.abs(dy) ? dx : dy) > 0 ? SCROLL_STEP : -SCROLL_STEP));
    this.show(this.pending);
  }

  /** Kamera um Bildschirm-Pixel verschieben (Pfeiltasten und Mausrad) */
  scrollBy(px: number): void {
    const cam = this.cameras.main;
    cam.scrollX += px / cam.zoom;
  }

  /** Zeigt die Welt (null = leer). Vor dem Laden der Grafiken wird sie gemerkt. */
  show(world: World | null): void {
    this.pending = world;
    if (!this.ready) return;
    this.view?.destroy();
    this.view = null;
    if (!world) return;
    this.view = new StageView(this, world);
    this.view.renderer.sync(world);
    const key = levelKey(world);
    this.fit(key !== this.centeredFor); // nur ein anderes Level setzt die Kamera zurück zur Burg
    this.centeredFor = key;
    this.applyLook();
  }

  /** Tageszeit-Helligkeit und Hintergrund-Schichten übernehmen; gilt auch für später gezeichnete Welten. */
  setLook(look: Look): void {
    this.look = look;
    if (this.ready) this.applyLook();
  }

  private applyLook(): void {
    this.fx?.colorMatrix.brightness(this.look.brightness);
    // Schichten sind TileSprites mit Schlüssel bg:<biom>:<n>; die Spiegelung im Fluss nutzt dieselbe Textur und folgt mit.
    for (const o of this.view?.layer.list ?? []) {
      if (o instanceof Phaser.GameObjects.TileSprite && o.texture.key.startsWith('bg:')) o.setVisible(!this.look.hiddenLayers.has(o.texture.key));
    }
  }

  /** Zeigt Figuren (null = zurück zur Welt). Antwortet, wenn alle Bilder geladen und gezeichnet sind, mit Lücke und
   *  Bildzahl je Figur. Eine neuere Anfrage macht eine ältere ungültig (Antwort leer). */
  async showFigures(items: readonly FigureItem[] | null): Promise<FigureInfo[]> {
    this.figureItems = items;
    if (!this.ready) return [];
    this.clearFigures();
    if (!items) {
      this.show(this.pending);
      return [];
    }
    this.view?.destroy();
    this.view = null;
    await loadFigureSheets(this, items);
    if (this.figureItems !== items) return [];
    // Ein unbekanntes Sheet ist eine Lücke ohne Figur (makeSprite würde abstürzen).
    const views = items.map((i) => (SHEETS[i.spec.sheet] ? new FigureView(this, i) : null));
    this.figures = views.filter((v): v is FigureView => v !== null);
    this.fitFigures(items);
    this.setControl(this.control);
    return views.map((v, i) => (v ? { gap: v.gap, frames: v.frames() } : { gap: resolveAnim(items[i]!.spec.sheet, items[i]!.anim).gap, frames: 1 }));
  }

  /** Wiedergabe (Schleife, Tempo, Einzelbild) aller Figuren */
  setControl(control: Control): void {
    this.control = control;
    for (const f of this.figures) f.apply(control);
  }

  update(): void {
    for (const f of this.figures) f.follow();
  }

  private clearFigures(): void {
    for (const f of this.figures) f.destroy();
    for (const o of this.extras) o.destroy();
    this.figures = [];
    this.extras = [];
  }

  /** Kamera: alle Figuren und die Unit-Skala im Bild, Bodenlinie unten */
  private fitFigures(items: readonly FigureItem[]): void {
    const xs = items.map((i) => i.x);
    const from = Math.floor(Math.min(...xs, 0) - 6);
    const to = Math.ceil(Math.max(...xs, 0) + 6);
    this.extras = drawScale(this, from, to);
    const cam = this.cameras.main;
    const zoom = Math.min(stageZoom(this.scale.height), this.scale.width / ((to - from) * UNIT_PX));
    cam.setZoom(zoom);
    cam.removeBounds();
    cam.centerOn(((from + to) / 2) * UNIT_PX, GROUND_Y - 140);
  }

  /** Kamera: Spielhöhe passt in die Bühne; `center` stellt die Burg in die Mitte. */
  private fit(center = false): void {
    const world = this.pending;
    if (!world || !this.view) return;
    const cam = this.cameras.main;
    const zoom = stageZoom(this.scale.height);
    cam.setZoom(zoom);
    cam.setBounds(0, 0, world.widthUnits * UNIT_PX, GAME_HEIGHT);
    this.view.scaleTexts(1 / zoom);
    if (center) cam.centerOnX(world.castle.x * UNIT_PX);
  }
}

/** Phaser-Spiel im Element `parent`; die Größe folgt dem Element (Höhe per CSS festlegen), Pfeiltasten scrollen, solange
 *  das Element den Fokus hat. `onLoadError` meldet Grafiken, die nicht geladen werden konnten. */
/** Lücke und Bildzahl einer gezeichneten Figur */
export interface FigureInfo {
  gap: Gap | null;
  frames: number;
}

export class PreviewStage {
  private readonly game: Phaser.Game;
  private readonly scene: PreviewScene;
  private readonly resize: ResizeObserver;
  private readonly onKey: (e: KeyboardEvent) => void;

  constructor(private readonly parent: HTMLElement, onLoadError: (files: string[]) => void = () => {}) {
    this.scene = new PreviewScene(onLoadError);
    this.game = new Phaser.Game({
      type: Phaser.AUTO,
      parent,
      backgroundColor: '#101018',
      scale: { mode: Phaser.Scale.RESIZE, width: '100%', height: '100%' },
      audio: { noAudio: true },
      input: { gamepad: false, keyboard: false },
      scene: this.scene,
    });
    // Phaser hört nur auf das Fenster: ein Beobachter am Element fängt auch Layout-Änderungen ohne Fenster-Resize ab.
    this.resize = new ResizeObserver(() => this.game.scale.refresh());
    this.resize.observe(parent);
    parent.tabIndex = 0;
    this.onKey = (e) => {
      if (e.key === 'ArrowLeft') this.scene.scrollBy(-SCROLL_STEP);
      else if (e.key === 'ArrowRight') this.scene.scrollBy(SCROLL_STEP);
    };
    parent.addEventListener('keydown', this.onKey);
  }

  show(world: World | null): void {
    this.scene.show(world);
  }

  /** Figuren statt der Welt zeigen (null = zurück zur Welt); antwortet mit Lücke und Bildzahl je Figur (WZ10.1) */
  async showFigures(items: readonly FigureItem[] | null): Promise<FigureInfo[]> {
    await this.scene.whenReady;
    return this.scene.showFigures(items);
  }

  /** Wiedergabe der Figuren: Schleife, Tempo, Einzelbild */
  setControl(control: Control): void {
    this.scene.setControl(control);
  }

  /** Helligkeit und Hintergrund-Schichten setzen */
  setLook(look: Look): void {
    this.scene.setLook(look);
  }

  /** Größe neu lesen, z. B. nachdem das Element sichtbar wurde */
  refresh(): void {
    this.game.scale.refresh();
  }

  destroy(): void {
    this.resize.disconnect();
    this.parent.removeEventListener('keydown', this.onKey);
    this.game.destroy(true);
  }
}
