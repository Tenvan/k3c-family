import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { nameOf, t } from '../core/texts';
import { BIOMES } from '../model/biome';
import { ECONOMY } from '../model/data';
import type { GameEvent, World } from '../model/types';
import type { GameScene } from './GameScene';
import { SHARED_LINE_HEIGHT, sharedAnchor } from './layout';
import { debugEnabled } from './debugOverlay';
import { DebugOverlay } from './debugOverlayView';
import { gameNotice } from './lobbyLogic';
import { FONTS, fontStyle } from './fontRules';
import { RadarLayer, type RadarCell } from './radarView';
import { SkillMenuLayer } from './skillMenuView';
import { ActionOverlay } from './actionOverlay';
import { GuideOverlay } from './guideOverlay';
import { pauseButton } from './pauseButton';
import { GamepadInput, type PlayerInput } from '../input/playerInput';
import { TouchInput } from '../input/touchInput';
import type { Device } from '../input/slotBindings';
import { resourceName, siteName } from './worldRenderer';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
const BANNER_SECONDS = 2.8;
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
  private skills!: SkillMenuLayer;
  private actions!: ActionOverlay;
  private guide!: GuideOverlay;
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
    this.skills = new SkillMenuLayer(this);
    this.actions = new ActionOverlay(this);
    this.guide = new GuideOverlay(this);

    const shared = { ...STYLE, ...fontStyle('shared') };
    this.shared = this.add.text(GAME_WIDTH - 24, 16, '', shared).setOrigin(1, 0);
    this.clock = this.add.text(GAME_WIDTH - 24, 56, '', shared).setOrigin(1, 0);
    this.fight = this.add.text(GAME_WIDTH - 24, 96, '', { ...STYLE, ...fontStyle('fight') }).setOrigin(1, 0);
    this.banner = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, '', { ...STYLE, ...fontStyle('banner'), strokeThickness: 10, align: 'center' })
      .setOrigin(0.5)
      .setVisible(false);
    this.joinHint = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, t('hud.joinCenter'), { ...STYLE, ...fontStyle('joinCenter') })
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
    const seats = [...game.client.you].sort((a, b) => a.slot - b.slot); // Reihenfolge wie `hudCells` (cell.seat)
    const slotOf = (seat: number) => seats[seat]?.slot;
    const device = (slot: number) => deviceOf(game.slots.bound[slot]) ?? game.lastDevice;
    this.actions.draw(cells, game.cameras.cameras, slotOf, device);
    this.guide.draw(cells, game.cameras.cameras, slotOf, device);
    this.skills.draw(cells, game.skillMenus, slotOf, device);
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
    this.controlsHint.setText(t(`hud.hint.${game.lastDevice}`));
    this.joinHint.setText(world ? t(`hud.join.${game.lastDevice}`) : (client.notice ?? t('net.connecting')));
    this.joinHint.setVisible(waiting || client.you.length < (client.limits?.slotsPerDevice ?? 4));
    this.joinHint.setY(waiting ? GAME_HEIGHT / 2 + 120 : GAME_HEIGHT - 100);
    this.joinHint.setFontSize(waiting ? FONTS.joinCenter.px : FONTS.joinCorner.px);
    const taken = client.monarchs.filter((m) => m !== 'free').length;
    this.info.setText(client.roomCode ? t('hud.room', { code: client.roomCode, name: client.roomName, n: taken }) : '').setVisible(!!client.roomCode);
    // ☰ bei Touch oben links: die Raum-Zeile steht rechts daneben auf Höhe seiner Oberkante, unter dem Home-Button (B-336)
    const button = pauseButton().rect();
    if (button) this.info.setPosition(this.scale.transformX(button.right) + 16, this.scale.transformY(button.top));
    else this.info.setPosition(20, 60);
  }

  /** Gemeinsame Anzeigen stehen je nach Layout oben rechts oder mittig am Kreuzpunkt (B-084). */
  private placeShared(game: GameScene): void {
    const { x, y, originX } = sharedAnchor(game.hudCells().map((h) => h.cell));
    [this.shared, this.clock, this.fight].forEach((text, i) => text.setPosition(x, y + i * SHARED_LINE_HEIGHT).setOrigin(originX, 0));
  }

  /** Vorrat, Tageszeit, Kampf und Reise. */
  private showWorld(world: World): void {
    const tr = world.travel;
    const target = tr ? t(tr.via === 'stairsUp' ? 'hud.travelUp' : 'hud.travelDown', { depth: tr.toDepth }) : '';
    this.travel.setText(tr ? `${target}  ${'▮'.repeat(Math.ceil(tr.progress * 10))}${'▯'.repeat(10 - Math.ceil(tr.progress * 10))}` : '');
    const stock = (['wood', 'stone', 'copper'] as const).filter((r) => r === world.biome.primaryResource || world.stock[r] > 0).map((r) => `${resourceName(r)} ${world.stock[r]}`);
    if (world.skillPoints > 0) stock.push(t('hud.skillPoints', { n: world.skillPoints }));
    this.shared.setText(stock.join('  ·  '));
    this.clock.setText(clockText(world));
    const enemies = world.enemies.length + world.spawnQueue.length;
    this.fight.setText(enemies > 0 ? t('hud.wave', { wave: world.wave, n: enemies }) : '');
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
    this.playerLabels.forEach((l) => l.setVisible(false));
    let label = 0;
    for (const { cell, monarch, world } of cells) {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      if (!p || !world) continue;
      const text = (this.playerLabels[label] ??= this.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue') }));
      label += 1;
      text.setVisible(true).setPosition(cell.x + 24, cell.y + 16);
      const status = p.respawnIn > 0 ? t('hud.down', { s: Math.ceil(p.respawnIn) }) : t('hud.hp', { hp: Math.ceil(p.hp) });
      const stage = nameOf('biome', world.biome.id, world.biome.name);
      const short = p.respawnIn > 0 ? t('hud.downShort', { s: Math.ceil(p.respawnIn) }) : status;
      text.setText(t(compact ? 'hud.playerShort' : 'hud.playerFull', { p: p.index + 1, gold: p.gold, max: ECONOMY.purse.maxGold, status: compact ? short : status, stage }));
    }
  }
}

/** Gerät einer Eingabe, damit jede Skill-Leiste die Tasten ihres Spielers zeigt */
function deviceOf(input: PlayerInput | null | undefined): Device | undefined {
  if (input instanceof GamepadInput) return 'pad';
  if (input instanceof TouchInput) return 'touch';
  return input ? 'keyboard' : undefined;
}

function formatTime(seconds: number): string {
  const s = Math.max(0, Math.ceil(seconds));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`;
}

function clockText(w: World): string {
  const left = formatTime(w.cycle.secondsLeft / w.cycleSpeed);
  const key = w.cycle.phase === 'day' ? 'hud.clockDay' : w.cycle.phase === 'dusk' ? 'hud.clockDusk' : 'hud.clockNight';
  let time = t(key, { day: w.cycle.day, left });
  if (w.aggression !== null) time += t('hud.aggression', { pct: Math.floor(w.aggression) });
  return time;
}

/** Der Server nennt den Namen der Daten; die Textdatei übersetzt ihn über die Biom-ID. */
function biomeName(name: string): string {
  const biome = BIOMES.find((b) => b.name === name);
  return biome ? nameOf('biome', biome.id, name) : name;
}

function eventText(e: GameEvent): string | null {
  switch (e.type) {
    case 'dusk':
      return t('ev.dusk');
    case 'night':
      return t('ev.night', { day: e.day });
    case 'dawn':
      return t('ev.dawn', { day: e.day });
    case 'wave':
      return e.count > 0 ? t('ev.wave', { n: e.count }) : null;
    case 'chest':
      return t('ev.chest', { p: e.player + 1, gold: e.gold });
    case 'skillPoint':
      return t('ev.skillPoint');
    case 'recruited':
      return null; // passiert oft, sieht man in der Welt
    case 'armed':
      return t('ev.armed');
    case 'built':
      return t('ev.built', { name: siteName(e.kind) });
    case 'destroyed':
      return t('ev.destroyed', { name: siteName(e.kind) });
    case 'gathered':
      return null;
    case 'goldStolen':
      return t('ev.goldStolen', { p: e.player + 1, amount: e.amount });
    case 'playerDown':
      return t('ev.playerDown', { p: e.player + 1 });
    case 'castleFallen':
      return t('ev.castleFallen');
    case 'arrived':
      return biomeName(e.name);
    default:
      return null;
  }
}
