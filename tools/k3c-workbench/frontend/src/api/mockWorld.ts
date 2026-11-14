import type { WorldBackend, WorldDetail, WorldLevel, WorldValueEntry, WorldValuesView } from './typesWorld';

// Mock des Welten-Editors (B-879, `?mock=1` und Vitest): zwei Welten, ein festes Level je Stufe und ein paar Werte.
// Overrides leben nur im Speicher; ein Text in einem Zahlenfeld wird abgelehnt wie die Prüfung in Go.

const DETAILS: Record<string, WorldDetail> = {
  standard: {
    id: 'standard', name: 'Standard', notes: 'Standard-Welt (Mock)', extends: '', error: '',
    islands: [{
      id: 'island1', name: 'Insel 1', depths: [0, 1, 2], biomes: ['forest', 'cave', 'mine'],
      plannedChunks: { 0: [{ slot: -2, kind: 'harbor' }, { slot: 1, kind: 'clearing', objects: [{ kind: 'chest', at: 8 }] }] },
      chunkCounts: {},
    }],
  },
  wueste: {
    id: 'wueste', name: 'Wüste', notes: '', extends: 'standard', error: '',
    islands: [{ id: 'island1', name: 'Sandinsel', depths: [0], biomes: ['forest'], plannedChunks: {}, chunkCounts: {} }],
  },
};

const KINDS = ['edge', 'forest', 'portal', 'harbor', 'forest', 'hub', 'hub', 'clearing', 'rocks', 'chest', 'portal', 'edge'];
const CHUNK = 20;

function mockLevel(detail: WorldDetail, island: string, stage: number, seed: string): WorldLevel {
  const empty = { seed, biomeId: '', widthUnits: 0, chunkWidthUnits: CHUNK, hubCenterUnits: 0, chunks: [], entities: [], planned: [], warnings: [] };
  const isl = detail.islands.find((i) => i.id === island);
  if (!isl) return { ...empty, error: `die Insel "${island}" gibt es nicht` };
  if (stage < 0 || stage >= isl.depths.length) return { ...empty, error: `die Stufe ${stage} gibt es auf der Insel "${island}" nicht` };
  if (seed === '') return { ...empty, error: 'Seed braucht 1 bis 64 Zeichen' };
  const chunks = KINDS.map((kind, index) => ({ index, kind, startUnits: index * CHUNK }));
  const entities = [
    { kind: 'castle', x: 6 * CHUNK }, { kind: 'portal', x: 2 * CHUNK + 10 }, { kind: 'portal', x: 10 * CHUNK + 10 },
    { kind: 'tree', x: 1 * CHUNK + 5 }, { kind: 'tree', x: 4 * CHUNK + 12 }, { kind: 'chest', x: 9 * CHUNK + 8 },
    { kind: 'rock', x: 8 * CHUNK + 4 }, { kind: 'exit', x: 11 * CHUNK + 10 },
  ];
  const planned = isl.plannedChunks[String(isl.depths[stage])] ?? [];
  return { ...empty, biomeId: isl.biomes[stage] ?? '', widthUnits: KINDS.length * CHUNK, hubCenterUnits: 6 * CHUNK, chunks, entities, planned, error: '' };
}

const BASE: WorldValueEntry[] = [
  { file: 'buildings', path: ['wall', 'cost'], base: 3 },
  { file: 'buildings', path: ['wall', 'hp'], base: 40 },
  { file: 'economy', path: ['purse', 'startGold'], base: 14 },
  { file: 'economy', path: ['purse', 'maxGold'], base: 30 },
  { file: 'economy', path: ['chestGold'], base: [3, 6] },
  { file: 'monarch', path: ['name'], base: 'Monarch' },
];

const key = (file: string, path: readonly string[]) => `${file}:${path.join('.')}`;
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);

export function mockWorld(): WorldBackend {
  const overrides = new Map<string, Map<string, unknown>>([['wueste', new Map([[key('economy', ['purse', 'startGold']), 20]])]]);
  const own = (id: string) => overrides.get(id) ?? new Map<string, unknown>();
  const values = (id: string): WorldValuesView => {
    const detail = DETAILS[id];
    if (!detail) return { world: id, extends: '', children: [], entries: [], error: `data/worlds/${id}.json: Datei fehlt` };
    const children = Object.values(DETAILS).filter((d) => d.extends === id).map((d) => d.id);
    const parent = own(detail.extends);
    const entries = BASE.map((e) => {
      const base = parent.get(key(e.file, e.path)) ?? e.base;
      const ov = own(id).get(key(e.file, e.path));
      return ov === undefined ? { ...e, base } : { ...e, base, override: ov };
    });
    return { world: id, extends: detail.extends, children, entries, error: '' };
  };
  return {
    worldList: async () => Object.values(DETAILS).map((d) => ({ id: d.id, name: d.name })),
    worldDetail: async (id) => DETAILS[id] ?? { id, name: id, notes: '', extends: '', islands: [], error: `data/worlds/${id}.json: Datei fehlt` },
    worldLevel: async (id, island, stage, seed) => {
      const detail = DETAILS[id];
      return detail ? mockLevel(detail, island, stage, seed) : { ...mockLevel(DETAILS.standard!, '', 0, seed), error: `die Welt "${id}" gibt es nicht` };
    },
    worldValues: async (id) => values(id),
    worldOverrideSet: async (id, file, path, value) => {
      const view = values(id);
      const entry = view.entries.find((e) => e.file === file && same(e.path, path));
      if (!entry) return { ...view, error: `das Feld ${file} › ${path.join('.')} gibt es nicht` };
      if (typeof entry.base === 'number' && value !== null && typeof value !== 'number') {
        return { ...view, error: `${file}.${path.join('.')}: Zahl erwartet` };
      }
      const map = own(id);
      if (value === null || same(value, entry.base)) map.delete(key(file, path));
      else map.set(key(file, path), value);
      overrides.set(id, map);
      return values(id);
    },
  };
}
