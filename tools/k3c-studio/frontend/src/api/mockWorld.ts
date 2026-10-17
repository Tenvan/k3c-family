import type { WorldLevelView, WorldSource } from './types';

// Erfundenes Level für den Mock (Browser, Tests): immer gleich zu gleichem Seed, 1 Abschnitt = 30 Units. Der Seed
// „fehler“ liefert den Hinweis einer abgelehnten Anfrage.

const KINDS = ['edge', 'normal', 'normal', 'recruitCamp', 'normal', 'hub', 'normal', 'chest', 'portal', 'normal', 'normal', 'exit'];

export function mockWorldLevel(seed: string, source: WorldSource): WorldLevelView {
  const empty = { seed, biomeId: source.biome ?? '', widthUnits: 0, chunkWidthUnits: 30, hubCenterUnits: 0, chunks: [], entities: [], warnings: [] };
  if (seed === 'fehler') return { ...empty, error: 'Seed braucht 1 bis 64 Zeichen' };
  const chunks = KINDS.map((kind, i) => ({ index: i, kind, startUnits: i * 30 })) as WorldLevelView['chunks'];
  const entities = [
    { kind: 'castle', x: 165 }, { kind: 'portal', x: 255 }, { kind: 'exit', x: 345 }, { kind: 'tree', x: 40 }, { kind: 'tree', x: 70 },
    { kind: 'rock', x: 100 }, { kind: 'chest', x: 225 }, { kind: 'recruitCamp', x: 105 },
  ];
  return { ...empty, widthUnits: KINDS.length * 30, hubCenterUnits: 165, chunks, entities, warnings: seed === 'warnung' ? ['Level zu kurz'] : [], error: '' };
}
