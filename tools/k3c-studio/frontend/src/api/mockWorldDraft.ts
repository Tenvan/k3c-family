import type { MockIsland, MockWorld } from './mockWorlds';
import type { PlanSaved } from './types';
import type { DraftField, WorldCheckView, WorldDraft, WorldDraftApi } from './worldTypes';

// Entwurf einer Welt im Mock (Browser, Tests): Die Prüfung kennt die Fälle der Prüfung des Ladens, die die Oberfläche
// zeigt (Go prüft alles, siehe sim.ParseWorld); Meldungen im Muster `<Datei>: Insel <id>: <Grund>` wie in Go.

const ID = /^[a-z0-9][a-z0-9-]{0,31}$/;
const DEPTHS = [0, 1, 2, 3, 4];

/** Grund der ersten verletzten Regel einer Insel mit ihrem Feld, oder null. */
function islandFault(d: WorldDraft, i: number): [DraftField, string] | null {
  const isl = d.islands[i]!;
  if (!ID.test(isl.id)) return ['id', `Insel ${JSON.stringify(isl.id)}: id ungültig (a-z, 0-9, -, höchstens 32 Zeichen)`];
  if (d.islands.slice(0, i).some((o) => o.id === isl.id)) return ['id', `Insel ${isl.id}: doppelt`];
  if (!isl.name) return ['name', `Insel ${isl.id}: name ist leer`];
  if (isl.depths.length === 0) return ['depths', `Insel ${isl.id}: ohne Tiefen`];
  const unknown = isl.depths.find((x) => !DEPTHS.includes(x));
  if (unknown !== undefined) return ['depths', `Insel ${isl.id}: unbekannte Tiefe ${unknown}`];
  const deepest = Math.max(...isl.depths);
  if (deepest !== 4) return ['depths', `Insel ${isl.id}: kein Endboss in der tiefsten Stufe ${deepest}`];
  const s = isl.depthScaling;
  if (!(s.hp > 0 && s.damage > 0 && s.speed > 0)) return ['depthScaling', `Insel ${isl.id}: depthScaling braucht Faktoren > 0`];
  return null;
}

/** Prüfung eines Entwurfs der Welt id; die Vorschau ist der Entwurf als JSON. */
export function mockCheckWorld(id: string, d: WorldDraft): WorldCheckView {
  const file = `data/worlds/${id}.json`;
  const preview = JSON.stringify({ id, ...d, islands: d.islands.map(({ from: _from, ...rest }) => rest) }, null, 2);
  const fail = (island: number, field: DraftField, reason: string): WorldCheckView => ({ ok: false, island, field, error: `${file}: ${reason}`, preview });
  if (!d.name) return fail(-1, 'name', 'name ist leer');
  if (d.islands.length === 0) return fail(-1, 'islands', 'keine Insel');
  for (let i = 0; i < d.islands.length; i++) {
    const fault = islandFault(d, i);
    if (fault) return fail(i, ...fault);
  }
  return { ok: true, island: -1, field: '', error: '', preview };
}

/** REG-Hinweis nach jedem Speichern, wie Go (planRegHint). */
export const HINT = 'REG-Änderung: Commit mit Begründung, `task go:golden` nur bei gewollter Änderung';

/** Ablehnung auf veraltetem Stand, wie Go (staleError in engine/studio/plan_file.go). */
export const staleText = (id: string) => `data/worlds/${id}.json geändert, bitte neu laden`;

/** Die Welt nach dem Entwurf: Inseln in seiner Reihenfolge, Planung und Chunk-Anzahl gehen mit der Herkunft (from). */
function applyDraft(w: MockWorld, d: WorldDraft): MockWorld {
  const islands = d.islands.map((isl): MockIsland => {
    const old = w.islands.find((o) => isl.from !== '' && o.id === isl.from);
    return { id: isl.id, name: isl.name, depths: [...isl.depths], depthScaling: { ...isl.depthScaling }, planned: old?.planned ?? {}, counts: old?.counts ?? {} };
  });
  return { name: d.name, notes: d.notes, islands };
}

/** Entwurfs-Teil des Mock-Plans: prüfen und speichern auf dem Stand (`stand`), danach `bump` (neuer Stand). */
export function mockWorldDraft(active: () => string, world: () => MockWorld, replace: (w: MockWorld) => void, stand: () => string, bump: () => void): Omit<WorldDraftApi, 'planWorldCopy'> {
  return {
    planWorldCheck: async (d) => mockCheckWorld(active(), d),
    planWorldSave: async (d, at): Promise<PlanSaved> => {
      if (at !== stand()) return { stand: '', hint: '', error: staleText(active()), stale: true };
      const check = mockCheckWorld(active(), d);
      if (!check.ok) return { stand: '', hint: '', error: check.error, stale: false };
      replace(applyDraft(world(), d));
      bump();
      return { stand: stand(), hint: HINT, error: '', stale: false };
    },
  };
}
