// @vitest-environment happy-dom
import { describe, expect, it } from 'vitest';
import { CONVERT_HINT, convertChunk, convertLock, inSync } from './convert';

// B-485/AC-06 (WZ17.5): reine Logik der Umwandlung eines Seed-Chunks in feste Objekte.

describe('Seed-Chunk umwandeln', () => {
  const seeded = { slot: -3, kind: 'clearing', seed: 'lichtung' };

  it('bereit nur mit Chunk-Seed, ohne feste Objekte und bei geprüftem Entwurf', () => {
    expect(convertLock(seeded, true)).toBeUndefined();
    expect(convertLock({ slot: -3, kind: 'clearing' }, true)).toContain('Chunk-Seed');
    expect(convertLock({ ...seeded, objects: [{ kind: 'tree', at: 4 }] }, true)).toContain('feste Objekte');
    expect(convertLock(seeded, false)).toContain('Prüfung');
    expect(CONVERT_HINT).toBe('Das Bild bleibt gleich');
  });

  it('inSync vergleicht Entwurf und Anzahl mit der Quelle der Vorschauen', () => {
    const source = { island: 'i', stage: 0, planned: [seeded], chunkCount: 24 };
    expect(inSync([{ ...seeded, objects: [] }], 24, source)).toBe(true);
    expect(inSync([seeded], 40, source)).toBe(false);
    expect(inSync([{ ...seeded, seed: 'x' }], 24, source)).toBe(false);
    expect(inSync([seeded], 0, undefined)).toBe(false);
    expect(inSync([], 0, { island: 'i', stage: 0, planned: [] })).toBe(true);
  });

  it('convertChunk ersetzt den Seed durch die Objekte, Platz, Art und andere Chunks bleiben', () => {
    const other = { slot: 6, kind: 'portal' };
    const objects = [{ kind: 'tree', at: 7.5 }, { kind: 'rock', at: 20 }];
    expect(convertChunk([seeded, other], 0, objects)).toEqual([{ slot: -3, kind: 'clearing', objects }, other]);
  });
});
