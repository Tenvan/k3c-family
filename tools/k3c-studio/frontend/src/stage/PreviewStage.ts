import Phaser from 'phaser';
import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import type { World } from '@game/model/types';
import { StageView } from '@game/scenes/render/stageView';
import { prepareWorld, preloadWorld } from '@game/scenes/render/objectView';
import { TEXT, prepareBuildings, preloadBuildings, siteName } from '@game/scenes/render/siteView';
import { createSpriteAnims, preloadSprites } from '@game/scenes/render/sprites';
import { DEFAULT_CONTROL, SHEETS, resolveAnim, type Control, type Gap } from '../playback/figure';
import { FigureView, GROUND_Y, drawScale, loadFigureSheets, type FigureItem } from '../playback/figureView';
import type { KampfWorld } from '@game/scenes/kampfAnzeige';
import { hudPx } from './hudBar';
import { KampfOverlay } from './kampfOverlay';
import { siteBox } from './siteBox';
import { refreshToParent, settleToParent } from './stageResize';

// Die wiederverwendbare Phaser-Bühne des Asset-Studios (B-460, WZ7.3; WZ10/WZ11 nutzen sie wieder, eine zweite entsteht
// nicht): lädt die Grafiken wie das Spiel (LoadScene) und zeichnet eine `World` mit `StageView`/`WorldRenderer` aus
// `src/scenes/` (unverändert, über @game). Die Bühne rechnet nichts: sie zeichnet, was sie bekommt.

/** Position und Zoom, von außen gesetzt: linke Kante in Units, Maßstab in Pixel je Unit (wie der Chunk-Streifen) */
export interface StagePosition {
  left: number;
  scale: number;
}

const DEFAULT_VIEW: StagePosition = { left: 0, scale: 12 };

/** Kamera-Zoom der Figuren-Ansicht, bei dem die Spielhöhe in die Höhe der Bühne passt. */
const stageZoom = (stageHeightPx: number): number => (stageHeightPx > 0 ? stageHeightPx / GAME_HEIGHT : 1);

/** Darstellung der Bühne: Kamerahelligkeit (Tageszeit) und ausgeblendete Hintergrund-Schichten (Textur-Schlüssel) */
export interface Look {
  brightness: number;
  hiddenLayers: ReadonlySet<string>;
}

/** Beschriftung der Plätze: Namen über jedem Gebäude (Planer) und ein dicker blauer Rahmen um die gewählten (x in Units) */
export interface Marks {
  names: boolean;
  selected: readonly number[];
}

const NO_MARKS: Marks = { names: false, selected: [] };
const FRAME_COLOR = 0x2f81f7;
const FRAME_PX = 5;

const DEFAULT_LOOK: Look = { brightness: 1, hiddenLayers: new Set() };

class PreviewScene extends Phaser.Scene {
  private ready = false;
  private pending: World | null = null;
  private view: StageView | null = null;
  private position: StagePosition = DEFAULT_VIEW;
  private look: Look = DEFAULT_LOOK;
  private marks: Marks = NO_MARKS;
  private markObjects: Phaser.GameObjects.GameObject[] = [];
  private markZoom = 0;
  private fx: Phaser.Filters.ColorMatrix | null = null;
  private dragX: number | null = null;
  // Figuren-Modus (WZ10.1): statt der Welt zeichnet die Bühne Figuren auf einer Bodenlinie mit Unit-Skala
  private figures: FigureView[] = [];
  private figureBase = 1; // Kamera-Zoom, bei dem alle Figuren im Bild stehen; `figureFactor` (Zoom der Seite) wirkt darauf
  private figureFactor = 1;
  private kampf: KampfOverlay | null = null;
  private kampfWorld: KampfWorld | null = null;
  private extras: Phaser.GameObjects.GameObject[] = [];
  private figureItems: readonly FigureItem[] | null = null;
  private control: Control = DEFAULT_CONTROL;

  constructor(private readonly onLoadError: (files: string[]) => void, private readonly onPan: (px: number) => void) {
    super('preview');
  }

  preload(): void {
    // Die Bilder des Spiels liegen unter der Wurzel des Servers; das Studio liegt unter /studio/, relative Adressen träfen dort ins Leere.
    this.load.setBaseURL(new URL('../', document.baseURI).href);
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
    this.scale.on('resize', () => this.applyView());
    // Die Position gehört der Seite: Ziehen und Mausrad melden nur die Bewegung (Bildschirm-Pixel), die Seite setzt sie neu.
    this.input.on('pointerdown', (p: Phaser.Input.Pointer) => { this.dragX = p.x; });
    this.input.on('pointermove', (p: Phaser.Input.Pointer) => {
      if (this.dragX === null || !p.isDown) return;
      this.onPan(this.dragX - p.x);
      this.dragX = p.x;
    });
    this.input.on('pointerup', () => { this.dragX = null; });
    this.show(this.pending);
  }

  /** Zeigt die Welt (null = leer). Vor dem Laden der Grafiken wird sie gemerkt. */
  show(world: World | null): void {
    this.pending = world;
    if (!this.ready) return;
    this.view?.destroy();
    this.view = null;
    this.clearMarks();
    if (!world) return;
    this.view = new StageView(this, world);
    this.view.onBackground = () => this.applyLook(); // Bilder ersetzen die Silhouetten nach dem Laden: Schalter neu anwenden
    this.cameras.main.setBackgroundColor(world.biome.palette.sky); // wie GameScene: der Himmel des Bioms, die Helligkeit der Tageszeit liegt als Filter darüber
    this.view.renderer.sync(world);
    this.applyView();
    this.applyLook();
    this.drawMarks();
  }

  /** Namen und Rahmen setzen (Gebäude-Planer) */
  setMarks(marks: Marks): void {
    this.marks = marks;
    if (this.ready) this.drawMarks();
  }

  private clearMarks(): void {
    for (const o of this.markObjects) o.destroy();
    this.markObjects = [];
    this.markZoom = 0;
  }

  /** Namen über allen Plätzen und blauer Rahmen um die gewählten, in Welt-Koordinaten; Schrift und Linie bleiben beim Zoom gleich groß. */
  private drawMarks(): void {
    this.clearMarks();
    const world = this.pending;
    if (!world || !this.view || (!this.marks.names && this.marks.selected.length === 0)) return;
    const zoom = this.position.scale / UNIT_PX;
    this.markZoom = zoom;
    const frame = this.add.graphics().setDepth(60).lineStyle(FRAME_PX / zoom, FRAME_COLOR);
    this.markObjects.push(frame);
    const pad = 4 / zoom;
    for (const s of world.sites) {
      const box = siteBox(s, this.view.layer.list);
      const cx = (box.left + box.right) / 2;
      if (this.marks.names) {
        this.markObjects.push(this.add.text(cx, box.top - 12 / zoom, siteName(s.kind), { ...TEXT, fontSize: '16px', color: '#ffffff' }).setOrigin(0.5, 1).setDepth(61).setScale(1 / zoom));
      }
      if (this.marks.selected.includes(s.x)) frame.strokeRect(box.left - pad, box.top - pad, box.right - box.left + 2 * pad, box.bottom - box.top + 2 * pad);
    }
  }

  /** Die Ansicht so verschieben, dass x (Units) in der Mitte steht; die Seite setzt die Position über `onPan` */
  focusX(x: number): void {
    if (!this.ready) return;
    const { left, scale } = this.position;
    const target = Math.max(0, x - this.scale.width / scale / 2);
    this.onPan((target - left) * scale);
  }

  /** Tageszeit-Helligkeit und Hintergrund-Schichten übernehmen; gilt auch für später gezeichnete Welten. */
  setLook(look: Look): void {
    this.look = look;
    if (this.ready) this.applyLook();
  }

  private applyLook(): void {
    this.fx?.colorMatrix.brightness(this.look.brightness);
    // Schichten sind Container mit Namen bg:<biom>:<n> (GEB18); die Spiegelung im Fluss heißt gleich und folgt mit.
    for (const o of this.view?.layer.list ?? []) {
      if (o instanceof Phaser.GameObjects.Container && o.name.startsWith('bg:')) o.setVisible(!this.look.hiddenLayers.has(o.name));
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
    this.drawKampf();
    return views.map((v, i) => (v ? { gap: v.gap, frames: v.frames() } : { gap: resolveAnim(items[i]!.spec.sheet, items[i]!.anim).gap, frames: 1 }));
  }

  /** Wiedergabe (Schleife, Tempo, Einzelbild) aller Figuren */
  setControl(control: Control): void {
    this.control = control;
    for (const f of this.figures) f.apply(control);
  }

  update(): void {
    for (const f of this.figures) f.follow();
    this.kampf?.place();
  }

  /** Boss-Anzeigen des Spiels für den Beispielzustand `world` (null = keine, B-469); wirkt in der Figuren-Ansicht. */
  setKampf(world: KampfWorld | null): void {
    this.kampfWorld = world;
    if (this.ready) this.drawKampf();
  }

  private drawKampf(): void {
    this.kampf ??= new KampfOverlay(this);
    this.kampf.show(this.figures.length > 0 ? this.kampfWorld : null);
  }

  private clearFigures(): void {
    for (const f of this.figures) f.destroy();
    for (const o of this.extras) o.destroy();
    this.figures = [];
    this.extras = [];
    this.kampf?.clear();
  }

  /** Kamera: alle Figuren und die Unit-Skala im Bild, Bodenlinie unten */
  private fitFigures(items: readonly FigureItem[]): void {
    const xs = items.map((i) => i.x);
    const from = Math.floor(Math.min(...xs, 0) - 6);
    const to = Math.ceil(Math.max(...xs, 0) + 6);
    this.extras = drawScale(this, from, to);
    const cam = this.cameras.main;
    const zoom = Math.min(stageZoom(this.scale.height), this.scale.width / ((to - from) * UNIT_PX));
    this.figureBase = zoom;
    cam.setZoom(zoom * this.figureFactor);
    cam.removeBounds();
    cam.centerOn(((from + to) / 2) * UNIT_PX, GROUND_Y - 140);
  }

  /** Zoom der Seite (1 = alle Figuren im Bild, B-502) auf die Figuren-Ansicht anwenden; die Mitte bleibt. */
  setFigureZoom(factor: number): void {
    this.figureFactor = factor;
    if (this.figures.length > 0) this.cameras.main.setZoom(this.figureBase * factor);
  }

  /** Position und Zoom von der Seite übernehmen (gleicher Maßstab wie der Chunk-Streifen). */
  setPosition(pos: StagePosition): void {
    this.position = pos;
    if (this.ready) this.applyView();
  }

  /** Kamera: Maßstab (Pixel je Unit) als Zoom, linke Kante an die gesetzte Position; der Boden steht unten in der Bühne. */
  private applyView(): void {
    const world = this.pending;
    if (!world || !this.view) return;
    const cam = this.cameras.main;
    const { left, scale } = this.position;
    const zoom = scale / UNIT_PX;
    // Oben die Leiste des globalen HUDs (leer, Überlagerung der Seite), die Kamera zeigt darunter den Weltausschnitt wie im Spiel.
    const bar = Math.min(hudPx(scale), this.scale.height);
    cam.setViewport(0, bar, this.scale.width, this.scale.height - bar);
    const heightPx = (this.scale.height - bar) / zoom;
    cam.setZoom(zoom);
    cam.setBounds(0, GAME_HEIGHT - heightPx, world.widthUnits * UNIT_PX, heightPx);
    this.view.scaleTexts(1 / zoom);
    if (zoom !== this.markZoom) this.drawMarks();
    cam.centerOnX((left + this.scale.width / scale / 2) * UNIT_PX);
  }
}

/** Lücke und Bildzahl einer gezeichneten Figur */
export interface FigureInfo {
  gap: Gap | null;
  frames: number;
}

/** Phaser-Spiel im Element `parent`; die Größe folgt dem Element (Höhe per CSS festlegen); Ziehen meldet `onPan`, das Mausrad `onWheel`
 *  (die Pfeiltasten fängt die Seite, sonst würde jeder Druck doppelt scrollen). `onLoadError` meldet Grafiken, die nicht geladen werden konnten. */
export class PreviewStage {
  private readonly game: Phaser.Game;
  private readonly scene: PreviewScene;
  private readonly resize: ResizeObserver;
  private readonly layout: ResizeObserver;
  private readonly wheel: (e: WheelEvent) => void;
  private readonly parent: HTMLElement;

  constructor(parent: HTMLElement, onLoadError: (files: string[]) => void = () => {}, onPan: (px: number) => void = () => {}, onWheel: (e: WheelEvent) => void = () => {}) {
    this.parent = parent;
    this.scene = new PreviewScene(onLoadError, onPan);
    this.game = new Phaser.Game({
      type: Phaser.AUTO,
      parent,
      backgroundColor: '#101018',
      scale: { mode: Phaser.Scale.RESIZE, width: '100%', height: '100%' },
      audio: { noAudio: true },
      input: { gamepad: false, keyboard: false, mouse: { preventDefaultWheel: false } }, // das Rad entscheidet die Seite (wheelAction)
      scene: this.scene,
    });
    // Phaser hört nur auf das Fenster: ein Beobachter am Element fängt auch Layout-Änderungen ohne Fenster-Resize ab und
    // misst die Elterngröße dabei neu (refresh allein rechnet mit der alten, WZ17.5).
    this.resize = new ResizeObserver(() => refreshToParent(this.game.scale));
    this.resize.observe(parent);
    // Wächst oder schrumpft die Seite (Zeilen darüber, auch aus lokalem Zustand einer Kind-Komponente), kann sich nur die
    // Lage der Bühne ändern; Phaser prüft dann Größe und Lage (WZ17.5).
    this.layout = new ResizeObserver(() => this.settle());
    this.layout.observe(document.body);
    this.wheel = (e) => onWheel(e);
    parent.addEventListener('wheel', this.wheel, { passive: false }); // nicht passiv: die Seite bricht das Ereignis bei Zoom und Scrollen ab
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

  /** Boss-Anzeigen des Spiels für einen Beispielzustand (null = keine) */
  setKampf(world: KampfWorld | null): void {
    this.scene.setKampf(world);
  }

  /** Zoom der Figuren-Ansicht (1 = alle Figuren im Bild) */
  setFigureZoom(factor: number): void {
    this.scene.setFigureZoom(factor);
  }

  /** Position und Zoom setzen */
  setPosition(pos: StagePosition): void {
    this.scene.setPosition(pos);
  }

  /** Helligkeit und Hintergrund-Schichten setzen */
  setLook(look: Look): void {
    this.scene.setLook(look);
  }

  /** Namen über den Plätzen und Rahmen um die gewählten (Gebäude-Planer) */
  setMarks(marks: Marks): void {
    this.scene.setMarks(marks);
  }

  /** x (Units) in die Mitte der Ansicht bringen */
  focusX(x: number): void {
    this.scene.focusX(x);
  }

  /** Nach einer Layout-Änderung der Seite: neu skalieren, wenn sich Größe oder Lage des Elements geändert hat */
  settle(): void {
    settleToParent(this.game.scale);
  }

  /** Größe neu lesen, z. B. nachdem das Element sichtbar wurde */
  refresh(): void {
    refreshToParent(this.game.scale);
  }

  destroy(): void {
    this.resize.disconnect();
    this.layout.disconnect();
    this.parent.removeEventListener('wheel', this.wheel);
    this.game.destroy(true);
  }
}
