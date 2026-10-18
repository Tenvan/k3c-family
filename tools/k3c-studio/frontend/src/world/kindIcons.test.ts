import { describe, expect, it } from 'vitest';
import { FALLBACK_ICON, iconOf, nameOf, rankOptions, resolveKind } from './kindIcons';
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

describe('Deutsche Namen je Art (B-485/AC-01)', () => {
  it('jede Chunk-Art und jedes Objekt hat einen deutschen Namen (Begriffe aus dem Glossar, wo es sie gibt)', () => {
    for (const kind of [...CHUNK_KINDS, ...OBJECT_KINDS]) expect(nameOf(kind), kind).not.toBe(kind);
    expect(nameOf('clearing')).toBe('Lichtung');
    expect(nameOf('chest')).toBe('Truhe');
    expect(nameOf('stoneVein')).toBe('Stein-Ader');
    expect(nameOf('rock')).toBe('Fels');
  });

  it('unbekannte Arten zeigen den Wert; der Wert bleibt der technische Name', () => {
    expect(nameOf('volcano')).toBe('volcano');
    expect(nameOf('constructor')).toBe('constructor');
    expect(resolveKind('Lichtung', CHUNK_KINDS)).toBe('clearing');
    expect(resolveKind('lichtung', CHUNK_KINDS)).toBe('clearing');
    expect(resolveKind('clearing', CHUNK_KINDS)).toBe('clearing');
    expect(resolveKind('volcano', CHUNK_KINDS)).toBe('volcano');
    expect(resolveKind('Fels', CHUNK_KINDS)).toBe('Fels'); // ein Objekt-Name wird in der Chunk-Liste nicht zur Objekt-Art
    expect(resolveKind('Fels', OBJECT_KINDS)).toBe('rock');
  });

  it('der Filter findet den deutschen Namen und den technischen Wert', () => {
    expect(rankOptions(CHUNK_KINDS, 'lich')[0]).toEqual({ value: 'clearing', match: true });
    expect(rankOptions(CHUNK_KINDS, 'clearing')[0]).toEqual({ value: 'clearing', match: true });
    expect(rankOptions(OBJECT_KINDS, 'ader').filter((o) => o.match).map((o) => o.value)).toEqual(['stoneVein', 'copperVein', 'ironVein', 'crystalVein']);
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
