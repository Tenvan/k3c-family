import { describe, expect, it } from 'vitest';
import { BIOMES, biomeForDepth } from './biome';

// B-290/AC-02: Der Client kennt jedes Biom aus data/biomes/, nicht nur eine feste Liste.
const FILES = Object.keys(import.meta.glob('../../data/biomes/*.json')).map((f) => f.replace(/^.*\/(\w+)\.json$/, '$1'));

describe('BIOMES', () => {
  it('enthält jede Datei aus data/biomes, Tiefen lückenlos ab 0', () => {
    expect(BIOMES.map((b) => b.id).sort()).toEqual([...FILES].sort());
    expect(BIOMES.map((b) => b.depth)).toEqual(BIOMES.map((_, i) => i));
  });

  it('jedes Biom hat id, name und eine volle Palette', () => {
    for (const b of BIOMES) {
      expect(b.id, b.id).toBeTruthy();
      expect(b.name, b.id).toBeTruthy();
      for (const key of ['sky', 'far', 'near', 'ground'] as const) expect(b.palette[key], `${b.id}.${key}`).toMatch(/^#[0-9a-f]{6}$/i);
    }
  });

  it('Tiefe 3 ist der Eisenstollen, Tiefe 4 die Kristallhöhle', () => {
    expect(biomeForDepth(3).id).toBe('ironhold');
    expect(biomeForDepth(4).id).toBe('crystal');
    expect(() => biomeForDepth(5)).toThrow();
  });
});
