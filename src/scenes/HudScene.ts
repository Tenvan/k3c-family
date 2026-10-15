import Phaser from 'phaser';
import { GAME_WIDTH } from '../core/constants';
import { BAR_HEIGHT } from './layout';
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
import { BOX, HudBox, freeAreas } from './hudElements';
import { PLAYER_COLORS } from '../core/constants';
import { hudItems, hudLayout, type HAlign, type Rect, type Size } from './hudLayout';
import { cellRadar, radarRect, RADAR_HEIGHT } from './radar';
import type { HintDevice } from './glyphs';
import { joinHintDevices } from './joinHint';
import { stepControlsHint, type ControlsTrack } from './controlsHint';
import { stepGoldFlash, type GoldFlash } from './goldFlash';
import { enqueueBanner } from './bannerQueue';
import { loadSettings } from '../core/settings';
import { GamepadInput, type PlayerInput } from '../input/playerInput';
import { TouchInput } from '../input/touchInput';
import { siteName } from './worldRenderer';
import { hubText, stockTexts } from './wirtschaftAnzeige';
import { buergerLines } from './buergerAnzeige';
import { formatTime } from './kampfAnzeige';
import { KampfHud } from './kampfView';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
const BANNER_SECONDS = 2.8;
/** Leiste oben: dunkelblau, damit sichtbar ist, was auf ihr steht (POC B-413) */
const BAR_COLOR = 0x0a1a4a;
/**
 * Bildschirmfeste Anzeigen: pro Split-Screen-Feld Spielerwerte, oben über die ganze Breite die Leiste für alle (Vorrat, Hub, Tageszeit, Kampf), Meldungen in der Mitte.
 * Jede Anzeige ist ein HUD-Element (`HudBox`); die Lage kommt aus `hudLayout` (B-337), neu berechnet nur bei Änderung.
 */
export class HudScene extends Phaser.Scene {
  private joinHint!: Phaser.GameObjects.Text;
  private bar!: Phaser.GameObjects.Graphics;
  /** Lebensbalken und Goldtopf der Spielerkarten, jedes Bild neu gezeichnet */
  private cards!: Phaser.GameObjects.Graphics;
  /** Zeichenbreite der Spielerzeile: die Karten reservieren ihre Bildflächen mit Leerzeichen (Festschrift) */
  private probe!: Phaser.GameObjects.Text;
  private cardData = new Map<number, CardData>();
  private goldFlash = new Map<number, GoldFlash>();
  private shared!: Phaser.GameObjects.Text;
  private clock!: Phaser.GameObjects.Text;
  private fight!: Phaser.GameObjects.Text;
  private banner!: Phaser.GameObjects.Text;
  private debug: DebugOverlay | null = null;
  private controlsHint!: Phaser.GameObjects.Text;
  private controlsTrack: ControlsTrack | null = null;
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
  private kampf!: KampfHud;
  /** Stehen die Figuren-Animationen wegen der Pause still? (nur beim Wechsel umschalten) */
  private animsPaused = false;
  private bannerQueue: string[] = [];
  private bannerLeft = 0;

  constructor() {
    super('hud');
  }

  create(): void {
    this.players = new Map();
    this.cardData = new Map();
    this.goldFlash = new Map();
    this.boxes = new Map();
    this.layoutKey = '';
    this.bannerQueue = [];
    this.controlsTrack = null;
    this.bannerLeft = 0;
    this.radar = new RadarLayer(this);
    this.skills = new SkillMenuLayer(this);
    this.actions = new ActionOverlay(this);
    this.guide = new GuideOverlay(this);
    this.pause = new PauseBanner(this);
    this.kampf = new KampfHud(this);
    this.animsPaused = false;
    // `leaveRoom` stoppt das HUD mitten in der Pause: Der Animations-Manager ist global, die Figuren sollen im nächsten Raum laufen
    this.events.once(Phaser.Scenes.Events.SHUTDOWN, () => {
      if (this.animsPaused) this.anims.resumeAll();
      this.animsPaused = false;
    });

    // Leiste für alle Spieler: gemeinsamer Hintergrund, die Texte darin haben keinen eigenen
    this.bar = this.add.graphics().fillStyle(BAR_COLOR, 1).fillRect(0, 0, GAME_WIDTH, BAR_HEIGHT).setVisible(false);
    const shared = { ...STYLE, ...fontStyle('shared') };
    this.shared = this.boxed('shared', this.add.text(0, 0, '', shared), false);
    this.clock = this.boxed('clock', this.add.text(0, 0, '', shared), false);
    this.fight = this.boxed('fight', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('fight') }), false);
    this.boxed('places', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('roomInfo'), strokeThickness: 4, align: 'right' }), false);
    this.cards = this.add.graphics();
    this.probe = this.add.text(0, 0, 'MMMMMMMMMM', { ...STYLE, ...fontStyle('playerValue') }).setVisible(false);
    this.boxed('citizens', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('shared') }), false);
    this.banner = this.boxed('banner', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('banner'), strokeThickness: 4, align: 'center' }).setVisible(false));
    this.joinHint = this.boxed('join', this.add.text(0, 0, t('hud.joinCenter'), { ...STYLE, ...fontStyle('joinCenter') }));
    this.debug = debugEnabled(location.search) ? new DebugOverlay(this) : null;
    this.controlsHint = this.boxed('controls', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('controlsHint'), strokeThickness: 4 }));
    this.info = this.boxed('info', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('roomInfo'), strokeThickness: 4 }), false);
    this.travel = this.boxed('travel', this.add.text(0, 0, '', { ...STYLE, ...fontStyle('travel') }));
  }

  private boxed(id: string, text: Phaser.GameObjects.Text, background = true): Phaser.GameObjects.Text {
    this.boxes.set(id, new HudBox(this, text, { background, frame: false }));
    return text;
  }

  update(_time: number, deltaMs: number): void {
    const game = this.game.scene.getScene('game') as GameScene;
    const world = game.world;
    this.debug?.update(game.client, world ?? null);
    const waiting = this.showHints(game, world);
    this.bar.setVisible(!!world);
    if (!world) {
      this.kampf.draw(null);
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
    // Leiste: Stufe der ersten Zelle dieses Geräts, die Leiste hat immer Platz für die lange Form
    const shown = cells.find((c) => c.cell.kind === 'player' && c.world)?.world ?? world;
    this.showWorld(shown, false);
    this.kampf.draw(shown);
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
    this.drawCards();
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
    const controls = stepControlsHint(this.controlsTrack, performance.now(), { seats: client.you.length, device: game.lastDevice, guide: loadSettings().guideHints });
    this.controlsTrack = controls.track;
    this.controlsHint.setText(t(`hud.hint.${game.lastDevice}`)).setVisible(controls.visible);
    const joinText = joinHintDevices(game.joinCandidates(), game.lastDevice).map((d) => t(`hud.join.${d}`)).join('  ·  ');
    this.joinHint.setText(world ? joinText : (client.notice ?? t('net.connecting')));
    this.joinHint.setVisible((waiting || client.you.length < (client.limits?.slotsPerDevice ?? 4)) && (!world || joinText !== ''));
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
    const lines = [stock.join('  ·  '), ...(hub ? [hub] : []), ...buergerLines(world, compact)];
    this.shared.setText(lines.join('\n'));
    this.clock.setText(clockText(world));
    const enemies = world.enemies.length + world.spawnQueue.length;
    this.fight.setText(enemies > 0 ? t('hud.wave', { wave: world.wave, n: enemies }) : '');
  }

  private showBanner(game: GameScene, deltaMs: number): void {
    for (const e of game.pendingEvents.splice(0)) {
      const text = eventText(e);
      if (text) this.bannerQueue = enqueueBanner(this.bannerQueue, this.banner.visible ? this.banner.text : null, text);
    }
    this.bannerLeft -= deltaMs / 1000;
    if (this.bannerLeft > 0) return;
    const next = this.bannerQueue.shift();
    this.banner.setVisible(!!next);
    if (!next) return;
    this.banner.setText(next);
    this.bannerLeft = this.bannerQueue.length > 2 ? BANNER_SECONDS / 2 : BANNER_SECONDS;
  }

  /**
   * Spielerkarte je Feld in der Leiste: Name, Lebensbalken, Goldtopf mit Stand; dazu die Orts-Tabelle mit jedem Spieler.
   * Die Welt der Stufe des Feldes liefert die Werte; ein Feld ohne geladene Stufe zeigt nichts.
   */
  private showCells(cells: readonly RadarCell[]): void {
    const cw = this.probe.width / 10;
    const pad = (px: number) => ' '.repeat(Math.ceil(px / cw));
    for (const b of this.players.values()) b.text.setVisible(false);
    this.cardData.clear();
    const places: string[] = [];
    for (const [i, { monarch, world }] of cells.entries()) {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      if (!p || !world) continue;
      let box = this.players.get(i);
      if (!box) this.players.set(i, (box = new HudBox(this, this.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue') }), { background: false, frame: false })));
      const max = world.purseMax ?? ECONOMY.purse.maxGold;
      const gold = p.respawnIn > 0 ? t('hud.downShort', { s: Math.ceil(p.respawnIn) }) : `${p.gold}/${max}`;
      box.text.setVisible(true).setText(`P${p.index + 1} ${pad(CARD.bar)} ${pad(CARD.pot)} ${gold}`);
      const flash = stepGoldFlash(this.goldFlash.get(p.index), p.gold, performance.now());
      this.goldFlash.set(p.index, flash.state);
      this.cardData.set(i, { cw, hp: Math.max(0, p.hp / p.maxHp), down: p.respawnIn > 0, color: PLAYER_COLORS[p.index % PLAYER_COLORS.length]!, flash: flash.flash });
      places.push(t('hud.place', { p: p.index + 1, stage: nameOf('biome', world.biome.id, world.biome.name) }));
    }
    this.boxes.get('places')?.text.setText(places.join('\n'));
  }

  /** Lebensbalken und Goldtopf in die reservierten Flächen der Karten zeichnen (nach `arrange`, wenn die Lage feststeht) */
  private drawCards(): void {
    const g = this.cards.clear();
    for (const [i, d] of this.cardData) {
      const r = this.layout.get(`player:${i}`);
      if (!r) continue;
      const x = r.x + BOX.padX + d.cw * 3; // hinter „P1 “
      const y = r.y + BOX.padY + 8;
      g.fillStyle(0x000000, 0.7).fillRect(x, y, CARD.bar, CARD.barH);
      const hp = d.hp;
      g.fillStyle(hp > 0.5 ? 0x52b788 : hp > 0.25 ? 0xf4a261 : 0xe63946, 1).fillRect(x + 2, y + 2, Math.max(0, (CARD.bar - 4) * hp), CARD.barH - 4);
      g.lineStyle(2, d.color, 1).strokeRect(x, y, CARD.bar, CARD.barH);
      const px = x + CARD.bar + d.cw; // Goldtopf: bauchiger Topf mit Rand
      g.fillStyle(0xffd166, 1).fillEllipse(px + 12, y + CARD.barH - 2, 24, 22).fillRect(px + 4, y - 6, 16, 6);
      g.fillStyle(0x9c6644, 1).fillRect(px + 2, y - 8, 20, 3);
      if (d.flash) g.lineStyle(3, 0xffffff, 1).strokeEllipse(px + 12, y + CARD.barH - 2, 34, 32);
    }
  }
}

interface CardData {
  cw: number;
  hp: number;
  down: boolean;
  color: number;
  /** Goldzahl hat sich eben geändert: Topf hervorheben (B-398) */
  flash: boolean;
}

/** Maße der Spielerkarte: Breite des Lebensbalkens (und seine Höhe) und der Fläche des Goldtopfs, in px */
const CARD = { bar: 150, barH: 20, pot: 30 } as const;

/** Gerät einer Eingabe, damit jede Skill-Leiste die Tasten ihres Spielers zeigt; `keyboard2` = Spieler 2 an der Tastatur */
function deviceOf(input: PlayerInput | null | undefined, keyboard2: PlayerInput): HintDevice | undefined {
  if (input instanceof GamepadInput) return 'pad';
  if (input instanceof TouchInput) return 'touch';
  if (input === keyboard2) return 'keyboard2';
  return input ? 'keyboard' : undefined;
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
