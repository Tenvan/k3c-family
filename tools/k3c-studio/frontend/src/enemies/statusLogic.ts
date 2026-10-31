import { REPEAT_PAUSE_MS, type TimedEffect } from '../playback/timeline';
import { skillStatus } from '../monarch/skillTarget';

// Modus „Status“ der Gegner-Seite (B-676): die Status-Effekte des Spiels an Gegnern (B-675) nacheinander über ihre Dauer,
// je einer aus dem Skill, der ihn auslöst (Dauer und Radius aus `data/monarch.json`).

/** Skills, deren Wirkung am Gegner zu sehen ist: Betäubung, Verlangsamung, Spott, Sturm */
export const ENEMY_STATUS_SKILLS = ['shieldBash', 'iceWall', 'taunt', 'lightningStorm'] as const;

/** Die Status-Effekte am Gegner nacheinander in einem gemeinsamen Zyklus (jeder nach dem vorigen, dann Pause). */
export function enemyStatusEffects(): TimedEffect[] {
  const parts = ENEMY_STATUS_SKILLS.flatMap((id) => skillStatus(id, { kind: 'goblin', distance: 0 }).onTarget)
    .filter((e): e is Extract<TimedEffect, { kind: 'status' }> => e.kind === 'status');
  const total = parts.reduce((sum, e) => sum + e.durationMs, 0) + REPEAT_PAUSE_MS;
  let delay = 0;
  return parts.map((e) => {
    const out: TimedEffect = { ...e, delayMs: delay, cycleMs: total };
    delay += e.durationMs;
    return out;
  });
}
