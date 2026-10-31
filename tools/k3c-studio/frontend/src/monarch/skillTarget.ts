import hubJson from '@data/hub.json';
import monarchJson from '@data/monarch.json';
import { UNIT_PX } from '@game/core/constants';
import { SHIELD_MAX, type StatusKind } from '@game/scenes/effects/statusLooks';
import { gap, type Gap } from '../playback/figure';
import { REPEAT_PAUSE_MS, type TimedEffect } from '../playback/timeline';

// Ziel-Gegner und Status-Effekte der Monarch-Seite (B-676): Flächenzauber wirken am Ziel mit ihrem Radius (Ereignis
// `skillCast`, B-674), Kontrolleffekte und der Sturm erscheinen am Ziel, Schild, Last Stand und Auren am Monarchen über
// ihre Dauer (B-675). Welche Skills ein Ziel brauchen, folgt der Simulation (`spellCenter`, `castStun`); Dauer, Radius,
// Reichweite und Schild-HP stammen aus `data/monarch.json`.

interface SkillEffect {
  type: string;
  radius?: number;
  range?: number;
  duration?: number;
  hp?: number;
  aura?: number;
  castle?: boolean;
}

const SKILLS = (monarchJson as unknown as { skills: Record<string, { kind: string; effect: SkillEffect }> }).skills;

/** Abstand des Ziel-Gegners zum Monarchen in Units (Beispiel aus B-676: Goblin in 8 Units) */
export const TARGET_DISTANCE = 8;
/** Gegner, der ohne frühere Wahl Ziel ist */
export const DEFAULT_TARGET = 'goblin';

/** Gewählter Ziel-Gegner: Gegner-ID und Abstand in Units */
export interface Target {
  kind: string;
  distance: number;
}

/** effect.type, die nur mit einem lebenden Gegner in Reichweite wirken (sonst `castFailed`, „Skill ohne Ziel“) */
const NEEDS_TARGET = new Set(['areaDamage', 'slow', 'damageOverTime', 'stun']);
/** Flächenzauber: Ring am Ziel mit dem Radius des Skills (`areaEffects` der Simulation) */
const AREA = new Set(['areaDamage', 'slow', 'damageOverTime']);

const effectOf = (id: string): SkillEffect | undefined => SKILLS[id]?.effect;

/** Braucht der Skill ein Ziel in Reichweite? */
export const needsTarget = (id: string): boolean => NEEDS_TARGET.has(effectOf(id)?.type ?? '');
/** Ist der Skill ein Flächenzauber (Wirkort am Ziel)? */
export const isArea = (id: string): boolean => AREA.has(effectOf(id)?.type ?? '');

/** Wirkt der Skill auf dieses Ziel? Ohne Ziel oder außerhalb von `range` nicht (wie im Spiel). */
export function reaches(id: string, target: Target | null): boolean {
  if (!needsTarget(id)) return true;
  return target !== null && target.distance <= (effectOf(id)?.range ?? 0);
}

/** Radius des Flächenzaubers in Pixeln (Units × `UNIT_PX`, wie `SkillCasts` den Ring zeichnet); 0 ohne Fläche */
export const areaRadiusPx = (id: string): number => (isArea(id) ? (effectOf(id)?.radius ?? 0) * UNIT_PX : 0);

/** Dauer eines Status-Effekts als Zeitachsen-Effekt mit Pause bis zur Wiederholung */
const status = (s: Omit<Extract<TimedEffect, { kind: 'status' }>, 'kind' | 'cycleMs'>): TimedEffect =>
  ({ kind: 'status', ...s, cycleMs: (s.delayMs ?? 0) + s.durationMs + REPEAT_PAUSE_MS });

/** Burgradius des Spiels (`data/hub.json` › `castleRadiusUnits`), Radius von Holy Ground */
const CASTLE_RADIUS = (hubJson as unknown as { castleRadiusUnits: number }).castleRadiusUnits;

const HOLY_GAP = gap('display', 'Lücke: Holy Ground wirkt im Spiel im Burgradius; das Studio zeigt den Ring am Monarchen');

/** Wirkort eines Passivs wie `AurasOf` der Simulation: Radius in Units, oder nichts (Passiv ohne Wirkort). */
function auraRadius(e: SkillEffect): number | null {
  if (e.type === 'regen') return e.castle ? null : (e.aura ?? 0);
  if (e.type === 'stat' && (e.aura ?? 0) > 0) return e.aura!;
  if (e.type === 'guardian') return e.radius ?? 0;
  return e.type === 'frostArmor' ? 0 : null;
}


type Status = { onTarget: TimedEffect[]; onMonarch: TimedEffect[]; gaps: Gap[] };

/** Status eines Passivs am Monarchen: Aura-Ring mit Radius (Holy Ground mit Burgradius und Lücke) */
function passiveStatus(id: string, e: SkillEffect, out: Status): Status {
  const holy = e.type === 'regen' && e.castle;
  const r = holy ? CASTLE_RADIUS : auraRadius(e);
  if (r !== null) out.onMonarch.push(status({ status: 'aura', durationMs: 2000, radius: r, skill: id }));
  if (holy) out.gaps.push(HOLY_GAP);
  return out;
}

/** effect.type eines aktiven Skills → Status am Ziel-Gegner (nur, wenn er wirkt) */
const ON_TARGET: Record<string, StatusKind> = { stun: 'stun', slow: 'slow', damageOverTime: 'storm', taunt: 'taunt' };

/** Status-Effekte eines Skills: `onTarget` am Ziel-Gegner, `onMonarch` am Monarchen, dazu Lücken. Nur, was das Spiel zeigt. */
export function skillStatus(id: string, target: Target | null): Status {
  const e = effectOf(id);
  const out: Status = { onTarget: [], onMonarch: [], gaps: [] };
  if (!e) return out;
  if (SKILLS[id]!.kind === 'passive') return passiveStatus(id, e, out);
  const durationMs = (e.duration ?? 0) * 1000;
  const kind = ON_TARGET[e.type];
  // Spott wirkt im Radius um den Monarchen (castTaunt), die übrigen auf das Ziel in Reichweite
  const hit = target !== null && (e.type === 'taunt' ? target.distance <= (e.radius ?? 0) : reaches(id, target));
  if (kind && hit) out.onTarget.push(status({ status: kind, durationMs, ...(kind === 'storm' ? { radius: e.radius ?? 0 } : {}) }));
  if (e.type === 'shield') out.onMonarch.push(status({ status: 'shield', durationMs, share: Math.min(1, (e.hp ?? 0) / SHIELD_MAX) }));
  if (e.type === 'lastStand') out.onMonarch.push(status({ status: 'lastStand', durationMs }));
  return out;
}

/** Wert der Auswahl „ohne Ziel“ (kein Gegner auf der Bühne) */
export const NO_TARGET = 'ohne';