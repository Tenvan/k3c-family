import type { AnimName } from './sprites';

/**
 * Welche Animation der Monarch zeigt (GR9.3, B-505). Rein: liest nur den Snapshot des Spielers, den des letzten
 * gezeichneten Frames und einen Hinweis aus den Ereignissen (`coinGive` → `give`); entscheidet nichts über das Spiel.
 * Tod und Respawn folgen `respawnIn` (> 0 = tot), ein Treffer dem Sinken von `hp` (der Effekt `hit` kann wegen der
 * Blitzgrenze entfallen, B-164). Fehlt dem Sheet eine Animation, gilt der bisherige Rückfall (idle, tot: Deckkraft 0,25).
 */

export interface MonarchFrame {
  respawnIn: number;
  hp: number;
  moving: boolean;
}

export interface MonarchPick {
  anim: AnimName;
  /** Einmal-Animation von vorn beginnen (neuer Treffer, neuer Wurf) */
  restart: boolean;
  alpha: number;
}

/** Deckkraft eines toten Monarchen ohne eigene Tod-Animation */
export const DEAD_ALPHA = 0.25;

export function pickMonarchAnim(prev: MonarchFrame | undefined, cur: MonarchFrame, cue: 'give' | null, has: (anim: AnimName) => boolean): MonarchPick {
  const loop: MonarchPick = { anim: cur.moving && has('run') ? 'run' : 'idle', restart: false, alpha: 1 };
  if (cur.respawnIn > 0) return has('death') ? { anim: 'death', restart: false, alpha: 1 } : { anim: 'idle', restart: false, alpha: DEAD_ALPHA };
  if (prev && prev.respawnIn > 0) return has('respawn') ? { anim: 'respawn', restart: true, alpha: 1 } : loop;
  if (prev && cur.hp < prev.hp && has('hit')) return { anim: 'hit', restart: true, alpha: 1 };
  if (cue === 'give' && has('give')) return { anim: 'give', restart: true, alpha: 1 };
  return loop;
}
