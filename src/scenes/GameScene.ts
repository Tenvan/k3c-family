import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y, MAX_PLAYERS, PLAYER_COLORS, UNIT_PX } from '../core/constants';
import { GamepadInput, KeyboardInput, type PlayerInput } from '../input/playerInput';
import { biomeForDepth, type BiomeConfig } from '../world/biome';
import { generateLevel, type LevelEntity, type LevelLayout } from '../world/levelGenerator';
import { Monarch } from '../world/monarch';

export interface GameSceneData {
  depth: number;
  seed: string;
}

/**
 * Spielwelt: generiert ein Level aus Seed + Biom-Config, rendert Platzhalter,
 * verwaltet Couch-Koop-Spieler (Beitritt per A / Leertaste) und Split-Screen-Kameras.
 */
export class GameScene extends Phaser.Scene {
  biome!: BiomeConfig;
  level!: LevelLayout;
  seed = '';
  readonly players: Monarch[] = [];

  private keyboard!: KeyboardInput;
  private pads: GamepadInput[] = [];

  constructor() {
    super('game');
  }

  init(data: GameSceneData): void {
    this.biome = biomeForDepth(data.depth);
    this.seed = data.seed;
    this.level = generateLevel(this.biome, this.seed);
    this.players.length = 0;
    this.pads = [];
  }

  create(): void {
    const widthPx = this.level.widthUnits * UNIT_PX;
    this.drawBackground(widthPx);
    this.level.entities.forEach((e) => this.drawEntity(e));
    this.add.rectangle(0, GROUND_Y, widthPx, GAME_HEIGHT - GROUND_Y, Phaser.Display.Color.HexStringToColor(this.biome.palette.ground).color).setOrigin(0, 0);

    this.cameras.main.setBounds(0, 0, widthPx, GAME_HEIGHT);
    this.cameras.main.centerOn(this.level.hubCenterUnits * UNIT_PX, GAME_HEIGHT / 2);
    this.cameras.main.setBackgroundColor(this.biome.palette.sky);

    this.keyboard = new KeyboardInput(this.input.keyboard!);
    // Browser melden Gamepads erst nach dem ersten Tastendruck. Alle bekannten + neue Pads beobachten.
    const gamepads = this.input.gamepad!;
    const addPad = (pad: Phaser.Input.Gamepad.Gamepad) => {
      if (pad && !this.pads.some((p) => p.pad === pad)) this.pads.push(new GamepadInput(pad));
    };
    gamepads.gamepads.forEach(addPad);
    gamepads.on('connected', addPad);

    // Dev-Hilfe: N = neues Level mit zufälligem Seed, 1/2/3 = Tiefe wechseln.
    this.input.keyboard!.on('keydown-N', () => this.restartWith(this.biome.depth, Math.random().toString(36).slice(2, 8)));
    this.input.keyboard!.on('keydown-ONE', () => this.restartWith(0, this.seed));
    this.input.keyboard!.on('keydown-TWO', () => this.restartWith(1, this.seed));
    this.input.keyboard!.on('keydown-THREE', () => this.restartWith(2, this.seed));

    this.scene.launch('hud');
  }

  update(_time: number, deltaMs: number): void {
    const dt = Math.min(deltaMs / 1000, 0.1);
    this.keyboard.update();
    this.pads.forEach((p) => p.update());

    this.handleJoin();

    const all: PlayerInput[] = [this.keyboard, ...this.pads];
    if (all.some((i) => i.justPressed('fullscreen'))) this.scale.toggleFullscreen();

    const maxX = this.level.widthUnits * UNIT_PX;
    for (const monarch of this.players) monarch.step(dt, 0, maxX);
  }

  private handleJoin(): void {
    if (this.players.length >= MAX_PLAYERS) return;
    const joined = new Set(this.players.map((p) => p.controls));
    const candidates: PlayerInput[] = [this.keyboard, ...this.pads];
    for (const input of candidates) {
      if (joined.has(input) || !input.justPressed('confirm')) continue;
      const index = this.players.length;
      const spawnX = this.level.hubCenterUnits * UNIT_PX + (index === 0 ? -60 : 60);
      this.players.push(new Monarch(this, spawnX, index, input, PLAYER_COLORS[index]));
      this.layoutCameras();
      if (this.players.length >= MAX_PLAYERS) return;
    }
  }

  /** 1 Spieler: Vollbild. 2+ Spieler: horizontale Streifen übereinander (K2C-Stil). */
  private layoutCameras(): void {
    const n = this.players.length;
    const stripHeight = GAME_HEIGHT / n;
    const zoom = stripHeight / GAME_HEIGHT;
    const widthPx = this.level.widthUnits * UNIT_PX;

    // Zusätzliche Kameras entfernen, main bleibt Spieler 1.
    this.cameras.cameras.filter((c) => c !== this.cameras.main).forEach((c) => this.cameras.remove(c));

    this.players.forEach((monarch, i) => {
      const cam = i === 0 ? this.cameras.main : this.cameras.add(0, 0, GAME_WIDTH, stripHeight);
      cam.setViewport(0, i * stripHeight, GAME_WIDTH, stripHeight);
      cam.setZoom(zoom);
      cam.setBounds(0, 0, widthPx, GAME_HEIGHT);
      cam.setBackgroundColor(this.biome.palette.sky);
      cam.startFollow(monarch, true, 0.1, 0.1);
    });
  }

  private restartWith(depth: number, seed: string): void {
    this.scene.stop('hud');
    this.scene.restart({ depth, seed } satisfies GameSceneData);
  }

  private drawBackground(widthPx: number): void {
    const far = Phaser.Display.Color.HexStringToColor(this.biome.palette.far).color;
    const near = Phaser.Display.Color.HexStringToColor(this.biome.palette.near).color;
    // Zwei Parallax-Ebenen mit gezackter Silhouette (Berge / Höhlenwände).
    this.drawRidge(widthPx, far, 0.3, 520, 180);
    this.drawRidge(widthPx, near, 0.6, 700, 120);
  }

  private drawRidge(widthPx: number, color: number, scrollFactor: number, baseY: number, amplitude: number): void {
    const g = this.add.graphics().setScrollFactor(scrollFactor, 1);
    g.fillStyle(color, 1);
    const points: Phaser.Types.Math.Vector2Like[] = [{ x: 0, y: GROUND_Y }];
    for (let x = 0; x <= widthPx * scrollFactor + GAME_WIDTH * 2; x += 160) {
      points.push({ x, y: baseY - Math.abs(Math.sin(x * 0.0021) + Math.sin(x * 0.0057)) * amplitude * 0.6 });
    }
    points.push({ x: widthPx * scrollFactor + GAME_WIDTH * 2, y: GROUND_Y });
    g.fillPoints(points, true);
  }

  /** Platzhalter-Grafiken. Später durch Sprites/Atlas ersetzen, Positionen bleiben. */
  private drawEntity(e: LevelEntity): void {
    const x = e.x * UNIT_PX;
    const y = GROUND_Y;
    switch (e.kind) {
      case 'tree':
        this.add.rectangle(x, y - 40, 14, 80, 0x6b4226);
        this.add.triangle(x, y - 110, 0, 90, 40, 0, 80, 90, 0x2d6a4f);
        break;
      case 'bush':
        this.add.circle(x, y - 14, 18, 0x40916c);
        break;
      case 'rock':
        this.add.ellipse(x, y - 16, 50, 34, 0x8d99ae);
        break;
      case 'copperOre':
        this.add.ellipse(x, y - 16, 44, 30, 0xb87333).setStrokeStyle(3, 0x6d3f1f);
        break;
      case 'castle':
        this.add.rectangle(x, y - 130, 300, 260, 0x6c757d).setStrokeStyle(4, 0x343a40);
        this.add.rectangle(x, y - 40, 60, 80, 0x343a40);
        this.add.text(x, y - 280, 'HUB', { fontSize: '32px', color: '#ffffff', fontStyle: 'bold' }).setOrigin(0.5);
        break;
      case 'portal':
        this.add.ellipse(x, y - 110, 110, 220, 0x7b2cbf).setStrokeStyle(6, 0x240046);
        break;
      case 'exit':
        this.add.rectangle(x, y - 90, 160, 180, 0x111111).setStrokeStyle(6, 0x555555);
        this.add.text(x, y - 210, `Tiefe ${this.biome.depth + 1}`, { fontSize: '24px', color: '#ffffff' }).setOrigin(0.5);
        break;
      case 'chest':
        this.add.rectangle(x, y - 18, 44, 36, 0x9c6644).setStrokeStyle(3, 0x5c3d2e);
        break;
      case 'recruitCamp':
        this.add.triangle(x, y - 45, 0, 90, 60, 0, 120, 90, 0xe9c46a).setStrokeStyle(3, 0x7f5539);
        break;
      case 'skillPoint':
        this.add.star(x, y - 60, 5, 8, 18, 0xffd60a).setStrokeStyle(2, 0xffffff);
        break;
      default:
        this.add.rectangle(x, y - 20, 30, 40, 0xff00ff); // unbekannter Typ => pink = auffällig
    }
  }
}
