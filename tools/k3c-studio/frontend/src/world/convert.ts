import { backend } from '../api';
import type { PlanObjectsView, PlannedChunk, PlannedObject } from '../api';
import type { PlanSource } from '../stage/stageLogic';
import { normalized, type Draft } from './plan';

// Umwandlung eines Seed-Chunks in feste Objekte (B-485/AC-06, geplante-chunks.md § 7): Der Inhalt kommt aus dem
// Vorschau-Level des Go-Backends (PlanChunkObjects) zur Quelle der Vorschauen; der Chunk behält Platz und Art, bekommt
// die Objekte als feste Objekte und verliert den Seed. Reine Logik plus der eine Aufruf, kein DOM.

/** Hinweis am Knopf, wenn er bereit ist */
export const CONVERT_HINT = 'Das Bild bleibt gleich';

/** Warum der Chunk sich nicht umwandeln lässt; undefined = bereit. synced: die Vorschauen zeigen genau den Entwurf. */
export function convertLock(chunk: PlannedChunk, synced: boolean): string | undefined {
  if (!chunk.seed) return 'Nur ein geplanter Chunk mit Chunk-Seed lässt sich umwandeln';
  if (chunk.objects && chunk.objects.length > 0) return 'Der Chunk hat schon feste Objekte';
  if (!synced) return 'Erst die Prüfung des Entwurfs abwarten (Fehler beheben)';
  return undefined;
}

/** Zeigen die Vorschauen (source) genau den Entwurf? Nur dann ist der Seed-Inhalt der, den man sieht. */
export function inSync(draft: Draft, count: number, source: PlanSource | undefined): boolean {
  return source !== undefined && (source.chunkCount ?? 0) === count && JSON.stringify(normalized(draft)) === JSON.stringify(normalized(source.planned));
}

/** Der Chunk am Index mit festen Objekten statt Seed (Platz und Art bleiben). */
export function convertChunk(draft: Draft, index: number, objects: PlannedObject[]): PlannedChunk[] {
  return draft.map((c, i) => (i === index ? { slot: c.slot, kind: c.kind, objects: objects.map((o) => ({ kind: o.kind, at: o.at })) } : c));
}

/** Seed-Inhalt des Platzes zur Quelle der Vorschauen; eine Ablehnung des Aufrufs wird zum Fehler (wie B-486). */
export async function seedObjects(seed: string, source: PlanSource, slot: number): Promise<PlanObjectsView> {
  try {
    return await backend.planChunkObjects(seed, { island: source.island, stage: source.stage, planned: source.planned, chunkCount: source.chunkCount ?? 0 }, slot);
  } catch (e) {
    return { objects: [], error: `Umwandeln fehlgeschlagen: ${e instanceof Error ? e.message : String(e)}` };
  }
}
