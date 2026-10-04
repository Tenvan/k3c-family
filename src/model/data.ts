import buildingsJson from '../../data/buildings.json';
import economyJson from '../../data/economy.json';
import enemiesJson from '../../data/enemies.json';
import hubJson from '../../data/hub.json';
import monarchJson from '../../data/monarch.json';
import troopsJson from '../../data/troops.json';
import wavesJson from '../../data/waves.json';
import type { Range } from './biome';
import type { SiteKind } from './types';

/** Typisierter Zugriff auf die Balancing-Daten aus data/. Werte gehören in die JSON, nicht in den Code. */

export type ResourceKind = 'wood' | 'stone' | 'copper';
export type Cost = Partial<Record<ResourceKind | 'gold', number>>;

export interface BuildingData {
  name: string;
  hp: number;
  buildSeconds: number;
  cost: Cost;
  archerSlots?: number;
  rangeBonus?: number;
  bowRack?: number;
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
  gold: Range;
  traits: string[];
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
export const ENEMIES = enemiesJson as unknown as Record<string, EnemyData>;
export const MONARCH = monarchJson;
export const HUB = hubJson as {
  sites: { kind: SiteKind; offsetUnits: number; fromDepth?: number; needsDeeper?: boolean }[];
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
  recruitCamp: { maxVagrants: number; respawnSeconds: number; wanderUnits: number };
};
export const WAVES = wavesJson as unknown as {
  table: WaveRow[];
  spawnSpreadSeconds: number;
  depthScaling: { hp: number; damage: number; speed: number };
  attacksPerSecond: number;
  stealGold: number;
};
