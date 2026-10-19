import type { Backend, PlannedChunk, WorldsView } from './types';

/** Eine Welt des Mocks: eigene Inseln und Planung, wie eine Welt-Datei des Backends. */
export interface MockWorld {
  name: string;
  planned: Record<string, PlannedChunk[]>;
  counts: Record<string, number>;
  islands: number;
}

export const newWorld = (name: string): MockWorld => ({ name, planned: {}, counts: {}, islands: 1 });

type WorldsPart = Pick<Backend, 'planWorlds' | 'planWorldSelect' | 'planWorldAdd' | 'planWorldRemove'>;

/** Welten-Teil des Mock-Plans: wählen, anlegen (wird gewählt) und entfernen (Standard bleibt). */
export function mockWorlds(worlds: Record<string, MockWorld>, active: () => string, choose: (id: string) => void): WorldsPart {
  const view = (error = ''): WorldsView => ({ worlds: Object.entries(worlds).map(([id, w]) => ({ id, name: w.name })), active: active(), error });
  return {
    planWorlds: async () => view(),
    planWorldSelect: async (id) => {
      if (!worlds[id]) return view(`die Welt "${id}" gibt es nicht`);
      choose(id);
      return view();
    },
    planWorldAdd: async (name) => {
      const id = name.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
      if (!id || id === 'standard' || worlds[id]) return view(`die Welt "${id}" gibt es schon oder ist ungültig`);
      worlds[id] = newWorld(name.trim());
      choose(id);
      return view();
    },
    planWorldRemove: async (id) => {
      if (id === 'standard' || !worlds[id]) return view('die Standard-Welt bleibt');
      delete worlds[id];
      choose('standard');
      return view();
    },
  };
}
