import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { ECONOMY } from '../model/data';
import type { GameEvent, World } from '../model/types';
import type { GameScene } from './GameScene';
import { SHARED_LINE_HEIGHT, sharedAnchor } from './layout';
import { debugEnabled } from './debugOverlay';
import { DebugOverlay } from './debugOverlayView';
import { gameNotice } from './lobbyLogic';
import { FONTS, fontStyle } from './fontRules';
import { RadarLayer, type RadarCell } from './radarView';
import { RESOURCE_NAMES } from './worldRenderer';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
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
const SITE_NAMES = { wall: 'Mauer', tower: 'Turm', workshop: 'Werkstatt', storage: 'Lager', stairsUp: 'Treppe hoch', stairsDown: 'Treppe runter' } as const;

/** Bildschirmfeste Anzeigen: pro Split-Screen-Hälfte Spielerwerte, oben rechts Hub-Vorrat und Tageszeit, Meldungen in der Mitte. */
export class HudScene extends Phaser.Scene {
  private joinHint!: Phaser.GameObjects.Text;
  private shared!: Phaser.GameObjects.Text;
  private clock!: Phaser.GameObjects.Text;
  private fight!: Phaser.GameObjects.Text;
  private banner!: Phaser.GameObjects.Text;
  private debug: DebugOverlay | null = null;
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

    const shared = { ...STYLE, ...fontStyle('shared') };
    this.shared = this.add.text(GAME_WIDTH - 24, 16, '', shared).setOrigin(1, 0);
    this.clock = this.add.text(GAME_WIDTH - 24, 56, '', shared).setOrigin(1, 0);
    this.fight = this.add.text(GAME_WIDTH - 24, 96, '', { ...STYLE, ...fontStyle('fight') }).setOrigin(1, 0);
    this.banner = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, '', { ...STYLE, ...fontStyle('banner'), strokeThickness: 10, align: 'center' })
      .setOrigin(0.5)
      .setVisible(false);
    this.joinHint = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, 'Drücke  A  (Controller), Leertaste oder die Münz-Taste zum Beitreten', { ...STYLE, ...fontStyle('joinCenter') })
      .setOrigin(0.5);
    this.debug = debugEnabled(location.search) ? new DebugOverlay(this) : null;
    this.controlsHint = this.add.text(GAME_WIDTH - 20, GAME_HEIGHT - 40, '', { ...STYLE, ...fontStyle('controlsHint'), strokeThickness: 4 }).setOrigin(1, 0);
    this.info = this.add.text(20, 16, '', { ...STYLE, ...fontStyle('roomInfo'), strokeThickness: 4 });
    this.travel = this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 170, '', { ...STYLE, ...fontStyle('travel') }).setOrigin(0.5);
  }

  update(_time: number, deltaMs: number): void {
    const game = this.game.scene.getScene('game') as GameScene;
    const world = game.world;
    this.debug?.update(game.client, world ?? null);
    this.showHints(game, world);
    if (!world) return;
    const cells = game.hudCells();
    this.showCells(cells);
    this.radar.draw(cells);
    this.showWorld(cells.find((c) => c.cell.kind === 'player' && c.world)?.world ?? world); // gemeinsamer Block: Stufe der ersten Zelle dieses Geräts
    this.placeShared(game);
    this.showBanner(game, deltaMs);
  }

  /** Hinweise: Verbindungsstand, Beitritt, Steuerung, Raum. */
  private showHints(game: GameScene, world: World | undefined): void {
    const client = game.client;
    const away = gameNotice(client);
    if (away) {
      this.joinHint.setText(away).setVisible(true).setY(GAME_HEIGHT / 2 + 120).setFontSize(FONTS.joinCenter.px); // Verbindung weg: Hinweis statt Standbild
      return;
    }
    const waiting = !world || game.waitingForJoin();
    this.controlsHint.setText(CONTROL_HINTS[game.lastDevice]);
    this.joinHint.setText(world ? JOIN_HINTS[game.lastDevice] : (client.notice ?? 'Verbinde …'));
    this.joinHint.setVisible(waiting || client.you.length < (client.limits?.slotsPerDevice ?? 4));
    this.joinHint.setY(waiting ? GAME_HEIGHT / 2 + 120 : GAME_HEIGHT - 100);
    this.joinHint.setFontSize(waiting ? FONTS.joinCenter.px : FONTS.joinCorner.px);
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

  /** Spielerwerte je Feld, aus der Welt der Stufe dieses Feldes; Feld ohne geladene Stufe zeigt nichts. */
  private showCells(cells: readonly RadarCell[]): void {
    const compact = cells.some((c) => c.cell.w < GAME_WIDTH); // 3 bis 4 Spieler: Text kürzen statt verkleinern (Q03)
    this.playerLabels.forEach((t) => t.setVisible(false));
    let label = 0;
    for (const { cell, monarch, world } of cells) {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      if (!p || !world) continue;
      const text = (this.playerLabels[label] ??= this.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue') }));
      label += 1;
      text.setVisible(true).setPosition(cell.x + 24, cell.y + 16);
      const status = p.respawnIn > 0 ? `gefallen · zurück in ${Math.ceil(p.respawnIn)} s` : `HP ${Math.ceil(p.hp)}`;
      const stage = world.biome.name;
      text.setText(
        compact
          ? `P${p.index + 1} · ${p.gold}/${ECONOMY.purse.maxGold} · ${p.respawnIn > 0 ? `${Math.ceil(p.respawnIn)} s` : status} · ${stage}`
          : `P${p.index + 1}  ·  Gold ${p.gold}/${ECONOMY.purse.maxGold}  ·  ${status}  ·  ${stage}`,
      );
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
    default:
      return null;
  }
}
