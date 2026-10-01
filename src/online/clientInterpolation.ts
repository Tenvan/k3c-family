import type { WorldState } from './clientProtocol';

/**
 * Interpolation zwischen den zwei neuesten Zuständen (B-039): Die Darstellung läuft einen Tick hinter dem Server.
 * Bewegliche Einträge (mit `id` und `x`) gleiten linear, alles andere springt zum neueren Zustand.
 * Es gibt keine Extrapolation: Fällt ein Zustand aus, bleiben die Figuren stehen.
 */

const MOVING = ['players', 'troops', 'enemies', 'projectiles'] as const;
/** Größere Sprünge (Respawn, Teleport) werden nicht überblendet */
const TELEPORT_UNITS = 15;

type Mover = { id: number; x: number; y?: number };

const lerp = (a: number, b: number, t: number) => a + (b - a) * t;

function blendList<T extends Mover>(from: readonly T[], to: readonly T[], alpha: number): T[] {
  const before = new Map(from.map((e) => [e.id, e]));
  return to.map((e) => {
    const old = before.get(e.id);
    if (!old || Math.abs(e.x - old.x) > TELEPORT_UNITS) return e;
    const blended: T = { ...e, x: lerp(old.x, e.x, alpha) };
    if (typeof old.y === 'number' && typeof e.y === 'number') blended.y = lerp(old.y, e.y, alpha);
    return blended;
  });
}

/** Zustand zwischen `a` (älter) und `b` (neuer): `alpha` 0 = a, 1 = b. */
export function interpolate(a: WorldState, b: WorldState, alpha: number): WorldState {
  if (alpha >= 1) return b;
  const t = Math.max(0, alpha);
  const next: WorldState = { ...b };
  for (const key of MOVING) (next[key] as Mover[]) = blendList(a[key] as Mover[], b[key] as Mover[], t);
  return next;
}

/** Anteil des Weges von Zustand a nach b: beginnt beim Eintreffen von b und dauert einen Tick. */
export function blendAlpha(now: number, receivedAt: number, tickMs: number): number {
  return Math.min(1, Math.max(0, (now - receivedAt) / tickMs));
}
