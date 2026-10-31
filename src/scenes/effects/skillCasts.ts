import { GROUND_Y, UNIT_PX } from '../../core/constants';
import { MONARCH } from '../../model/data';
import type { GameEvent, Player } from '../../model/types';
import type { Effect } from './effects';
import { skillLook } from './skillLooks';

/**
 * Skill-Effekte (SK2.3, B-503; je Skill eigener Look, B-578): ein aktiver Skill wirkt laut Ereignis `skillCast` der Simulation
 * (B-674) am gemeldeten Wirkort, ein Flächenzauber mit dem Radius des Skills; ein passiver Skill zeigt seinen Ring am Monarchen,
 * wenn er neu in `skills` steht. Kein Spielzustand und keine Regel im Client.
 */

const SKILLS = MONARCH.skills as Record<string, { kind: string }>;

/** Höhe über dem Boden, in der der Ring erscheint (am Monarchen und am Wirkort) */
const RING_PX = 50;

/** Wirken (aktiv) oder Lernen (passiv) eines Skills durch einen Spieler; `x` in Units (SO8.2, Ton je Skill). */
export interface SkillCast {
  index: number;
  skill: string;
  x: number;
}

type CastEvent = Extract<GameEvent, { type: 'skillCast' }>;

export class SkillCasts {
  private readonly seen = new Map<number, Set<string>>();

  /** `onCast` hört jedes Wirken und Lernen, auch ohne Ring (z. B. für den Ton). */
  constructor(private readonly onCast?: (cast: SkillCast) => void) {}

  /** Neues Spiel oder neue Stufe: gelernte Skills vergessen, der erste Zustand danach zeigt keinen Lern-Ring. */
  reset(): void {
    this.seen.clear();
  }

  /** Ringe für diesen Zustand: Wirken aus den Ereignissen `skillCast`, Lernen passiver Skills aus dem Vergleich je Spieler. */
  diff(players: readonly Player[], events: readonly GameEvent[] = []): Effect[] {
    const out: Effect[] = [];
    for (const e of events) if (e.type === 'skillCast') this.cast(out, players, e);
    for (const p of players) {
      const prev = this.seen.get(p.index);
      const now = new Set(p.skills ?? []);
      this.seen.set(p.index, now);
      if (!prev) continue;
      for (const id of now) if (!prev.has(id) && SKILLS[id]?.kind === 'passive') this.learned(out, p, id);
    }
    return out;
  }

  private cast(out: Effect[], players: readonly Player[], e: CastEvent): void {
    const look = skillLook(e.skill);
    if (!look) return; // unbekannter Skill: ignorieren
    const index = players.find((p) => p.id === e.from)?.index ?? -1;
    this.onCast?.({ index, skill: e.skill, x: e.x });
    const radius = e.radius > 0 ? e.radius * UNIT_PX : look.radius;
    out.push({ kind: 'skill', x: e.x * UNIT_PX, y: GROUND_Y - RING_PX, color: look.color, look: { ...look, radius } });
  }

  private learned(out: Effect[], p: Player, id: string): void {
    this.onCast?.({ index: p.index, skill: id, x: p.x });
    const look = skillLook(id);
    if (look) out.push({ kind: 'skill', x: p.x * UNIT_PX, y: GROUND_Y - RING_PX, color: look.color, look });
  }
}
