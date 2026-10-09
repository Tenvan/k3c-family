import type buildingsJson from '../../data/buildings.json';
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
  /** Gelernte Skills (IDs, Lernreihenfolge), fehlt = keine (docs/protocol.md › Skills und Aktionen). */
  skills?: string[];
  /** Aktive Skills in den Slots 1 bis 4 (Index 0 bis 3, "" = frei), fehlt = keine. */
  slots?: string[];
  /** Abklingzeit je Slot in Sekunden, fehlt = 0. */
  cooldowns?: number[];
  /** Sekunden bis zum nächsten Schlag, fehlt = 0. */
  attackCooldown?: number;
  /** Verfügbare Skill-Punkte (Pool der Insel minus gelernte Skills), vom Server berechnet. */
  points: number;
  /** Gültige Aktionen am Ort des Spielers, vom Server berechnet. */
  actions: PlayerAction[];
}

/** Eintrag der Aktionsliste; `skill`: Slot 1 bis 4 wie `input.p[].skill`. */
export type PlayerAction =
  | { action: 'attack' }
  | { action: 'skill'; slot: number; skill: string }
  | { action: 'learn' }
  | { action: 'respec' };

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
  /** Beruf eines Bauern (`miner`, `builder`, `craftsman`); fehlt = keiner. */
  profession?: string;
  /** Arbeitsplatz (Site-ID) des Berufs; fehlt = keiner. */
  workSite?: number;
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

/** Jedes Gebäude aus data/buildings.json außer der Burg kann ein Bauplatz sein (der Server legt sie aus den Daten an). */
export type SiteKind = Exclude<keyof typeof buildingsJson, 'castle'>;
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
  /** Ausbau-Stufe (Mauer, Turm); fehlt = 1. */
  level?: number;
  /** Wartegrund des laufenden Ausbaus; fehlt = kein Ausbau oder Gold offen. */
  upgrade?: 'waitingMaterial' | 'waitingWorker';
  /** Bezahltes Gold des laufenden Ausbaus; fehlt = 0. */
  upgradePaid?: number;
}

/** Ausbau auf die nächste Hub-Stufe (docs/protocol.md › Wirtschaft), vom Server berechnet. */
export interface HubUpgrade {
  gold: number;
  material: Partial<Record<ResourceKind | 'iron' | 'crystal', number>>;
  paid: number;
  /** Fehlt, solange Gold offen ist. */
  state?: 'waitingMaterial' | 'waitingWorker';
}

/** Anwesender Händler: Material, Abreisetag, Gold des laufenden Kaufs (fehlt = 0). */
export interface Merchant {
  resource: string;
  leaves: number;
  buyPaid?: number;
}

/** Ausrüstung am Boden (Figur oder Beruf, Verlust-Kaskade). */
export interface Drop {
  id: number;
  kind: string;
  x: number;
  workSite?: number;
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

/**
 * Ereignis eines Ticks. Auf einer Insel trägt jedes Ereignis `stage` (Index seiner Stufe); Orte `x` in Units.
 * Feedback-Ereignisse (hit … revive, B-139) laufen nur im Zustand der eigenen Stufe, Liste in `docs/protocol.md`.
 */
export type GameEvent = (
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
  | { type: 'arrived'; depth: number; name: string; player?: number }
  | { type: 'hit'; x: number; target: 'player' | 'troop' | 'enemy' | 'castle' | 'site'; id: number; damage: number }
  | { type: 'kill'; kind: string; x: number; gold: number }
  | { type: 'arrow'; from: number; to: number; x: number; team: 'player' | 'enemy' }
  | { type: 'strike'; from: number; x: number; hit?: boolean }
  | { type: 'castFailed'; from: number; slot: number; x: number }
  | { type: 'coinPickup'; player: number; x: number }
  | { type: 'coinGive'; player: number; x: number; to: 'site' | 'recruit' | 'mark' }
  | { type: 'buildProgress'; site: number; kind: SiteKind; x: number; percent: 25 | 50 | 75 }
  | { type: 'revive'; player: number; x: number }
  | { type: 'revived'; player: number; x: number }
  | { type: 'disarmed'; kind: string; x: number; cause: string }
  | { type: 'equipmentTaken'; kind: string; x: number }
) & { stage?: number };

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

  /** Wirtschaft (docs/protocol.md › Wirtschaft), alle vom Server; fehlt = nicht vorhanden bzw. älterer Server. */
  stockMax?: number;
  hubLevel?: number;
  hubUpgrade?: HubUpgrade;
  /** Nacht oder Gegner: Bau und Ausbau warten (`waitingWorker` heißt dann Gefahr). */
  danger?: boolean;
  merchant?: Merchant;
  drops?: Drop[];
  armorLevel?: number;
  /** Kämpfer der Stufe und ihr Truppen-Limit (B-332), für den Limit-Text „Kämpfer/Limit“. */
  fighters?: number;
  troopLimit?: number;

  /** Alle Spieler stehen an einem Tiefen-Eingang / einer Treppe: Fortschritt 0..1, bei 1 wechselt die Kampagne die Stufe. */
  travel: Travel | null;

  /** Ereignisse des letzten Ticks (für HUD-Meldungen), werden bei jedem step() geleert. */
  events: GameEvent[];
  /** Vom Server verworfene Ereignisse dieses Ticks (Obergrenze je Tick); fehlt bei 0. */
  eventsDropped?: number;
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
