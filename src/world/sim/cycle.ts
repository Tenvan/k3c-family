import { BIOMES } from '../../model/biome';
import type { CycleInfo } from '../../model/types';

export type { CycleInfo, Phase } from '../../model/types';

/**
 * Tag/Nacht-Zyklus: Tag → Dämmerung → Nacht → (nächster Tag).
 * Unter Tage läuft der globale Zyklus der Oberwelt weiter (Skelette nachts), Wellen kommen dort aber über den Aggressionspool.
 */
export interface DayNightConfig {
  dayMinutes: number;
  twilightMinutes: number;
  nightMinutes: number;
}

export function cycleAt(cfg: DayNightConfig, seconds: number): CycleInfo {
  const day = cfg.dayMinutes * 60;
  const dusk = cfg.twilightMinutes * 60;
  const night = cfg.nightMinutes * 60;
  const length = day + dusk + night;
  const n = Math.floor(seconds / length);
  const t = seconds - n * length;
  if (t < day) return { phase: 'day', day: n + 1, progress: t / day, secondsLeft: day - t };
  if (t < day + dusk) return { phase: 'dusk', day: n + 1, progress: (t - day) / dusk, secondsLeft: day + dusk - t };
  return { phase: 'night', day: n + 1, progress: (t - day - dusk) / night, secondsLeft: length - t };
}

/** Der globale Zyklus = der Zyklus des ersten Tag/Nacht-Bioms (Oberwelt). */
export function globalDayNight(): DayNightConfig {
  for (const b of BIOMES) if (b.cycle.type === 'dayNight') return b.cycle;
  throw new Error('Kein Biom mit Tag/Nacht-Zyklus');
}

/** Helligkeit für die Darstellung: 1 = Tag, 0 = tiefste Nacht. */
export function daylight(info: CycleInfo): number {
  if (info.phase === 'day') return 1;
  if (info.phase === 'dusk') return 1 - info.progress;
  // Letzte 10% der Nacht: Morgengrauen
  return info.progress > 0.9 ? (info.progress - 0.9) * 10 : 0;
}
