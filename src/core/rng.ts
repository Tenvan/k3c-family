/**
 * Deterministischer Zufallsgenerator (mulberry32).
 * Gleicher Seed => gleiche Zahlenfolge => gleiches Level. Niemals Math.random() in der Generierung verwenden.
 */
export interface Rng {
  /** Float in [0, 1) */
  next(): number;
  /** Ganzzahl in [min, max] (beide inklusive) */
  int(min: number, max: number): number;
  pick<T>(items: readonly T[]): T;
  weighted<K extends string>(weights: Readonly<Record<K, number>>): K;
  shuffle<T>(items: T[]): T[];
}

/** Wandelt beliebige Seeds (z.B. "familie-2026") in eine 32-bit Zahl um (FNV-1a). */
export function hashSeed(seed: string | number): number {
  if (typeof seed === 'number') return seed >>> 0;
  let h = 0x811c9dc5;
  for (let i = 0; i < seed.length; i++) {
    h ^= seed.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return h >>> 0;
}

export function createRng(seed: string | number): Rng {
  let state = hashSeed(seed);

  const next = (): number => {
    state = (state + 0x6d2b79f5) >>> 0;
    let t = state;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };

  const int = (min: number, max: number): number => min + Math.floor(next() * (max - min + 1));

  return {
    next,
    int,
    pick: (items) => {
      if (items.length === 0) throw new Error('pick() auf leerer Liste');
      return items[int(0, items.length - 1)];
    },
    weighted: (weights) => {
      const entries = Object.entries(weights) as [keyof typeof weights, number][];
      const total = entries.reduce((sum, [, w]) => sum + w, 0);
      if (total <= 0) throw new Error('weighted() braucht mindestens ein positives Gewicht');
      let roll = next() * total;
      for (const [key, w] of entries) {
        roll -= w;
        if (roll < 0) return key;
      }
      return entries[entries.length - 1][0];
    },
    shuffle: (items) => {
      for (let i = items.length - 1; i > 0; i--) {
        const j = int(0, i);
        [items[i], items[j]] = [items[j], items[i]];
      }
      return items;
    },
  };
}
