import { UNIT_PX } from '@game/core/constants';
import { EFFECT_CONFIG, type EffectKind, type SkillLook } from '@game/scenes/effects/effects';
import { skillFrame, type SkillPiece } from '@game/scenes/effects/skillLooks';
import { SHIELD_MAX, statusMarks, type StatusKind, type StatusMark, type StatusWorld } from '@game/scenes/effects/statusLooks';
import type { Enemy, Player } from '@game/model/types';
import type { Control } from './figure';

// Zeitachse der abgespielten Effekte (B-671, B-672): Skill-Effekte, Geschosse, Warnkreis und Beschwörung laufen in einem
// Zyklus, den Schleife, Tempo und Einzelbild der Wiedergabe steuern. Reine Logik ohne Phaser; gezeichnet wird in
// `effectView.ts` mit den Funktionen des Spiels (`skillFrame`).

/** Ein Effekt, der im Takt der Wiedergabe läuft. `cycleMs` ist der Abstand zweier Auslösungen (Intervall), `at` die Lage
 *  relativ zur Figur in Units, `y` die Höhe über dem Boden in Pixeln. */
export type TimedEffect =
  /** Ring und Partikel wie `StageEffects.spawn` (Skill mit Look, sonst ein Effekt aus `EFFECT_CONFIG`); `delayMs` nach der Auslösung */
  | { kind: 'burst'; look: SkillLook; cycleMs: number; at?: number; y?: number; delayMs?: number }
  /** Geschoss des Spiels von der Figur bis `range` Units in Richtung `dir` mit `speed` Units/s */
  | { kind: 'projectile'; range: number; dir: 1 | -1; speed: number; color: number; cycleMs: number }
  /** Warnkreis des Spiels (`WarnView`) mit Radius in Units, Auslösung am Ende jedes Zyklus */
  | { kind: 'warn'; radius: number; cycleMs: number }
  /** Die Figur selbst erscheint mit jeder Auslösung (Beschwörung) */
  | { kind: 'appear'; cycleMs: number }
  /** Platzhalter-Rahmen an der Stelle einer Figur ohne Grafik (Lücke), erscheint wie `appear` */
  | { kind: 'frame'; cycleMs: number; at: number }
  /** Status-Effekt des Spiels (B-675, B-676) an der Figur für `durationMs`, `delayMs` nach der Auslösung: Radius in Units
   *  (Sturm, Aura), `skill` färbt die Aura, `share` füllt den Schild-Balken (0..1) */
  | { kind: 'status'; status: StatusKind; cycleMs: number; durationMs: number; delayMs?: number; radius?: number; skill?: string; share?: number; castleAt?: number };

/** Geschoss des Spiels (`worldRenderer`: `scene.add.rectangle(0, 0, 26, 3, …)` in `G - 70`, Farbe des Gegner-Teams) und
 *  sein Tempo (`engine/sim`: 20 Units/s für Fernkämpfer und Splitter); ein Test prüft die Werte an den Quellen. */
export const PROJECTILE = { width: 26, height: 3, y: 70, enemyColor: 0xff4d6d, speed: 20 } as const;

/** Pause nach einem Effekt, bevor er in der Schleife wieder beginnt (wie der Ring der Aktionen) */
export const REPEAT_PAUSE_MS = 700;
/** Dauer des Ein- und Ausblendens einer beschworenen Figur */
export const APPEAR_MS = 400;

/** Effekt aus `EFFECT_CONFIG` als Look (Bewegung `burst`), wie `StageEffects.spawn` ihn ohne eigenen Look zeichnet */
export const configLook = (kind: EffectKind): SkillLook => ({ ...EFFECT_CONFIG[kind], motion: 'burst' });

/** Sichtbare Dauer eines Effekts in Millisekunden (ohne Pause bis zur nächsten Auslösung) */
export function durationOf(e: TimedEffect): number {
  switch (e.kind) {
    case 'burst': return (e.delayMs ?? 0) + e.look.durationMs;
    case 'projectile': return projectileMs(e.range, e.speed);
    case 'status': return (e.delayMs ?? 0) + e.durationMs;
    default: return e.cycleMs;
  }
}

/** Flugzeit eines Geschosses bis `range` Units bei `speed` Units/s */
export const projectileMs = (range: number, speed: number): number => (speed > 0 ? (Math.abs(range) / speed) * 1000 : 0);

/** Fortschritt eines Einzelbilds (0..1, nie 0): Bild `frame` von `frames` steht in der Mitte seines Abschnitts. */
export const frameProgress = (frame: number, frames: number): number => (frames > 0 ? Math.min(1, (frame + 0.5) / frames) : 0);

/** Laufzeit nach `deltaMs` Bildschirmzeit: Tempo wirkt, Einzelbild hält an. */
export const advance = (elapsedMs: number, deltaMs: number, c: Control): number => (c.frame === null ? elapsedMs + deltaMs * c.tempo : elapsedMs);

/** Zeit im Zyklus: im Einzelbild der Fortschritt des Bilds über die sichtbare Dauer, ohne Schleife nach dem ersten Zyklus
 *  das Ende (nichts mehr zu sehen), sonst die Zeit seit der letzten Auslösung. */
export function cycleTime(e: TimedEffect, c: Control, elapsedMs: number, frames: number): number {
  const visible = durationOf(e);
  if (c.frame !== null) return frameProgress(c.frame, frames) * visible;
  if (!c.loop && elapsedMs >= e.cycleMs) return e.kind === 'appear' || e.kind === 'frame' ? visible / 2 : e.cycleMs;
  return e.cycleMs > 0 ? elapsedMs % e.cycleMs : 0;
}

/** Ring und Partikel eines Bursts zur Zeit `t` im Zyklus (relativ zum Effekt-Ort); leer vor der Verzögerung und nach dem Ende.
 *  Die Teile rechnet `skillFrame` des Spiels. */
export function burstPieces(look: SkillLook, t: number, delayMs = 0): SkillPiece[] {
  const local = t - delayMs;
  if (local < 0 || local >= look.durationMs) return [];
  return skillFrame(look, local / look.durationMs);
}

/** Lage des Geschosses in Pixeln relativ zur Figur zur Zeit `t`; null, solange keines fliegt. */
export function projectileOffset(e: Extract<TimedEffect, { kind: 'projectile' }>, t: number): number | null {
  const ms = projectileMs(e.range, e.speed);
  if (ms <= 0 || t >= ms) return null;
  return e.dir * e.range * UNIT_PX * (t / ms);
}

/** Sekunden bis zur Auslösung des Warnkreises (`warn.in` des Spiels) zur Zeit `t` */
export const warnIn = (cycleMs: number, t: number): number => Math.max(0, cycleMs - t) / 1000;

/** Deckkraft einer beschworenen Figur: blendet nach der Auslösung ein und vor der nächsten aus. */
export function appearAlpha(cycleMs: number, t: number): number {
  const fade = Math.min(APPEAR_MS, cycleMs / 4);
  if (fade <= 0) return 1;
  return Math.max(0, Math.min(1, t / fade, (cycleMs - t) / fade));
}

type StatusEffect = Extract<TimedEffect, { kind: 'status' }>;

/** Zustand, in dem das Spiel genau diesen Status-Effekt an einer Figur bei x 0 zeigt (Gegner bzw. Monarch, lebend). */
function statusWorld(e: StatusEffect): StatusWorld {
  const enemy = (over: Partial<Enemy>) => ({ enemies: [{ id: 1, x: 0, hp: 1, ...over } as Enemy] });
  const player = (over: Partial<Player>, castle?: { x: number }): StatusWorld => ({ players: [{ id: 1, x: 0, respawnIn: 0, ...over } as Player], ...(castle ? { castle } : {}) });
  switch (e.status) {
    case 'stun': return enemy({ stun: 1 });
    case 'slow': return enemy({ slow: 0.5, slowFor: 1 });
    case 'taunt': return enemy({ tauntId: 1, tauntFor: 1 });
    case 'storm': return { storms: [{ x: 0, radius: e.radius ?? 0, perSecond: 0, left: 1, owner: 1 }] };
    case 'shield': return player({ shield: (e.share ?? 1) * SHIELD_MAX, shieldFor: 1 });
    case 'lastStand': return player({ lastStandFor: 1 });
    default: {
      // Holy Ground (castleAt gesetzt) wirkt im Spiel an der Burg, nicht am Monarchen
      const castle = e.castleAt !== undefined;
      return player({ auras: [{ skill: e.skill ?? '', radius: e.radius ?? 0, ...(castle ? { castle: true } : {}) }] }, castle ? { x: e.castleAt! } : undefined);
    }
  }
}

/** Markierungen eines Status-Effekts zur Zeit `t` im Zyklus, wie `statusMarks` des Spiels sie für die Figur bei x 0 liefert
 *  (Pixel der Spiel-Ebene); leer vor der Verzögerung und nach der Dauer. */
export function statusAt(e: StatusEffect, t: number): StatusMark[] {
  const local = t - (e.delayMs ?? 0);
  if (local < 0 || local >= e.durationMs) return [];
  return statusMarks(statusWorld(e));
}
