import type { NatureGroup, NatureTrait, NatureValues, WorldBackend, WorldDetail, WorldInfo, WorldLevel, WorldValueEntry, WorldValuesView } from './typesWorld';

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
  const fixed = mockFixed.get(`${detail.id}:${island}:${stage}`) ?? {};
  const nature = { traits: { forestDensity: 2, ...fixed.traits }, weights: { treeSpecies: { tanne: 3, birke: 1, ...fixed.weights?.treeSpecies } } };
  return { ...empty, biomeId: isl.biomes[stage] ?? '', widthUnits: KINDS.length * CHUNK, hubCenterUnits: 6 * CHUNK, chunks, entities, planned, error: '',
    nature, natureFixed: fixed, natureTraits: MOCK_TRAITS, natureWeights: MOCK_WEIGHTS };
}

// Natur im Mock (B-891): eine Eigenschaft und eine Gewichts-Gruppe, feste Werte je Welt, Insel und Stufe.
const MOCK_TRAITS: NatureTrait[] = [{ key: 'forestDensity', name: 'Walddichte', min: 1, max: 5, default: 3, scales: ['tree'] }];
const MOCK_WEIGHTS: NatureGroup[] = [{ key: 'treeSpecies', name: 'Baumarten', kind: 'tree', options: [{ key: 'tanne', min: 0, max: 5 }, { key: 'birke', min: 0, max: 5 }] }];
const mockFixed = new Map<string, NatureValues>();

/** Festsetzen im Mock: Bereich wie im Backend, null entfernt den Wert. */
function mockNatureSet(id: string, island: string, stage: number, key: string, value: number | null): string {
  const [group, option] = key.split('.');
  const range = option ? MOCK_WEIGHTS.find((g) => g.key === group)?.options.find((o) => o.key === option) : MOCK_TRAITS.find((t) => t.key === key);
  if (!range) return `unbekannter Natur-Schlüssel ${key}`;
  if (value !== null && (value < range.min || value > range.max)) return `${key} = ${value} liegt außerhalb ${range.min}–${range.max}`;
  const at = `${id}:${island}:${stage}`;
  const cur = mockFixed.get(at) ?? {};
  const next: NatureValues = { traits: { ...cur.traits }, weights: { treeSpecies: { ...cur.weights?.treeSpecies } } };
  const bucket = option ? next.weights!.treeSpecies! : next.traits!;
  if (value === null) delete bucket[option ?? key];
  else bucket[option ?? key] = value;
  mockFixed.set(at, next);
  return '';
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

type Overrides = Map<string, Map<string, unknown>>;

function mockValues(overrides: Overrides, id: string): WorldValuesView {
  const own = (w: string) => overrides.get(w) ?? new Map<string, unknown>();
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
}

/** Anzeige-Werte der Biome (Mock; echte Werte liefert das Backend aus data/biomes). */
const withInfo = (d: WorldDetail | undefined): WorldDetail | undefined => d && {
  ...d,
  biomeInfo: Object.fromEntries(d.islands.flatMap((i) => i.biomes).map((b, n) => [b, { portalHp: 500 + 300 * (n % 5), palette: { sky: '#8ecae6', far: '#6a8caf', near: '#3a5a40', ground: '#588157' } }])),
};

const mockCreate: WorldBackend['worldCreate'] = async (id, name, extendsId) => {
  const fail = (error: string) => ({ id, warnings: [], error });
  if (!/^[a-z0-9][a-z0-9-]{0,31}$/.test(id)) return fail(`ungültige Welt-ID "${id}"`);
  if (name.trim() === '') return fail('der Name fehlt');
  if (DETAILS[id]) return fail(`data/worlds/${id}.json gibt es schon (ändern mit world_set)`);
  if (extendsId !== '' && !DETAILS[extendsId]) return fail(`extends: die Welt "${extendsId}" gibt es nicht`);
  DETAILS[id] = { id, name: name.trim(), notes: '', extends: extendsId, error: '',
    islands: [{ id: 'island1', name: 'Insel 1', depths: [0, 1, 2, 3, 4], biomes: ['forest', 'cave', 'mine', 'ironhold', 'crystal'], plannedChunks: {}, chunkCounts: {} }] };
  return { id, warnings: [], error: '' };
};

const mockEtappe: WorldBackend['worldIslandEtappe'] = async (id, island, etappe) => {
  const isl = DETAILS[id]?.islands.find((i) => i.id === island);
  if (!isl) return { id, warnings: [], error: `die Insel "${island}" gibt es nicht` };
  if (etappe === null) delete isl.etappe;
  else isl.etappe = etappe;
  return { id, warnings: [], error: '' };
};

export function mockWorld(): WorldBackend {
  const overrides: Overrides = new Map([['wueste', new Map([[key('economy', ['purse', 'startGold']), 20]])]]);
  const values = (id: string) => mockValues(overrides, id);
  return {
    worldList: async () => {
      const files = Object.values(DETAILS).map((d): WorldInfo => ({ id: d.id, name: d.name }));
      const fixed = ['sengoku', 'nil', 'britannien'].map((id): WorldInfo => ({ id, name: id, missing: true }));
      return [...files.filter((w) => w.id === 'standard'), ...fixed, ...files.filter((w) => w.id !== 'standard')]; // wie List() im Backend
    },
    worldDetail: async (id) => withInfo(DETAILS[id]) ?? { id, name: id, notes: '', extends: '', islands: [], error: `data/worlds/${id}.json: Datei fehlt` },
    worldLevel: async (id, island, stage, seed) => {
      const detail = DETAILS[id];
      return detail ? mockLevel(detail, island, stage, seed) : { ...mockLevel(DETAILS.standard!, '', 0, seed), error: `die Welt "${id}" gibt es nicht` };
    },
    worldValues: async (id) => values(id),
    worldCreate: mockCreate,
    worldIslandEtappe: mockEtappe,
    worldNatureSet: async (id, island, stage, key, value, seed) => {
      const detail = DETAILS[id] ?? DETAILS.standard!;
      const rejected = mockNatureSet(id, island, stage, key, value);
      return { ...mockLevel(detail, island, stage, seed), ...(rejected ? { rejected } : {}) };
    },
    worldOverrideSet: async (id, file, path, value) => {
      const view = values(id);
      const entry = view.entries.find((e) => e.file === file && same(e.path, path));
      if (!entry) return { ...view, error: `das Feld ${file} › ${path.join('.')} gibt es nicht` };
      if (typeof entry.base === 'number' && value !== null && typeof value !== 'number') {
        return { ...view, error: `${file}.${path.join('.')}: Zahl erwartet` };
      }
      const map = overrides.get(id) ?? new Map<string, unknown>();
      if (value === null || same(value, entry.base)) map.delete(key(file, path));
      else map.set(key(file, path), value);
      overrides.set(id, map);
      return values(id);
    },
  };
}
