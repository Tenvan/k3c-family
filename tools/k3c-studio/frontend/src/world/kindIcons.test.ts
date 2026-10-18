import { describe, expect, it } from 'vitest';
import { FALLBACK_ICON, iconOf, rankOptions } from './kindIcons';
import { CHUNK_KINDS, OBJECT_KINDS } from './plan';

// B-485/AC-01: Emoji je Art und Reihenfolge der durchsuchbaren Liste (reine Logik, ohne DOM).
const EMOJI = /^\p{Extended_Pictographic}$/u;

describe('Emoji je Art (B-485/AC-01)', () => {
  it('jede Chunk-Art und jedes Objekt hat ein Emoji aus einem Codepunkt (nicht das Ersatz-Emoji)', () => {
    for (const kind of [...CHUNK_KINDS, ...OBJECT_KINDS]) {
      expect(iconOf(kind), kind).toMatch(EMOJI);
      expect(iconOf(kind), kind).not.toBe(FALLBACK_ICON);
    }
  });

  it('unbekannte Arten bekommen das Ersatz-Emoji', () => {
    expect(iconOf('volcano')).toBe(FALLBACK_ICON);
    expect(iconOf('constructor')).toBe(FALLBACK_ICON); // freier Text trifft keine Objekt-Eigenschaft
    expect(FALLBACK_ICON).toMatch(EMOJI);
  });
});

describe('Reihenfolge der Liste (B-485/AC-01)', () => {
  const kinds = ['forest', 'rocks', 'stream', 'oreVein', 'portal'];

  it('ohne Suchtext: alle Einträge in der festen Reihenfolge, keiner abgeblendet', () => {
    expect(rankOptions(kinds, '')).toEqual(kinds.map((value) => ({ value, match: true })));
  });

  it('Tippen stellt Treffer nach oben (Anfang vor Teilwort), der Rest bleibt abgeblendet in der Liste', () => {
    expect(rankOptions(kinds, 'R')).toEqual([
      { value: 'rocks', match: true },
      { value: 'forest', match: true },
      { value: 'stream', match: true },
      { value: 'oreVein', match: true },
      { value: 'portal', match: true },
    ]);
    expect(rankOptions(kinds, 'st')).toEqual([
      { value: 'stream', match: true },
      { value: 'forest', match: true },
      { value: 'rocks', match: false },
      { value: 'oreVein', match: false },
      { value: 'portal', match: false },
    ]);
  });
});
