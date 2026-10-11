import Phaser from 'phaser';
import { GAME_WIDTH } from '../core/constants';
import { nameOf, t } from '../core/texts';
import { BIOMES } from '../model/biome';
import { ECONOMY } from '../model/data';
import type { GameEvent, World } from '../model/types';
import type { GameScene } from './GameScene';
import { debugEnabled } from './debugOverlay';
import { DebugOverlay } from './debugOverlayView';
import { gameNotice } from './lobbyLogic';
import { FONTS, fontStyle } from './fontRules';
import { PauseBanner, pauseShown } from './pauseBanner';
import { RadarLayer, type RadarCell } from './radarView';
import { SkillMenuLayer } from './skillMenuView';
import { ActionOverlay } from './actionOverlay';
import { GuideOverlay } from './guideOverlay';
import { HudBox, freeAreas } from './hudElements';
import { hudItems, hudLayout, type HAlign, type Rect, type Size } from './hudLayout';
import { cellRadar, radarRect, RADAR_HEIGHT } from './radar';
import type { HintDevice } from './glyphs';
import { GamepadInput, type PlayerInput } from '../input/playerInput';
import { TouchInput } from '../input/touchInput';
import { siteName } from './worldRenderer';
import { hubText, stockTexts } from './wirtschaftAnzeige';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
const BANNER_SECONDS = 2.8;
/**
 * Bildschirmfeste Anzeigen: pro Split-Screen-Hälfte Spielerwerte, oben rechts Hub-Vorrat und Tageszeit, Meldungen in der Mitte.
 * Jede Anzeige ist ein HUD-Element (`HudBox`); die Lage kommt aus `hudLayout` (B-337), neu berechnet nur bei Änderung.
 */
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
  /** Spielerzeile je Zellen-Index */
  private players = new Map<number, HudBox>();
  /** Feste Elemente nach Layout-Id */
  private boxes = new Map<string, HudBox>();
  private layoutKey = '';
  private layout = new Map<string, Rect | null>();
  private aligns = new Map<string, HAlign>();
  private radar!: RadarLayer;
  private skills!: SkillMenuLayer;
  private actions!: ActionOverlay;
  private guide!: GuideOverlay;
  private pause!: PauseBanner;
  /** Stehen die Figuren-Animationen wegen der Pause still? (nur beim Wechsel umschalten) */
  private animsPaused = false;
  private bannerQueue: string[] = [];
  private bannerLeft = 0;

  constructor() {
    super('hud');
  }

  create(): void {
    this.players = new Map();
    this.boxes = new Map();
    this.layoutKey = '';
    this.bannerQueue = [];
    this.bannerLeft = 0;
    this.radar = new RadarLayer(this);
    this.skills = new SkillMenuLayer(this);
    this.actions = new ActionOverlay(this);
    this.guide = new GuideOverlay(this);
    this.pause = new PauseBanner(this);
    this.animsPaused = false;
    // `leaveRoom` stoppt das HUD mitten in der Pause: Der Animations-Manager ist global, die Figuren sollen im nächsten Raum laufen
    this.events.once(Phaser.Scenes.Events.SHUTDOWN, () => {
      if (this.animsPaused) this.anims.resumeAll();
      this.animsPaused = false;
    });

    const shared = { ...STYLE, ...fontStyle('shared') };
    this.shared = this.boxed('shared', this.add.text(0, 0, '', shared));
    this.clock = this.boxed('clock', this.add.text(0, 0, '', shared));
    this.fight = this.boxed('fight', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('fight') }));
    this.banner = this.boxed('banner', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('banner'), strokeThickness: 10, align: 'center' }).setVisible(false));
    this.joinHint = this.boxed('join', this.add.text(0, 0, t('hud.joinCenter'), { ...STYLE, ...fontStyle('joinCenter') }));
    this.debug = debugEnabled(location.search) ? new DebugOverlay(this) : null;
    this.controlsHint = this.boxed('controls', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('controlsHint'), strokeThickness: 4 }));
    this.info = this.boxed('info', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('roomInfo'), strokeThickness: 4 }));
    this.travel = this.boxed('travel', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('travel') }));
  }

  private boxed(id: string, text: Phaser.GameObjects.Text): Phaser.GameObjects.Text {
    this.boxes.set(id, new HudBox(this, text));
    return text;
  }

  update(_time: number, deltaMs: number): void {
    const game = this.game.scene.getScene('game') as GameScene;
    const world = game.world;
    this.debug?.update(game.client, world ?? null);
    const waiting = this.showHints(game, world);
    if (!world) {
      this.showPause(undefined, null, []);
      this.arrange([], waiting);
      return;
    }
    const cells = game.hudCells();
    this.showCells(cells);
    const seats = [...game.client.you].sort((a, b) => a.slot - b.slot); // Reihenfolge wie `hudCells` (cell.seat)
    const slotOf = (seat: number) => seats[seat]?.slot;
    const device = (slot: number) => deviceOf(game.slots.bound[slot], game.keyboards[1]) ?? game.lastDevice;
    this.actions.draw(cells, game.cameras.cameras, slotOf, device);
    this.guide.draw(cells, game.cameras.cameras, slotOf, device);
    this.skills.draw(cells, game.skillMenus, slotOf, device);
    // gemeinsamer Block: Stufe der ersten Zelle dieses Geräts; kurze Form bei Viertel-Zellen
    this.showWorld(cells.find((c) => c.cell.kind === 'player' && c.world)?.world ?? world, cells.some((c) => c.cell.w < GAME_WIDTH));
    this.showBanner(game, deltaMs);
    this.showPause(world, gameNotice(game.client), cells);
    this.arrange(cells, waiting);
  }

  /** Pause-Anzeige je Zelle; die Figuren-Animationen halten nur beim Wechsel an bzw. laufen weiter (AZ1.2, B-333) */
  private showPause(world: World | undefined, away: string | null, cells: readonly RadarCell[]): void {
    const shown = pauseShown(world, away);
    this.pause.draw(cells, shown);
    if (shown === this.animsPaused) return;
    this.animsPaused = shown;
    if (shown) this.anims.pauseAll();
    else this.anims.resumeAll();
  }

  /** Legt alle HUD-Elemente über `hudLayout` ab; neu gerechnet wird nur, wenn sich Elemente, Größen oder Freiflächen ändern. */
  private arrange(cells: readonly RadarCell[], waiting: boolean): void {
    const all = new Map(this.boxes);
    for (const [i, b] of this.players) all.set(`player:${i}`, b);
    for (const [i, b] of this.skills.bars) all.set(`skills:${i}`, b);
    const sizes = new Map<string, Size>();
    for (const [id, b] of all) {
      const s = b.size();
      if (s) sizes.set(id, s);
    }
    cells.forEach((c, i) => cellRadar(c) && sizes.set(`radar:${i}`, { w: radarRect(c.cell).w, h: RADAR_HEIGHT }));
    const items = hudItems(cells.map((c) => c.cell), sizes, waiting);
    const free = freeAreas(this, this.debug?.visible ?? false);
    const key = JSON.stringify([items, free]);
    if (key !== this.layoutKey) {
      this.layoutKey = key;
      this.layout = hudLayout(items, free);
      this.aligns = new Map(items.map((it) => [it.id, it.h]));
    }
    for (const [id, b] of all) b.place(this.layout.get(id), this.aligns.get(id) ?? 'left');
    this.radar.draw(cells, (i) => this.layout.get(`radar:${i}`));
  }

  /** Hinweise: Verbindungsstand, Beitritt, Steuerung, Raum. Rückgabe: Der Beitritts-Hinweis steht groß in der Mitte. */
  private showHints(game: GameScene, world: World | undefined): boolean {
    const client = game.client;
    const away = gameNotice(client);
    if (away) {
      this.joinHint.setText(away).setVisible(true).setFontSize(FONTS.joinCenter.px); // Verbindung weg: Hinweis statt Standbild
      return true;
    }
    const waiting = !world || game.waitingForJoin();
    this.controlsHint.setText(t(`hud.hint.${game.lastDevice}`));
    this.joinHint.setText(world ? t(`hud.join.${game.lastDevice}`) : (client.notice ?? t('net.connecting')));
    this.joinHint.setVisible(waiting || client.you.length < (client.limits?.slotsPerDevice ?? 4));
    this.joinHint.setFontSize(waiting ? FONTS.joinCenter.px : FONTS.joinCorner.px);
    const taken = client.monarchs.filter((m) => m !== 'free').length;
    this.info.setText(client.roomCode ? t('hud.room', { code: client.roomCode, name: client.roomName, n: taken }) : '').setVisible(!!client.roomCode);
    return waiting; // ☰, Home-Button und Touch-Knöpfe spart `arrange` als Freiflächen aus (B-336, B-337)
  }

  /** Vorrat, Tageszeit, Kampf und Reise. */
  private showWorld(world: World, compact: boolean): void {
    const tr = world.travel;
    const target = tr ? t(tr.via === 'stairsUp' ? 'hud.travelUp' : 'hud.travelDown', { depth: tr.toDepth }) : '';
    this.travel.setText(tr ? `${target}  ${'▮'.repeat(Math.ceil(tr.progress * 10))}${'▯'.repeat(10 - Math.ceil(tr.progress * 10))}` : '');
    const stock = stockTexts(world, compact);
    if (world.skillPoints > 0) stock.push(t('hud.skillPoints', { n: world.skillPoints }));
    const hub = hubText(world, compact);
    this.shared.setText(hub ? `${stock.join('  ·  ')}\n${hub}` : stock.join('  ·  '));
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
    for (const b of this.players.values()) b.text.setVisible(false);
    for (const [i, { monarch, world }] of cells.entries()) {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      if (!p || !world) continue;
      let box = this.players.get(i);
      if (!box) this.players.set(i, (box = new HudBox(this, this.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue') }))));
      const status = p.respawnIn > 0 ? t('hud.down', { s: Math.ceil(p.respawnIn) }) : t('hud.hp', { hp: Math.ceil(p.hp) });
      const stage = nameOf('biome', world.biome.id, world.biome.name);
      const short = p.respawnIn > 0 ? t('hud.downShort', { s: Math.ceil(p.respawnIn) }) : status;
      box.text.setVisible(true).setText(t(compact ? 'hud.playerShort' : 'hud.playerFull', { p: p.index + 1, gold: p.gold, max: ECONOMY.purse.maxGold, status: compact ? short : status, stage }));
    }
  }
}

/** Gerät einer Eingabe, damit jede Skill-Leiste die Tasten ihres Spielers zeigt; `keyboard2` = Spieler 2 an der Tastatur */
function deviceOf(input: PlayerInput | null | undefined, keyboard2: PlayerInput): HintDevice | undefined {
  if (input instanceof GamepadInput) return 'pad';
  if (input instanceof TouchInput) return 'touch';
  if (input === keyboard2) return 'keyboard2';
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
