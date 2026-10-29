import type { PlannedChunk, WorldDraft } from '../api';
import { normalized } from './plan';

/** Zahl der Änderungen am Entwurf einer Stufe gegenüber dem gespeicherten Stand (B-664/AC-03): geänderte, neue und
 *  entfernte Chunks je Platz, dazu eine geänderte Inselgröße. */
export function planChanges(draft: readonly PlannedChunk[], saved: readonly PlannedChunk[], count: number, savedCount: number): number {
  const by = (xs: readonly PlannedChunk[]) => new Map(normalized([...xs]).map((c) => [c.slot, JSON.stringify(c)] as const));
  const a = by(draft);
  const b = by(saved);
  let n = count !== savedCount ? 1 : 0;
  for (const [slot, v] of a) if (b.get(slot) !== v) n++;
  for (const slot of b.keys()) if (!a.has(slot)) n++;
  return n;
}

/** Zahl der Änderungen am Welt-Entwurf: Name, Notizen und je Insel (Stelle in der Liste) ein Eintrag; fehlende oder neue Inseln zählen einzeln. */
export function worldChanges(draft: WorldDraft | null, saved: WorldDraft | null): number {
  if (!draft || !saved) return 0;
  let n = (draft.name !== saved.name ? 1 : 0) + (draft.notes !== saved.notes ? 1 : 0);
  const len = Math.max(draft.islands.length, saved.islands.length);
  for (let i = 0; i < len; i++) if (JSON.stringify(draft.islands[i]) !== JSON.stringify(saved.islands[i])) n++;
  return n;
}

/** „2 Änderungen“, „1 Änderung“ */
export const changesText = (n: number) => (n === 1 ? '1 Änderung' : `${n} Änderungen`);
