import { GROUND_Y, UNIT_PX } from '../../core/constants';
import { MONARCH } from '../../model/data';
import type { Player } from '../../model/types';
import { LINE_COLORS, type Effect } from './effects';

/**
 * Skill-Ringe aus dem Zustand (SK2.3, B-503; Default B-561): ein aktiver Skill wirkt, wenn die Abklingzeit seines Slots
 * zwischen zwei Zuständen steigt (die Sim setzt sie beim Wirken auf den vollen Wert); ein passiver Skill zeigt seinen Ring,
 * wenn er neu in `skills` steht. Reiner Vergleich je Spieler-Index, kein Spielzustand und keine Regel im Client.
 */

type SkillInfo = { line: string; kind: string };
const SKILLS = MONARCH.skills as Record<string, SkillInfo>;

/** Höhe über dem Boden, in der der Ring am Monarchen erscheint */
const RING_PX = 50;

interface Seen {
  cooldowns: number[];
  skills: Set<string>;
}

export class SkillCasts {
  private readonly seen = new Map<number, Seen>();

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
        if (cd > (prev.cooldowns[i] ?? 0) + 0.5) push(out, p, p.slots?.[i]);
      });
      for (const id of now.skills) if (!prev.skills.has(id) && SKILLS[id]?.kind === 'passive') push(out, p, id);
    }
    return out;
  }
}

function push(out: Effect[], p: Player, id: string | undefined): void {
  const color = id ? LINE_COLORS[SKILLS[id]?.line ?? ''] : undefined;
  if (color !== undefined) out.push({ kind: 'skill', x: p.x * UNIT_PX, y: GROUND_Y - RING_PX, color });
}
