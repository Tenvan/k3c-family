import Phaser from 'phaser';
import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import type { World } from '@game/model/types';
import { StageView } from '@game/scenes/stageView';
import { prepareWorld, preloadWorld } from '@game/scenes/objectView';
import { prepareBuildings, preloadBuildings } from '@game/scenes/siteView';
import { createSpriteAnims, preloadSprites } from '@game/scenes/sprites';
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
