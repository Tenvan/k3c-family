import type { Rng } from '../../core/rng';
import type { BiomeConfig } from '../biome';
import type { LevelLayout } from '../levelGenerator';
import type { CycleInfo } from './cycle';
import type { ResourceKind } from './data';

/**
 * Zustand der Simulation. Reine Daten, alle Positionen in Units, Zeiten in Sekunden.
 * Phaser liest diesen Zustand nur zum Zeichnen (src/scenes/worldRenderer.ts).
 */

export type { ResourceKind };
export type Stock = Record<ResourceKind, number>;

/** Eingabe eines Spielers für einen Tick. */
export interface PlayerCommand {
  /** -1 (links) bis 1 (rechts) */
  moveX: number;
  sprint: boolean;
  /** Bezahl-Taste gehalten (A / Leertaste): Münze geben bzw. fallen lassen */
  pay: boolean;
}

export const IDLE: PlayerCommand = { moveX: 0, sprint: false, pay: false };

export interface Player {
  id: number;
  index: number;
  x: number;
  vx: number;
  facing: 1 | -1;
  gold: number;
  hp: number;
  maxHp: number;
  /** > 0: tot, Sekunden bis zum Respawn an der Burg */
  respawnIn: number;
  payCooldown: number;
  paying: boolean;
  /** Ziel, an dem gerade gezahlt wird ("site:3"), und wie viel Gold davon noch nicht vollendet ist */
  payKey: string | null;
  payAmount: number;
}

export interface Coin {
  id: number;
  x: number;
  /** Wer die Münze fallen lässt, kann sie kurz nicht wieder aufheben (sonst geht Geben nicht). */
  blockedPlayerId: number | null;
  blockedUntil: number;
}

export type TroopKind = 'vagrant' | 'peasant' | 'archer';

export type Job =
  | { type: 'build'; siteId: number }
  | { type: 'gather'; nodeId: number }
  | { type: 'carry'; resource: ResourceKind; amount: number }
  | { type: 'fetchBow'; siteId: number };

export interface Troop {
  id: number;
  kind: TroopKind;
  x: number;
  hp: number;
  maxHp: number;
  /** Bezugspunkt: Camp (Landstreicher) bzw. Hub */
  anchorX: number;
  /** Aktuelles Laufziel */
  targetX: number;
  job: Job | null;
  /** Angriffs-Cooldown bzw. Warten beim Umherwandern */
  cooldown: number;
  towerId: number | null;
  /** Landstreicher: bereits bezahltes Rekrutierungs-Gold */
  paidGold: number;
}

/** Baum, Fels, Kupfererz: per Münze markieren, dann holt ein Bauer die Ressource. */
export interface ResourceNode {
  id: number;
  kind: string;
  x: number;
  paidGold: number;
  marked: boolean;
  workerId: number | null;
  /** 0..1 */
  progress: number;
}

export type SiteKind = 'wall' | 'tower' | 'workshop' | 'stairsUp' | 'stairsDown';
export type SiteState = 'unpaid' | 'waitingMaterial' | 'waitingWorker' | 'built';

export interface Site {
  id: number;
  kind: SiteKind;
  x: number;
  state: SiteState;
  paidGold: number;
  /** 0..1 */
  buildProgress: number;
  hp: number;
  maxHp: number;
  workerId: number | null;
  /** Werkstatt: fertige Bögen im Regal */
  bows: number;
  /** Werkstatt: bereits bezahltes Gold für den nächsten Bogen */
  bowPaidGold: number;
}

export interface Castle {
  id: number;
  x: number;
  hp: number;
  maxHp: number;
}

export interface Enemy {
  id: number;
  kind: string;
  x: number;
  hp: number;
  maxHp: number;
  damage: number;
  speed: number;
  range: number;
  traits: readonly string[];
  cooldown: number;
  fleeing: boolean;
  carriedGold: number;
  /** Portal, aus dem der Gegner kam (Fluchtziel) */
  homeX: number;
}

export interface Projectile {
  id: number;
  x: number;
  targetId: number;
  team: 'player' | 'enemy';
  damage: number;
  speed: number;
}

export interface Pickup {
  id: number;
  kind: 'chest' | 'skillPoint';
  x: number;
}

export interface Camp {
  x: number;
  respawnIn: number;
}

export interface QueuedSpawn {
  kind: string;
  x: number;
  at: number;
}

export type GameEvent =
  | { type: 'dusk' }
  | { type: 'night'; day: number }
  | { type: 'dawn'; day: number }
  | { type: 'wave'; wave: number; count: number }
  | { type: 'chest'; player: number; gold: number }
  | { type: 'skillPoint'; player: number; total: number }
  | { type: 'recruited'; player: number }
  | { type: 'armed' }
  | { type: 'built'; kind: SiteKind }
  | { type: 'destroyed'; kind: SiteKind }
  | { type: 'gathered'; resource: ResourceKind; amount: number }
  | { type: 'goldStolen'; player: number; amount: number }
  | { type: 'playerDown'; player: number }
  | { type: 'castleFallen' }
  | { type: 'arrived'; depth: number; name: string };

export interface World {
  seed: string;
  biome: BiomeConfig;
  level: LevelLayout;
  rng: Rng;
  /** Simulierte Sekunden seit Start */
  time: number;
  /** Faktor für den Tag/Nacht-Zyklus (Dev: ?fast=1) */
  cycleSpeed: number;
  nextId: number;

  widthUnits: number;
  hubX: number;
  cycle: CycleInfo;
  /** Nur unter Tage (Aggressionspool-Biome), sonst null. 0..100 */
  aggression: number | null;
  wave: number;

  players: Player[];
  coins: Coin[];
  troops: Troop[];
  nodes: ResourceNode[];
  sites: Site[];
  castle: Castle;
  enemies: Enemy[];
  projectiles: Projectile[];
  pickups: Pickup[];
  camps: Camp[];
  portals: number[];
  spawnQueue: QueuedSpawn[];

  /** Baumaterial gehört allen gemeinsam (Hub-Vorrat), Gold hat jeder Spieler selbst. */
  stock: Stock;
  skillPoints: number;

  /** Alle Spieler stehen an einem Tiefen-Eingang / einer Treppe: Fortschritt 0..1, bei 1 wechselt die Kampagne die Stufe. */
  travel: Travel | null;

  /** Ereignisse des letzten Ticks (für HUD-Meldungen), werden bei jedem step() geleert. */
  events: GameEvent[];
}

export interface TravelPoint {
  x: number;
  toDepth: number;
  via: 'exit' | 'stairsUp' | 'stairsDown';
}

export interface Travel extends TravelPoint {
  progress: number;
}
