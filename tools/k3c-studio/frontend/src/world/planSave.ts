import { backend } from '../api';
import type { PlanSaved, PlanView, PlannedChunk } from '../api';

// Speichern des Entwurfs einer Stufe (B-461, B-486, B-485/AC-04): Planung und Chunk-Anzahl in einem Aufruf; die
// Meldung für den Editor und die geladene Sicht nach dem Speichern. Ausgelagert aus usePlan (Funktion ≤ 60 Zeilen).

export type Notice = { kind: 'hint' | 'error' | 'stale'; text: string } | null;

/** Speichern; lehnt der Aufruf ab (Wails mit Text), wird die Ablehnung zum Fehler, sonst bliebe sie ohne Meldung (B-486). */
async function saveOrError(...args: Parameters<typeof backend.planSave>): Promise<PlanSaved> {
  try {
    return await backend.planSave(...args);
  } catch (e) {
    return { stand: '', hint: '', stale: false, error: `Speichern fehlgeschlagen: ${e instanceof Error ? e.message : String(e)}` };
  }
}

/** Die Sicht nach dem Speichern: neuer Stand, Planung und Anzahl der Tiefe (0 = Biom-Länge, Eintrag entfällt). */
function savedView(view: PlanView, island: string, depth: number, chunks: PlannedChunk[], count: number, stand: string): PlanView {
  const key = String(depth);
  return {
    ...view, stand,
    islands: view.islands.map((i) => {
      if (i.id !== island) return i;
      const chunkCounts = { ...i.chunkCounts };
      if (count > 0) chunkCounts[key] = count;
      else delete chunkCounts[key];
      return { ...i, planned: { ...i.planned, [key]: chunks }, chunkCounts };
    }),
  };
}

/** Speichert Planung und Anzahl der Tiefe auf dem Stand der Sicht; view fehlt, wenn nicht gespeichert wurde. */
export async function savePlan(view: PlanView, island: string, depth: number, chunks: PlannedChunk[], count: number): Promise<{ view?: PlanView; notice: Notice }> {
  const result = await saveOrError(island, depth, chunks, count, view.stand);
  if (result.error) return { notice: { kind: result.stale ? 'stale' : 'error', text: result.error } };
  return { view: savedView(view, island, depth, chunks, count, result.stand), notice: { kind: 'hint', text: result.hint } };
}
