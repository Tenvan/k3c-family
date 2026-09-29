import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, MAX_PLAYERS } from '../core/constants';
import { ECONOMY } from '../world/sim/data';
import type { GameEvent, World } from '../world/sim/types';
import type { GameScene } from './GameScene';
import { RESOURCE_NAMES } from './worldRenderer';

const STYLE = { fontSize: '28px', color: '#ffffff', stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
const BANNER_SECONDS = 2.8;
/** Hinweise passend zum zuletzt benutzten Eingabegerät */
const CONTROL_HINTS = {
  touch: 'Münz-Taste halten = Münzen geben · » = sprinten',
  pad: 'A halten = Münzen geben · RT = sprinten · RS = Vollbild',
  keyboard: 'Leertaste halten = Münzen geben · Shift = sprinten · F = Vollbild · Dev: N neuer Seed · 1/2/3 Tiefe',
} as const;
const JOIN_HINTS = {
  touch: 'Münz-Taste drücken zum Beitreten',
  pad: 'A drücken zum Beitreten',
  keyboard: 'Leertaste drücken zum Beitreten',
} as const;
const SITE_NAMES = { wall: 'Mauer', tower: 'Turm', workshop: 'Werkstatt' } as const;

/** Bildschirmfeste Anzeigen: pro Split-Screen-Hälfte Spielerwerte, oben rechts Hub-Vorrat und Tageszeit, Meldungen in der Mitte. */
export class HudScene extends Phaser.Scene {
  private joinHint!: Phaser.GameObjects.Text;
  private shared!: Phaser.GameObjects.Text;
  private clock!: Phaser.GameObjects.Text;
  private fight!: Phaser.GameObjects.Text;
  private banner!: Phaser.GameObjects.Text;
  private fps!: Phaser.GameObjects.Text;
  private controlsHint!: Phaser.GameObjects.Text;
  private playerLabels: Phaser.GameObjects.Text[] = [];
  private bannerQueue: string[] = [];
  private bannerLeft = 0;

  constructor() {
    super('hud');
  }

  create(): void {
    const game = this.game.scene.getScene('game') as GameScene;
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
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, 'Drücke  A  (Controller), Leertaste oder die Münz-Taste zum Beitreten', { ...STYLE, fontSize: '44px' })
      .setOrigin(0.5);
    this.fps = this.add.text(20, GAME_HEIGHT - 40, '', { ...STYLE, fontSize: '20px', strokeThickness: 4 });
    this.controlsHint = this.add.text(GAME_WIDTH - 20, GAME_HEIGHT - 40, '', { ...STYLE, fontSize: '20px', strokeThickness: 4 }).setOrigin(1, 0);
    this.add.text(20, 16, `${game.world.biome.name} · Seed "${game.world.seed}"`, { ...STYLE, fontSize: '20px', strokeThickness: 4 }).setName('info');
  }

  update(_time: number, deltaMs: number): void {
    const game = this.game.scene.getScene('game') as GameScene;
    const world = game.world;
    const players = world.players;
    const shown = game.hudPlayers();
    const stripHeight = GAME_HEIGHT / Math.max(1, shown.length);

    this.controlsHint.setText(CONTROL_HINTS[game.lastDevice]);
    this.joinHint.setText(JOIN_HINTS[game.lastDevice]);
    this.joinHint.setVisible(!game.online && players.length < MAX_PLAYERS);
    this.joinHint.setY(players.length === 0 ? GAME_HEIGHT / 2 + 120 : GAME_HEIGHT - 100);
    this.joinHint.setFontSize(players.length === 0 ? 44 : 26);
    const info = this.byName('info');
    info?.setVisible(players.length === 0 || !!game.online);
    info?.setY(game.online ? 60 : 16);
    if (game.online) info?.setText(`Online · Raum "${game.online.room}" · ${players.length} Spieler`);

    shown.forEach((playerIndex, i) => {
      const p = players[playerIndex];
      if (!p) return;
      let label = this.playerLabels[i];
      if (!label) {
        label = this.add.text(24, 0, '', STYLE);
        this.playerLabels[i] = label;
      }
      label.setY(i * stripHeight + 16);
      const status = p.respawnIn > 0 ? `gefallen · zurück in ${Math.ceil(p.respawnIn)} s` : `HP ${Math.ceil(p.hp)}`;
      label.setText(`P${p.index + 1}  ·  Gold ${p.gold}/${ECONOMY.purse.maxGold}  ·  ${status}`);
    });

    const stock = (['wood', 'stone', 'copper'] as const).filter((r) => r === 'wood' || world.stock[r] > 0).map((r) => `${RESOURCE_NAMES[r]} ${world.stock[r]}`);
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

  private byName(name: string): Phaser.GameObjects.Text | null {
    return this.children.getByName(name) as Phaser.GameObjects.Text | null;
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
  }
}
