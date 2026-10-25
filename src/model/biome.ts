export type Range = [min: number, max: number];

export type EventChunkKind = 'chest' | 'recruitCamp';

/** Die "festen Eckdaten" einer Stufe. Wird aus data/biomes/*.json geladen. */
export interface BiomeConfig {
  id: string;
  name: string;
  depth: number;
  lengthUnits: { min: number; max: number };
  chunkWidthUnits: number;
  hubWidthUnits: number;
  exitSide: 'left' | 'right';
  portals: { count: number; minDistanceFromHubUnits: number };
  /** Gewichtung der normalen Chunk-Typen (z.B. forest: 5 => 5x so häufig wie ein Typ mit Gewicht 1) */
  chunkWeights: Record<string, number>;
  /** Chunks mit genau einem besonderen Objekt (belegen einen eigenen Chunk) */
  eventChunks: Record<EventChunkKind, { min: number; max: number }>;
  /** Versteckte Skill-Punkte, liegen innerhalb normaler Chunks */
  skillPoints: { min: number; max: number };
  /** Pro Chunk-Typ: Ressource -> [min, max] Anzahl */
  resourcesPerChunk: Record<string, Record<string, Range>>;
  primaryResource: string;
  enemies: { portal: string[]; night: string[] };
  cycle:
    | { type: 'dayNight'; dayMinutes: number; nightMinutes: number; twilightMinutes: number }
    | { type: 'aggressionPool'; percentPerMinute: number; percentPerKill: number; percentPerGather: number };
  palette: { sky: string; far: string; near: string; ground: string };
  /** Wurzeln je Baum-Knoten im Höhle-Look (GEB15.1); ab der zweiten reine Deko des Clients, ohne Feld 1 */
  rootsPerNode?: number;
}

// Alle Biome aus data/biomes/, nach Tiefe sortiert (B-290). JSON kennt keine Tupel/Literal-Typen, daher der Cast;
// biome.test.ts prüft Pflichtfelder und lückenlose Tiefen.
const FILES = import.meta.glob('../../data/biomes/*.json', { eager: true, import: 'default' });
export const BIOMES: readonly BiomeConfig[] = (Object.values(FILES) as BiomeConfig[]).sort((a, b) => a.depth - b.depth);

export function biomeForDepth(depth: number): BiomeConfig {
  const biome = BIOMES.find((b) => b.depth === depth);
  if (!biome) throw new Error(`Kein Biom für Tiefe ${depth}`);
  return biome;
}
