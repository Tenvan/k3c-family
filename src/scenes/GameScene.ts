import Phaser from 'phaser';
import { toggleFullscreen } from '../core/fullscreen';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, MAX_PLAYERS, UNIT_PX } from '../core/constants';
import { storeSave, type SaveTarget } from '../core/saveStore';
import { GamepadInput, KeyboardInput, type PlayerInput } from '../input/playerInput';
import { TouchInput, wantsTouchControls } from '../input/touchInput';
import type { OnlineClient } from '../online/client';
import { applySnapshot } from '../online/protocol';
import { biomeForDepth } from '../world/biome';
import { createCampaign, currentWorld, fromSave, joinPlayer, toSave, travel, type Campaign, type SaveGame } from '../world/sim/campaign';
import { daylight } from '../world/sim/cycle';
import { giveGold } from '../world/sim/economy';
import type { GameEvent, PlayerCommand, World } from '../world/sim/types';
import { createWorld, step } from '../world/sim/world';
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
  /** Geladener Spielstand (?continue=1) */
  save?: SaveGame | null;
  /** Automatisch speichern (?save=1): bei Tagesanbruch, beim Stufenwechsel und beim Verlassen der Seite */
  persist?: boolean;
}

const FAST_CYCLE = 8;
/** Der Mitspieler oben wechselt erst, wenn ein anderer um so viele Units näher ist */
const PARTNER_SWITCH_UNITS = 10;

interface CamStrip {
  player: number;
  height: number;
}
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
  /** Online: Kamera folgt schon dem eigenen Monarchen */
  private onlineCameraReady = false;
  /** Online: Mitspieler im oberen Drittel (null = allein, dann Vollbild) */
  private partnerIndex: number | null = null;
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
    this.touch = undefined;
    this.onlineCameraReady = false;
    this.partnerIndex = null;
    this.lastDevice = wantsTouchControls() ? 'touch' : 'keyboard';
  }

  create(): void {
    this.buildWorldView();

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
    // Dev-Hilfe: N = neues Level mit zufälligem Seed, 1/2/3 = Tiefe wechseln (jeweils neues Spiel, ohne Speichern).
    if (!this.data_.online) {
    kb.on('keydown-N', () => this.restartWith(this.world.biome.depth, Math.random().toString(36).slice(2, 8)));
    kb.on('keydown-ONE', () => this.restartWith(0, this.world.seed));
    kb.on('keydown-TWO', () => this.restartWith(1, this.world.seed));
    kb.on('keydown-THREE', () => this.restartWith(2, this.world.seed));
    }
    if (this.data_.dev && !this.data_.online) {
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
    this.touch?.update();

    this.trackLastDevice();

    const all = this.allInputs();
    if (all.some((i) => i.justPressed('fullscreen'))) void toggleFullscreen(); // über die Shell, damit Vollbild beim Seitenwechsel bleibt

    if (this.data_.online) {
      this.updateOnline(all);
      return;
    }
    this.handleJoin();

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

    // Der Server ist in eine andere Stufe gewechselt: Spiegelwelt der Stufe neu aufbauen (das Level kommt aus dem Seed).
    const stageChanged = snapshot.depth !== this.world.biome.depth;
    if (stageChanged) {
      this.campaign.worlds.set(snapshot.depth, createWorld(biomeForDepth(snapshot.depth), this.campaign.seed, { cycleSpeed: this.campaign.cycleSpeed }));
      this.campaign.depth = snapshot.depth;
    }
    applySnapshot(this.world, snapshot);
    this.pendingEvents.push(...snapshot.events);
    if (stageChanged) this.buildWorldView();
    else this.renderer_.sync(this.world);

    const brightness = NIGHT_BRIGHTNESS + (1 - NIGHT_BRIGHTNESS) * daylight(this.world.cycle);
    for (const fx of this.nightFx) fx.colorMatrix.brightness(brightness);
    const partner = this.pickPartner(client.you);
    if ((!this.onlineCameraReady || partner !== this.partnerIndex) && this.renderer_.playerView(client.you)) {
      this.onlineCameraReady = true;
      this.partnerIndex = partner;
      this.layoutCameras(this.onlineStrips());
    }
  }

  /** Bildschirmstreifen, für die das HUD Spielerwerte zeigt (Split-Screen: alle lokalen, online: nur der eigene) */
  hudStrips(): { player: number; y: number }[] {
    const online = this.data_.online;
    const result: { player: number; y: number }[] = [];
    let y = 0;
    for (const s of online ? this.onlineStrips() : this.equalStrips()) {
      if (!online || s.player === online.you) result.push({ player: s.player, y });
      y += s.height;
    }
    return result;
  }

  private equalStrips(): CamStrip[] {
    const n = Math.max(1, this.world.players.length);
    return this.world.players.map((_, i) => ({ player: i, height: GAME_HEIGHT / n }));
  }

  /** Online (ein Monarch pro Gerät): unten 2/3 der eigene, oben 1/3 der nächste Mitspieler. Allein: Vollbild. */
  private onlineStrips(): CamStrip[] {
    const you = this.data_.online!.you;
    if (this.partnerIndex === null) return [{ player: you, height: GAME_HEIGHT }];
    return [
      { player: this.partnerIndex, height: GAME_HEIGHT / 3 },
      { player: you, height: (GAME_HEIGHT * 2) / 3 },
    ];
  }

  /** Nächster Mitspieler; der bisherige bleibt, solange kein anderer deutlich näher ist (kein Flackern). */
  private pickPartner(you: number): number | null {
    const me = this.world.players[you];
    const others = this.world.players.filter((p) => p.index !== you);
    if (!me || others.length === 0) return null;
    const dist = (p: { x: number }) => Math.abs(p.x - me.x);
    const nearest = others.reduce((a, b) => (dist(b) < dist(a) ? b : a));
    const current = others.find((p) => p.index === this.partnerIndex);
    return current && dist(current) - dist(nearest) < PARTNER_SWITCH_UNITS ? current.index : nearest.index;
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

  /** Spielstand sichern, wenn diese Partie gespeichert werden soll (?save=1) oder per Dev-Taste. */
  private async autosave(force = false, leaving = false): Promise<void> {
    if (this.data_.online || (!this.data_.persist && !force)) return;
    if (this.world.players.length === 0) return; // noch niemand beigetreten: nichts überschreiben
    const at = this.world.time;
    const target = await storeSave(toSave(this.campaign, new Date().toISOString()), undefined, leaving);
    this.saveStatus = { text: SAVE_TEXT[target], at };
  }

  private handleJoin(): void {
    if (this.controls.length >= MAX_PLAYERS) return;
    for (const input of this.allInputs()) {
      if (this.controls.includes(input) || !input.justPressed('confirm')) continue;
      joinPlayer(this.campaign);
      this.controls.push(input);
      this.renderer_.sync(this.world);
      this.layoutCameras(this.equalStrips());
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
    if (world.players.length > 0) this.layoutCameras(this.data_.online ? this.onlineStrips() : this.equalStrips());
    this.onlineCameraReady = !!this.data_.online && !!this.renderer_.playerView(this.data_.online.you);
  }

  /** 1 Spieler: Vollbild. 2+ Spieler: horizontale Streifen übereinander (K2C-Stil). */
  private layoutCameras(strips: CamStrip[]): void {
    const widthPx = this.world.widthUnits * UNIT_PX;

    // Zusätzliche Kameras entfernen, main bleibt Spieler 1.
    this.cameras.cameras.filter((c) => c !== this.cameras.main).forEach((c) => this.cameras.remove(c));
    this.nightFx = this.nightFx.slice(0, 1);

    let y = 0;
    strips.forEach(({ player: playerIndex, height: stripHeight }, i) => {
      const cam = i === 0 ? this.cameras.main : this.cameras.add(0, 0, GAME_WIDTH, stripHeight);
      if (i > 0) this.addNightFx(cam);
      cam.setViewport(0, y, GAME_WIDTH, stripHeight);
      y += stripHeight;
      cam.setZoom(stripHeight / GAME_HEIGHT);
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
