import buildingsJson from '../../data/buildings.json';
import economyJson from '../../data/economy.json';
import enemiesJson from '../../data/enemies.json';
import hubJson from '../../data/hub.json';
import lightJson from '../../data/light.json';
import monarchJson from '../../data/monarch.json';
import troopsJson from '../../data/troops.json';
import wavesJson from '../../data/waves.json';
import type { Range } from './biome';
import type { SiteKind } from './types';

/** Typisierter Zugriff auf die Balancing-Daten aus data/. Werte gehören in die JSON, nicht in den Code. */

/** Rohstoffe, die der Server im Vorrat bei 0 weglässt (docs/protocol.md › Wirtschaft). */
export type RareResource = 'iron' | 'crystal';
export type ResourceKind = 'wood' | 'stone' | 'copper' | RareResource;
export type Cost = Partial<Record<ResourceKind | 'gold', number>>;

export interface BuildingData {
  name: string;
  hp: number;
  buildSeconds: number;
  cost: Cost;
  archerSlots?: number;
  rangeBonus?: number;
  bowRack?: number;
  /** Ausbau-Stufen am selben Platz (B-569); fehlt: keine eigenen Stufen */
  levels?: { cost: Cost; hp: number; buildSeconds: number }[];
}

export interface TroopData {
  name: string;
  hp: number;
  speed: number;
  damage: number;
  range: number;
  attacksPerSecond: number;
  cost?: Cost;
  recruitCost?: Cost;
}

export interface EnemyData {
  name: string;
  depth: number;
  tier: 'standard' | 'elite';
  hp: number;
  damage: number;
  speed: number;
  range: number;
  attacksPerSecond: number;
  gold: Range;
  traits: string[];
  /** Greif-Reichweite für Ausrüstung am Boden (Units) */
  grabRange?: number;
  /** Abstand, den Fernkämpfer halten (Units) */
  kiteDistance?: number;
  /** Flächenschaden: Radius (Units) und Abstand der Treffer (s) */
  aoe?: { radius: number; intervalSeconds: number };
  swarmSize?: number;
  /** Phasen: alle `everySeconds` für `durationSeconds` */
  phases?: { everySeconds: number; durationSeconds: number };
  notes?: string;
}

export interface GatherableData {
  resource: ResourceKind;
  amount: number;
  workSeconds: number;
  markCost: number;
}

export interface WaveRow {
  fromWave: number;
  standard: Range;
  elite: Range;
}

export const BUILDINGS = buildingsJson as unknown as Record<string, BuildingData>;
export const TROOPS = troopsJson as unknown as Record<'vagrant' | 'peasant' | 'archer', TroopData>;
/**
 * Form einer JSON-Datei zum Typ T: JSON-Importe kennen keine Tupel und keine Literal-Typen. Die Zuweisung an `JsonOf<T>`
 * prüft, dass die Datei alle Felder von T hat; danach ist der Cast auf T nur noch die Verengung (B-324).
 */
type JsonOf<T> = T extends readonly (infer E)[] ? JsonOf<E>[] : T extends string ? string : T extends object ? { [K in keyof T]: JsonOf<T[K]> } : T;

const enemiesFile: Record<string, JsonOf<EnemyData>> = enemiesJson;
export const ENEMIES = enemiesFile as Record<string, EnemyData>;
export const MONARCH = monarchJson;
/** Bauplatz aus `data/hub.json` (`sites`, `islandSites`); `hubLevel` = ab welcher Hub-Stufe er frei ist. */
export interface HubSite {
  kind: SiteKind;
  offsetUnits: number;
  hubLevel: number;
  fromDepth?: number;
  needsDeeper?: boolean;
}

export const HUB = hubJson as {
  sites: HubSite[];
  islandSites: HubSite[];
  wallLines: {
    wallUnits: number[];
    fixedLines: number;
    jitterOutwardUnits: number;
    towerInsetUnits: number;
    gateOutsetUnits: number;
    gateHubLevel: number;
  };
  castleRadiusUnits: number;
  homeRadiusUnits: number;
  startTroops: Partial<Record<'peasant' | 'archer', number>>;
  travel: { rangeUnits: number; seconds: number };
};
export const ECONOMY = economyJson as unknown as {
  purse: { startGold: number; maxGold: number };
  payRangeUnits: number;
  payIntervalSeconds: number;
  pickupRangeUnits: number;
  dropPickupDelaySeconds: number;
  dawnGoldPerPlayer: number;
  chestGold: Range;
  enemyResourceDrop: { chance: number; amount: number };
  gatherables: Record<string, GatherableData>;
  veins: Record<string, unknown>;
  recruitCamp: { maxVagrants: number; respawnSeconds: number; wanderUnits: number };
};
export interface WavesData {
  table: WaveRow[];
  spawnSpreadSeconds: number;
  /** Zuschlag auf die Gegnerzahl je weiterem Spieler */
  perExtraPlayer: number;
  depthScaling: { hp: number; damage: number; speed: number };
  stealGold: number;
}
const wavesFile: JsonOf<WavesData> = wavesJson;
export const WAVES = wavesFile as WavesData;

/** Lichtquelle in Nacht und Morgengrauen (B-472): Radius in Units, Aufhellung in der Mitte 0..1 (1 = Tag). */
export interface LightSource {
  radius: number;
  strength: number;
}
export type LightSourceKind = 'torch' | 'campfire';
/** `nightBrightness`: Kamerahelligkeit in tiefster Nacht (W13); `glowColor`: Farbe des Lichtscheins (`#rrggbb`) */
export const LIGHT = lightJson as { nightBrightness: number; glowColor: string; sources: Partial<Record<LightSourceKind, LightSource>> };
