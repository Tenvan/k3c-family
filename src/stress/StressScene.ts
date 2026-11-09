import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, GROUND_Y } from '../core/constants';
import { t } from '../tools/texts';
import { textureMiB } from '../core/renderInfo';
import { ENEMY_SPRITES, TROOP_SPRITES, createSpriteAnims, face, makeSprite, playAnim, preloadSprites } from '../scenes/render/sprites';
import { type Figure, type KnownTypes, type Motion, type Scenario, distinctTypes, figuresAt, motionFor, stageCount } from './stressPlan';
import type { RunResult, StressRun } from './stressRun';
import type { RenderLog } from '../core/renderLog';
import { buildScenery, preloadScenery } from './scenery';
import { RenderInfoView } from '../scenes/debug/renderInfoView';

/** Abstand der Spieler-Bereiche in der Welt: Figuren des einen Spielers liegen nie im Bild des anderen */
const PLAYER_GAP = 5000;
const STYLE = { fontSize: '26px', color: '#ffffff', stroke: '#000000', strokeThickness: 5, fontStyle: 'bold' };

export interface StressStart {
  scenario: Scenario;
  players: 1 | 2;
  ramp: boolean;
  seed: number;
  known: KnownTypes;
  run: StressRun;
  /** Render-Log des Spiels (Start- und Ladezeile) und Versionstext */
  renderLog: RenderLog;
  version: string;
  onEnd(result: RunResult): void;
}

interface Entity {
  sprite: Phaser.GameObjects.Sprite;
  motion: Motion;
  player: number;
  dir: number;
}

/**
 * Stress-Szene (B-765): zeichnet N Figuren mit dem echten Renderer und den Papierschnitt-Sheets für 1 oder 2 Spieler (Split-Screen),
 * ohne Server. Die Rampe erhöht die Last stufenweise; Messung und Log liegen in `StressRun`.
 */
export class StressScene extends Phaser.Scene {
  private cfg!: StressStart;
  private entities = new Map<number, Entity>();
  private cams: Phaser.Cameras.Scene2D.Camera[] = [];
  private stage = 1;
  private total = 1;
  private time_s = 0;
  private hud!: Phaser.GameObjects.Text;
  private over = false;
  private tech!: RenderInfoView;
  private readyAt = 0;
  private missing = new Set<string>();

  constructor() {
    super('stress');
  }

  init(data: StressStart): void {
    this.cfg = data;
    this.entities = new Map();
    this.stage = 1;
    this.time_s = 0;
    this.over = false;
    this.missing = new Set();
  }

  preload(): void {
    this.readyAt = performance.now();
    preloadSprites(this);
    preloadScenery(this, this.cfg.scenario.scenery, this.worldWidth(), GAME_WIDTH);
  }

  private worldWidth(): number {
    return PLAYER_GAP * this.cfg.players + 8000;
  }

  create(): void {
    const { scenario: s, players, run } = this.cfg;
    createSpriteAnims(this);
    this.total = stageCount(s, this.cfg.ramp);
    this.cameras.main.setBackgroundColor('#6fa8dc');
    this.add.rectangle(-4000, GROUND_Y, this.worldWidth(), 600, 0x4b6b3a).setOrigin(0, 0).setDepth(-1);
    this.buildCameras(players, s);
    const scenery = buildScenery(this, s.scenery, this.worldWidth(), GAME_WIDTH / s.camera.zoom, this.cfg.seed, this.cams.map((_, i) => this.centerX(i)));
    this.hud = this.add.text(16, 8, '', STYLE).setDepth(1e6).setScrollFactor(0);
    this.cams.slice(1).forEach((c) => c.ignore(this.hud));
    this.tech = new RenderInfoView(this);
    this.cams.slice(1).forEach((c) => c.ignore(this.tech.object));
    this.watchContext();
    const textures = Object.values(this.textures.list) as unknown as Parameters<typeof textureMiB>[0];
    this.cfg.renderLog.loaded('stress', textures.length, textureMiB(textures), performance.now() - this.readyAt);
    this.cfg.renderLog.start({ userAgent: navigator.userAgent, version: this.cfg.version });
    run.begin();
    run.event('Szene', { backgroundImages: scenery.backgroundImages, foreground: scenery.foreground, biome: s.scenery.biome });
    this.enterStage(1);
  }

  private buildCameras(players: number, s: Scenario): void {
    const h = GAME_HEIGHT / players;
    this.cams = [this.cameras.main];
    this.cameras.main.setViewport(0, 0, GAME_WIDTH, h);
    for (let i = 1; i < players; i++) this.cams.push(this.cameras.add(0, h * i, GAME_WIDTH, h).setBackgroundColor('#6fa8dc'));
    this.cams.forEach((c, i) => c.setZoom(s.camera.zoom).centerOn(this.centerX(i), this.centerY(h)));
  }

  private centerX(player: number): number {
    return this.cfg.scenario.camera.x + player * PLAYER_GAP;
  }

  private centerY(viewH: number): number {
    return GROUND_Y - (viewH / this.cfg.scenario.camera.zoom) * 0.25;
  }

  private watchContext(): void {
    const canvas = this.game.canvas;
    const lost = (): void => this.finish('contextLost', 'WebGL Context lost');
    canvas.addEventListener('webglcontextlost', lost);
    const hidden = (): void => this.cfg.run.event(document.hidden ? 'Tab im Hintergrund' : 'Tab wieder sichtbar');
    document.addEventListener('visibilitychange', hidden);
    const clean = (): void => {
      canvas.removeEventListener('webglcontextlost', lost);
      document.removeEventListener('visibilitychange', hidden);
    };
    this.events.once('shutdown', clean);
    this.events.once('destroy', clean);
  }

  /** Legt die Figuren der Stufe an, die es noch nicht gibt, und startet die Messung der Stufe */
  private enterStage(stage: number): void {
    const { scenario: s, players, known, seed, run } = this.cfg;
    const figs = figuresAt(s, known, stage, players).filter((f) => this.hasAsset(f));
    const viewW = GAME_WIDTH / s.camera.zoom;
    for (const f of figs) if (!this.entities.has(f.key)) this.spawn(f, motionFor(seed, f.key, this.centerX(f.player), viewW));
    run.startStage(stage, figs.length, distinctTypes(figs));
    this.stage = stage;
  }

  private spec(f: Figure): (typeof ENEMY_SPRITES)[string] | undefined {
    return f.kind === 'enemy' ? ENEMY_SPRITES[f.type] : TROOP_SPRITES[f.type];
  }

  /** Hat die Figur ein Asset? Fehlende Typen werden einmal gemeldet und nicht mitgezählt */
  private hasAsset(f: Figure): boolean {
    if (this.spec(f)) return true;
    if (!this.missing.has(f.type)) {
      this.missing.add(f.type);
      this.cfg.run.event(`Kein Asset für ${f.type}`, { type: f.type });
    }
    return false;
  }

  private spawn(f: Figure, motion: Motion): void {
    const spec = this.spec(f)!;
    const sprite = makeSprite(this, spec);
    sprite.setPosition(motion.x, GROUND_Y + motion.lane).setDepth(GROUND_Y + motion.lane);
    playAnim(sprite, 'run');
    this.cams.forEach((c, i) => i !== f.player && c.ignore(sprite));
    this.entities.set(f.key, { sprite, motion, player: f.player, dir: 1 });
  }

  update(): void {
    if (this.over) return;
    const delta = this.game.loop.rawDelta;
    this.time_s += delta / 1000;
    this.tech.update();
    this.move();
    this.sweep();
    if (this.cfg.run.frame(delta)) this.nextStage();
    this.hud.setText(t('stress.hud', { stage: this.stage, total: this.total, figures: this.entities.size, fps: Math.round(this.game.loop.actualFps) }));
  }

  private move(): void {
    for (const e of this.entities.values()) {
      const a = e.motion.phase + this.time_s * e.motion.speed;
      e.sprite.x = e.motion.x + Math.sin(a) * e.motion.amp;
      const dir = Math.cos(a) >= 0 ? 1 : -1;
      if (dir !== e.dir) {
        e.dir = dir;
        face(e.sprite, dir);
      }
    }
  }

  private sweep(): void {
    if (this.cfg.scenario.camera.pan !== 'sweep') return;
    const dx = Math.sin(this.time_s * 0.4) * 300;
    this.cams.forEach((c, i) => c.centerOn(this.centerX(i) + dx, this.centerY(GAME_HEIGHT / this.cfg.players)));
  }

  private inView(): number[] {
    return this.cams.map((c, i) => {
      let n = 0;
      for (const e of this.entities.values()) if (e.player === i && c.worldView.contains(e.sprite.x, e.sprite.y)) n++;
      return n;
    });
  }

  private nextStage(): void {
    this.cfg.run.endStage(this.inView());
    if (this.stage < this.total) this.enterStage(this.stage + 1);
    else this.finish('done');
  }

  private finish(reason: 'done' | 'contextLost', event?: string): void {
    if (this.over) return;
    this.over = true;
    if (event) this.cfg.run.event(event);
    const result = this.cfg.run.finish(reason);
    const lim = result.densityLimit;
    this.hud.setText(t('stress.done', { limit: lim ? `${lim.stage} (${lim.entities})` : '–' }));
    this.cfg.onEnd(result);
  }
}
