import { describe, expect, it } from 'vitest';

/**
 * SP08 AC-13: Der Browser rechnet nichts. `src/scenes` darf aus `src/world` nur Typen, Daten und Biom-Daten laden,
 * keine Simulationsfunktionen (`world`, `campaign`, `economy`, `units`, `travel` …).
 */
const sources = import.meta.glob('./*.ts', { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

/** Module unter `world/sim/`, die nur Typen oder Daten enthalten */
const ALLOWED_SIM = new Set(['types', 'data']);
const IMPORT = /^import\s+(type\s+)?[^;]*?from\s+'([^']+)'/gm;

function worldImports(text: string): { typeOnly: boolean; path: string }[] {
  return [...text.matchAll(IMPORT)].map((m) => ({ typeOnly: !!m[1], path: m[2]! })).filter((i) => /(^|\/)world(\/|$)/.test(i.path));
}

describe('src/scenes rechnet nichts (AC-13)', () => {
  const files = Object.entries(sources).filter(([name]) => !name.endsWith('.test.ts'));

  it('findet die Szenen-Dateien', () => {
    expect(files.map(([n]) => n)).toEqual(expect.arrayContaining(['./GameScene.ts', './HudScene.ts', './worldRenderer.ts']));
  });

  it.each(files)('%s importiert keine Simulationsfunktionen', (_name, text) => {
    for (const { typeOnly, path } of worldImports(text)) {
      if (typeOnly) continue;
      const sim = /world\/sim\/([\w-]+)$/.exec(path)?.[1];
      if (sim) expect(ALLOWED_SIM.has(sim), `${path} ist keine Daten-/Typ-Datei`).toBe(true);
      else expect(path, `${path}: nur Biom-Daten (world/biome) sind erlaubt`).toMatch(/world\/biome$/);
    }
  });

  it('erkennt einen verbotenen Import', () => {
    const bad = worldImports("import { step } from '../world/sim/world';\nimport type { World } from '../world/sim/types';");
    expect(bad).toEqual([
      { typeOnly: false, path: '../world/sim/world' },
      { typeOnly: true, path: '../world/sim/types' },
    ]);
  });
});
