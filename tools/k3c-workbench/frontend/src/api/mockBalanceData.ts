import type { BalanceBackend, BalanceDataFile, BalanceDataValue } from './types';

// Mock der Datei-Liste mit Beschreibungen (WZ28.10): dieselben 17 Dateien wie Go (internal/datadoc), Beschreibungen und
// Zahlenwerte aus der Repo-Wurzel (docs/data/*.md, data/*.json). Die Dateien laden erst beim ersten Aufruf (eigene
// Chunks), die echte Oberfläche lädt sie nie.

const SIM = 'Simulation', LEVEL = 'Level-Generator', TESTER = 'Balancing-Tester';

/** Name, Beschreibung, Leser, variierbar – wie `files` in internal/datadoc/datadoc.go. */
const LIST: [string, string, string[], boolean][] = [
  ['economy.json', 'economy', [SIM, LEVEL], true],
  ['buildings.json', 'buildings', [SIM, LEVEL], true],
  ['hub.json', 'hub', [SIM, LEVEL], true],
  ['troops.json', 'troops', [SIM], true],
  ['enemies.json', 'enemies', [SIM], true],
  ['waves.json', 'waves', [SIM], true],
  ['difficulty.json', 'difficulty', [SIM], false],
  ['bosses.json', 'bosses', [SIM], false],
  ['guards.json', 'guards', [LEVEL], false],
  ['events.json', 'events', [SIM], false],
  ['goals.json', 'goals', [SIM], false],
  ['island-sizes.json', 'island-sizes', [SIM], false],
  ['ruins.json', 'ruins', [LEVEL], false],
  ['monarch.json', 'monarch', [SIM], true],
  ['biomes/*.json', 'biomes', [SIM, LEVEL], false],
  ['worlds/*.json', 'worlds', [SIM], false],
  ['balance-targets.json', 'balance-targets', [TESTER], false],
];

const DOCS = import.meta.glob<string>('../../../../../docs/data/*.md', { query: '?raw', import: 'default' });
const DATA = import.meta.glob<string>('../../../../../data/*.json', { query: '?raw', import: 'default' });

const loader = (all: Record<string, () => Promise<string>>, file: string) =>
  Object.entries(all).find(([path]) => path.endsWith(`/${file}`))?.[1];

const line = (md: string, field: string) => new RegExp(`^- \\*\\*${field}:\\*\\*\\s*(.+)$`, 'm').exec(md)?.[1].trim() ?? '';

/** Alle Zahlen einer JSON mit Pfad `a.b.0.c`, in Dateireihenfolge (wie datadoc.Values). */
export function flattenNumbers(v: unknown, path = '', out: BalanceDataValue[] = []): BalanceDataValue[] {
  if (typeof v === 'number') out.push({ path, value: v });
  else if (Array.isArray(v)) v.forEach((c, i) => flattenNumbers(c, path ? `${path}.${i}` : String(i), out));
  else if (v && typeof v === 'object') for (const [k, c] of Object.entries(v)) flattenNumbers(c, path ? `${path}.${k}` : k, out);
  return out;
}

async function doc(name: string): Promise<string> {
  const load = LIST.some(([, d]) => d === name) ? loader(DOCS, `${name}.md`) : undefined;
  if (!load) throw new Error(`docs/data/${name}.md fehlt noch`);
  return load();
}

async function file([name, d, readBy, vary]: (typeof LIST)[number]): Promise<BalanceDataFile> {
  const f: BalanceDataFile = { name, doc: d, readBy, purpose: '', key: '', vary, missing: false };
  try {
    const md = await doc(d);
    return { ...f, purpose: line(md, 'Zweck'), key: line(md, 'Wichtigste Werte') };
  } catch {
    return { ...f, missing: true };
  }
}

export function mockBalanceData(): Pick<BalanceBackend, 'balanceDataFiles' | 'balanceDataDoc' | 'balanceDataValues'> {
  return {
    balanceDataFiles: () => Promise.all(LIST.map(file)),
    balanceDataDoc: doc,
    balanceDataValues: async (name) => {
      const load = LIST.some(([n, , , vary]) => n === name && vary) ? loader(DATA, name) : undefined;
      if (!load) throw new Error(`"${name}": nur variierbare Dateien der Liste`);
      return flattenNumbers(JSON.parse(await load()));
    },
  };
}
