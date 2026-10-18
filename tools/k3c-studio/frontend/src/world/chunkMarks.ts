import type { PlanCheckView, PlannedChunk } from '../api';
import { nameOf } from './kindIcons';
import type { ModelChunk } from './levelView';

// Hinweise am Chunk der Chunk-Vorschau (B-485, WZ17.4): ein Hinweis gehört zu einem Chunk-Platz und erscheint als farbiger
// unterer Rand am Abschnitt des Levels. Reine Logik, kein DOM. Neue Hinweis-Arten: Art in `MarkKind` ergänzen, in
// `slotHints` (oder einer weiteren Quelle) erzeugen und in styles/world.css die Klasse `.wd-mark-<art>` mit Farbe anlegen.

/** Art des Hinweises; bestimmt die Farbe (`.wd-mark-<art>`). Spätere Arten hier ergänzen. */
export type MarkKind = 'planned' | 'error';

export interface SlotHint {
  slot: number;
  kind: MarkKind;
  /** Text des Tooltips */
  label: string;
}

const plural = (n: number) => (n === 0 ? 'ohne feste Objekte' : n === 1 ? '1 festes Objekt' : `${n} feste Objekte`);

/** Hinweise aus dem Entwurf (gleich der gespeicherten Planung, solange nichts geändert ist): jeder geplante Chunk wird
 *  `planned`; der Platz mit dem Prüffehler wird `error`. Ein Fehler ohne Platz in der Liste bleibt am Editor. */
export function slotHints(draft: readonly PlannedChunk[], check: PlanCheckView): SlotHint[] {
  return draft.map((c): SlotHint => {
    if (!check.ok && check.slot === c.slot) return { slot: c.slot, kind: 'error', label: `Fehler: ${check.error}` };
    const seed = c.seed ? `Chunk-Seed „${c.seed}“` : 'ohne Chunk-Seed';
    return { slot: c.slot, kind: 'planned', label: `Geplant: ${nameOf(c.kind)} · ${seed} · ${plural(c.objects?.length ?? 0)}` };
  });
}

/** Hinweise je Abschnitt (Index im Level), über den Index wie im Generator (engine/level/planned_gen.go): +n ist der n-te
 *  Abschnitt rechts der Hub-Abschnitte, −n der n-te links davon (rules/geplante-chunks.md § 1). Ohne Hub, für Platz 0 und
 *  für Plätze außerhalb des Levels gibt es keinen Abschnitt. */
export function chunkMarks(level: { chunks: readonly ModelChunk[] }, hints: readonly SlotHint[]): Map<number, SlotHint> {
  const marks = new Map<number, SlotHint>();
  const hub = level.chunks.filter((c) => c.kind === 'hub').map((c) => c.index);
  if (hub.length === 0) return marks;
  const [first, last] = [Math.min(...hub), Math.max(...hub)];
  const known = new Set(level.chunks.map((c) => c.index));
  for (const h of hints) {
    if (h.slot === 0) continue;
    const index = h.slot > 0 ? last + h.slot : first + h.slot;
    if (known.has(index)) marks.set(index, h);
  }
  return marks;
}
