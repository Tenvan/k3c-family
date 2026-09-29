import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, MAX_PLAYERS } from '../core/constants';
import { ECONOMY } from '../world/sim/data';
import type { GameEvent, World } from '../world/sim/types';
import type { GameScene } from './GameScene';
import { RESOURCE_NAMES } from './worldRenderer';

const STYLE = { fontSize: '28px', color: '#ffffff', stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
const BANNER_SECONDS = 2.8;
const SITE_NAMES = { wall: 'Mauer', tower: 'Turm', workshop: 'Werkstatt', stairsUp: 'Treppe hoch', stairsDown: 'Treppe runter' } as const;

/** Bildschirmfeste Anzeigen: pro Split-Screen-Hälfte Spielerwerte, oben rechts Hub-Vorrat und Tageszeit, Meldungen in der Mitte. */
export class HudScene extends Phaser.Scene {
  private joinHint!: Phaser.GameObjects.Text;
  private shared!: Phaser.GameObjects.Text;
  private clock!: Phaser.GameObjects.Text;
  private fight!: Phaser.GameObjects.Text;
  private banner!: Phaser.GameObjects.Text;
  private fps!: Phaser.GameObjects.Text;
  private info!: Phaser.GameObjects.Text;
  private travel!: Phaser.GameObjects.Text;
  private saved!: Phaser.GameObjects.Text;
  private playerLabels: Phaser.GameObjects.Text[] = [];
  private bannerQueue: string[] = [];
  private bannerLeft = 0;

  constructor() {
    super('hud');
  }

  create(): void {
    this.playerLabels = [];
    this.bannerQueue = [];
    this.bannerLeft = 0;

    this.shared = this.add.text(GAME_WIDTH - 24, 16, '', STYLE).setOrigin(1, 0);
    this.clock = this.add.text(GAME_WIDTH - 24, 56, '', STYLE).setOrigin(1, 0);
    this.fight = this.add.text(GAME_WIDTH - 24, 96, '', { ...STYLE, color: '#ff8fa3' }).setOrigin(1, 0);
    this.banner = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, '', { ...STYLE, fontSize: '56px', strokeThickness: 10, align: 'center' })
      .setOrigin(0.5)
      .setVisible(false);
    this.joinHint = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, 'Drücke  A  (Controller) oder  Leertaste  zum Beitreten', { ...STYLE, fontSize: '44px' })
      .setOrigin(0.5);
    this.fps = this.add.text(20, GAME_HEIGHT - 40, '', { ...STYLE, fontSize: '20px', strokeThickness: 4 });
    const dev = 'Dev: N = neuer Seed · 1/2/3 = Tiefe · F / RS = Vollbild';
    this.add
      .text(GAME_WIDTH - 20, GAME_HEIGHT - 40, `A / Leertaste halten = Münzen geben · ${dev}`, { ...STYLE, fontSize: '20px', strokeThickness: 4 })
      .setOrigin(1, 0);
    this.info = this.add.text(20, 16, '', { ...STYLE, fontSize: '20px', strokeThickness: 4 });
    this.travel = this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 170, '', { ...STYLE, fontSize: '40px', color: '#ffd166' }).setOrigin(0.5);
    this.saved = this.add.text(20, GAME_HEIGHT - 70, '', { ...STYLE, fontSize: '20px', strokeThickness: 4, color: '#b7e4c7' });
  }

  update(_time: number, deltaMs: number): void {
    const game = this.game.scene.getScene('game') as GameScene;
    const world = game.world;
    const players = world.players;
    const stripHeight = GAME_HEIGHT / Math.max(1, players.length);

    this.joinHint.setVisible(players.length < MAX_PLAYERS);
    this.joinHint.setY(players.length === 0 ? GAME_HEIGHT / 2 + 120 : GAME_HEIGHT - 100);
    this.joinHint.setFontSize(players.length === 0 ? 44 : 26);
    this.info.setText(`${world.biome.name} · Seed "${world.seed}"`).setVisible(players.length === 0);
    const t = world.travel;
    const target = t ? (t.via === 'stairsUp' ? 'Aufstieg' : 'Abstieg') + ` in Tiefe ${t.toDepth}` : '';
    this.travel.setText(t ? `${target}  ${'▮'.repeat(Math.ceil(t.progress * 10))}${'▯'.repeat(10 - Math.ceil(t.progress * 10))}` : '');
    const status = game.saveStatus;
    this.saved.setText(status && world.time - status.at < 4 ? status.text : '');

    players.forEach((p, i) => {
      let label = this.playerLabels[i];
      if (!label) {
        label = this.add.text(24, 0, '', STYLE);
        this.playerLabels[i] = label;
      }
      label.setY(i * stripHeight + 16);
      const status = p.respawnIn > 0 ? `gefallen · zurück in ${Math.ceil(p.respawnIn)} s` : `HP ${Math.ceil(p.hp)}`;
      label.setText(`P${i + 1}  ·  Gold ${p.gold}/${ECONOMY.purse.maxGold}  ·  ${status}`);
    });

    const stock = (['wood', 'stone', 'copper'] as const).filter((r) => r === world.biome.primaryResource || world.stock[r] > 0).map((r) => `${RESOURCE_NAMES[r]} ${world.stock[r]}`);
    if (world.skillPoints > 0) stock.push(`Skill-Punkte ${world.skillPoints}`);
    this.shared.setText(stock.join('  ·  '));
    this.clock.setText(clockText(world));
    const enemies = world.enemies.length + world.spawnQueue.length;
    this.fight.setText(enemies > 0 ? `Welle ${world.wave}: ${enemies} Gegner` : '');

    for (const e of game.pendingEvents.splice(0)) {
      const text = eventText(e);
      if (text) this.bannerQueue.push(text);
    }
    this.bannerLeft -= deltaMs / 1000;
    if (this.bannerLeft <= 0) {
      const next = this.bannerQueue.shift();
      this.banner.setVisible(!!next);
      if (next) {
        this.banner.setText(next);
        this.bannerLeft = this.bannerQueue.length > 2 ? BANNER_SECONDS / 2 : BANNER_SECONDS;
      }
    }

    this.fps.setText(`${Math.round(this.game.loop.actualFps)} FPS`);
  }

}

function formatTime(seconds: number): string {
  const s = Math.max(0, Math.ceil(seconds));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`;
}

function clockText(w: World): string {
  const left = formatTime(w.cycle.secondsLeft / w.cycleSpeed);
  let time: string;
  if (w.cycle.phase === 'day') time = `Tag ${w.cycle.day}  ·  Dämmerung in ${left}`;
  else if (w.cycle.phase === 'dusk') time = `Dämmerung  ·  Nacht in ${left}`;
  else time = `Nacht ${w.cycle.day}  ·  Morgen in ${left}`;
  if (w.aggression !== null) time += `  ·  Aggression ${Math.floor(w.aggression)}%`;
  return time;
}

function eventText(e: GameEvent): string | null {
  switch (e.type) {
    case 'dusk':
      return 'Nacht naht!';
    case 'night':
      return `Nacht ${e.day}`;
    case 'dawn':
      return `Tag ${e.day}  –  die Sonne geht auf`;
    case 'wave':
      return e.count > 0 ? `Portal öffnet sich!\n${e.count} Gegner kommen` : null;
    case 'chest':
      return `P${e.player + 1} findet ${e.gold} Gold`;
    case 'skillPoint':
      return 'Skill-Punkt gefunden!';
    case 'recruited':
      return null; // passiert oft, sieht man in der Welt
    case 'armed':
      return 'Neuer Bogenschütze';
    case 'built':
      return `${SITE_NAMES[e.kind]} fertig`;
    case 'destroyed':
      return `${SITE_NAMES[e.kind]} zerstört!`;
    case 'gathered':
      return null;
    case 'goldStolen':
      return `P${e.player + 1}: ${e.amount} Gold geklaut!`;
    case 'playerDown':
      return `P${e.player + 1} ist gefallen`;
    case 'castleFallen':
      return 'Die Burg ist gefallen!\nGebäude, Truppen und die Hälfte der Vorräte sind verloren';
    case 'arrived':
      return e.name;
  }
}
