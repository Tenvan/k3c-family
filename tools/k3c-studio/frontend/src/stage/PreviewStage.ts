import Phaser from 'phaser';
import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import type { World } from '@game/model/types';
import { StageView } from '@game/scenes/stageView';
import { prepareWorld, preloadWorld } from '@game/scenes/objectView';
import { prepareBuildings, preloadBuildings } from '@game/scenes/siteView';
import { createSpriteAnims, preloadSprites } from '@game/scenes/sprites';

// Die wiederverwendbare Phaser-Bühne des Asset-Studios (B-460, WZ7.3; WZ10/WZ11 nutzen sie wieder, eine zweite entsteht
// nicht): lädt die Grafiken wie das Spiel (LoadScene) und zeichnet eine `World` mit `StageView`/`WorldRenderer` aus
// `src/scenes/` (unverändert, über @game). Die Bühne rechnet nichts: sie zeichnet, was sie bekommt.

/** Position und Zoom, von außen gesetzt: linke Kante in Units, Maßstab in Pixel je Unit (wie der Chunk-Streifen) */
export interface StagePosition {
  left: number;
  scale: number;
}

const DEFAULT_VIEW: StagePosition = { left: 0, scale: 12 };

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
  private position: StagePosition = DEFAULT_VIEW;
  private look: Look = DEFAULT_LOOK;
  private fx: Phaser.Filters.ColorMatrix | null = null;
  private dragX: number | null = null;

  constructor(private readonly onLoadError: (files: string[]) => void, private readonly onPan: (px: number) => void) {
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
    if (!world) return;
    this.view = new StageView(this, world);
    this.view.renderer.sync(world);
    this.applyView();
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
    const heightPx = this.scale.height / zoom;
    cam.setZoom(zoom);
    cam.setBounds(0, GAME_HEIGHT - heightPx, world.widthUnits * UNIT_PX, heightPx);
    this.view.scaleTexts(1 / zoom);
    cam.centerOnX((left + this.scale.width / scale / 2) * UNIT_PX);
  }
}

/** Phaser-Spiel im Element `parent`; die Größe folgt dem Element (Höhe per CSS festlegen); Ziehen meldet `onPan`, das Mausrad `onWheel`
 *  (die Pfeiltasten fängt die Seite, sonst würde jeder Druck doppelt scrollen). `onLoadError` meldet Grafiken, die nicht geladen werden konnten. */
export class PreviewStage {
  private readonly game: Phaser.Game;
  private readonly scene: PreviewScene;
  private readonly resize: ResizeObserver;
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
    // Phaser hört nur auf das Fenster: ein Beobachter am Element fängt auch Layout-Änderungen ohne Fenster-Resize ab.
    this.resize = new ResizeObserver(() => this.game.scale.refresh());
    this.resize.observe(parent);
    this.wheel = (e) => onWheel(e);
    parent.addEventListener('wheel', this.wheel, { passive: false }); // nicht passiv: die Seite bricht das Ereignis bei Zoom und Scrollen ab
  }

  show(world: World | null): void {
    this.scene.show(world);
  }

  /** Position und Zoom setzen */
  setPosition(pos: StagePosition): void {
    this.scene.setPosition(pos);
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
    this.parent.removeEventListener('wheel', this.wheel);
    this.game.destroy(true);
  }
}
