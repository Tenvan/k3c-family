import type { BiomeConfig } from './biome';
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

export type SiteKind = 'wall' | 'tower' | 'workshop' | 'storage' | 'stairsUp' | 'stairsDown';
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

// ---------- vom Client genutzt, früher in world/ ----------

export type ChunkKind = string; // 'hub' | 'edge' | 'exit' | 'portal' | EventChunkKind | Biom-Chunk-Typ

export interface Chunk {
  index: number;
  kind: ChunkKind;
  startUnits: number;
}

export interface LevelEntity {
  kind: string; // 'castle' | 'portal' | 'exit' | 'tree' | 'rock' | 'chest' | 'skillPoint' | 'recruitCamp' | ...
  x: number; // Units
}

export interface LevelLayout {
  seed: string | number;
  biomeId: string;
  widthUnits: number;
  chunkWidthUnits: number;
  hubCenterUnits: number;
  chunks: Chunk[];
  entities: LevelEntity[];
}

export type Phase = 'day' | 'dusk' | 'night';

export interface CycleInfo {
  phase: Phase;
  /** 1 = erster Tag. Die Nacht gehört zum Tag davor. */
  day: number;
  /** 0..1 innerhalb der Phase */
  progress: number;
  secondsLeft: number;
}

export const SAVE_VERSION = 1;

export interface HubSave {
  depth: number;
  castleHp: number;
  stock: Stock;
  wave: number;
  aggression: number | null;
  sites: { kind: string; x: number; state: SiteState; paidGold: number; buildProgress: number; hp: number; bows: number; bowPaidGold: number }[];
  /** Nur Bauern und Bogenschützen. Landstreicher kommen von allein aus den Camps. */
  troops: { kind: TroopKind; x: number; anchorX: number }[];
  /** Schlüssel `kind@x` der Bäume/Felsen, die schon abgebaut sind bzw. markiert */
  nodesGone: string[];
  nodesMarked: string[];
  pickupsTaken: string[];
}

export interface SaveGame {
  version: number;
  /** Gleiche id = gleiches Spiel. Der Server legt beim Überschreiben eines anderen Spiels eine Sicherung an. */
  campaignId: string;
  savedAt: string;
  seed: string;
  depth: number;
  unlockedDepth: number;
  time: number;
  skillPoints: number;
  players: { gold: number }[];
  hubs: HubSave[];
}

/** Prüft grob, ob ein JSON ein Spielstand ist, den wir laden können. */
export function isSaveGame(data: unknown): data is SaveGame {
  const s = data as SaveGame;
  return (
    !!s &&
    s.version === SAVE_VERSION &&
    typeof s.campaignId === 'string' &&
    typeof s.seed === 'string' &&
    typeof s.depth === 'number' &&
    typeof s.time === 'number' &&
    Array.isArray(s.hubs) &&
    Array.isArray(s.players)
  );
}
