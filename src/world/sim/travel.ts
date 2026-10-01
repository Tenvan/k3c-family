import { BIOMES } from '../../model/biome';
import { isAlive } from './common';
import { HUB } from '../../model/data';
import type { TravelPoint, World } from '../../model/types';

/**
 * Stufenwechsel: Der Tiefen-Eingang am Levelende führt nach unten, gebaute Treppen im Hub verbinden die Hubs.
 * Alle lebenden Spieler müssen gemeinsam am selben Punkt stehen (Couch-Koop: niemand wird zurückgelassen).
 */

export const hasDepth = (depth: number): boolean => BIOMES.some((b) => b.depth === depth);

export function travelPoints(w: World): TravelPoint[] {
  const depth = w.biome.depth;
  const points: TravelPoint[] = [];
  if (hasDepth(depth + 1)) {
    for (const e of w.level.entities) if (e.kind === 'exit') points.push({ x: e.x, toDepth: depth + 1, via: 'exit' });
  }
  for (const s of w.sites) {
    if (s.state !== 'built') continue;
    if (s.kind === 'stairsUp' && hasDepth(depth - 1)) points.push({ x: s.x, toDepth: depth - 1, via: 'stairsUp' });
    if (s.kind === 'stairsDown' && hasDepth(depth + 1)) points.push({ x: s.x, toDepth: depth + 1, via: 'stairsDown' });
  }
  return points;
}

export function stepTravel(w: World, dt: number): void {
  const alive = w.players.filter(isAlive);
  const point =
    alive.length === 0 ? undefined : travelPoints(w).find((p) => alive.every((pl) => Math.abs(pl.x - p.x) <= HUB.travel.rangeUnits));
  if (!point) {
    w.travel = null;
    return;
  }
  if (!w.travel || w.travel.x !== point.x) w.travel = { ...point, progress: 0 };
  w.travel.progress = Math.min(1, w.travel.progress + dt / HUB.travel.seconds);
}
