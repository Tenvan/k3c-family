import forest from '../data/biomes/forest.json';
import cave from '../data/biomes/cave.json';
import mine from '../data/biomes/mine.json';

export type Range = [min: number, max: number];

export type EventChunkKind = 'chest' | 'recruitCamp';

/** Die "festen Eckdaten" einer Stufe. Wird aus src/data/biomes/*.json geladen. */
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
}

// JSON kennt keine Tupel/Literal-Typen, daher der Cast. Die Configs werden in levelGenerator.test.ts über 500 Seeds geprüft.
export const BIOMES: readonly BiomeConfig[] = [forest, cave, mine] as unknown as BiomeConfig[];

export function biomeForDepth(depth: number): BiomeConfig {
  const biome = BIOMES.find((b) => b.depth === depth);
  if (!biome) throw new Error(`Kein Biom für Tiefe ${depth}`);
  return biome;
}
