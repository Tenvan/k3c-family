import type { Backend, PlannedChunk, WorldsView } from './types';
import type { DraftScaling } from './worldTypes';

/** Eine Insel des Mocks wie in der Welt-Datei: Tiefen, Skalierung, Planung und Chunk-Anzahl je Tiefe. */
export interface MockIsland {
  id: string;
  name: string;
  depths: number[];
  depthScaling: DraftScaling;
  planned: Record<string, PlannedChunk[]>;
  counts: Record<string, number>;
}

/** Eine Welt des Mocks: Name, Notizen und eigene Inseln, wie eine Welt-Datei des Backends. */
export interface MockWorld {
  name: string;
  notes: string;
  islands: MockIsland[];
}

export const newIsland = (n: number): MockIsland => ({
  id: `island${n}`, name: `Insel ${n}`, depths: [0, 1, 2, 3, 4], depthScaling: { hp: 1.5, damage: 1.3, speed: 1.1 }, planned: {}, counts: {},
});
export const newWorld = (name: string): MockWorld => ({ name, notes: '', islands: [newIsland(1)] });

type WorldsPart = Pick<Backend, 'planWorlds' | 'planWorldSelect' | 'planWorldAdd' | 'planWorldCopy' | 'planWorldRemove'>;

const slug = (name: string) => name.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');

/** Welten-Teil des Mock-Plans: wählen, anlegen (leer oder als Kopie der gewählten, wird gewählt) und entfernen (Standard bleibt). */
export function mockWorlds(worlds: Record<string, MockWorld>, active: () => string, choose: (id: string) => void): WorldsPart {
  const view = (error = ''): WorldsView => ({ worlds: Object.entries(worlds).map(([id, w]) => ({ id, name: w.name })), active: active(), error });
  const add = (name: string, make: () => MockWorld) => {
    const id = slug(name);
    if (!id || id === 'standard' || worlds[id]) return view(`die Welt "${id}" gibt es schon oder ist ungültig`);
    worlds[id] = { ...make(), name: name.trim() };
    choose(id);
    return view();
  };
  return {
    planWorlds: async () => view(),
    planWorldSelect: async (id) => {
      if (!worlds[id]) return view(`die Welt "${id}" gibt es nicht`);
      choose(id);
      return view();
    },
    planWorldAdd: async (name) => add(name, () => newWorld(name)),
    planWorldCopy: async (name) => add(name, () => structuredClone(worlds[active()]!)),
    planWorldRemove: async (id) => {
      if (id === 'standard' || !worlds[id]) return view('die Standard-Welt bleibt');
      delete worlds[id];
      choose('standard');
      return view();
    },
  };
}
