import Phaser from 'phaser';
import { toggleFullscreen } from '../core/fullscreen';
import { GAME_HEIGHT, GAME_WIDTH, UNIT_PX } from '../core/constants';
import { GamepadInput, KeyboardInput, type PlayerInput } from '../input/playerInput';
import { TouchInput, wantsTouchControls } from '../input/touchInput';
import type { LevelInfo, RoomClient } from '../online/clientConnection';
import { applyState, createViewWorld } from '../online/clientWorld';
import { blendAlpha, interpolate } from '../online/clientInterpolation';
import type { Frame } from '../online/clientConnection';
import type { GameEvent, World } from '../model/types';
import { computeLayout, type Cell } from './layout';
import type { LobbySceneData } from './LobbyScene';
import { leavesGame } from './lobbyLogic';
import { LocalSlots } from './localSlots';
import type { RadarCell } from './radarView';
import { daylight } from './viewRules';
import { cellStages } from './cellStages';
import { PLACEHOLDER_BG, StageView, placeholderLayer, showOnly } from './stageView';
import { DEV_FOCUS_KEY, muteFocused } from './debugOverlayPanel';
import { MenuPress, idleCommands } from './optionsLogic';
import { pauseButton } from './pauseButton';

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
  /** Geladene Stufen nach Tiefe; mit dem heutigen Protokoll (B-176 offen) nur die Stufe von `client.level` */
  private stages = new Map<number, StageView>();
  private holders: Phaser.GameObjects.Layer[] = [];
  private keyboard!: KeyboardInput;
  private touch: TouchInput | undefined;
  private pads: GamepadInput[] = [];
  private nightFx: Phaser.Filters.ColorMatrix[] = [];
  private slots!: LocalSlots<PlayerInput>;
  private level: LevelInfo | null = null;
  private prev: Frame | null = null;
  private cur: Frame | null = null;
  private cells: Cell[] = [];
  private layoutKey = '';
  private partnerMonarch: number | null = null;
  private menuPress = new MenuPress();

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
    this.stages = new Map(); // die Szene hat ihre Ebenen beim Neustart schon zerstört
    this.holders = [];
    this.level = null;
    this.prev = this.cur = null;
    this.layoutKey = '';
    this.partnerMonarch = null;
    this.menuPress = new MenuPress();
    this.lastDevice = wantsTouchControls() ? 'touch' : 'keyboard';
  }

  create(): void {
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
    if (this.touch) {
      pauseButton().show(true);
      this.events.once('shutdown', () => pauseButton().show(false));
    }
  }

  update(): void {
    this.keyboard.update();
    this.pads.forEach((p) => p.update());
    this.touch?.update();
    this.trackLastDevice();

    const inputs = this.allInputs();
    if (inputs.some((i) => i.justPressed('fullscreen'))) void toggleFullscreen(); // über die Shell, damit Vollbild beim Seitenwechsel bleibt

    const client = this.client;
    if (leavesGame(client.status)) return this.leaveRoom(); // Raum geschlossen, 120 s ohne Verbindung, an anderer Stelle geöffnet: zurück zur Lobby
    if (client.status !== 'room') return;

    const seated = client.you.map((s) => s.slot);
    if (this.scene.isActive('options')) return this.paused(seated); // Optionen offen: Monarchen stehen, nur zeichnen
    if (this.wantsOptions()) this.scene.launch('options');
    const devFocus = this.registry.get(DEV_FOCUS_KEY) === true; // Dev-Fokus im Debug-Overlay (B-179): Controller bedienen die Liste
    const isPad = (i: PlayerInput | null) => this.pads.includes(i as GamepadInput);
    this.slots.join(devFocus ? inputs.filter((i) => !isPad(i)) : inputs, seated, client, performance.now());
    const p = muteFocused(this.slots.commands(seated), devFocus, (s) => isPad(this.slots.bound[s] ?? null));
    if (p.length > 0) client.sendInput(p);
    this.takeFrames();
    this.draw();
  }

  /** Optionen offen (Client-Anteil der Pause, S5.2): die Monarchen des Geräts stehen, Beitritt und Eingaben ruhen, das Spiel wird weiter gezeichnet. */
  private paused(seated: number[]): void {
    this.client.sendInput(idleCommands(seated));
    this.takeFrames();
    this.draw();
  }

  /** Esc, Touch-Schaltfläche oder Menu kurz (Pad, beim Loslassen; View + Menu bleibt „zurück zur Landingpage“) */
  private wantsOptions(): boolean {
    const menuShort = this.menuPress.update(this.pads.some((p) => p.held('pause')), this.pads.some((p) => p.held('skillMenu')), performance.now());
    return this.keyboard.justPressed('pause') || (this.touch !== undefined && pauseButton().take()) || menuShort;
  }

  /** Slots, deren Spieler noch auf seinen Beitritt wartet (Taste drücken) */
  waitingForJoin(): boolean {
    return this.slots.waiting(this.client.you.map((s) => s.slot));
  }

  /** Felder, für die das HUD Werte zeigt: Monarchen der lokalen Spieler, dazu der sichtbare Ausschnitt der Kamera (Radar) */
  hudCells(): RadarCell[] {
    const seats = [...this.client.you].sort((a, b) => a.slot - b.slot);
    const current = this.world_;
    const plan = cellStages(this.cells, seats, this.partnerMonarch === null || !current ? null : current.biome.depth, new Set(this.stages.keys()));
    return this.cells.map((cell, i) => {
      const view = this.cameras.cameras[i]?.worldView; // Kamera i gehört zu Feld i (layoutCameras)
      const depth = plan[i]?.depth ?? null;
      return {
        cell,
        monarch: cell.kind === 'player' ? (seats[cell.seat]?.monarch ?? null) : null,
        view: view ? { fromUnits: view.x / UNIT_PX, spanUnits: view.width / UNIT_PX } : null,
        depth,
        // Welt der Stufe dieser Zelle; mit dem heutigen Protokoll (B-176) ist nur die Stufe von `client.level` geladen
        world: plan[i]?.ready && current?.biome.depth === depth ? current : null,
      };
    });
  }

  /** Raum verlassen und zurück zur Lobby (die Hinweise zum Grund zeigt sie selbst) */
  private leaveRoom(): void {
    this.client.leave();
    this.scene.stop('hud');
    this.scene.stop('options');
    this.scene.start('lobby', { client: this.client, returned: true } satisfies LobbySceneData);
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
      if (this.level) this.dropStage(this.level.depth); // neue Stufe: nur deren Einheit neu aufbauen, nichts mit der alten mischen
      this.level = this.client.level;
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
      this.loadStage(this.world_);
    }
    this.stages.get(level.depth)?.renderer.sync(this.world_);
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
    const key = `${seats}|${this.partnerMonarch}|${this.client.you.map((s) => `${s.monarch}:${s.depth}`).join(',')}|${[...this.stages.keys()].join(',')}`;
    if (key === this.layoutKey) return;
    this.layoutKey = key;
    this.cells = computeLayout(seats, this.partnerMonarch !== null);
    this.layoutCameras(world);
  }

  /** Baut die Einheit der Stufe auf (Hintergrund, Welt-Objekte); die Kameras folgen im nächsten Layout. */
  private loadStage(world: World): void {
    this.dropStage(world.biome.depth);
    this.stages.set(world.biome.depth, new StageView(this, world));
    this.layoutKey = '';
  }

  private dropStage(depth: number): void {
    this.stages.get(depth)?.destroy();
    this.stages.delete(depth);
    this.layoutKey = '';
  }

  /** Eine Kamera je Feld, jede zeigt die Stufe ihres Spielers (`cellStages`); Spieler-Felder folgen ihrem Monarchen, das Feld des Mitspielers dem Partner. */
  private layoutCameras(world: World): void {
    const main = this.cameras.main;
    this.cameras.cameras.filter((c) => c !== main).forEach((c) => this.cameras.remove(c));
    this.nightFx = [];
    this.holders.forEach((h) => h.destroy());
    this.holders = [];
    const seats = [...this.client.you].sort((a, b) => a.slot - b.slot);
    const plan = cellStages(this.cells, seats, this.partnerMonarch === null ? null : world.biome.depth, new Set(this.stages.keys()));
    const own: Phaser.GameObjects.Layer[] = [];

    this.cells.forEach((cell, i) => {
      const cam = i === 0 ? main : this.cameras.add(0, 0, cell.w, cell.h);
      this.addNightFx(cam);
      cam.setViewport(cell.x, cell.y, cell.w, cell.h);
      cam.setZoom(cell.h / GAME_HEIGHT);
      cam.stopFollow();
      const stage = plan[i]!.ready ? this.stages.get(plan[i]!.depth!) : undefined;
      if (stage) {
        const monarch = cell.kind === 'partner' ? this.partnerMonarch : (seats[cell.seat]?.monarch ?? null);
        this.aimAtStage(cam, stage, monarch);
        own.push(stage.layer);
      } else {
        const holder = placeholderLayer(this, plan[i]!.depth); // Stufe noch nicht geladen: Platzhalter statt leerer Kamera
        this.holders.push(holder);
        cam.removeBounds().centerOn(GAME_WIDTH / 2, GAME_HEIGHT / 2).setBackgroundColor(PLACEHOLDER_BG);
        own.push(holder);
      }
    });
    showOnly(this.cameras.cameras, own, [...[...this.stages.values()].map((s) => s.layer), ...this.holders]);
  }

  private aimAtStage(cam: Phaser.Cameras.Scene2D.Camera, stage: StageView, monarch: number | null): void {
    const { world } = stage;
    cam.setBounds(0, 0, world.widthUnits * UNIT_PX, GAME_HEIGHT);
    cam.setBackgroundColor(world.biome.palette.sky);
    cam.centerOn(world.hubX * UNIT_PX, GAME_HEIGHT / 2);
    const view = monarch === null ? undefined : stage.renderer.playerView(monarch);
    if (view) cam.startFollow(view, true, 0.1, 0.1);
  }

  /** Nacht = Kamera abdunkeln. Nur mit WebGL (Filter), im Canvas-Modus bleibt es hell. */
  private addNightFx(cam: Phaser.Cameras.Scene2D.Camera): void {
    cam.filters.internal.clear(); // die Hauptkamera überlebt einen Szenen-Neustart, sonst stapelt sich die Abdunklung
    this.nightFx.push(cam.filters.internal.addColorMatrix());
  }
}
