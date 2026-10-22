import Phaser from 'phaser';
import { toggleFullscreen } from '../core/fullscreen';
import { GAME_HEIGHT, GAME_WIDTH, UNIT_PX } from '../core/constants';
import { GamepadInput, keyboardPlayers, type KeyboardInput, type PlayerInput } from '../input/playerInput';
import { TouchInput, wantsTouchControls } from '../input/touchInput';
import type { Frame, LevelInfo, RoomClient } from '../online/clientConnection';
import { applyState, createViewWorld } from '../online/clientWorld';
import { Timeline } from '../online/clientTimeline';
import { Predictor } from '../online/clientPredict';
import type { SlotInput } from '../online/clientProtocol';
import type { GameEvent, World } from '../model/types';
import { computeLayout, viewZoom, WORLD_TOP, type Cell } from './layout';
import type { LobbySceneData } from './LobbyScene';
import { leavesGame } from './lobbyLogic';
import { LEAVE_KEY, MenuPress, REMOVE_PLAYER_KEY, idleCommands, type OptionsSceneData } from './optionsLogic';
import { findOpener, removeTargets, type OpenSignal } from './menuOpener';
import { LocalSlots, pageBots } from './localSlots';
import type { RadarCell } from './hud/radarView';
import { daylight } from './render/viewRules';
import { refollow } from './render/cameraFollow';
import type { JoinCandidate } from './hud/joinHint';
import { cellStages } from './cellStages';
import { PLACEHOLDER_BG, StageView, placeholderLayer, showOnly } from './render/stageView';
import { clientLog } from '../core/clientLog';
import { audioCore } from '../audio/audioCore';
import { musicDirector, musicViewOf } from '../audio/music';
import { eventX, type Listener } from '../audio/events';
import { loadSettings } from '../core/settings';
import { EFFECT_CONFIG, SHAKE, effectFor, type BuildSpots } from './effects/effects';
import { hurtSeat, rumblePad, runEffect, shakeCell } from './effects/effectRules';
import { DEV_FOCUS_KEY, muteFocused } from './debug/debugOverlayPanel';
import { pauseButton } from './hud/pauseButton';
import { SkillMenus } from './hud/skillMenuLogic';

/** Ein Overlay pro Seite, auch über Szenen-Neustarts hinweg */
let sharedTouch: TouchInput | undefined;
const touchControls = (): TouchInput => (sharedTouch ??= new TouchInput());

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
  lastDevice: 'keyboard' | 'keyboard2' | 'pad' | 'touch' = 'keyboard';

  private data_!: GameSceneData;
  private world_: World | undefined;
  /** Geladene Stufen nach Tiefe; mit dem heutigen Protokoll (B-176 offen) nur die Stufe von `client.level` */
  private stages = new Map<number, StageView>();
  private readonly buildSpots: BuildSpots = new Map();
  private lastFlashAt: number | null = null;
  private rumbleWarned = false;
  private holders: Phaser.GameObjects.Layer[] = [];
  /** Kamera-ID der Mitspieler-Zelle: dort sind die Texte der Stufe ausgeblendet (B-197), 0 = keine */
  private partnerCamId = 0;
  /** Kameras mit Figur-Ziel; die Nachführung holt eine später entstehende View nach (B-392). */
  private follows: { cam: Phaser.Cameras.Scene2D.Camera; stage: StageView; monarch: number; target?: object }[] = [];
  keyboards!: [KeyboardInput, KeyboardInput]; // Spieler 1 und 2 an einer Tastatur (B-316)
  private touch: TouchInput | undefined;
  private pads: GamepadInput[] = [];
  private nightFx: Phaser.Filters.ColorMatrix[] = [];
  /** Eingabe je lokalem Slot (die HudScene liest das Gerät für die Tasten der Skill-Leiste) */
  slots!: LocalSlots<PlayerInput>;
  private level: LevelInfo | null = null;
  /** Gezeichnete Zeitleiste (B-277); `cur` ist der neueste empfangene Frame (Effekte) */
  private timeline!: Timeline;
  /** Anzeige-Vorhersage der lokalen Monarchen (B-277) aus den zuletzt gesendeten Eingaben `moves` */
  private predictor!: Predictor;
  private moves: SlotInput[] = [];
  private cur: Frame | null = null;
  private cells: Cell[] = [];
  private layoutKey = '';
  private partnerMonarch: number | null = null;
  private padMenus = new Map<GamepadInput, MenuPress>();
  /** Skill-Menüs der lokalen Spieler (S3.2); die HudScene zeichnet sie */
  skillMenus = new SkillMenus();

  constructor() {
    super('game');
  }

  get world(): World | undefined {
    return this.world_;
  }

  get client(): RoomClient {
    return this.data_.client;
  }

  /** Verzögerung der Zeitleiste in ms (Debug-Overlay) */
  get delayMs(): number {
    return this.timeline.delayMs;
  }

  init(data: GameSceneData): void {
    this.data_ = data;
    this.pendingEvents.length = 0;
    this.pads = [];
    this.nightFx = [];
    this.touch = undefined;
    this.slots = new LocalSlots<PlayerInput>(data.mock, pageBots()); // Bots (`?botfeed`) sind fest an ihre Slots gebunden
    this.world_ = undefined;
    musicDirector().reset();
    audioCore().onHubLevel(undefined); // neues Spiel: Hub-Stufe vergessen, kein Ausbau-Ton beim ersten Snapshot
    this.stages = new Map(); // die Szene hat ihre Ebenen beim Neustart schon zerstört
    this.holders = [];
    this.buildSpots.clear();
    this.level = null;
    this.timeline = new Timeline(1000 / data.client.tickHz);
    this.predictor = new Predictor(1000 / data.client.tickHz);
    this.moves = [];
    this.cur = null;
    this.layoutKey = '';
    this.partnerMonarch = null;
    this.padMenus = new Map();
    this.skillMenus = new SkillMenus();
    this.lastDevice = wantsTouchControls() ? 'touch' : 'keyboard';
  }

  create(): void {
    this.keyboards = keyboardPlayers(this.input.keyboard!);
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
    [...this.keyboards, ...this.pads, ...pageBots()].forEach((p) => p.update());
    this.touch?.update();
    this.trackLastDevice();

    const inputs = this.allInputs();
    if (inputs.some((i) => i.justPressed('fullscreen'))) void toggleFullscreen(); // über die Shell, damit Vollbild beim Seitenwechsel bleibt

    const client = this.client;
    if (leavesGame(client.status)) return this.leaveRoom(); // Raum geschlossen, 120 s ohne Verbindung, an anderer Stelle geöffnet: zurück zur Lobby
    if (client.status !== 'room') return;
    client.setPaused(this.scene.isActive('options')); // Pause-Menü offen = Gerät am Server pausiert (BD7.3)

    if (this.registry.get(LEAVE_KEY) === true) return this.leaveRoom(); // „Spiel verlassen“ im Menü (B-293)
    const seated = client.you.map((s) => s.slot);
    const removing = this.registry.get(REMOVE_PLAYER_KEY);
    if (typeof removing === 'number') {
      this.registry.set(REMOVE_PLAYER_KEY, null); // „Spieler abmelden“ im Menü (B-411): genau der gewählte Slot
      if (this.slots.removeAt(removing, seated, client) === 'leave') return this.leaveRoom(); // letzter lokaler Spieler: wie „Spiel verlassen“
    }
    if (this.scene.isActive('options')) return this.paused(seated); // Optionen offen: Monarchen stehen, nur zeichnen
    const opener = this.optionsOpener();
    if (opener) {
      const targets = removeTargets(opener.input, this.slots.bound, this.keyboards, seated);
      this.scene.launch('options', { targets, world: client.roomWorld } satisfies OptionsSceneData);
    }
    const devFocus = this.registry.get(DEV_FOCUS_KEY) === true; // Cheat-Dialog offen (B-179, B-317): Controller und Tastatur bedienen ihn
    this.slots.join(devFocus ? [] : inputs, seated, client, performance.now());
    const player = (slot: number) => this.world_?.players.find((q) => q.index === client.you.find((s) => s.slot === slot)?.monarch);
    // Dialog offen: ohne Eingaben schließen die Skill-Menüs, Leertaste/A im Dialog lernt keinen Skill (Review U5.3)
    const p = this.skillMenus.route(muteFocused(this.slots.commands(seated), devFocus), devFocus ? [] : this.slots.bound, player, client);
    this.moves = p;
    if (p.length > 0) client.sendInput(p);
    this.takeFrames();
    this.draw();
  }

  /** Optionen offen (Client-Anteil der Pause, S5.2): die Monarchen des Geräts stehen, Beitritt und Eingaben ruhen, das Spiel wird weiter gezeichnet. */
  private paused(seated: number[]): void {
    this.moves = idleCommands(seated);
    this.client.sendInput(this.moves);
    this.takeFrames();
    this.draw();
  }

  /** Esc, Touch-Schaltfläche oder Menu kurz (Pad, beim Loslassen; View + Menu bleibt „zurück zur Landingpage“) */
  private optionsOpener(): { input: PlayerInput } | null {
    const now = performance.now();
    const signals: OpenSignal<PlayerInput>[] = [
      { input: this.keyboards[0], opens: this.keyboards[0].justPressed('pause') },
      ...(this.touch ? [{ input: this.touch, opens: pauseButton().take() }] : []),
      ...this.pads.map((p) => {
        let press = this.padMenus.get(p);
        if (!press) this.padMenus.set(p, (press = new MenuPress())); // je Pad, damit der Öffner bekannt ist
        return { input: p, opens: press.update(p.held('pause'), p.viewHeld(), now) };
      }),
    ];
    return findOpener(signals);
  }

  /** Slots, deren Spieler noch auf seinen Beitritt wartet (Taste drücken) */
  waitingForJoin(): boolean {
    return this.slots.waiting(this.client.you.map((s) => s.slot));
  }

  /** Geräte, die beitreten können, und ob sie schon einen Spieler steuern (Beitritts-Hinweis, B-391) */
  joinCandidates(): JoinCandidate[] {
    const bound = (i: PlayerInput) => this.slots.bound.includes(i);
    return [
      { device: 'keyboard', bound: bound(this.keyboards[0]) },
      { device: 'keyboard2', bound: bound(this.keyboards[1]) },
      ...this.pads.map((p): JoinCandidate => ({ device: 'pad', bound: bound(p) })),
      ...(this.touch ? [{ device: 'touch', bound: bound(this.touch) } as JoinCandidate] : []),
    ];
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
    this.registry.set(LEAVE_KEY, false);
    this.client.leave();
    this.scene.stop('hud');
    this.scene.stop('options');
    this.scene.start('lobby', { client: this.client, returned: true } satisfies LobbySceneData);
  }

  private allInputs(): PlayerInput[] {
    return [...this.keyboards, ...this.pads, ...(this.touch ? [this.touch] : [])];
  }

  private trackLastDevice(): void {
    const used = (i: PlayerInput) => i.moveX() !== 0 || i.held('confirm') || i.held('attack');
    if (this.allInputs().some(used)) void audioCore().onInput(); // erste Eingabe entsperrt den Ton (SO1.2)
    if (this.touch && used(this.touch)) this.lastDevice = 'touch';
    else if (this.pads.some(used)) this.lastDevice = 'pad';
    else if (this.keyboards.some(used)) this.lastDevice = used(this.keyboards[1]) ? 'keyboard2' : 'keyboard';
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
      this.timeline = new Timeline(1000 / this.client.tickHz); // Puffer leeren
      this.predictor = new Predictor(1000 / this.client.tickHz);
      this.cur = null;
    }
    for (const frame of this.client.takeFrames()) {
      this.timeline.push(frame);
      this.predictor.observe(frame, this.client.you);
      this.cur = frame;
      this.pendingEvents.push(...frame.state.events);
      if (this.world_) musicDirector().update(musicViewOf(this.world_), frame.state.events); // Musik: Ereignisse wie castleFallen
      audioCore().onHubLevel(this.world_?.hubLevel); // Hub-Ausbau-Ton (SO6.4)
      this.spawnEffects(frame.state.events);
    }
  }

  /** Effekte nur aus den Ereignissen des Frames (nicht aus `pendingEvents`, das die HudScene leert); ändert keinen Zustand. */
  private spawnEffects(events: readonly GameEvent[]): void {
    const view = this.level ? this.stages.get(this.level.depth) : undefined;
    if (!view) return;
    view.renderer.noteEvents(events); // Wimpel-Farbe: wer markiert (GEB10)
    const settings = loadSettings(); // je Frame gelesen: eine Änderung in den Optionen wirkt ohne Neuladen
    const playerX = (i: number): number | undefined => this.cur?.state.players.find((p) => p.index === i)?.x;
    const now = performance.now();
    for (const e of events) {
      this.feedback(e, settings.screenshake);
      const fx = effectFor(e, playerX, this.buildSpots);
      audioCore().onEvent(e, eventX(e, fx ? fx.x / UNIT_PX : undefined), this.listeners(), this.world_?.cycle.phase);
      if (!fx) continue;
      const isFlash = EFFECT_CONFIG[fx.kind].flash;
      if (!runEffect(isFlash, settings, now, this.lastFlashAt)) continue;
      if (isFlash) this.lastFlashAt = now;
      view.spawn(fx);
    }
  }

  /** Sichtbare Ausschnitte der lokalen Spieler für die Positions-Dämpfung des Tons (Mitte und Breite in Units). */
  private listeners(): Listener[] {
    return this.hudCells().flatMap((c) => (c.monarch !== null && c.view ? [{ center: c.view.fromUnits + c.view.spanUnits / 2, span: c.view.spanUnits }] : []));
  }

  /** Treffer an einem lokalen Spieler: nur dessen Kamera schüttelt (Schalter `screenshake`), nur dessen Controller vibriert. */
  private feedback(e: GameEvent, shake: boolean): void {
    const seat = hurtSeat(e, this.client.you);
    if (seat === null) return;
    if (shake) {
      const cam = this.cameras.cameras[shakeCell(e, this.client.you, this.cells) ?? -1];
      cam?.shake(SHAKE.durationMs, SHAKE.intensity);
    }
    const slot = [...this.client.you].sort((a, b) => a.slot - b.slot)[seat]?.slot ?? -1;
    const input = this.slots.bound[slot];
    if (input instanceof GamepadInput) {
      rumblePad(input.pad.vibration, () => {
        if (this.rumbleWarned) return;
        this.rumbleWarned = true;
        clientLog('info', '🎮 Controller ohne Vibration');
      });
    }
  }

  private draw(): void {
    const level = this.client.level;
    const now = performance.now();
    const sampled = this.timeline.sample(now);
    if (!sampled || !level) return;
    const state = this.predictor.draw(sampled, now, this.moves, this.client.you, this.client.latency?.mean ?? null); // lokale Monarchen sofort (B-277)
    if (this.world_) applyState(this.world_, state);
    else {
      this.world_ = createViewWorld(level, state);
      this.loadStage(this.world_);
    }
    musicDirector().update(musicViewOf(this.world_));
    const unattended = new Set<number>();
    this.client.monarchs.forEach((m, i) => m !== 'taken' && unattended.add(i));
    const stage = this.stages.get(level.depth);
    stage?.renderer.sync(this.world_, unattended);
    // Schrift in der Welt: Zoom der kleinsten Zelle ausgleichen, damit sie nie unter ihre Größe fällt; die Mitspieler-Zelle zählt nicht (Texte dort ausgeblendet, B-197)
    const textCells = this.cells.filter((c) => c.kind !== 'partner');
    if (stage && textCells.length > 0) stage.scaleTexts(1 / Math.min(...textCells.map((c) => viewZoom(c.h))), this.partnerCamId);
    const brightness = NIGHT_BRIGHTNESS + (1 - NIGHT_BRIGHTNESS) * daylight(this.world_.cycle);
    for (const fx of this.nightFx) fx.colorMatrix.brightness(brightness);
    this.updateLayout(this.world_);
    this.follows.forEach((f) => (f.target = refollow(f.cam, f.target, f.stage.renderer.playerView(f.monarch))));
  }

  /** Nächster Mitspieler eines anderen Geräts; der bisherige bleibt, solange kein anderer deutlich näher ist (kein Flackern). */
  private pickPartner(world: World): number | null {
    const mine = new Set(this.client.you.map((s) => s.monarch));
    const me = world.players.find((p) => mine.has(p.index));
    // nur Monarchen mit Spieler: freie und wartende (abgemeldet, getrennt) sind ausgeblendet und kein Partner (B-411)
    const taken = (i: number) => (this.client.monarchs[i] ?? 'taken') === 'taken';
    const others = world.players.filter((p) => !mine.has(p.index) && taken(p.index));
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
    this.follows = [];
    this.holders.forEach((h) => h.destroy());
    this.holders = [];
    const seats = [...this.client.you].sort((a, b) => a.slot - b.slot);
    const plan = cellStages(this.cells, seats, this.partnerMonarch === null ? null : world.biome.depth, new Set(this.stages.keys()));
    const own: Phaser.GameObjects.Layer[] = [];
    this.partnerCamId = 0;

    this.cells.forEach((cell, i) => {
      const cam = i === 0 ? main : this.cameras.add(0, 0, cell.w, cell.h);
      if (cell.kind === 'partner') this.partnerCamId = cam.id;
      this.addNightFx(cam);
      cam.setViewport(cell.x, cell.y, cell.w, cell.h);
      cam.setZoom(viewZoom(cell.h));
      cam.stopFollow();
      const stage = plan[i]!.ready ? this.stages.get(plan[i]!.depth!) : undefined;
      if (stage) {
        const monarch = cell.kind === 'partner' ? this.partnerMonarch : (seats[cell.seat]?.monarch ?? null);
        this.aimAtStage(cam, stage, monarch);
        own.push(stage.layer);
      } else {
        const holder = placeholderLayer(this, plan[i]!.depth, viewZoom(cell.h)); // Stufe noch nicht geladen: Platzhalter statt leerer Kamera
        this.holders.push(holder);
        cam.removeBounds().centerOn(GAME_WIDTH / 2, GAME_HEIGHT / 2).setBackgroundColor(PLACEHOLDER_BG);
        own.push(holder);
      }
    });
    showOnly(this.cameras.cameras, own, [...[...this.stages.values()].map((s) => s.layer), ...this.holders]);
  }

  private aimAtStage(cam: Phaser.Cameras.Scene2D.Camera, stage: StageView, monarch: number | null): void {
    const { world } = stage;
    cam.setBounds(0, WORLD_TOP, world.widthUnits * UNIT_PX, GAME_HEIGHT - WORLD_TOP); // ohne Himmel
    cam.setBackgroundColor(world.biome.palette.sky);
    cam.centerOn(world.hubX * UNIT_PX, (WORLD_TOP + GAME_HEIGHT) / 2);
    if (monarch === null) return;
    const view = stage.renderer.playerView(monarch);
    if (view) cam.startFollow(view, true, 0.1, 0.1);
    this.follows.push({ cam, stage, monarch, target: view });
  }

  /** Nacht = Kamera abdunkeln. Nur mit WebGL (Filter), im Canvas-Modus bleibt es hell. */
  private addNightFx(cam: Phaser.Cameras.Scene2D.Camera): void {
    cam.filters.internal.clear(); // die Hauptkamera überlebt einen Szenen-Neustart, sonst stapelt sich die Abdunklung
    this.nightFx.push(cam.filters.internal.addColorMatrix());
  }
}
