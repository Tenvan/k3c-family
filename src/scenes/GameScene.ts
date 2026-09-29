import Phaser from 'phaser';
import { toggleFullscreen } from '../core/fullscreen';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, MAX_PLAYERS, UNIT_PX } from '../core/constants';
import { storeSave, type SaveTarget } from '../core/saveStore';
import { GamepadInput, KeyboardInput, type PlayerInput } from '../input/playerInput';
import { createCampaign, currentWorld, fromSave, joinPlayer, toSave, travel, type Campaign, type SaveGame } from '../world/sim/campaign';
import { daylight } from '../world/sim/cycle';
import { giveGold } from '../world/sim/economy';
import type { GameEvent, PlayerCommand, World } from '../world/sim/types';
import { step } from '../world/sim/world';
import { WorldRenderer } from './worldRenderer';

export interface GameSceneData {
  depth: number;
  seed: string;
  /** Tag/Nacht 8x schneller (?fast=1) */
  fast?: boolean;
  /** Dev-Tasten aktiv (Dev-Server oder ?dev=1) */
  dev?: boolean;
  /** Geladener Spielstand (?continue=1) */
  save?: SaveGame | null;
  /** Automatisch speichern (?save=1): bei Tagesanbruch, beim Stufenwechsel und beim Verlassen der Seite */
  persist?: boolean;
}

const FAST_CYCLE = 8;
/** Helligkeit in tiefster Nacht */
const NIGHT_BRIGHTNESS = 0.45;
const SAVE_TEXT: Record<SaveTarget, string> = { server: 'Gespeichert', local: 'Gespeichert (nur im Browser)', none: 'Speichern fehlgeschlagen' };

/**
 * Spielwelt: rechnet die Simulation (src/world/sim) und zeichnet sie über den WorldRenderer.
 * Couch-Koop: Beitritt per A / Leertaste, jeder Spieler bekommt einen eigenen Split-Screen-Streifen.
 * Die Kampagne hält alle Stufen. Beim Stufenwechsel wird nur die Darstellung neu aufgebaut, die Eingaben bleiben.
 */
export class GameScene extends Phaser.Scene {
  campaign!: Campaign;
  /** Eingabe pro Spieler (Index = world.players-Index) */
  readonly controls: PlayerInput[] = [];
  /** Ereignisse für die HudScene, die sie abholt und leert */
  readonly pendingEvents: GameEvent[] = [];
  /** Letzte Speicher-Meldung für die HudScene */
  saveStatus: { text: string; at: number } | null = null;

  private data_!: GameSceneData;
  private renderer_!: WorldRenderer;
  private keyboard!: KeyboardInput;
  private pads: GamepadInput[] = [];
  private nightFx: Phaser.Filters.ColorMatrix[] = [];

  constructor() {
    super('game');
  }

  get world(): World {
    return currentWorld(this.campaign);
  }

  init(data: GameSceneData): void {
    this.data_ = data;
    const cycleSpeed = data.fast ? FAST_CYCLE : 1;
    this.campaign = data.save
      ? fromSave(data.save, cycleSpeed)
      : createCampaign(data.seed, { id: `${data.seed}-${Date.now().toString(36)}`, cycleSpeed, depth: data.depth });
    this.controls.length = 0;
    this.pendingEvents.length = 0;
    this.saveStatus = null;
    this.pads = [];
    this.nightFx = [];
  }

  create(): void {
    this.buildWorldView();

    this.keyboard = new KeyboardInput(this.input.keyboard!);
    // Browser melden Gamepads erst nach dem ersten Tastendruck. Alle bekannten + neue Pads beobachten.
    const gamepads = this.input.gamepad!;
    const addPad = (pad: Phaser.Input.Gamepad.Gamepad) => {
      if (pad && !this.pads.some((p) => p.pad === pad)) this.pads.push(new GamepadInput(pad));
    };
    gamepads.gamepads.forEach(addPad);
    gamepads.on('connected', addPad);

    const kb = this.input.keyboard!;
    // Dev-Hilfe: N = neues Level mit zufälligem Seed, 1/2/3 = Tiefe wechseln (jeweils neues Spiel, ohne Speichern).
    kb.on('keydown-N', () => this.restartWith(this.world.biome.depth, Math.random().toString(36).slice(2, 8)));
    kb.on('keydown-ONE', () => this.restartWith(0, this.world.seed));
    kb.on('keydown-TWO', () => this.restartWith(1, this.world.seed));
    kb.on('keydown-THREE', () => this.restartWith(2, this.world.seed));
    if (this.data_.dev) {
      // G = +10 Gold, H = +50 Baumaterial, T = zur nächsten Tageszeit springen, S = jetzt speichern
      kb.on('keydown-G', () => this.world.players.forEach((p) => giveGold(this.world, p, 10)));
      kb.on('keydown-H', () => (['wood', 'stone', 'copper'] as const).forEach((r) => (this.world.stock[r] += 50)));
      kb.on('keydown-T', () => (this.world.time += this.world.cycle.secondsLeft / this.world.cycleSpeed + 0.01));
      kb.on('keydown-S', () => void this.autosave(true));
    }

    // Verlassen der Seite (Home-Button / View+Menu): letzten Stand noch schnell sichern.
    const onHide = () => void this.autosave(false, true);
    window.addEventListener('pagehide', onHide);
    this.events.once(Phaser.Scenes.Events.SHUTDOWN, () => window.removeEventListener('pagehide', onHide));

    this.scene.launch('hud');
  }

  update(_time: number, deltaMs: number): void {
    const dt = Math.min(deltaMs / 1000, 0.1);
    this.keyboard.update();
    this.pads.forEach((p) => p.update());

    this.handleJoin();

    const all: PlayerInput[] = [this.keyboard, ...this.pads];
    if (all.some((i) => i.justPressed('fullscreen'))) void toggleFullscreen(); // über die Shell, damit Vollbild beim Seitenwechsel bleibt

    const world = this.world;
    const commands: PlayerCommand[] = this.controls.map((c) => ({ moveX: c.moveX(), sprint: c.sprint(), pay: c.held('confirm') }));
    step(world, commands, dt);
    this.pendingEvents.push(...world.events);
    if (world.events.some((e) => e.type === 'dawn')) void this.autosave();

    if (world.travel && world.travel.progress >= 1) {
      const target = travel(this.campaign, world.travel.toDepth);
      this.pendingEvents.push(...target.events);
      this.buildWorldView();
      void this.autosave();
      return;
    }

    this.renderer_.sync(world);
    const brightness = NIGHT_BRIGHTNESS + (1 - NIGHT_BRIGHTNESS) * daylight(world.cycle);
    for (const fx of this.nightFx) fx.colorMatrix.brightness(brightness);
  }

  /** Spielstand sichern, wenn diese Partie gespeichert werden soll (?save=1) oder per Dev-Taste. */
  private async autosave(force = false, leaving = false): Promise<void> {
    if (!this.data_.persist && !force) return;
    if (this.world.players.length === 0) return; // noch niemand beigetreten: nichts überschreiben
    const at = this.world.time;
    const target = await storeSave(toSave(this.campaign, new Date().toISOString()), undefined, leaving);
    this.saveStatus = { text: SAVE_TEXT[target], at };
  }

  private handleJoin(): void {
    if (this.controls.length >= MAX_PLAYERS) return;
    for (const input of [this.keyboard, ...this.pads]) {
      if (this.controls.includes(input) || !input.justPressed('confirm')) continue;
      joinPlayer(this.campaign);
      this.controls.push(input);
      this.renderer_.sync(this.world);
      this.layoutCameras();
      if (this.controls.length >= MAX_PLAYERS) return;
    }
  }

  /** Baut alles Sichtbare für die aktuelle Stufe (neu) auf: Hintergrund, Welt-Objekte, Kameras. */
  private buildWorldView(): void {
    const world = this.world;
    this.children.removeAll(true);
    this.cameras.cameras.filter((c) => c !== this.cameras.main).forEach((c) => this.cameras.remove(c));
    this.nightFx = [];

    const widthPx = world.widthUnits * UNIT_PX;
    const palette = world.biome.palette;
    this.drawBackground(widthPx);
    this.add.rectangle(0, GROUND_Y, widthPx, GAME_HEIGHT - GROUND_Y, Phaser.Display.Color.HexStringToColor(palette.ground).color).setOrigin(0, 0);
    this.renderer_ = new WorldRenderer(this, world);
    this.renderer_.sync(world);

    const main = this.cameras.main;
    main.stopFollow();
    main.setViewport(0, 0, GAME_WIDTH, GAME_HEIGHT).setZoom(1);
    main.setBounds(0, 0, widthPx, GAME_HEIGHT);
    main.centerOn(world.hubX * UNIT_PX, GAME_HEIGHT / 2);
    main.setBackgroundColor(palette.sky);
    this.addNightFx(main);
    if (world.players.length > 0) this.layoutCameras();
  }

  /** 1 Spieler: Vollbild. 2+ Spieler: horizontale Streifen übereinander (K2C-Stil). */
  private layoutCameras(): void {
    const n = this.world.players.length;
    const stripHeight = GAME_HEIGHT / n;
    const zoom = stripHeight / GAME_HEIGHT;
    const widthPx = this.world.widthUnits * UNIT_PX;

    // Zusätzliche Kameras entfernen, main bleibt Spieler 1.
    this.cameras.cameras.filter((c) => c !== this.cameras.main).forEach((c) => this.cameras.remove(c));
    this.nightFx = this.nightFx.slice(0, 1);

    this.world.players.forEach((_, i) => {
      const cam = i === 0 ? this.cameras.main : this.cameras.add(0, 0, GAME_WIDTH, stripHeight);
      if (i > 0) this.addNightFx(cam);
      cam.setViewport(0, i * stripHeight, GAME_WIDTH, stripHeight);
      cam.setZoom(zoom);
      cam.setBounds(0, 0, widthPx, GAME_HEIGHT);
      cam.setBackgroundColor(this.world.biome.palette.sky);
      const view = this.renderer_.playerView(i);
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
    this.scene.restart({ ...this.data_, depth, seed, save: null, persist: false } satisfies GameSceneData);
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
