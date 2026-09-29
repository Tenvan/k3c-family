import type { Rng } from '../../core/rng';
import type { BiomeConfig } from '../biome';
import { ENEMIES, WAVES, type WaveRow } from './data';

export interface SpawnOrder {
  kind: string;
  /** Portal-Position (Units) */
  x: number;
  /** Sekunden nach Wellenstart */
  delay: number;
}

export function waveRow(wave: number): WaveRow {
  let row = WAVES.table[0];
  for (const r of WAVES.table) if (wave >= r.fromWave) row = r;
  return row;
}

/**
 * Stellt eine Welle zusammen: Anzahl laut Tabelle, Gegnertypen aus dem Biom (nachts inkl. Nacht-Gegnern),
 * gleichmäßig auf die Portale verteilt und über `spawnSpreadSeconds` gestaffelt.
 */
export function planWave(biome: BiomeConfig, wave: number, rng: Rng, portals: readonly number[], night: boolean): SpawnOrder[] {
  if (portals.length === 0) return [];
  const row = waveRow(wave);
  const kinds = [...biome.enemies.portal, ...(night ? biome.enemies.night : [])];
  const standard = kinds.filter((k) => ENEMIES[k]?.tier === 'standard');
  const elite = kinds.filter((k) => ENEMIES[k]?.tier === 'elite');

  const picks: string[] = [];
  if (standard.length > 0) for (let n = rng.int(...row.standard); n > 0; n--) picks.push(rng.pick(standard));
  if (elite.length > 0) for (let n = rng.int(...row.elite); n > 0; n--) picks.push(rng.pick(elite));

  const spread = WAVES.spawnSpreadSeconds;
  return picks.map((kind, i) => ({
    kind,
    x: portals[i % portals.length],
    delay: (i / Math.max(1, picks.length)) * spread + rng.next(),
  }));
}
