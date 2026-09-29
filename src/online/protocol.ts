import type { GameEvent, PlayerCommand, World } from '../world/sim/types';

/**
 * Online-Modus: Der Server rechnet die Simulation, Clients schicken nur ihre Eingabe und zeichnen den Zustand.
 * Statisches (Biom, Level, Seed) baut jeder Client selbst aus dem Seed, nur Dynamisches geht über die Leitung.
 */

export const ONLINE_PATH = '/ws';
export const MAX_ONLINE_PLAYERS = 8;
export const TICK_HZ = 30;

export type ClientMessage =
  | { t: 'join'; room: string; clientId: string; depth: number; fast: boolean; seed?: string }
  | ({ t: 'input' } & PlayerCommand);

export type ServerMessage =
  | { t: 'welcome'; you: number; room: string; seed: string; depth: number; fast: boolean }
  | { t: 'state'; s: Snapshot }
  | { t: 'error'; message: string };

const STATIC_KEYS = ['seed', 'biome', 'level', 'rng', 'widthUnits'] as const;
export type Snapshot = Omit<World, (typeof STATIC_KEYS)[number]> & { events: GameEvent[] };

export function snapshotWorld(w: World): Snapshot {
  const copy: Record<string, unknown> = { ...w };
  for (const key of STATIC_KEYS) delete copy[key];
  return copy as unknown as Snapshot;
}

export function applySnapshot(w: World, s: Snapshot): void {
  Object.assign(w, s);
}

export function sanitizeInput(m: Partial<PlayerCommand>): PlayerCommand {
  const x = Number(m.moveX);
  return { moveX: Number.isFinite(x) ? Math.max(-1, Math.min(1, x)) : 0, sprint: m.sprint === true, pay: m.pay === true };
}

export function sanitizeRoom(code: unknown): string | null {
  return typeof code === 'string' && /^[\w-]{1,24}$/.test(code) ? code : null;
}
