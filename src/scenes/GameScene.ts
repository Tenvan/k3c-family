import Phaser from 'phaser';
import { toggleFullscreen } from '../core/fullscreen';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, UNIT_PX } from '../core/constants';
import { GamepadInput, KeyboardInput, type PlayerInput } from '../input/playerInput';
import { TouchInput, wantsTouchControls } from '../input/touchInput';
import type { RoomClient } from '../online/clientConnection';
import { applyState, createViewWorld } from '../online/clientWorld';
import { blendAlpha, interpolate } from '../online/clientInterpolation';
import type { Frame } from '../online/clientConnection';
import type { GameEvent, World } from '../world/sim/types';
import { computeLayout, type Cell } from './layout';
import { LocalSlots } from './localSlots';
import { createSpriteAnims, preloadSprites } from './sprites';
import { daylight } from './viewRules';
import { WorldRenderer } from './worldRenderer';

/** Ein Overlay pro Seite, auch über Szenen-Neustarts hinweg */
let sharedTouch: TouchInput | undefined;
function touchControls(): TouchInput {
  return (sharedTouch ??= new TouchInput());
}

export interface GameSceneData {
  /** Verbindung zum Server (Protokoll v2): der Browser rechnet nichts, er sendet Eingaben und zeichnet */
  client: RoomClient;
  /** Slots ohne Eingabe, die nur stehen (Testseite, B-082) */
  mock?: number[];
}

/** Der Mitspieler oben wechselt erst, wenn ein anderer um so viele Units näher ist */
const PARTNER_SWITCH_UNITS = 10;
/** Helligkeit in tiefster Nacht */
const NIGHT_BRIGHTNESS = 0.45;

/**
 * Spielwelt im Browser: sendet die Eingaben der lokalen Spieler (Slots) an den Server und zeichnet die interpolierten Zustände.
 * Beitritt eines weiteren Spielers per A / Leertaste (`addSlot`), Layout nach `layout.ts`. Rechnet nichts.
 */
export class GameScene extends Phaser.Scene {
  /** Ereignisse für die HudScene, die sie abholt und leert */
  readonly pendingEvents: GameEvent[] = [];
  /** Zuletzt benutztes Eingabegerät, damit die Hinweise im HUD zur Steuerung passen */
  lastDevice: 'keyboard' | 'pad' | 'touch' = 'keyboard';

  private data_!: GameSceneData;
  private world_: World | undefined;
  private renderer_!: WorldRenderer;
  private keyboard!: KeyboardInput;
  private touch: TouchInput | undefined;
  private pads: GamepadInput[] = [];
  private nightFx: Phaser.Filters.ColorMatrix[] = [];
  private slots!: LocalSlots<PlayerInput>;
  private level: object | null = null;
  private prev: Frame | null = null;
  private cur: Frame | null = null;
  private cells: Cell[] = [];
  private layoutKey = '';
  private partnerMonarch: number | null = null;

  constructor() {
    super('game');
  }

  get world(): World | undefined {
    return this.world_;
  }

  get client(): RoomClient {
    return this.data_.client;
  }

  init(data: GameSceneData): void {
    this.data_ = data;
    this.pendingEvents.length = 0;
    this.pads = [];
    this.nightFx = [];
    this.touch = undefined;
    this.slots = new LocalSlots(data.mock);
    this.world_ = undefined;
    this.level = null;
    this.prev = this.cur = null;
    this.layoutKey = '';
    this.partnerMonarch = null;
    this.lastDevice = wantsTouchControls() ? 'touch' : 'keyboard';
  }

  preload(): void {
    preloadSprites(this);
  }

  create(): void {
    createSpriteAnims(this);
    this.keyboard = new KeyboardInput(this.input.keyboard!);
    if (wantsTouchControls()) this.touch = touchControls();
    // Browser melden Gamepads erst nach dem ersten Tastendruck. Alle bekannten + neue Pads beobachten.
    const gamepads = this.input.gamepad!;
    const addPad = (pad: Phaser.Input.Gamepad.Gamepad) => {
      if (pad && !this.pads.some((p) => p.pad === pad)) this.pads.push(new GamepadInput(pad));
    };
    gamepads.gamepads.forEach(addPad);
    gamepads.on('connected', addPad);
    gamepads.on('disconnected', (pad: Phaser.Input.Gamepad.Gamepad) => this.padLost(pad));
    this.scene.launch('hud');
  }

  update(): void {
    this.keyboard.update();
    this.pads.forEach((p) => p.update());
    this.touch?.update();
    this.trackLastDevice();

    const inputs = this.allInputs();
    if (inputs.some((i) => i.justPressed('fullscreen'))) void toggleFullscreen(); // über die Shell, damit Vollbild beim Seitenwechsel bleibt

    const client = this.client;
    if (client.status === 'lobby' && this.world_) return this.leaveRoom(); // Raum geschlossen: zurück zur Auswahl
    if (client.status !== 'room') return;

    const seated = client.you.map((s) => s.slot);
    this.slots.join(inputs, seated, client, performance.now());
    const p = this.slots.commands(seated);
    if (p.length > 0) client.sendInput(p);
    this.takeFrames();
    this.draw();
  }

  /** Slots, deren Spieler noch auf seinen Beitritt wartet (Taste drücken) */
  waitingForJoin(): boolean {
    return this.slots.waiting(this.client.you.map((s) => s.slot));
  }

  /** Felder, für die das HUD Werte zeigt: Monarchen der lokalen Spieler und ggf. das Info-Feld */
  hudCells(): { cell: Cell; monarch: number | null }[] {
    const seats = [...this.client.you].sort((a, b) => a.slot - b.slot);
    return this.cells.map((cell) => ({ cell, monarch: cell.kind === 'player' ? (seats[cell.seat]?.monarch ?? null) : null }));
  }

  /** Raum verlassen und zurück zur Auswahl. Die Lobby folgt in SP08.3; bis dahin lädt die Seite neu. */
  private leaveRoom(): void {
    this.client.leave();
    window.location.reload();
  }

  private allInputs(): PlayerInput[] {
    return [this.keyboard, ...this.pads, ...(this.touch ? [this.touch] : [])];
  }

  private trackLastDevice(): void {
    const used = (i: PlayerInput) => i.moveX() !== 0 || i.held('confirm');
    if (this.touch && used(this.touch)) this.lastDevice = 'touch';
    else if (this.pads.some(used)) this.lastDevice = 'pad';
    else if (used(this.keyboard)) this.lastDevice = 'keyboard';
  }

  /** Controller getrennt: sein Spieler verlässt den Raum (Monarch wird frei); der letzte Spieler verlässt den Raum ganz. */
  private padLost(pad: Phaser.Input.Gamepad.Gamepad): void {
    const input = this.pads.find((p) => p.pad === pad);
    if (!input) return;
    this.pads = this.pads.filter((p) => p !== input);
    if (this.client.status !== 'room') return;
    if (this.slots.lose(input, this.client.you.map((s) => s.slot), this.client) === 'leave') this.leaveRoom();
  }

  private takeFrames(): void {
    if (this.client.level !== this.level) {
      this.level = this.client.level; // neue Stufe: Darstellung neu aufbauen, nichts mit der alten mischen
      this.world_ = undefined;
      this.prev = this.cur = null;
    }
    for (const frame of this.client.takeFrames()) {
      this.prev = this.cur;
      this.cur = frame;
      this.pendingEvents.push(...frame.state.events);
    }
  }

  private draw(): void {
    const level = this.client.level;
    if (!this.cur || !level) return;
    const alpha = blendAlpha(performance.now(), this.cur.receivedAt, 1000 / this.client.tickHz);
    const state = this.prev ? interpolate(this.prev.state, this.cur.state, alpha) : this.cur.state;
    if (this.world_) applyState(this.world_, state);
    else {
      this.world_ = createViewWorld(level, state);
      this.buildWorldView(this.world_);
    }
    this.renderer_.sync(this.world_);
    const brightness = NIGHT_BRIGHTNESS + (1 - NIGHT_BRIGHTNESS) * daylight(this.world_.cycle);
    for (const fx of this.nightFx) fx.colorMatrix.brightness(brightness);
    this.updateLayout(this.world_);
  }

  /** Nächster Mitspieler eines anderen Geräts; der bisherige bleibt, solange kein anderer deutlich näher ist (kein Flackern). */
  private pickPartner(world: World): number | null {
    const mine = new Set(this.client.you.map((s) => s.monarch));
    const me = world.players.find((p) => mine.has(p.index));
    const others = world.players.filter((p) => !mine.has(p.index));
    if (!me || others.length === 0) return null;
    const dist = (p: { x: number }) => Math.abs(p.x - me.x);
    const nearest = others.reduce((a, b) => (dist(b) < dist(a) ? b : a));
    const current = others.find((p) => p.index === this.partnerMonarch);
    return current && dist(current) - dist(nearest) < PARTNER_SWITCH_UNITS ? current.index : nearest.index;
  }

  /** Kameras neu anordnen, wenn sich Spielerzahl oder Mitspieler ändern. */
  private updateLayout(world: World): void {
    const seats = this.client.you.length;
    this.partnerMonarch = seats === 1 ? this.pickPartner(world) : null;
    const key = `${seats}|${this.partnerMonarch}|${this.client.you.map((s) => s.monarch).join(',')}`;
    if (key === this.layoutKey) return;
    this.layoutKey = key;
    this.cells = computeLayout(seats, this.partnerMonarch !== null);
    this.layoutCameras(world);
  }

  /** Baut alles Sichtbare für die aktuelle Stufe (neu) auf: Hintergrund, Welt-Objekte, Kameras. */
  private buildWorldView(world: World): void {
    this.children.removeAll(true);
    this.cameras.cameras.filter((c) => c !== this.cameras.main).forEach((c) => this.cameras.remove(c));
    this.nightFx = [];

    const widthPx = world.widthUnits * UNIT_PX;
    const palette = world.biome.palette;
    this.drawBackground(world, widthPx);
    this.add.rectangle(0, GROUND_Y, widthPx, GAME_HEIGHT - GROUND_Y, Phaser.Display.Color.HexStringToColor(palette.ground).color).setOrigin(0, 0);
    this.renderer_ = new WorldRenderer(this, world);

    const main = this.cameras.main;
    main.stopFollow();
    main.setViewport(0, 0, GAME_WIDTH, GAME_HEIGHT).setZoom(1);
    main.setBounds(0, 0, widthPx, GAME_HEIGHT);
    main.centerOn(world.hubX * UNIT_PX, GAME_HEIGHT / 2);
    main.setBackgroundColor(palette.sky);
    this.addNightFx(main);
    this.layoutKey = '';
  }

  /** Eine Kamera je Feld; Spieler-Felder folgen ihrem Monarchen, das Feld des Mitspielers dem Partner, Info-Felder haben keine. */
  private layoutCameras(world: World): void {
    const widthPx = world.widthUnits * UNIT_PX;
    this.cameras.cameras.filter((c) => c !== this.cameras.main).forEach((c) => this.cameras.remove(c));
    this.nightFx = this.nightFx.slice(0, 1);
    const seats = [...this.client.you].sort((a, b) => a.slot - b.slot);

    const cams = this.cells.filter((c) => c.kind !== 'info');
    cams.forEach((cell, i) => {
      const cam = i === 0 ? this.cameras.main : this.cameras.add(0, 0, cell.w, cell.h);
      if (i > 0) this.addNightFx(cam);
      cam.setViewport(cell.x, cell.y, cell.w, cell.h);
      cam.setZoom(cell.h / GAME_HEIGHT);
      cam.setBounds(0, 0, widthPx, GAME_HEIGHT);
      cam.setBackgroundColor(world.biome.palette.sky);
      const monarch = cell.kind === 'partner' ? this.partnerMonarch : (seats[cell.seat]?.monarch ?? null);
      const view = monarch === null ? undefined : this.renderer_.playerView(monarch);
      if (view) cam.startFollow(view, true, 0.1, 0.1);
    });
  }

  /** Nacht = Kamera abdunkeln. Nur mit WebGL (Filter), im Canvas-Modus bleibt es hell. */
  private addNightFx(cam: Phaser.Cameras.Scene2D.Camera): void {
    cam.filters.internal.clear(); // die Hauptkamera überlebt einen Szenen-Neustart, sonst stapelt sich die Abdunklung
    this.nightFx.push(cam.filters.internal.addColorMatrix());
  }

  private drawBackground(world: World, widthPx: number): void {
    const palette = world.biome.palette;
    const far = Phaser.Display.Color.HexStringToColor(palette.far).color;
    const near = Phaser.Display.Color.HexStringToColor(palette.near).color;
    // Zwei Parallax-Ebenen mit gezackter Silhouette (Berge / Höhlenwände).
    this.drawRidge(widthPx, far, 0.3, 520, 180);
    this.drawRidge(widthPx, near, 0.6, 700, 120);
  }

  private drawRidge(widthPx: number, color: number, scrollFactor: number, baseY: number, amplitude: number): void {
    const g = this.add.graphics().setScrollFactor(scrollFactor, 1);
    g.fillStyle(color, 1);
    const points = [new Phaser.Math.Vector2(0, GROUND_Y)];
    for (let x = 0; x <= widthPx * scrollFactor + GAME_WIDTH * 2; x += 160) {
      points.push(new Phaser.Math.Vector2(x, baseY - Math.abs(Math.sin(x * 0.0021) + Math.sin(x * 0.0057)) * amplitude * 0.6));
    }
    points.push(new Phaser.Math.Vector2(widthPx * scrollFactor + GAME_WIDTH * 2, GROUND_Y));
    g.fillPoints(points, true);
  }
}
