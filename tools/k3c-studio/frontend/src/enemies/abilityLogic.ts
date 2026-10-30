import spritesJson from '@data/sprites.json';
import { SHEETS, animInfo, gap, type Anim, type FigureSpec, type Gap } from '../playback/figure';
import type { FigureItem } from '../playback/figureView';
import { PROJECTILE, configLook, projectileMs, type TimedEffect } from '../playback/timeline';
import { BOSS_DATA, abilityIn, phaseCount, type Ability, type BossData } from './bossLogic';
import type { EnemyDef } from './enemyLogic';

// Angriffe und Boss-Fähigkeiten abspielen (B-672): reine Logik, welche Effekte des Spiels eine Figur im Takt der Wiedergabe
// zeigt. Was das Spiel zeichnet: Treffer (`hit` aus `EFFECT_CONFIG` am Ziel), Geschoss (`worldRenderer`, 20 Units/s),
// Warnkreis (`WarnView`, Flächenschlag) und beschworene Gegner als gewöhnliche Figuren. Die Bodenfläche der Flammenspur
// (`Hazards`) zeichnet der Client nicht: Lücke.

const SPRITES = spritesJson as unknown as { enemies: Record<string, FigureSpec> };

/** Höhe des Treffer-Effekts über dem Boden (`BODY_PX` in `src/scenes/effects/effects.ts`); ein Test prüft den Wert. */
export const HIT_Y = 40;
/** Pause nach Treffer bzw. Einschlag, bevor der nächste Angriff beginnt */
const ATTACK_PAUSE_MS = 400;
/** Abstand der beschworenen Gegner neben dem Boss in Units */
export const SUMMON_STEP = 1.5;

const hitAt = (at: number, delayMs: number, cycleMs: number): TimedEffect => ({ kind: 'burst', look: configLook('hit'), at, y: HIT_Y, delayMs, cycleMs });

/** Dauer der Angriffs-Animation des Sheets in ms (0, wenn es keine hat) */
function attackMs(sheet: string): number {
  const a = animInfo(sheet, 'attack');
  return a && a.fps > 0 ? (a.frames / a.fps) * 1000 : 0;
}

/** Angriff eines Gegners wie im Spiel: Fernkampf (`ranged`) mit dem Geschoss bis zur Reichweite und dem Treffer dort,
 *  Nahkampf mit dem Treffer in Reichweite zur Mitte der Angriffs-Animation. Takt aus `attacksPerSecond`, mindestens so lang
 *  wie der Angriff. `dir` ist die Richtung zum Ziel. */
export function attackEffects(def: EnemyDef | null, sheet: string, dir: 1 | -1): TimedEffect[] {
  if (!def) return [];
  const ranged = def.traits.includes('ranged');
  const flight = ranged ? projectileMs(def.range, PROJECTILE.speed) : 0;
  const delay = ranged ? flight : attackMs(sheet) / 2;
  const visible = delay + configLook('hit').durationMs + ATTACK_PAUSE_MS;
  const cycleMs = Math.max(visible, def.attacksPerSecond > 0 ? 1000 / def.attacksPerSecond : 0);
  const out: TimedEffect[] = [hitAt(dir * def.range, delay, cycleMs)];
  if (ranged) out.unshift({ kind: 'projectile', range: def.range, dir, speed: PROJECTILE.speed, color: PROJECTILE.enemyColor, cycleMs });
  return out;
}

// ---- Boss-Fähigkeiten ----------------------------------------------------------------------------------------------

/** Eine abspielbare Fähigkeit: Boss, beim Endboss die Phase (1-basiert), die dort geltende Fähigkeit */
export interface AbilityEntry {
  key: string;
  bossId: string;
  phase?: number;
  ability: Ability;
  /** Die Phase hat keine eigene Fähigkeit und erbt die der vorigen (wie `abilityText` sie nennt) */
  inherited: boolean;
  label: string;
}

/** Alle Fähigkeiten der Bosse aus `data/bosses.json`; eine Phase ohne geltende Fähigkeit fehlt (kein ▶). */
export function abilityEntries(bosses: readonly BossData[] = BOSS_DATA): AbilityEntry[] {
  return bosses.flatMap((b): AbilityEntry[] => {
    const n = phaseCount(b);
    if (n === 0) return b.ability ? [{ key: b.id, bossId: b.id, ability: b.ability, inherited: false, label: `${b.name} · ${b.ability.type}` }] : [];
    return Array.from({ length: n }, (_, i) => i + 1).flatMap((phase): AbilityEntry[] => {
      const ability = abilityIn(b, phase);
      if (!ability) return [];
      const inherited = !b.phases?.[phase - 1]?.ability;
      return [{ key: `${b.id}:${phase}`, bossId: b.id, phase, ability, inherited, label: `${b.name} · Phase ${phase} · ${ability.type}` }];
    });
  });
}

/** Was die Bühne für eine Fähigkeit zeigt: Effekte am Boss, zusätzliche Figuren (Beschwörung), Animation des Bosses, Lücken */
export interface AbilityPlay {
  timed: TimedEffect[];
  extra: FigureItem[];
  anim: Anim;
  gaps: Gap[];
}

const ms = (seconds: number | undefined): number => Math.max(1, seconds ?? 1) * 1000;

/** Beschworene Gegner neben dem Boss (abwechselnd rechts und links), Grafik aus ihrem Sheet; ohne Sheet ein Rahmen und Lücke. */
function summonPlay(a: Ability, bossX: number): AbilityPlay {
  const cycleMs = ms(a.intervalSeconds);
  const spec = a.enemy ? SPRITES.enemies[a.enemy] : undefined;
  const play: AbilityPlay = { timed: [], extra: [], anim: 'idle', gaps: [] };
  for (let k = 0; k < (a.count ?? 1); k++) {
    const at = (k % 2 === 0 ? 1 : -1) * SUMMON_STEP * (Math.floor(k / 2) + 1);
    if (spec && SHEETS[spec.sheet]) play.extra.push({ id: `summon-${a.enemy}-${k}`, x: bossX + at, spec, anim: 'idle', timed: [{ kind: 'appear', cycleMs }] });
    else play.timed.push({ kind: 'frame', at, cycleMs });
  }
  if (play.timed.length > 0) play.gaps.push(gap('sheet', `Lücke: beschworener Gegner ${a.enemy ?? '?'} ohne Grafik`));
  return play;
}

/** Darstellung einer Fähigkeit wie im Spiel. Radius und Intervall stammen aus den Daten (Units bzw. Sekunden). */
export function abilityPlay(a: Ability, boss: { x: number; range: number }, dir: 1 | -1 = 1): AbilityPlay {
  const cycleMs = ms(a.intervalSeconds);
  switch (a.type) {
    case 'aoe':
      return { timed: [{ kind: 'warn', radius: a.radius ?? 0, cycleMs }], extra: [], anim: 'idle', gaps: [] };
    case 'summon':
      return summonPlay(a, boss.x);
    case 'shardThrow': {
      const flight = projectileMs(boss.range, PROJECTILE.speed);
      const timed: TimedEffect[] = [
        { kind: 'projectile', range: boss.range, dir, speed: PROJECTILE.speed, color: PROJECTILE.enemyColor, cycleMs },
        hitAt(dir * boss.range, flight, cycleMs),
      ];
      return { timed, extra: [], anim: 'attack', gaps: [gap('display', `Lücke: Splitter-Radius ${a.radius ?? '?'} ohne Darstellung im Spiel`)] };
    }
    case 'flameTrail':
      return { timed: [], extra: [], anim: 'idle', gaps: [gap('display', 'Lücke: Fähigkeit flameTrail ohne Darstellung (das Spiel zeichnet die Bodenfläche nicht)')] };
    default:
      return { timed: [], extra: [], anim: 'idle', gaps: [gap('display', `Lücke: Fähigkeit ${a.type} ohne Darstellung`)] };
  }
}
