import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { ECONOMY } from '../model/data';
import type { GameEvent, World } from '../model/types';
import type { GameScene } from './GameScene';
import { SHARED_LINE_HEIGHT, sharedAnchor } from './layout';
import { gameNotice } from './lobbyLogic';
import { RadarLayer } from './radarView';
import { RESOURCE_NAMES } from './worldRenderer';

const STYLE = { fontSize: '28px', color: '#ffffff', stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
const BANNER_SECONDS = 2.8;
/** Hinweise passend zum zuletzt benutzten Eingabegerät */
const CONTROL_HINTS = {
  touch: 'Links/rechts berühren = laufen · Münz-Taste halten = Münzen geben',
  pad: 'A halten = Münzen geben · RT = sprinten · RS = Vollbild',
  keyboard: 'Leertaste halten = Münzen geben · Shift = sprinten · F = Vollbild',
} as const;
const JOIN_HINTS = {
  touch: 'Münz-Taste drücken zum Beitreten',
  pad: 'A drücken zum Beitreten',
  keyboard: 'Leertaste drücken zum Beitreten',
} as const;
const SITE_NAMES = { wall: 'Mauer', tower: 'Turm', workshop: 'Werkstatt', stairsUp: 'Treppe hoch', stairsDown: 'Treppe runter' } as const;

/** Bildschirmfeste Anzeigen: pro Split-Screen-Hälfte Spielerwerte, oben rechts Hub-Vorrat und Tageszeit, Meldungen in der Mitte. */
export class HudScene extends Phaser.Scene {
  private joinHint!: Phaser.GameObjects.Text;
  private shared!: Phaser.GameObjects.Text;
  private clock!: Phaser.GameObjects.Text;
  private fight!: Phaser.GameObjects.Text;
  private banner!: Phaser.GameObjects.Text;
  private fps!: Phaser.GameObjects.Text;
  private controlsHint!: Phaser.GameObjects.Text;
  private info!: Phaser.GameObjects.Text;
  private travel!: Phaser.GameObjects.Text;
  private playerLabels: Phaser.GameObjects.Text[] = [];
  private radar!: RadarLayer;
  private bannerQueue: string[] = [];
  private bannerLeft = 0;

  constructor() {
    super('hud');
  }

  create(): void {
    this.playerLabels = [];
    this.bannerQueue = [];
    this.bannerLeft = 0;
    this.radar = new RadarLayer(this);

    this.shared = this.add.text(GAME_WIDTH - 24, 16, '', STYLE).setOrigin(1, 0);
    this.clock = this.add.text(GAME_WIDTH - 24, 56, '', STYLE).setOrigin(1, 0);
    this.fight = this.add.text(GAME_WIDTH - 24, 96, '', { ...STYLE, color: '#ff8fa3' }).setOrigin(1, 0);
    this.banner = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, '', { ...STYLE, fontSize: '56px', strokeThickness: 10, align: 'center' })
      .setOrigin(0.5)
      .setVisible(false);
    this.joinHint = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, 'Drücke  A  (Controller), Leertaste oder die Münz-Taste zum Beitreten', { ...STYLE, fontSize: '44px' })
      .setOrigin(0.5);
    this.fps = this.add.text(20, GAME_HEIGHT - 40, '', { ...STYLE, fontSize: '20px', strokeThickness: 4 });
    this.controlsHint = this.add.text(GAME_WIDTH - 20, GAME_HEIGHT - 40, '', { ...STYLE, fontSize: '20px', strokeThickness: 4 }).setOrigin(1, 0);
    this.info = this.add.text(20, 16, '', { ...STYLE, fontSize: '20px', strokeThickness: 4 });
    this.travel = this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 170, '', { ...STYLE, fontSize: '40px', color: '#ffd166' }).setOrigin(0.5);
  }

  update(_time: number, deltaMs: number): void {
    const game = this.game.scene.getScene('game') as GameScene;
    const world = game.world;
    this.showHints(game, world);
    if (!world) return;
    this.showCells(game, world.players);
    this.radar.draw(game.hudCells(), world);
    this.showWorld(world);
    this.placeShared(game);
    this.showBanner(game, deltaMs);
    this.fps.setText(`${Math.round(this.game.loop.actualFps)} FPS`);
  }

  /** Hinweise: Verbindungsstand, Beitritt, Steuerung, Raum. */
  private showHints(game: GameScene, world: World | undefined): void {
    const client = game.client;
    const away = gameNotice(client);
    if (away) {
      this.joinHint.setText(away).setVisible(true).setY(GAME_HEIGHT / 2 + 120).setFontSize(44); // Verbindung weg: Hinweis statt Standbild
      return;
    }
    const waiting = !world || game.waitingForJoin();
    this.controlsHint.setText(CONTROL_HINTS[game.lastDevice]);
    this.joinHint.setText(world ? JOIN_HINTS[game.lastDevice] : (client.notice ?? 'Verbinde …'));
    this.joinHint.setVisible(waiting || client.you.length < (client.limits?.slotsPerDevice ?? 4));
    this.joinHint.setY(waiting ? GAME_HEIGHT / 2 + 120 : GAME_HEIGHT - 100);
    this.joinHint.setFontSize(waiting ? 44 : 26);
    const taken = client.monarchs.filter((m) => m !== 'free').length;
    this.info.setText(client.roomCode ? `Raum ${client.roomCode} · ${client.roomName} · ${taken} Spieler` : '').setVisible(!!client.roomCode).setY(60);
  }

  /** Gemeinsame Anzeigen stehen je nach Layout oben rechts oder mittig am Kreuzpunkt (B-084). */
  private placeShared(game: GameScene): void {
    const { x, y, originX } = sharedAnchor(game.hudCells().map((h) => h.cell));
    [this.shared, this.clock, this.fight].forEach((text, i) => text.setPosition(x, y + i * SHARED_LINE_HEIGHT).setOrigin(originX, 0));
  }

  /** Vorrat, Tageszeit, Kampf und Reise. */
  private showWorld(world: World): void {
    const t = world.travel;
    const target = t ? (t.via === 'stairsUp' ? 'Aufstieg' : 'Abstieg') + ` in Tiefe ${t.toDepth}` : '';
    this.travel.setText(t ? `${target}  ${'▮'.repeat(Math.ceil(t.progress * 10))}${'▯'.repeat(10 - Math.ceil(t.progress * 10))}` : '');
    const stock = (['wood', 'stone', 'copper'] as const).filter((r) => r === world.biome.primaryResource || world.stock[r] > 0).map((r) => `${RESOURCE_NAMES[r]} ${world.stock[r]}`);
    if (world.skillPoints > 0) stock.push(`Skill-Punkte ${world.skillPoints}`);
    this.shared.setText(stock.join('  ·  '));
    this.clock.setText(clockText(world));
    const enemies = world.enemies.length + world.spawnQueue.length;
    this.fight.setText(enemies > 0 ? `Welle ${world.wave}: ${enemies} Gegner` : '');
  }

  private showBanner(game: GameScene, deltaMs: number): void {
    for (const e of game.pendingEvents.splice(0)) {
      const text = eventText(e);
      if (text) this.bannerQueue.push(text);
    }
    this.bannerLeft -= deltaMs / 1000;
    if (this.bannerLeft > 0) return;
    const next = this.bannerQueue.shift();
    this.banner.setVisible(!!next);
    if (!next) return;
    this.banner.setText(next);
    this.bannerLeft = this.bannerQueue.length > 2 ? BANNER_SECONDS / 2 : BANNER_SECONDS;
  }

  /** Spielerwerte je Feld. */
  private showCells(game: GameScene, players: World['players']): void {
    let label = 0;
    for (const { cell, monarch } of game.hudCells()) {
      const p = monarch === null ? undefined : players.find((q) => q.index === monarch);
      if (!p) continue;
      const text = (this.playerLabels[label] ??= this.add.text(0, 0, '', STYLE));
      label += 1;
      text.setPosition(cell.x + 24, cell.y + 16);
      const status = p.respawnIn > 0 ? `gefallen · zurück in ${Math.ceil(p.respawnIn)} s` : `HP ${Math.ceil(p.hp)}`;
      text.setText(`P${p.index + 1}  ·  Gold ${p.gold}/${ECONOMY.purse.maxGold}  ·  ${status}`);
    }
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
