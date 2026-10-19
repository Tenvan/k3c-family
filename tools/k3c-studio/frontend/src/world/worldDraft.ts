import type { IslandDraft, PlanView, WorldDraft } from '../api';

// Reine Logik des Welt-Entwurfs (WZ18.6, B-488/AC-05): Welt-Kopf und Inseln (anlegen, umordnen, entfernen, Felder);
// Planung und Chunk-Anzahl bleiben beim Editor für geplante Chunks. Geprüft wird im Backend (planWorldCheck).

/** Entwurf zum geladenen Stand: jede Insel mit ihrer id als Herkunft. */
export function draftOf(view: PlanView): WorldDraft {
  return {
    name: view.name,
    notes: view.notes,
    islands: view.islands.map((i) => ({ from: i.id, id: i.id, name: i.name, depths: [...i.depths], depthScaling: { ...i.depthScaling } })),
  };
}

export const sameDraft = (a: WorldDraft | null, b: WorldDraft | null): boolean => JSON.stringify(a) === JSON.stringify(b);

/** Neue Insel am Ende als Kopie der letzten (Tiefen, Skalierung) mit freier id `islandN` und Name „Insel N“. */
export function addIsland(d: WorldDraft): WorldDraft {
  const last = d.islands.at(-1);
  let n = d.islands.length + 1;
  while (d.islands.some((i) => i.id === `island${n}`)) n++;
  const island: IslandDraft = {
    from: '', id: `island${n}`, name: `Insel ${n}`,
    depths: last ? [...last.depths] : [0, 1, 2, 3, 4],
    depthScaling: last ? { ...last.depthScaling } : { hp: 1.5, damage: 1.3, speed: 1.1 },
  };
  return { ...d, islands: [...d.islands, island] };
}

/** Insel i um delta Plätze verschieben (−1 hoch, +1 runter); außerhalb der Liste bleibt alles. */
export function moveIsland(d: WorldDraft, i: number, delta: number): WorldDraft {
  const to = i + delta;
  if (to < 0 || to >= d.islands.length) return d;
  const islands = [...d.islands];
  [islands[i], islands[to]] = [islands[to]!, islands[i]!];
  return { ...d, islands };
}

export const removeIsland = (d: WorldDraft, i: number): WorldDraft => ({ ...d, islands: d.islands.filter((_, k) => k !== i) });

export const updateIsland = (d: WorldDraft, i: number, patch: Partial<IslandDraft>): WorldDraft =>
  ({ ...d, islands: d.islands.map((isl, k) => (k === i ? { ...isl, ...patch } : isl)) });

/** Tiefe ein- oder ausschalten; die Tiefen bleiben aufsteigend. */
export const toggleDepth = (depths: number[], depth: number): number[] =>
  (depths.includes(depth) ? depths.filter((d) => d !== depth) : [...depths, depth].sort((a, b) => a - b));
