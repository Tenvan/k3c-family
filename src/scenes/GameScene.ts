import Phaser from 'phaser';
import { toggleFullscreen } from '../core/fullscreen';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, MAX_PLAYERS, UNIT_PX } from '../core/constants';
import { GamepadInput, KeyboardInput, type PlayerInput } from '../input/playerInput';
import { TouchInput, wantsTouchControls } from '../input/touchInput';
import type { OnlineClient } from '../online/client';
import { applySnapshot } from '../online/protocol';
import { biomeForDepth } from '../world/biome';
import { daylight } from '../world/sim/cycle';
import { giveGold } from '../world/sim/economy';
import type { GameEvent, PlayerCommand, World } from '../world/sim/types';
import { addPlayer, createWorld, step } from '../world/sim/world';
import { WorldRenderer } from './worldRenderer';

/** Ein Overlay pro Seite, auch über Szenen-Neustarts hinweg (N/1/2/3) */
let sharedTouch: TouchInput | undefined;
function touchControls(): TouchInput {
  return (sharedTouch ??= new TouchInput());
}

export interface GameSceneData {
  depth: number;
  seed: string;
  /** Tag/Nacht 8x schneller (?fast=1) */
  fast?: boolean;
  /** Dev-Tasten aktiv (Dev-Server oder ?dev=1) */
  dev?: boolean;
  /** Online-Modus: Der Server rechnet, diese Szene sendet nur Eingaben und zeichnet */
  online?: OnlineClient;
}

const FAST_CYCLE = 8;
/** Helligkeit in tiefster Nacht */
const NIGHT_BRIGHTNESS = 0.45;

/**
 * Spielwelt: rechnet die Simulation (src/world/sim) und zeichnet sie über den WorldRenderer.
 * Couch-Koop: Beitritt per A / Leertaste, jeder Spieler bekommt einen eigenen Split-Screen-Streifen.
 */
export class GameScene extends Phaser.Scene {
  world!: World;
  /** Eingabe pro Spieler (Index = world.players-Index) */
  readonly controls: PlayerInput[] = [];
  /** Ereignisse für die HudScene, die sie abholt und leert */
  readonly pendingEvents: GameEvent[] = [];

  private data_!: GameSceneData;
  /** Online: Kamera folgt schon dem eigenen Monarchen */
  private onlineCameraReady = false;
  private renderer_!: WorldRenderer;
  private keyboard!: KeyboardInput;
  private touch: TouchInput | undefined;
  /** Zuletzt benutztes Eingabegerät, damit die Hinweise im HUD zur Steuerung passen */
  lastDevice: 'keyboard' | 'pad' | 'touch' = 'keyboard';
  private pads: GamepadInput[] = [];
  private nightFx: Phaser.Filters.ColorMatrix[] = [];

  constructor() {
    super('game');
  }

  init(data: GameSceneData): void {
    this.data_ = data;
    this.world = createWorld(biomeForDepth(data.depth), data.seed, { cycleSpeed: data.fast ? FAST_CYCLE : 1 });
    this.controls.length = 0;
    this.pendingEvents.length = 0;
    this.pads = [];
    this.nightFx = [];
    this.touch = undefined;
    this.onlineCameraReady = false;
    this.lastDevice = wantsTouchControls() ? 'touch' : 'keyboard';
  }

  create(): void {
    const widthPx = this.world.widthUnits * UNIT_PX;
    const palette = this.world.biome.palette;
    this.drawBackground(widthPx);
    this.add.rectangle(0, GROUND_Y, widthPx, GAME_HEIGHT - GROUND_Y, Phaser.Display.Color.HexStringToColor(palette.ground).color).setOrigin(0, 0);
    this.renderer_ = new WorldRenderer(this, this.world);

    this.cameras.main.setBounds(0, 0, widthPx, GAME_HEIGHT);
    this.cameras.main.centerOn(this.world.hubX * UNIT_PX, GAME_HEIGHT / 2);
    this.cameras.main.setBackgroundColor(palette.sky);
    this.addNightFx(this.cameras.main);

    this.keyboard = new KeyboardInput(this.input.keyboard!);
    if (wantsTouchControls()) this.touch = touchControls();
    // Browser melden Gamepads erst nach dem ersten Tastendruck. Alle bekannten + neue Pads beobachten.
    const gamepads = this.input.gamepad!;
    const addPad = (pad: Phaser.Input.Gamepad.Gamepad) => {
      if (pad && !this.pads.some((p) => p.pad === pad)) this.pads.push(new GamepadInput(pad));
    };
    gamepads.gamepads.forEach(addPad);
    gamepads.on('connected', addPad);

    const kb = this.input.keyboard!;
    // Dev-Hilfe: N = neues Level mit zufälligem Seed, 1/2/3 = Tiefe wechseln.
    if (!this.data_.online) {
    kb.on('keydown-N', () => this.restartWith(this.world.biome.depth, Math.random().toString(36).slice(2, 8)));
    kb.on('keydown-ONE', () => this.restartWith(0, this.world.seed));
    kb.on('keydown-TWO', () => this.restartWith(1, this.world.seed));
    kb.on('keydown-THREE', () => this.restartWith(2, this.world.seed));
    }
    if (this.data_.dev && !this.data_.online) {
      // G = +10 Gold, H = +50 Baumaterial, T = zur nächsten Tageszeit springen
      kb.on('keydown-G', () => this.world.players.forEach((p) => giveGold(this.world, p, 10)));
      kb.on('keydown-H', () => (['wood', 'stone', 'copper'] as const).forEach((r) => (this.world.stock[r] += 50)));
      kb.on('keydown-T', () => (this.world.time += this.world.cycle.secondsLeft / this.world.cycleSpeed + 0.01));
    }

    this.scene.launch('hud');
  }

  update(_time: number, deltaMs: number): void {
    const dt = Math.min(deltaMs / 1000, 0.1);
    this.keyboard.update();
    this.pads.forEach((p) => p.update());
    this.touch?.update();

    this.trackLastDevice();

    const all = this.allInputs();
    if (all.some((i) => i.justPressed('fullscreen'))) void toggleFullscreen(); // über die Shell, damit Vollbild beim Seitenwechsel bleibt

    if (this.data_.online) this.updateOnline(all);
    else {
      this.handleJoin();
      const commands: PlayerCommand[] = this.controls.map((c) => ({ moveX: c.moveX(), sprint: c.sprint(), pay: c.held('confirm') }));
      step(this.world, commands, dt);
      this.pendingEvents.push(...this.world.events);
      this.renderer_.sync(this.world);
    }

    const brightness = NIGHT_BRIGHTNESS + (1 - NIGHT_BRIGHTNESS) * daylight(this.world.cycle);
    for (const fx of this.nightFx) fx.colorMatrix.brightness(brightness);
  }

  /** Online: Eingabe des lokalen Geräts an den Server, Zustand vom Server zeichnen. Alle lokalen Geräte steuern denselben Monarchen. */
  private updateOnline(inputs: PlayerInput[]): void {
    const client = this.data_.online!;
    client.sendInput({
      moveX: Math.max(-1, Math.min(1, inputs.reduce((sum, i) => sum + i.moveX(), 0))),
      sprint: inputs.some((i) => i.sprint()),
      pay: inputs.some((i) => i.held('confirm')),
    });
    const snapshot = client.takeState();
    if (!snapshot) return;
    applySnapshot(this.world, snapshot);
    this.pendingEvents.push(...snapshot.events);
    this.renderer_.sync(this.world);
    if (!this.onlineCameraReady && this.renderer_.playerView(client.you)) {
      this.onlineCameraReady = true;
      this.layoutCameras([client.you]);
    }
  }

  /** Welche Monarchen dieses Fenster zeigt (Split-Screen: alle lokalen, online: nur der eigene) */
  hudPlayers(): number[] {
    return this.data_.online ? [this.data_.online.you] : this.world.players.map((_, i) => i);
  }

  get online(): OnlineClient | undefined {
    return this.data_.online;
  }

  private trackLastDevice(): void {
    const used = (i: PlayerInput) => i.moveX() !== 0 || i.held('confirm');
    if (this.touch && used(this.touch)) this.lastDevice = 'touch';
    else if (this.pads.some(used)) this.lastDevice = 'pad';
    else if (used(this.keyboard)) this.lastDevice = 'keyboard';
  }

  private allInputs(): PlayerInput[] {
    return [this.keyboard, ...this.pads, ...(this.touch ? [this.touch] : [])];
  }

  private handleJoin(): void {
    if (this.controls.length >= MAX_PLAYERS) return;
    for (const input of this.allInputs()) {
      if (this.controls.includes(input) || !input.justPressed('confirm')) continue;
      addPlayer(this.world);
      this.controls.push(input);
      this.renderer_.sync(this.world);
      this.layoutCameras();
      if (this.controls.length >= MAX_PLAYERS) return;
    }
  }

  /** 1 Spieler: Vollbild. 2+ Spieler: horizontale Streifen übereinander (K2C-Stil). */
  private layoutCameras(indices: number[] = this.world.players.map((_, i) => i)): void {
    const n = indices.length;
    const stripHeight = GAME_HEIGHT / n;
    const zoom = stripHeight / GAME_HEIGHT;
    const widthPx = this.world.widthUnits * UNIT_PX;

    // Zusätzliche Kameras entfernen, main bleibt Spieler 1.
    this.cameras.cameras.filter((c) => c !== this.cameras.main).forEach((c) => this.cameras.remove(c));
    this.nightFx = this.nightFx.slice(0, 1);

    indices.forEach((playerIndex, i) => {
      const cam = i === 0 ? this.cameras.main : this.cameras.add(0, 0, GAME_WIDTH, stripHeight);
      if (i > 0) this.addNightFx(cam);
      cam.setViewport(0, i * stripHeight, GAME_WIDTH, stripHeight);
      cam.setZoom(zoom);
      cam.setBounds(0, 0, widthPx, GAME_HEIGHT);
      cam.setBackgroundColor(this.world.biome.palette.sky);
      const view = this.renderer_.playerView(playerIndex);
      if (view) cam.startFollow(view, true, 0.1, 0.1);
    });
  }

  /** Nacht = Kamera abdunkeln. Nur mit WebGL (Filter), im Canvas-Modus bleibt es hell. */
  private addNightFx(cam: Phaser.Cameras.Scene2D.Camera): void {
    cam.filters.internal.clear(); // die Hauptkamera überlebt einen Szenen-Neustart, sonst stapelt sich die Abdunklung
    this.nightFx.push(cam.filters.internal.addColorMatrix());
  }

  private restartWith(depth: number, seed: string): void {
    this.scene.stop('hud');
    this.scene.restart({ ...this.data_, depth, seed } satisfies GameSceneData);
  }

  private drawBackground(widthPx: number): void {
    const palette = this.world.biome.palette;
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
