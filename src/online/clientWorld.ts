import type { World } from '../model/types';
import type { LevelInfo } from './clientConnection';
import type { WorldState } from './clientProtocol';

/**
 * Das `World`-Objekt, das die Darstellung zeichnet: Statisches aus `level` (Layout, Biom), Dynamisches aus dem Zustand.
 * Der Browser rechnet damit nichts; `rng` gibt es nicht (nur der Server würfelt).
 */

function dynamic(state: WorldState): Omit<WorldState, 'depth'> {
  const { depth: _depth, ...rest } = state;
  return rest;
}

export function createViewWorld(level: LevelInfo, state: WorldState): World {
  const { layout, biome } = level;
  return { ...dynamic(state), seed: String(layout.seed), biome, level: layout, widthUnits: layout.widthUnits, rng: undefined as never };
}

/** Neuen Zustand in dasselbe Objekt schreiben, damit der Renderer seinen Verweis behält. */
export function applyState(world: World, state: WorldState): void {
  Object.assign(world, dynamic(state));
}
