import type { Backend, PlanCheckView, PlannedChunk, PlanView } from './types';

// Geplante Chunks im Mock (Browser, Tests): eine Insel mit fünf Tiefen im Speicher. Die Prüfung kennt nur ein paar
// Fehler: Platz 0, Platz jenseits von ±7 und die Art „volcano“ (Go prüft alles, siehe level.ValidatePlan).

const BIOMES = ['forest', 'cave', 'mine', 'ironhold', 'crystal'];
const sign = (n: number) => (n > 0 ? `+${n}` : String(n));

export function mockCheckPlan(chunks: PlannedChunk[]): PlanCheckView {
  for (const c of chunks) {
    if (c.slot === 0) return { ok: false, slot: 0, error: 'auf Platz 0: es gibt keinen Platz 0 (Hub)' };
    if (Math.abs(c.slot) > 7) return { ok: false, slot: c.slot, error: `auf Platz ${sign(c.slot)}: jenseits der kürzesten Länge (höchstens ±7)` };
    if (c.kind === 'volcano') return { ok: false, slot: c.slot, error: `auf Platz ${sign(c.slot)}: unbekannte Chunk-Art "volcano"` };
  }
  return { ok: true, slot: 0, error: '' };
}

/** Plan-Teil des Mock-Backends; `touch` simuliert eine Änderung der Datei von außen (neuer Stand). */
export function mockPlan(): Pick<Backend, 'planLoad' | 'planCheck' | 'planSave'> & { touch: () => void } {
  let planned: Record<string, PlannedChunk[]> = {};
  let version = 1;
  const stand = () => `mock-${version}`;
  const view = (): PlanView => ({
    islands: [{ id: 'island1', name: 'Insel 1', depths: [0, 1, 2, 3, 4], biomes: BIOMES, planned: structuredClone(planned) }],
    stand: stand(),
    error: '',
  });
  return {
    planLoad: async () => view(),
    planCheck: async (_island, _depth, chunks) => mockCheckPlan(chunks),
    planSave: async (_island, depth, chunks, at) => {
      if (at !== stand()) return { stand: '', hint: '', error: 'data/islands.json geändert, bitte neu laden', stale: true };
      const check = mockCheckPlan(chunks);
      if (!check.ok) return { stand: '', hint: '', error: check.error, stale: false };
      planned = { ...planned };
      if (chunks.length > 0) planned[String(depth)] = structuredClone(chunks);
      else delete planned[String(depth)];
      version++;
      return { stand: stand(), hint: 'REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung', error: '', stale: false };
    },
    touch: () => { version++; },
  };
}
