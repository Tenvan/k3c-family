import enemiesJson from '../../data/enemies.json';
import troopsJson from '../../data/troops.json';
import type { KnownTypes } from './stressPlan';

const keys = (o: Record<string, unknown>): string[] => Object.keys(o).filter((k) => !k.startsWith('_'));
const BIOME_FILES = Object.keys(import.meta.glob('../../data/biomes/*.json'));

/** Bekannte Gegner-, Truppen- und Biom-Schlüssel aus den Daten (`data/enemies.json`, `troops.json`, `biomes/<biom>.json`) */
export const KNOWN: KnownTypes = {
  enemies: keys(enemiesJson),
  troops: keys(troopsJson),
  biomes: BIOME_FILES.map((f) => /([^/]+)\.json$/.exec(f)![1]!),
};
