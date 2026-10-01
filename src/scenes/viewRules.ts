import { BIOMES } from '../world/biome';
import type { CycleInfo } from '../world/sim/cycle';
import type { Troop, World } from '../world/sim/types';

/**
 * Darstellungsregeln, die der Browser selbst auswertet, ohne die Simulation zu importieren (SP08, AC-13).
 * Sie spiegeln src/world/sim/{economy,travel,units,cycle}.ts; die TS-Simulation entfällt mit SP09.
 */

const RESOURCES = ['wood', 'stone', 'copper'] as const;
/** Abstand in Units, ab dem ein Bogenschütze als „auf dem Turm“ gilt (wie ARRIVE in sim/units.ts) */
const ON_TOWER_UNITS = 0.3;

export function canAfford(stock: World['stock'], cost: Partial<Record<(typeof RESOURCES)[number], number>>): boolean {
  return RESOURCES.every((r) => stock[r] >= (cost[r] ?? 0));
}

/** Gibt es eine Stufe in dieser Tiefe (aus den Biom-Daten)? */
export function hasDepth(depth: number): boolean {
  return BIOMES.some((b) => b.depth === depth);
}

export function isOnTower(world: Pick<World, 'sites'>, troop: Troop): boolean {
  if (troop.towerId === null) return false;
  const tower = world.sites.find((s) => s.id === troop.towerId);
  return !!tower && Math.abs(tower.x - troop.x) < ON_TOWER_UNITS;
}

/** Helligkeit für die Darstellung: 1 = Tag, 0 = tiefste Nacht. */
export function daylight(info: CycleInfo): number {
  if (info.phase === 'day') return 1;
  if (info.phase === 'dusk') return 1 - info.progress;
  // Letzte 10% der Nacht: Morgengrauen
  return info.progress > 0.9 ? (info.progress - 0.9) * 10 : 0;
}
