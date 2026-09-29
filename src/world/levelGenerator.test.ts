import { describe, expect, it } from 'vitest';
import { createRng } from '../core/rng';
import { BIOMES } from './biome';
import { generateLevel, validateLevel } from './levelGenerator';

describe('rng', () => {
  it('liefert für gleichen Seed die gleiche Folge', () => {
    const a = createRng('familie');
    const b = createRng('familie');
    expect(Array.from({ length: 20 }, () => a.next())).toEqual(Array.from({ length: 20 }, () => b.next()));
  });

  it('int() bleibt in den Grenzen', () => {
    const rng = createRng(42);
    for (let i = 0; i < 1000; i++) {
      const v = rng.int(3, 7);
      expect(v).toBeGreaterThanOrEqual(3);
      expect(v).toBeLessThanOrEqual(7);
    }
  });
});

describe('generateLevel', () => {
  for (const biome of BIOMES) {
    describe(biome.id, () => {
      it('ist deterministisch (gleicher Seed => gleiches Level)', () => {
        expect(generateLevel(biome, 'abc')).toEqual(generateLevel(biome, 'abc'));
      });

      it('erzeugt mit verschiedenen Seeds verschiedene Level', () => {
        const a = generateLevel(biome, 1).chunks.map((c) => c.kind).join();
        const b = generateLevel(biome, 2).chunks.map((c) => c.kind).join();
        expect(a).not.toEqual(b);
      });

      it('hält alle Spielbarkeits-Regeln über 500 Seeds ein', () => {
        for (let seed = 0; seed < 500; seed++) {
          const errors = validateLevel(generateLevel(biome, seed), biome);
          expect(errors, `Seed ${seed}`).toEqual([]);
        }
      });

      it('platziert den Hub in der Mitte', () => {
        const level = generateLevel(biome, 7);
        expect(level.hubCenterUnits).toBe(level.widthUnits / 2);
        expect(level.entities.find((e) => e.kind === 'castle')?.x).toBe(level.hubCenterUnits);
      });
    });
  }
});
