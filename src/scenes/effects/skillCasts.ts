import { GROUND_Y, UNIT_PX } from '../../core/constants';
import { MONARCH } from '../../model/data';
import type { Player } from '../../model/types';
import type { Effect } from './effects';
import { skillLook } from './skillLooks';

/**
 * Skill-Effekte aus dem Zustand (SK2.3, B-503; je Skill eigener Look, B-578): ein aktiver Skill wirkt, wenn die Abklingzeit seines Slots
 * zwischen zwei Zuständen steigt (die Sim setzt sie beim Wirken auf den vollen Wert); ein passiver Skill zeigt seinen Ring,
 * wenn er neu in `skills` steht. Reiner Vergleich je Spieler-Index, kein Spielzustand und keine Regel im Client.
 */

const SKILLS = MONARCH.skills as Record<string, { kind: string }>;

/** Höhe über dem Boden, in der der Ring am Monarchen erscheint */
const RING_PX = 50;

interface Seen {
  cooldowns: number[];
  skills: Set<string>;
}

/** Wirken (aktiv) oder Lernen (passiv) eines Skills durch einen Spieler; `x` in Units (SO8.2, Ton je Skill). */
export interface SkillCast {
  index: number;
  skill: string;
  x: number;
}

export class SkillCasts {
  private readonly seen = new Map<number, Seen>();

  /** `onCast` hört jedes Wirken und Lernen, auch ohne Ring (z. B. für den Ton). */
  constructor(private readonly onCast?: (cast: SkillCast) => void) {}

  /** Neues Spiel oder neue Stufe: alles vergessen, der erste Zustand danach zeigt keinen Ring. */
  reset(): void {
    this.seen.clear();
  }

  /** Ringe für diesen Zustand; merkt sich Abklingzeiten und gelernte Skills je Spieler. */
  diff(players: readonly Player[]): Effect[] {
    const out: Effect[] = [];
    for (const p of players) {
      const prev = this.seen.get(p.index);
      const now: Seen = { cooldowns: [...(p.cooldowns ?? [])], skills: new Set(p.skills ?? []) };
      this.seen.set(p.index, now);
      if (!prev) continue;
      now.cooldowns.forEach((cd, i) => {
        if (cd > (prev.cooldowns[i] ?? 0) + 0.5) this.push(out, p, p.slots?.[i]);
      });
      for (const id of now.skills) if (!prev.skills.has(id) && SKILLS[id]?.kind === 'passive') this.push(out, p, id);
    }
    return out;
  }

  private push(out: Effect[], p: Player, id: string | undefined): void {
    if (id) this.onCast?.({ index: p.index, skill: id, x: p.x });
    ring(out, p, id);
  }
}

function ring(out: Effect[], p: Player, id: string | undefined): void {
  const look = id ? skillLook(id) : undefined;
  if (look) out.push({ kind: 'skill', x: p.x * UNIT_PX, y: GROUND_Y - RING_PX, color: look.color, look });
}
