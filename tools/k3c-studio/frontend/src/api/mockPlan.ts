import type { Backend, IslandSize, PlanCheckView, PlannedChunk, PlanObjectsView, PlanView, WorldSource } from './types';

// Geplante Chunks im Mock (Browser, Tests): eine Insel mit fünf Tiefen im Speicher. Die Prüfung kennt nur ein paar
// Fehler: Platz 0, Platz jenseits der Länge (±7, mit Chunk-Anzahl n: ±(n/2 − 2)), die Art „volcano“ und eine Anzahl,
// die keine Inselgröße ist (Go prüft alles, siehe level.ValidatePlanFor).

const BIOMES = ['forest', 'cave', 'mine', 'ironhold', 'crystal'];
/** Inselgrößen wie data/island-sizes.json */
export const MOCK_SIZES: IslandSize[] = [
  { name: 'sehr klein', chunkCount: 16 }, { name: 'klein', chunkCount: 24 }, { name: 'mittel', chunkCount: 40 },
  { name: 'groß', chunkCount: 64 }, { name: 'sehr groß', chunkCount: 100 }, { name: 'mega groß', chunkCount: 160 },
];
const sign = (n: number) => (n > 0 ? `+${n}` : String(n));

export function mockCheckPlan(chunks: PlannedChunk[], count = 0): PlanCheckView {
  if (count !== 0 && !MOCK_SIZES.some((s) => s.chunkCount === count)) {
    return { ok: false, slot: 0, error: `die Chunk-Anzahl ${count} ist keine Inselgröße (gültig: ${MOCK_SIZES.map((s) => s.chunkCount).join(', ')})` };
  }
  const reach = count > 0 ? count / 2 - 2 : 7;
  for (const c of chunks) {
    if (c.slot === 0) return { ok: false, slot: 0, error: 'auf Platz 0: es gibt keinen Platz 0 (Hub)' };
    if (Math.abs(c.slot) > reach) return { ok: false, slot: c.slot, error: `auf Platz ${sign(c.slot)}: jenseits der Länge (höchstens ±${reach})` };
    if (c.kind === 'volcano') return { ok: false, slot: c.slot, error: `auf Platz ${sign(c.slot)}: unbekannte Chunk-Art "volcano"` };
  }
  return { ok: true, slot: 0, error: '' };
}

/** Seed-Inhalt im Mock: zwei Objekte, Lage aus der Länge des Seeds; Gründe der Ablehnung wie im Go-Binding. */
function mockChunkObjects(planned: PlannedChunk[], slot: number): PlanObjectsView {
  const c = planned.find((x) => x.slot === slot);
  const fail = (error: string): PlanObjectsView => ({ objects: [], error });
  if (!c) return fail(`der Platz ${sign(slot)} ist nicht geplant`);
  if (!c.seed) return fail(`der Platz ${sign(slot)} hat keinen Chunk-Seed, es gibt nichts umzuwandeln`);
  if (c.objects && c.objects.length > 0) return fail(`der Platz ${sign(slot)} hat schon feste Objekte`);
  return { objects: [{ kind: 'tree', at: 4 + c.seed.length }, { kind: 'rock', at: 20 }], error: '' };
}

type PlanPart = Pick<Backend, 'planLoad' | 'planCheck' | 'planSave' | 'planChunkObjects'>;

/** Plan-Teil des Mock-Backends; `touch` simuliert eine Änderung der Datei von außen (neuer Stand). */
export function mockPlan(): PlanPart & { touch: () => void } {
  let planned: Record<string, PlannedChunk[]> = {};
  let counts: Record<string, number> = {};
  let version = 1;
  const stand = () => `mock-${version}`;
  const view = (): PlanView => ({
    islands: [{ id: 'island1', name: 'Insel 1', depths: [0, 1, 2, 3, 4], biomes: BIOMES, planned: structuredClone(planned), chunkCounts: { ...counts } }],
    sizes: MOCK_SIZES,
    stand: stand(),
    error: '',
  });
  const plannedOf = (source: WorldSource) => source.planned ?? planned[String(source.stage ?? 0)] ?? [];
  return {
    planLoad: async () => view(),
    planCheck: async (_island, _depth, chunks, count) => mockCheckPlan(chunks, count),
    planSave: async (_island, depth, chunks, count, at) => {
      if (at !== stand()) return { stand: '', hint: '', error: 'data/islands.json geändert, bitte neu laden', stale: true };
      const check = mockCheckPlan(chunks, count);
      if (!check.ok) return { stand: '', hint: '', error: check.error, stale: false };
      planned = { ...planned };
      counts = { ...counts };
      if (chunks.length > 0) planned[String(depth)] = structuredClone(chunks);
      else delete planned[String(depth)];
      if (count > 0) counts[String(depth)] = count;
      else delete counts[String(depth)];
      version++;
      return { stand: stand(), hint: 'REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung', error: '', stale: false };
    },
    planChunkObjects: async (_seed, source, slot) => mockChunkObjects(plannedOf(source), slot),
    touch: () => { version++; },
  };
}
