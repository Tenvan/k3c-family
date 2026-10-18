import type { PlanIsland, PlannedChunk, PlannedObject } from '../api';

// Reine Logik des Editors für geplante Chunks (B-461, WZ8.5): der Entwurf ist eine Liste geplanter Chunks einer Tiefe;
// jede Änderung liefert eine neue Liste. Kein DOM, kein Backend.

export type Draft = readonly PlannedChunk[];

/** Platz als Text: +3, −3 (der Platz 0 gibt es nicht) */
export const slotLabel = (slot: number): string => (slot > 0 ? `+${slot}` : String(slot));

/** Chunk-Arten und Objekt-Arten, die der Editor vorschlägt (das Biom prüft, ob sie gelten). */
export const CHUNK_KINDS = ['forest', 'clearing', 'rocks', 'stream', 'tunnel', 'cavern', 'crystals', 'chasm', 'shaft', 'rails', 'oreVein', 'gallery', 'forge', 'lava', 'grotto', 'geode', 'portal', 'chest', 'recruitCamp'];
export const OBJECT_KINDS = ['tree', 'bush', 'rock', 'copperOre', 'stoneVein', 'copperVein', 'ironVein', 'crystalVein', 'chest', 'recruitCamp', 'portal'];

const bySlot = (a: PlannedChunk, b: PlannedChunk) => a.slot - b.slot;

/** Neuer Chunk am Platz slot (nach Platz sortiert); ein doppelter Platz bleibt stehen, die Prüfung nennt ihn. */
export function addChunk(draft: Draft, slot: number, kind: string): PlannedChunk[] {
  return [...draft, { slot, kind }].sort(bySlot);
}

export function updateChunk(draft: Draft, index: number, patch: Partial<PlannedChunk>): PlannedChunk[] {
  return draft.map((c, i) => (i === index ? { ...c, ...patch } : c)).sort(bySlot);
}

export function removeChunk(draft: Draft, index: number): PlannedChunk[] {
  return draft.filter((_, i) => i !== index);
}

function withObjects(draft: Draft, index: number, change: (objects: PlannedObject[]) => PlannedObject[]): PlannedChunk[] {
  return draft.map((c, i) => (i === index ? { ...c, objects: change([...(c.objects ?? [])]) } : c));
}

export const addObject = (draft: Draft, index: number, object: PlannedObject): PlannedChunk[] => withObjects(draft, index, (o) => [...o, object]);
/** Verschiebt ein Objekt: neue Lage `at` in Units ab Chunk-Anfang */
export const moveObject = (draft: Draft, index: number, at: number, to: number): PlannedChunk[] =>
  withObjects(draft, index, (o) => o.map((x, i) => (i === at ? { ...x, at: to } : x)));
export const removeObject = (draft: Draft, index: number, at: number): PlannedChunk[] => withObjects(draft, index, (o) => o.filter((_, i) => i !== at));

/** Form zum Prüfen und Speichern: leerer Seed und leere Objektliste fehlen, die Reihenfolge der Felder ist fest. */
export function normalized(draft: Draft): PlannedChunk[] {
  return draft.map((c) => ({
    slot: c.slot,
    kind: c.kind,
    ...(c.seed ? { seed: c.seed } : {}),
    ...(c.objects && c.objects.length > 0 ? { objects: c.objects.map((o) => ({ kind: o.kind, at: o.at })) } : {}),
  }));
}

/** Ist der Entwurf anders als die gespeicherte Planung? (Vergleich der normalisierten Form) */
export const isDirty = (draft: Draft, saved: Draft): boolean => JSON.stringify(normalized(draft)) !== JSON.stringify(normalized(saved));

/** Die gespeicherte Planung einer Stufe (Index in den Tiefen der Insel) */
export function savedPlan(island: PlanIsland | undefined, stage: number): PlannedChunk[] {
  const depth = island?.depths[stage];
  return depth === undefined ? [] : normalized(island?.planned[String(depth)] ?? []);
}
