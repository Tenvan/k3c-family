import { mockWorldDraft, HINT, staleText } from './mockWorldDraft';
import { mockWorlds, newWorld, type MockIsland, type MockWorld } from './mockWorlds';
import type { Backend, IslandSize, PlanCheckView, PlanIsland, PlannedChunk, PlanObjectsView, PlanSaved, PlanView, WorldSource } from './types';

// Geplante Chunks im Mock (Browser, Tests): Welten mit Inseln (Tiefen 0–4) im Speicher. Die Prüfung kennt nur ein paar
// Fehler: Platz 0, Platz jenseits der Länge (±7, mit Chunk-Anzahl n: ±(n/2 − 2)), die Art „volcano“, eine Ruine fremder Art
// oder mehr als eine Ruine und eine Anzahl,
// die keine Inselgröße ist (Go prüft alles, siehe level.ValidatePlanFor).

const BIOMES = ['forest', 'cave', 'mine', 'ironhold', 'crystal'];
/** Inselgrößen wie data/island-sizes.json */
export const MOCK_SIZES: IslandSize[] = [
  { name: 'sehr klein', chunkCount: 16 }, { name: 'klein', chunkCount: 24 }, { name: 'mittel', chunkCount: 40 },
  { name: 'groß', chunkCount: 64 }, { name: 'sehr groß', chunkCount: 100 }, { name: 'mega groß', chunkCount: 160 },
];
/** Ruinen-Arten wie data/ruins.json › kinds */
export const MOCK_RUIN_KINDS = ['workshop', 'storage', 'farm', 'barracks', 'tavern', 'healer', 'smithy', 'armory'];
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
    const ruin = c.buildings?.[0];
    if (ruin && !MOCK_RUIN_KINDS.includes(ruin.kind)) return { ok: false, slot: c.slot, error: `auf Platz ${sign(c.slot)}: "${ruin.kind}" darf keine Ruine sein` };
    if ((c.buildings?.length ?? 0) > 1) return { ok: false, slot: c.slot, error: `auf Platz ${sign(c.slot)}: höchstens eine Ruine je Chunk` };
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

type PlanPart = Pick<Backend, 'planLoad' | 'planCheck' | 'planSave' | 'planChunkObjects' | 'planWorlds' | 'planWorldSelect' | 'planWorldAdd' | 'planWorldCopy' | 'planWorldRemove' | 'planWorldCheck' | 'planWorldSave' | 'planWorldAnswer'>;

const islandView = (i: MockIsland): PlanIsland => ({
  id: i.id, name: i.name, depths: [...i.depths], biomes: i.depths.map((d) => BIOMES[d] ?? ''), planned: structuredClone(i.planned), chunkCounts: { ...i.counts }, depthScaling: { ...i.depthScaling },
});

/** Plan-Teil des Mock-Backends; `touch` simuliert eine Änderung der Datei von außen (neuer Stand), `active` nennt die gewählte
 *  Welt. Mehrere Welten wie im Backend: jede mit eigenen Inseln und Planung, gewählt ist immer eine. */
export function mockPlan(): PlanPart & { touch: () => void; active: () => string } {
  const worlds: Record<string, MockWorld> = { standard: newWorld('Standard') };
  let active = 'standard';
  let version = 1;
  const cur = () => worlds[active]!;
  const stand = () => `mock-${version}`;
  const view = (): PlanView => ({ name: cur().name, notes: cur().notes, islands: cur().islands.map(islandView), sizes: MOCK_SIZES, ruinKinds: MOCK_RUIN_KINDS, stand: stand(), error: '' });
  const islandOf = (id?: string) => cur().islands.find((i) => i.id === id) ?? cur().islands[0]!;
  const plannedOf = (source: WorldSource) => source.planned ?? islandOf(source.island).planned[String(islandOf(source.island).depths[source.stage ?? 0])] ?? [];
  const refused = (error: string, stale = false): PlanSaved => ({ stand: '', hint: '', error, stale });
  return {
    planLoad: async () => view(),
    planCheck: async (_island, _depth, chunks, count) => mockCheckPlan(chunks, count),
    planSave: async (island, depth, chunks, count, at) => {
      if (at !== stand()) return refused(staleText(active), true);
      const check = mockCheckPlan(chunks, count);
      if (!check.ok) return refused(check.error);
      const isl = islandOf(island);
      isl.planned = { ...isl.planned };
      isl.counts = { ...isl.counts };
      if (chunks.length > 0) isl.planned[String(depth)] = structuredClone(chunks);
      else delete isl.planned[String(depth)];
      if (count > 0) isl.counts[String(depth)] = count;
      else delete isl.counts[String(depth)];
      version++;
      return { stand: stand(), hint: HINT, error: '', stale: false };
    },
    ...mockWorlds(worlds, () => active, (id) => { active = id; version++; }),
    ...mockWorldDraft(() => active, cur, (w) => { worlds[active] = w; }, stand, () => { version++; }),
    planChunkObjects: async (_seed, source, slot) => mockChunkObjects(plannedOf(source), slot),
    touch: () => { version++; },
    active: () => active,
  };
}
