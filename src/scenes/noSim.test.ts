import { describe, expect, it } from 'vitest';

/**
 * SP08 AC-13 und SP09 AC-05: Der Browser rechnet nichts und kennt `src/world` nicht. Der Client (`src/scenes`, `src/core`,
 * `src/online/client*`, `src/online/protocol.ts`) importiert Typen und Daten nur aus `src/model`.
 */
const sources = import.meta.glob(
  ['./*.ts', '../core/*.ts', '../online/client*.ts', '../online/protocol.ts'],
  { query: '?raw', import: 'default', eager: true },
) as Record<string, string>;

/** Alle Importe und Re-Exports mit einem Pfad, der `world/` enthält. */
const FROM = /^(?:import|export)\s[^;]*?from\s+'([^']+)'/gm;

function worldImports(text: string): string[] {
  return [...text.matchAll(FROM)].map((m) => m[1]!).filter((path) => /(^|\/)world(\/|$)/.test(path));
}

describe('Client kennt src/world nicht (AC-05)', () => {
  const files = Object.entries(sources).filter(([name]) => !name.endsWith('.test.ts'));

  it('findet die Client-Dateien', () => {
    expect(files.map(([n]) => n)).toEqual(
      expect.arrayContaining(['./GameScene.ts', './HudScene.ts', './worldRenderer.ts', '../core/saveStore.ts', '../online/clientConnection.ts']),
    );
  });

  it.each(files)('%s importiert nichts aus world/', (_name, text) => {
    expect(worldImports(text)).toEqual([]);
  });

  it('erkennt einen verbotenen Import und Re-Export', () => {
    const bad = worldImports("import { step } from '../world/sim/world';\nexport type { World } from '../../world/sim/types';\nimport type { World } from '../model/types';");
    expect(bad).toEqual(['../world/sim/world', '../../world/sim/types']);
  });
});
