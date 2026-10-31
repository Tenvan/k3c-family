import type { Overrides, WorldValueEntry, WorldValuesView } from './worldTypes';

// Werte-Overrides im Mock (B-682): ein paar Felder der Basis-Dateien. Die Prüfung kennt nur den Typ: ein Override, dessen
// Basis eine Zahl ist, muss eine Zahl sein (Go prüft alles, sim.ResolveWorldValues).

export const MOCK_BASE: { file: string; path: string[]; base: unknown }[] = [
  { file: 'buildings', path: ['wall', 'cost', 'wood'], base: 10 },
  { file: 'buildings', path: ['wall', 'levels', '1', 'hp'], base: 500 },
  { file: 'enemies', path: ['wolf', 'gold'], base: [3, 5] },
  { file: 'waves', path: ['perExtraPlayer'], base: 0.3 },
];

/** Wert am Pfad im Override (undefined = geerbt). */
export function mockOverrideAt(ov: Overrides | undefined, file: string, path: string[]): unknown {
  let cur: unknown = ov?.[file];
  for (const k of path) cur = cur && typeof cur === 'object' && !Array.isArray(cur) ? (cur as Record<string, unknown>)[k] : undefined;
  return cur;
}

export function mockValues(world: string, ov: Overrides | undefined, children: string[]): WorldValuesView {
  const entries: WorldValueEntry[] = MOCK_BASE.map((e) => {
    const o = mockOverrideAt(ov, e.file, e.path);
    return o === undefined ? { ...e } : { ...e, override: o };
  });
  return { world, extends: '', children, entries, error: '' };
}

/** Fehler wie Go (welten.md § 1.2) für den ersten Override mit falschem Typ, sonst null. */
export function mockOverrideFault(world: string, ov: Overrides | undefined): string | null {
  for (const e of MOCK_BASE) {
    const o = mockOverrideAt(ov, e.file, e.path);
    if (o !== undefined && typeof e.base === 'number' && typeof o !== 'number') {
      return `data/worlds/${world}.json: overrides › data/${e.file}.json › ${e.path.join('.')}: Zahl erwartet, ist Text`;
    }
  }
  return null;
}
